// Copyright 2026 Ethan Wu
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// SPDX-License-Identifier: Apache-2.0

//! Rendering the Linux device tree from the lowered IR on top of a vendor base.
//!
//! Two output forms share one analysis pass:
//!
//! - **Merge** ([`render_merge`]): re-emits the parsed base tree and appends the
//!   UIO/`reserved-memory` nodes plus label overrides that strip a remoteproc
//!   node's `mboxes`/`mbox-names` and disable peripherals the base claimed that
//!   the UIO devices now own. This is the only form that can delete properties.
//! - **Overlay** ([`render_overlay`]): emits a `/plugin/;` `.dtso` with the
//!   `fragment@N { target…; __overlay__ { … }; }` form. It can add nodes and
//!   disable conflicts but cannot delete a remoteproc node's mailbox
//!   properties — [`render_overlay`] returns a warning when a fix is skipped.
//!
//! The analysis distinguishes the two node kinds a base can define: a plain
//! definition (`foo: bar { … }`, a [`consortium_dts`] root node) and a label
//! override (`&foo { … }`, a [`consortium_dts::NodeOverlay`]); a property such
//! as `mboxes` may live in either, so both are scanned.

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use consortium_cfg_common::config::DtsMode;
use consortium_dts::utils::edit;
use consortium_dts::{
    Cell, Deletion, File, Node, NodeOverlay, PropValue, Property, Reference, ToDts, TopLevelItem,
    parse_dts,
};

use crate::bridge::LoweredIr;
use crate::components::remoteproc::RemoteprocMemoryLayout;
use crate::components::uio::{RegLayout, UioBacking, reserved_memory_container};
use crate::error::CfgError;

/// A rendered Linux device tree plus any non-fatal warnings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderedDts {
    /// Artifact file name, `.dts` for merge mode or `.dtso` for overlay mode.
    pub file_name: String,
    /// The rendered device-tree text.
    pub contents: String,
    /// Warnings the caller should surface (e.g. an overlay-mode remoteproc fix
    /// that could not be expressed).
    pub warnings: Vec<String>,
    /// The exact BSP sources read from the supplied Linux checkout.
    pub source_closure: Vec<(String, String)>,
}

/// Resolves the base device tree for a chip, parses it and its local include
/// closure, and renders the Linux device tree for the lowered UIO layout.
///
/// Returns `Ok(None)` when the IR declares no UIO or remoteproc devices.
/// `target_cores` are the coprocessor core names the build targets, used to
/// scope remoteproc fixups.
///
/// # Errors
///
/// [`CfgError::Manifest`] if the selected source is absent from the supplied
/// kernel checkout, or [`CfgError::DtsParse`] if a source fails to parse.
pub fn render_linux_dts(
    chip: &str,
    devkit: Option<&str>,
    base_override: Option<&str>,
    mode: DtsMode,
    ir: &LoweredIr,
    target_cores: &[String],
    linux: &Path,
) -> Result<Option<RenderedDts>, CfgError> {
    if ir.uio.is_empty() && ir.remoteproc.is_empty() {
        consortium_log::trace!("no UIO or remoteproc devices in IR; skipping Linux DTS render");
        return Ok(None);
    }

    if mode.is_overlay() && !ir.remoteproc.is_empty() {
        return Err(CfgError::Manifest(
            "remoteproc deployment requires `[dts] mode = \"merge\"`; overlays cannot replace mailbox and memory-region properties".into(),
        ));
    }

    consortium_log::trace!("resolving base device tree for chip '{}'", chip);
    let base = resolve_kernel_base(linux, chip, devkit, base_override)?;
    let sources = kernel_include_closure(linux, base.vendor, &base.entry)?;

    let mut files = Vec::with_capacity(sources.len());
    for (name, contents) in &sources {
        consortium_log::trace!("parsing device-tree source '{}'", name);
        let file = parse_dts(contents).map_err(|e| {
            consortium_log::trace!("device-tree source '{}' failed to parse: {}", name, e);
            CfgError::DtsParse {
                file: name.clone(),
                detail: e.to_string(),
            }
        })?;
        files.push(file);
    }
    // The entry file is first in the closure; it is the tree we re-emit for merge
    // mode. Parse it again standalone rather than relying on ordering assumptions.
    let base_file = files.first().cloned().unwrap_or_default();

    let mut analysis = analyze(&files, ir, target_cores);
    analyze_remoteproc(&files, ir, &mut analysis)?;

    let rendered = if mode.is_overlay() {
        let (contents, warnings) = render_overlay(&analysis, ir);
        RenderedDts {
            file_name: overlay_file_name(&base),
            contents,
            warnings,
            source_closure: sources,
        }
    } else {
        RenderedDts {
            file_name: merge_file_name(&base),
            contents: render_merge(&base_file, &analysis, ir),
            warnings: Vec::new(),
            source_closure: sources,
        }
    };
    Ok(Some(rendered))
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BaseDts {
    vendor: &'static str,
    entry: String,
}

fn resolve_kernel_base(
    linux: &Path,
    chip: &str,
    devkit: Option<&str>,
    base_override: Option<&str>,
) -> Result<BaseDts, CfgError> {
    let normalized = chip.to_ascii_lowercase();
    let (vendor, generic) = if let Some(rest) = normalized.strip_prefix("mimx9") {
        let line = rest.chars().next().ok_or_else(|| {
            CfgError::Manifest(format!("cannot derive an i.MX DTS name from chip `{chip}`"))
        })?;
        ("freescale", format!("imx9{line}.dtsi"))
    } else if chip.to_ascii_lowercase().starts_with("stm32mp2") {
        let digits: String = normalized
            .chars()
            .skip("stm32mp".len())
            .take_while(char::is_ascii_digit)
            .collect();
        if digits.len() < 3 {
            return Err(CfgError::Manifest(format!(
                "cannot derive an STM32MP DTS name from chip `{chip}`"
            )));
        }
        ("st", format!("stm32mp{}.dtsi", &digits[..3]))
    } else {
        return Err(CfgError::Manifest(format!(
            "no Linux DTS directory mapping for chip `{chip}`"
        )));
    };
    let root = linux.join("arch/arm64/boot/dts").join(vendor);
    let Some(stem) = base_override.or(devkit) else {
        if !root.join(&generic).is_file() {
            return Err(CfgError::Manifest(format!(
                "Linux checkout has no generic DTS source `{}`",
                root.join(&generic).display()
            )));
        }
        return Ok(BaseDts {
            vendor,
            entry: generic,
        });
    };
    let stem = stem
        .strip_suffix(".dts")
        .or_else(|| stem.strip_suffix(".dtsi"))
        .unwrap_or(stem);
    let entry = [format!("{stem}.dts"), format!("{stem}.dtsi")]
        .into_iter()
        .find(|entry| root.join(entry).is_file())
        .ok_or_else(|| {
            CfgError::Manifest(format!(
                "configured kernel tree has no DTS source `{stem}.dts` or `{stem}.dtsi` under {}",
                root.display()
            ))
        })?;
    Ok(BaseDts { vendor, entry })
}

/// The merged `.dts` artifact name, derived from the base file stem.
fn merge_file_name(base: &BaseDts) -> String {
    format!("consortium-{}.dts", stem(&base.entry))
}

/// The overlay `.dtso` artifact name.
fn overlay_file_name(base: &BaseDts) -> String {
    format!("consortium-{}.dtso", stem(&base.entry))
}

/// The file stem (drops a trailing `.dts`/`.dtsi`).
fn stem(entry: &str) -> &str {
    entry
        .strip_suffix(".dtsi")
        .or_else(|| entry.strip_suffix(".dts"))
        .unwrap_or(entry)
}

fn kernel_include_closure(
    linux: &Path,
    vendor: &str,
    entry: &str,
) -> Result<Vec<(String, String)>, CfgError> {
    let root = linux.join("arch/arm64/boot/dts").join(vendor);
    let mut queue = vec![PathBuf::from(entry)];
    let mut seen = HashSet::new();
    let mut out = Vec::new();

    while let Some(relative) = queue.pop() {
        let relative = normalize_relative(&relative)?;
        let name = relative.to_string_lossy().replace('\\', "/");
        if !seen.insert(name.clone()) {
            continue;
        }
        let path = root.join(&relative);
        let contents = fs::read_to_string(&path).map_err(|error| {
            CfgError::Manifest(format!(
                "configured kernel tree lacks required DTS source `{}`: {error}",
                path.display()
            ))
        })?;
        let parent = relative.parent().unwrap_or_else(|| Path::new(""));
        let mut includes: Vec<PathBuf> = quoted_includes(&contents)
            .into_iter()
            .map(|include| parent.join(include))
            .collect();
        includes.reverse();
        queue.extend(includes);
        out.push((name, contents));
    }
    Ok(out)
}

fn quoted_includes(source: &str) -> Vec<&str> {
    source
        .lines()
        .filter_map(|line| {
            let rest = line.trim().strip_prefix("#include")?.trim();
            let rest = rest.strip_prefix('"')?;
            Some(rest.split_once('"')?.0)
        })
        .collect()
}

fn normalize_relative(path: &Path) -> Result<PathBuf, CfgError> {
    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            std::path::Component::Normal(part) => out.push(part),
            std::path::Component::CurDir => {}
            std::path::Component::ParentDir => {
                if !out.pop() {
                    return Err(CfgError::Manifest(format!(
                        "DTS include `{}` escapes the vendor source directory",
                        path.display()
                    )));
                }
            }
            _ => {
                return Err(CfgError::Manifest(format!(
                    "DTS include `{}` is not a relative path",
                    path.display()
                )));
            }
        }
    }
    Ok(out)
}

/// `compatible` substrings that mark a node as a Cortex-M remoteproc core.
const RPROC_MARKERS: &[&str] = &["-cm33", "-cm7", "-cm4", "cortex-m"];

/// A remoteproc node that must be fixed up so the kernel remoteproc driver can
/// load it: its mailbox wiring is stripped and it is enabled.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RprocFix {
    /// The node's label (e.g. `cm33`), used as the override target.
    pub label: String,
    /// Whether the node (or an override of it) carries `mboxes`.
    pub delete_mboxes: bool,
    /// Whether the node (or an override of it) carries `mbox-names`.
    pub delete_mbox_names: bool,
    /// Whether the BSP supplies an old firmware carveout list.
    pub delete_memory_region: bool,
    /// Resolved existing or generated firmware reservations.
    pub memory_regions: Vec<Reference>,
}

/// A new reserved-memory node required by a remoteproc layout.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RprocCarveout {
    pub label: String,
    pub name: String,
    pub base: u64,
    pub length: u64,
}

/// The result of analyzing a base tree against the lowered UIO layout.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Analysis {
    /// Labels of peripherals the base declared that a UIO device now owns and
    /// that must therefore be disabled (in declaration order, de-duplicated).
    pub conflicts: Vec<String>,
    /// Remoteproc nodes to fix up.
    pub rprocs: Vec<RprocFix>,
    /// Firmware carveouts not already supplied by the BSP.
    pub carveouts: Vec<RprocCarveout>,
}

/// Analyzes the parsed base tree (`files` = the entry plus its local include
/// closure) against the lowered UIO layout.
///
/// `target_cores` are the coprocessor core names the build targets (e.g.
/// `["cm33"]`); only remoteproc nodes matching one of them are fixed up, so a
/// chip's unused coprocessor is left untouched.
pub fn analyze(files: &[File], ir: &LoweredIr, target_cores: &[String]) -> Analysis {
    let mut conflicts = Vec::new();
    for name in doorbell_labels(ir) {
        if !conflicts.contains(&name) && files.iter().any(|f| edit::label_exists(f, &name)) {
            conflicts.push(name);
        }
    }

    let mut rprocs: Vec<RprocFix> = Vec::new();
    for file in files {
        edit::for_each_node(file, |node| {
            let Some(label) = node.label.as_deref() else {
                return;
            };
            let is_rproc = RPROC_MARKERS.iter().any(|m| node.compatible_contains(m));
            let targets_our_core = target_cores
                .iter()
                .any(|core| label.contains(core.as_str()) || node.name.contains(core.as_str()));
            if is_rproc && targets_our_core && !rprocs.iter().any(|r| r.label == label) {
                rprocs.push(RprocFix {
                    label: label.to_string(),
                    delete_mboxes: label_has_property(files, label, "mboxes"),
                    delete_mbox_names: label_has_property(files, label, "mbox-names"),
                    delete_memory_region: false,
                    memory_regions: Vec::new(),
                });
            }
        });
    }

    Analysis {
        conflicts,
        rprocs,
        carveouts: Vec::new(),
    }
}

fn analyze_remoteproc(
    files: &[File],
    ir: &LoweredIr,
    analysis: &mut Analysis,
) -> Result<(), CfgError> {
    if ir.remoteproc.is_empty() {
        return Ok(());
    }
    analysis.rprocs.clear();

    let tee_only = files.iter().any(|file| {
        let mut found = false;
        edit::for_each_node(file, |node| {
            found |= node.compatible().contains(&"st,stm32mp2-m33-tee");
        });
        found
            || edit::overlays(file)
                .any(|overlay| overlay.compatible().contains(&"st,stm32mp2-m33-tee"))
    });
    let existing = existing_reserved_ranges(files);

    let mut layouts: Vec<&RemoteprocMemoryLayout> = ir.remoteproc.values().collect();
    layouts.sort_by_key(|layout| layout.core.to_string());
    for layout in layouts {
        let label = match find_compatible_label(files, &layout.compatible) {
            Some(label) => label,
            None if layout.compatible == "st,stm32mp2-m33" && tee_only => {
                return Err(CfgError::Manifest(
                    "STM32MP remoteproc supports only the non-signed `st,stm32mp2-m33` ELF path; the selected DTS composes the TEE-only remoteproc".into(),
                ));
            }
            None => {
                return Err(CfgError::Manifest(format!(
                    "selected BSP source lacks the required remoteproc node compatible `{}` for core `{}`",
                    layout.compatible, layout.core
                )));
            }
        };

        let desired = desired_reservations(layout)?;
        let mut refs = Vec::new();
        for (suffix, base, length) in desired {
            let end = base.checked_add(length).ok_or_else(|| {
                CfgError::Manifest(format!(
                    "remoteproc carveout for `{}` overflows",
                    layout.core
                ))
            })?;
            let mut covering = None;
            for range in &existing {
                let range_end = range.base + range.length;
                if range.base <= base && end <= range_end {
                    covering = Some(range.reference.clone());
                    break;
                }
                if base < range_end && range.base < end {
                    return Err(CfgError::Manifest(format!(
                        "remoteproc `{}` reservation {base:#x}..{end:#x} partially overlaps BSP reserved-memory {} ({:#x}..{:#x})",
                        layout.core, range.path, range.base, range_end
                    )));
                }
            }
            let reference = if let Some(reference) = covering {
                reference
            } else {
                let core = layout.core.to_string();
                let label = format!("consortium_{core}_{suffix}");
                analysis.carveouts.push(RprocCarveout {
                    label: label.clone(),
                    name: format!("{core}-firmware-{suffix}"),
                    base,
                    length,
                });
                Reference::label(label)
            };
            if !refs.contains(&reference) {
                refs.push(reference);
            }
        }

        analysis.rprocs.push(RprocFix {
            delete_mboxes: label_has_property(files, &label, "mboxes"),
            delete_mbox_names: label_has_property(files, &label, "mbox-names"),
            delete_memory_region: label_has_property(files, &label, "memory-region"),
            label,
            memory_regions: refs,
        });
    }
    Ok(())
}

fn find_compatible_label(files: &[File], compatible: &str) -> Option<String> {
    let mut found = None;
    for file in files {
        edit::for_each_node(file, |node| {
            if found.is_none() && node.compatible().contains(&compatible) {
                found = node.label.clone();
            }
        });
        if found.is_none() {
            found = edit::overlays(file).find_map(|overlay| {
                overlay
                    .compatible()
                    .contains(&compatible)
                    .then(|| overlay.target_label().map(str::to_owned))
                    .flatten()
            });
        }
    }
    found
}

fn desired_reservations(
    layout: &RemoteprocMemoryLayout,
) -> Result<Vec<(String, u64, u64)>, CfgError> {
    let mut out = Vec::new();
    if layout.vector.window.reserve {
        out.push((
            "vector".into(),
            layout.vector.window.origin,
            layout.vector.window.length,
        ));
    }
    if layout.flash.reserve {
        out.push(("flash".into(), layout.flash.origin, layout.flash.length));
    }
    if layout.shmem.window.reserve {
        out.push((
            "shmem".into(),
            layout.shmem.window.origin,
            layout.shmem.window.length,
        ));
    }
    if layout.ram.reserve {
        out.push(("ram".into(), layout.ram.origin, layout.ram.length));
    }
    out.extend(
        layout
            .regions
            .iter()
            .filter(|region| region.window.reserve)
            .map(|region| {
                (
                    region.name.to_ascii_lowercase(),
                    region.window.origin,
                    region.window.length,
                )
            }),
    );
    if out.is_empty() {
        return Err(CfgError::Manifest(format!(
            "remoteproc `{}` has no reservable firmware memory window",
            layout.core
        )));
    }
    Ok(out)
}

#[derive(Debug)]
struct ExistingReservation {
    base: u64,
    length: u64,
    path: String,
    reference: Reference,
}

fn existing_reserved_ranges(files: &[File]) -> Vec<ExistingReservation> {
    let mut out = Vec::new();
    for file in files {
        for item in &file.items {
            let TopLevelItem::Root(root) = item else {
                continue;
            };
            for reserved in root
                .children
                .iter()
                .filter(|node| node.name == "reserved-memory")
            {
                for node in &reserved.children {
                    let Some((base, length)) = node_reg(node) else {
                        continue;
                    };
                    let node_name = match &node.unit_address {
                        Some(unit) => format!("{}@{unit}", node.name),
                        None => node.name.clone(),
                    };
                    let path = format!("/reserved-memory/{node_name}");
                    let reference = node
                        .label
                        .clone()
                        .map(Reference::label)
                        .unwrap_or_else(|| Reference::path(path.clone()));
                    out.push(ExistingReservation {
                        base,
                        length,
                        path,
                        reference,
                    });
                }
            }
        }
    }
    out
}

fn node_reg(node: &Node) -> Option<(u64, u64)> {
    let PropValue::Cells(cells) = node.get_property("reg")?.value.as_ref()? else {
        return None;
    };
    let numbers: Vec<u64> = cells
        .iter()
        .filter_map(|cell| match cell {
            Cell::Number(value) => Some(*value),
            _ => None,
        })
        .collect();
    match numbers.as_slice() {
        [base, length] => Some((*base, *length)),
        [base_hi, base_lo, length_hi, length_lo] => {
            Some(((base_hi << 32) | base_lo, (length_hi << 32) | length_lo))
        }
        _ => None,
    }
}

/// Renders the merged full board `.dts`: the parsed base with the UIO layout and
/// the analysis-driven label overrides appended.
pub fn render_merge(base: &File, analysis: &Analysis, ir: &LoweredIr) -> String {
    let mut file = base.clone();
    // A merged tree is a full device tree; ensure the `/dts-v1/;` tag even when
    // the base was a `.dtsi` include.
    file.version = true;
    file.plugin = false;

    if let Some(root) = ir_root_node(ir, analysis) {
        file.items.push(root.into());
    }

    for label in &analysis.conflicts {
        file.items.push(
            NodeOverlay::label(label)
                .property(Property::string("status", "disabled"))
                .into(),
        );
    }

    for fix in &analysis.rprocs {
        file.items.push(rproc_override(fix).into());
    }

    file.to_dts()
}

/// Renders the `/plugin/;` overlay `.dtso` for the lowered UIO layout and the
/// analysis-driven conflict disables.
///
/// Returns the overlay text and a list of human-readable warnings for fixes the
/// overlay form cannot express (remoteproc mailbox stripping); the caller
/// surfaces them so the user can switch to [`render_merge`].
pub fn render_overlay(analysis: &Analysis, ir: &LoweredIr) -> (String, Vec<String>) {
    let mut root = Node::root();
    let mut fragment = 0u64;

    if let Some(payload) = ir_root_node(ir, analysis) {
        // fragment@0 targets `/` and carries the reserved-memory + UIO nodes.
        let overlay = Node::new("__overlay__").with_children(payload.children);
        root = root.child(
            Node::with_unit_address("fragment", fragment.to_string())
                .property(Property::string("target-path", "/"))
                .child(overlay),
        );
        fragment += 1;
    }

    for label in &analysis.conflicts {
        let overlay = Node::new("__overlay__").property(Property::string("status", "disabled"));
        root = root.child(
            Node::with_unit_address("fragment", fragment.to_string())
                .property(Property::cells(
                    "target",
                    [Cell::reference(Reference::label(label.clone()))],
                ))
                .child(overlay),
        );
        fragment += 1;
    }

    let warnings = analysis
        .rprocs
        .iter()
        .filter(|r| r.delete_mboxes || r.delete_mbox_names)
        .map(|r| {
            format!(
                "remoteproc node `&{}` needs its `mboxes`/`mbox-names` removed, but an overlay \
                 cannot delete properties; use `[dts] mode = \"merge\"` so remoteproc loads",
                r.label
            )
        })
        .collect();

    (File::dtso().item(root).to_dts(), warnings)
}

/// Builds the label override that strips a remoteproc node's mailbox wiring and
/// enables it.
fn rproc_override(fix: &RprocFix) -> NodeOverlay {
    let mut overlay = NodeOverlay::label(fix.label.clone());
    if fix.delete_mboxes {
        overlay = overlay.deletion(Deletion::Property("mboxes".to_string()));
    }
    if fix.delete_mbox_names {
        overlay = overlay.deletion(Deletion::Property("mbox-names".to_string()));
    }
    if fix.delete_memory_region {
        overlay = overlay.deletion(Deletion::Property("memory-region".to_string()));
    }
    if !fix.memory_regions.is_empty() {
        overlay = overlay.property(Property::cells(
            "memory-region",
            fix.memory_regions.iter().cloned().map(Cell::reference),
        ));
    }
    overlay.status_okay()
}

/// The `/` root node holding the aggregate `reserved-memory` container and one
/// `generic-uio` node per lowered device, or `None` when the IR declares none.
fn ir_root_node(ir: &LoweredIr, analysis: &Analysis) -> Option<Node> {
    if ir.uio.is_empty() && analysis.carveouts.is_empty() {
        return None;
    }
    let mut root = Node::root();

    let mut carveouts: Vec<Node> = ir
        .uio
        .iter()
        .flat_map(|(name, device)| device.reserved_memory_nodes(name.as_str()))
        .collect();
    carveouts.extend(analysis.carveouts.iter().map(remoteproc_carveout_node));
    if let Some(container) = reserved_memory_container(carveouts, RegLayout::Wide) {
        root = root.child(container);
    }

    for (name, device) in &ir.uio {
        root = root.child(device.to_node(name.as_str()));
    }
    Some(root)
}

fn remoteproc_carveout_node(carveout: &RprocCarveout) -> Node {
    Node::with_unit_address(&carveout.name, format!("{:x}", carveout.base))
        .label(&carveout.label)
        .property(Property::cells(
            "reg",
            [
                Cell::number(carveout.base >> 32),
                Cell::number(carveout.base & u32::MAX as u64),
                Cell::number(carveout.length >> 32),
                Cell::number(carveout.length & u32::MAX as u64),
            ],
        ))
        .property(Property::boolean("no-map"))
}

/// The labels of the register-backed (doorbell) UIO devices — the peripherals
/// whose base nodes a merged/overlaid tree must disable.
fn doorbell_labels(ir: &LoweredIr) -> Vec<String> {
    ir.uio
        .iter()
        .filter(|(_, device)| device.backing == UioBacking::Registers)
        .map(|(name, _)| name.clone())
        .collect()
}

/// Adds several children to a node in one call.
trait WithChildren {
    fn with_children(self, children: impl IntoIterator<Item = Node>) -> Self;
}

impl WithChildren for Node {
    fn with_children(mut self, children: impl IntoIterator<Item = Node>) -> Self {
        for child in children {
            self = self.child(child);
        }
        self
    }
}

/// Whether a label carries a property, scanning both plain definitions (root
/// nodes) and label overrides across every file.
fn label_has_property(files: &[File], label: &str, property: &str) -> bool {
    files.iter().any(|file| {
        edit::find_by_label(file, label).is_some_and(|n| n.has_property(property))
            || edit::overlays(file)
                .any(|o| o.target_label() == Some(label) && o.has_property(property))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use consortium_dts::parse_dts;

    use crate::components::uio::{UioBacking, UioDevice};

    fn test_kernel_tree(vendor: &str, file: &str, contents: &str) -> PathBuf {
        let unique = format!(
            "consortium-dts-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        );
        let root = std::env::temp_dir().join(unique);
        let dir = root.join("arch/arm64/boot/dts").join(vendor);
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join(file), contents).unwrap();
        root
    }

    fn ir_with(uio: Vec<(String, UioDevice)>) -> LoweredIr {
        // Only `uio` is read by this module; the rest is left empty via a helper
        // the bridge exposes for tests.
        LoweredIr::empty_with_uio(uio)
    }

    fn shm(name: &str, base: u32, len: u32) -> (String, UioDevice) {
        (
            name.to_string(),
            UioDevice {
                mmap: vec![(base, len)],
                irq: None,
                backing: UioBacking::ReservedMemory,
                dma_coherent: Vec::new(),
                interrupt_parent: None,
                clock: None,
            },
        )
    }

    fn doorbell(name: &str, base: u32) -> (String, UioDevice) {
        (
            name.to_string(),
            UioDevice {
                mmap: vec![(base, 0x1000)],
                irq: None,
                backing: UioBacking::Registers,
                dma_coherent: Vec::new(),
                interrupt_parent: None,
                clock: None,
            },
        )
    }

    fn remoteproc_ir() -> LoweredIr {
        let mut ir = ir_with(Vec::new());
        let core = "cm7".parse().unwrap();
        ir.remoteproc.insert(
            core,
            RemoteprocMemoryLayout {
                core,
                compatible: "fsl,imx95-cm7".into(),
                vector_section: ".interrupts".into(),
                vector: crate::components::remoteproc::RemoteprocNamedRegion {
                    name: "m_interrupts".into(),
                    attributes: crate::components::remoteproc::LinkerRegionAttributes::new(
                        true, false, true,
                    ),
                    window: crate::components::remoteproc::RemoteprocMemoryWindow {
                        origin: 0x8000_0000,
                        length: 0x1000,
                        reserve: true,
                    },
                    sections: Vec::new(),
                },
                flash: crate::components::remoteproc::RemoteprocMemoryWindow {
                    origin: 0x8000_1000,
                    length: 0x007f_f000,
                    reserve: true,
                },
                shmem: crate::components::remoteproc::RemoteprocNamedRegion {
                    name: "m_rsc_tbl_in_dram".into(),
                    attributes: crate::components::remoteproc::LinkerRegionAttributes::new(
                        true, true, false,
                    ),
                    window: crate::components::remoteproc::RemoteprocMemoryWindow {
                        origin: 0x8822_0000,
                        length: 0x4000,
                        reserve: true,
                    },
                    sections: Vec::new(),
                },
                ram: crate::components::remoteproc::RemoteprocMemoryWindow {
                    origin: 0x8080_0000,
                    length: 0x0080_0000,
                    reserve: true,
                },
                regions: Vec::new(),
            },
        );
        ir
    }

    const BASE: &str = r#"/dts-v1/;
#include "soc.dtsi"
/ {
    model = "Board";
    soc {
        mu7: mailbox@42430000 {
            compatible = "fsl,imx95-mu-v3.0";
            status = "okay";
        };
        cm33: remoteproc-cm33 {
            compatible = "fsl,imx93-cm33";
            status = "disabled";
        };
    };
};
&cm33 {
    mboxes = <&mu1 0 1>;
    mbox-names = "tx", "rx";
    status = "okay";
};
"#;

    #[test]
    fn analysis_finds_conflict_and_rproc() {
        let base = parse_dts(BASE).unwrap();
        let ir = ir_with(vec![
            shm("ipc-shm", 0x2040_0000, 0x8000),
            doorbell("mu7", 0x4243_0000),
        ]);
        let analysis = analyze(std::slice::from_ref(&base), &ir, &["cm33".to_string()]);
        assert_eq!(analysis.conflicts, vec!["mu7".to_string()]);
        assert_eq!(analysis.rprocs.len(), 1);
        let fix = &analysis.rprocs[0];
        assert_eq!(fix.label, "cm33");
        assert!(fix.delete_mboxes);
        assert!(fix.delete_mbox_names);
    }

    #[test]
    fn merge_appends_overrides_and_uio() {
        let base = parse_dts(BASE).unwrap();
        let ir = ir_with(vec![
            shm("ipc-shm", 0x2040_0000, 0x8000),
            doorbell("mu7", 0x4243_0000),
        ]);
        let analysis = analyze(std::slice::from_ref(&base), &ir, &["cm33".to_string()]);
        let dts = render_merge(&base, &analysis, &ir);

        // Base preserved.
        assert!(dts.contains("model = \"Board\""));
        assert!(dts.contains("#include \"soc.dtsi\""));
        // UIO + reserved-memory injected.
        assert!(dts.contains("compatible = \"generic-uio\""));
        assert!(dts.contains("reserved-memory {"));
        // Conflict disabled and remoteproc fixed.
        assert!(dts.contains("&mu7 {"));
        assert!(dts.contains("status = \"disabled\""));
        assert!(dts.contains("&cm33 {"));
        assert!(dts.contains("/delete-property/ mboxes;"));
        assert!(dts.contains("/delete-property/ mbox-names;"));
    }

    #[test]
    fn overlay_uses_fragment_form_and_warns_on_rproc() {
        let base = parse_dts(BASE).unwrap();
        let ir = ir_with(vec![
            shm("ipc-shm", 0x2040_0000, 0x8000),
            doorbell("mu7", 0x4243_0000),
        ]);
        let analysis = analyze(std::slice::from_ref(&base), &ir, &["cm33".to_string()]);
        let (dtso, warnings) = render_overlay(&analysis, &ir);

        assert!(dtso.contains("/plugin/;"));
        assert!(dtso.contains("fragment@0 {"));
        assert!(dtso.contains("target-path = \"/\""));
        assert!(dtso.contains("__overlay__ {"));
        assert!(dtso.contains("fragment@1 {"));
        assert!(dtso.contains("target = <&mu7>"));
        assert!(dtso.contains("status = \"disabled\""));
        // Overlay cannot strip mboxes; the user is warned.
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains("&cm33"));
    }

    #[test]
    fn merge_reads_imx_base_from_the_supplied_kernel_checkout() {
        let root = test_kernel_tree(
            "freescale",
            "imx95-test-board.dts",
            r#"/dts-v1/;
            / {
                model = "Kernel i.MX board";
                mu7: mailbox@42430000 { compatible = "fsl,imx95-mu"; };
            };"#,
        );
        let ir = ir_with(vec![
            shm("ipc-shm", 0x2040_0000, 0x8000),
            doorbell("mu7", 0x4243_0000),
        ]);
        let rendered = render_linux_dts(
            "mimx9596xvt",
            Some("imx95-test-board"),
            None,
            DtsMode::Merge,
            &ir,
            &["cm7".to_string()],
            &root,
        )
        .expect("render succeeds")
        .expect("uio present");

        assert_eq!(rendered.file_name, "consortium-imx95-test-board.dts");
        assert!(rendered.contents.contains("Kernel i.MX board"));
        assert!(rendered.contents.contains("compatible = \"generic-uio\""));
        assert!(rendered.contents.contains("&mu7 {"));
        assert!(rendered.warnings.is_empty());
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn overlay_reads_stm32_base_from_the_supplied_kernel_checkout() {
        let root = test_kernel_tree(
            "st",
            "stm32mp257-test-board.dts",
            r#"/dts-v1/; / { model = "Kernel STM32 board"; };"#,
        );
        let ir = ir_with(vec![shm("ipc-shm", 0x0a02_0000, 0x8000)]);
        let rendered = render_linux_dts(
            "stm32mp257fak3",
            Some("stm32mp257-test-board"),
            None,
            DtsMode::Overlay,
            &ir,
            &["cm33".to_string()],
            &root,
        )
        .expect("render succeeds")
        .expect("uio present");

        assert_eq!(rendered.file_name, "consortium-stm32mp257-test-board.dtso");
        assert!(rendered.contents.contains("/plugin/;"));
        assert!(rendered.contents.contains("fragment@0"));
        assert!(rendered.contents.contains("target-path = \"/\""));
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn unused_coprocessor_is_not_touched() {
        let base = parse_dts(BASE).unwrap();
        let ir = ir_with(vec![shm("ipc-shm", 0x2040_0000, 0x8000)]);
        // Target a core the base has no node for.
        let analysis = analyze(std::slice::from_ref(&base), &ir, &["cm7".to_string()]);
        assert!(analysis.rprocs.is_empty());
    }

    #[test]
    fn remoteproc_generates_direct_imx_regions_and_replaces_memory_region() {
        let base = parse_dts(
            r#"/dts-v1/;
            / {
                cm7: imx95-cm7 {
                    compatible = "fsl,imx95-cm7";
                    mboxes = <&mu7 0>;
                    memory-region = <&old_fw>;
                    fsl,cpu-id = <1>;
                    fsl,lmm-id = <1>;
                    fsl,startup-delay-ms = <50>;
                };
            };"#,
        )
        .unwrap();
        let ir = remoteproc_ir();
        let mut analysis = analyze(std::slice::from_ref(&base), &ir, &["cm7".into()]);
        analyze_remoteproc(std::slice::from_ref(&base), &ir, &mut analysis).unwrap();
        let dts = render_merge(&base, &analysis, &ir);

        assert!(dts.contains("consortium_cm7_vector: cm7-firmware-vector@80000000"));
        assert!(dts.contains("consortium_cm7_flash: cm7-firmware-flash@80001000"));
        assert!(dts.contains("consortium_cm7_ram: cm7-firmware-ram@80800000"));
        assert!(dts.contains("consortium_cm7_shmem: cm7-firmware-shmem@88220000"));
        assert!(dts.contains("no-map;"));
        assert!(dts.contains("/delete-property/ mboxes;"));
        assert!(dts.contains("/delete-property/ memory-region;"));
        assert!(dts.contains(
            "memory-region = <&consortium_cm7_vector &consortium_cm7_flash &consortium_cm7_shmem &consortium_cm7_ram>"
        ));
        assert!(dts.contains("fsl,cpu-id = <1>"));
    }

    #[test]
    fn manifest_origin_override_drives_both_linker_and_dts() {
        let ir = crate::Session::from_toml(
            r#"
                [profile]
                chip = "mimx9596xvt"
                shared = "./shared"
                deploy_root = "/opt/consortium"

                [endpoints]
                cm7 = "./mcu"

                [remoteproc.cm7]
                shmem = { origin = 0x90000000, length = 0x00008000 }
            "#,
        )
        .unwrap()
        .lower()
        .unwrap();
        let layout = &ir.remoteproc[&"cm7".parse().unwrap()];
        assert!(
            layout
                .linker_memory()
                .unwrap()
                .render()
                .contains("m_rsc_tbl_in_dram (rw) : ORIGIN = 0x90000000, LENGTH = 0x8000")
        );

        let base = parse_dts(
            r#"/dts-v1/;
            / { cm7: imx95-cm7 { compatible = "fsl,imx95-cm7"; }; };"#,
        )
        .unwrap();
        let mut analysis = Analysis::default();
        analyze_remoteproc(std::slice::from_ref(&base), &ir, &mut analysis).unwrap();
        assert!(
            analysis
                .carveouts
                .iter()
                .any(|carveout| carveout.name == "cm7-firmware-shmem"
                    && carveout.base == 0x9000_0000)
        );
    }

    #[test]
    fn remoteproc_reuses_covering_carveout_and_rejects_partial_overlap() {
        let source = |length| {
            parse_dts(&format!(
                r#"/dts-v1/;
                / {{
                    reserved-memory {{
                        #address-cells = <2>;
                        #size-cells = <2>;
                        fw@80000000 {{ reg = <0 0x80000000 0 {length:#x}>; no-map; }};
                    }};
                    cm7: imx95-cm7 {{ compatible = "fsl,imx95-cm7"; }};
                }};"#
            ))
            .unwrap()
        };
        let ir = remoteproc_ir();

        let covering = source(0x0200_0000);
        let mut analysis = Analysis::default();
        analyze_remoteproc(std::slice::from_ref(&covering), &ir, &mut analysis).unwrap();
        assert_eq!(analysis.carveouts.len(), 1);
        assert_eq!(analysis.carveouts[0].label, "consortium_cm7_shmem");
        assert_eq!(
            analysis.rprocs[0].memory_regions,
            [
                Reference::path("/reserved-memory/fw@80000000"),
                Reference::label("consortium_cm7_shmem"),
            ]
        );

        let partial = source(0x00c0_0000);
        let error = analyze_remoteproc(
            std::slice::from_ref(&partial),
            &ir,
            &mut Analysis::default(),
        )
        .unwrap_err();
        assert!(error.to_string().contains("partially overlaps"));
    }

    #[test]
    fn remoteproc_rejects_overlay_and_missing_bsp_node() {
        let ir = remoteproc_ir();
        let overlay = render_linux_dts(
            "mimx9596xvt",
            Some("imx95-19x19-evk"),
            None,
            DtsMode::Overlay,
            &ir,
            &["cm7".into()],
            Path::new("/unused"),
        )
        .unwrap_err();
        assert!(overlay.to_string().contains("requires `[dts] mode"));

        let missing = parse_dts("/dts-v1/; / { model = \"missing\"; };").unwrap();
        let error = analyze_remoteproc(
            std::slice::from_ref(&missing),
            &ir,
            &mut Analysis::default(),
        )
        .unwrap_err();
        assert!(
            error
                .to_string()
                .contains("lacks the required remoteproc node")
        );
    }

    #[test]
    fn configured_kernel_tree_is_authoritative() {
        let unique = format!(
            "consortium-dts-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        );
        let root = std::env::temp_dir().join(unique);
        let vendor = root.join("arch/arm64/boot/dts/freescale");
        fs::create_dir_all(&vendor).unwrap();
        fs::write(
            vendor.join("kernel-only-board.dts"),
            r#"/dts-v1/;
            / {
                model = "Kernel tree wins";
                cm7: imx95-cm7 { compatible = "fsl,imx95-cm7"; };
            };"#,
        )
        .unwrap();

        let rendered = render_linux_dts(
            "mimx9596xvt",
            None,
            Some("kernel-only-board"),
            DtsMode::Merge,
            &remoteproc_ir(),
            &["cm7".into()],
            &root,
        )
        .unwrap()
        .unwrap();
        assert!(rendered.contents.contains("Kernel tree wins"));
        assert_eq!(rendered.source_closure[0].0, "kernel-only-board.dts");

        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn stm32_tee_only_remoteproc_is_rejected() {
        let mut ir = remoteproc_ir();
        let mut layout = ir.remoteproc.remove(&"cm7".parse().unwrap()).unwrap();
        layout.core = "cm33".parse().unwrap();
        layout.compatible = "st,stm32mp2-m33".into();
        ir.remoteproc.insert(layout.core, layout);
        let tee = parse_dts(
            r#"/dts-v1/;
            / { m33_tee: m33@0 { compatible = "st,stm32mp2-m33-tee"; }; };"#,
        )
        .unwrap();
        let error = analyze_remoteproc(std::slice::from_ref(&tee), &ir, &mut Analysis::default())
            .unwrap_err();
        assert!(error.to_string().contains("non-signed"));
    }

    #[test]
    fn missing_selected_source_in_linux_checkout_is_rejected() {
        let root = test_kernel_tree("freescale", "another-board.dts", r#"/dts-v1/; / { };"#);
        let error = render_linux_dts(
            "mimx9596xvt",
            Some("missing-board"),
            None,
            DtsMode::Merge,
            &remoteproc_ir(),
            &["cm7".into()],
            &root,
        )
        .unwrap_err();
        assert!(error.to_string().contains("has no DTS source"));
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn mandatory_shmem_feeds_dts_as_a_direct_window() {
        let ir = crate::Session::from_toml(include_str!(
            "../../../../examples/melt-pot/stm32mp25/Consortium.toml"
        ))
        .unwrap()
        .lower()
        .unwrap();
        let base = parse_dts(
            r#"/dts-v1/;
            / { m33: m33@0 { compatible = "st,stm32mp2-m33"; }; };"#,
        )
        .unwrap();
        let mut analysis = Analysis::default();
        analyze_remoteproc(std::slice::from_ref(&base), &ir, &mut analysis).unwrap();
        let labels: Vec<_> = analysis
            .carveouts
            .iter()
            .map(|carveout| carveout.label.as_str())
            .collect();
        assert!(labels.contains(&"consortium_cm33_shmem"));
        assert_eq!(analysis.rprocs[0].memory_regions.len(), 4);
    }
}
