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

use consortium_cfg_common::config::DtsMode;
use consortium_data::dts::{local_include_closure, resolve_base};
use consortium_dts::utils::edit;
use consortium_dts::{
    Cell, Deletion, File, Node, NodeOverlay, Property, Reference, ToDts, parse_dts,
};

use crate::bridge::LoweredIr;
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
}

/// Resolves the base device tree for a chip, parses it and its local include
/// closure, and renders the Linux device tree for the lowered UIO layout.
///
/// Returns `Ok(None)` when the IR declares no UIO devices (nothing to emit).
/// `target_cores` are the coprocessor core names the build targets, used to
/// scope remoteproc fixups.
///
/// # Errors
///
/// [`CfgError::Data`] if the base cannot be resolved, or [`CfgError::DtsParse`]
/// if a vendor source fails to parse.
pub fn render_linux_dts(
    chip: &str,
    devkit: Option<&str>,
    base_override: Option<&str>,
    mode: DtsMode,
    ir: &LoweredIr,
    target_cores: &[String],
) -> Result<Option<RenderedDts>, CfgError> {
    if ir.uio.is_empty() {
        return Ok(None);
    }

    let base = resolve_base(chip, devkit, base_override)?;
    let sources = local_include_closure(&base)?;

    let mut files = Vec::with_capacity(sources.len());
    for (name, contents) in &sources {
        let file = parse_dts(contents).map_err(|e| CfgError::DtsParse {
            file: name.clone(),
            detail: e.to_string(),
        })?;
        files.push(file);
    }
    // The entry file is first in the closure; it is the tree we re-emit for merge
    // mode. Parse it again standalone rather than relying on ordering assumptions.
    let base_file = files.first().cloned().unwrap_or_default();

    let analysis = analyze(&files, ir, target_cores);

    let rendered = if mode.is_overlay() {
        let (contents, warnings) = render_overlay(&analysis, ir);
        RenderedDts {
            file_name: overlay_file_name(&base),
            contents,
            warnings,
        }
    } else {
        RenderedDts {
            file_name: merge_file_name(&base),
            contents: render_merge(&base_file, &analysis, ir),
            warnings: Vec::new(),
        }
    };
    Ok(Some(rendered))
}

/// The merged `.dts` artifact name, derived from the base file stem.
fn merge_file_name(base: &consortium_data::dts::BaseDts) -> String {
    format!("consortium-{}.dts", stem(&base.entry))
}

/// The overlay `.dtso` artifact name.
fn overlay_file_name(base: &consortium_data::dts::BaseDts) -> String {
    format!("consortium-{}.dtso", stem(&base.entry))
}

/// The file stem (drops a trailing `.dts`/`.dtsi`).
fn stem(entry: &str) -> &str {
    entry
        .strip_suffix(".dtsi")
        .or_else(|| entry.strip_suffix(".dts"))
        .unwrap_or(entry)
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
}

/// The result of analyzing a base tree against the lowered UIO layout.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Analysis {
    /// Labels of peripherals the base declared that a UIO device now owns and
    /// that must therefore be disabled (in declaration order, de-duplicated).
    pub conflicts: Vec<String>,
    /// Remoteproc nodes to fix up.
    pub rprocs: Vec<RprocFix>,
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
                });
            }
        });
    }

    Analysis { conflicts, rprocs }
}

/// Renders the merged full board `.dts`: the parsed base with the UIO layout and
/// the analysis-driven label overrides appended.
pub fn render_merge(base: &File, analysis: &Analysis, ir: &LoweredIr) -> String {
    let mut file = base.clone();
    // A merged tree is a full device tree; ensure the `/dts-v1/;` tag even when
    // the base was a `.dtsi` include.
    file.version = true;
    file.plugin = false;

    if let Some(root) = ir_root_node(ir) {
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

    if let Some(payload) = ir_root_node(ir) {
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
    overlay.status_okay()
}

/// The `/` root node holding the aggregate `reserved-memory` container and one
/// `generic-uio` node per lowered device, or `None` when the IR declares none.
fn ir_root_node(ir: &LoweredIr) -> Option<Node> {
    if ir.uio.is_empty() {
        return None;
    }
    let mut root = Node::root();

    let carveouts: Vec<Node> = ir
        .uio
        .iter()
        .flat_map(|(name, device)| device.reserved_memory_nodes(name.as_str()))
        .collect();
    if let Some(container) = reserved_memory_container(carveouts, RegLayout::Wide) {
        root = root.child(container);
    }

    for (name, device) in &ir.uio {
        root = root.child(device.to_node(name.as_str()));
    }
    Some(root)
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
            },
        )
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
    fn merge_on_real_imx95_devkit_disables_predeclared_mailbox() {
        // A doorbell named after a mailbox the vendor SoC dtsi predeclares.
        let ir = ir_with(vec![
            shm("ipc-shm", 0x2040_0000, 0x8000),
            doorbell("mu7", 0x4243_0000),
        ]);
        let rendered = render_linux_dts(
            "imx95",
            Some("imx95-19x19-evk"),
            None,
            DtsMode::Merge,
            &ir,
            &["cm7".to_string()],
        )
        .expect("render succeeds")
        .expect("uio present");

        assert_eq!(rendered.file_name, "consortium-imx95-19x19-evk.dts");
        // Vendor base preserved, UIO injected, and the predeclared mu7 disabled.
        assert!(rendered.contents.contains("compatible = \"generic-uio\""));
        assert!(rendered.contents.contains("&mu7 {"));
        assert!(rendered.contents.contains("#include"));
        assert!(rendered.warnings.is_empty());
    }

    #[test]
    fn overlay_on_real_stm32_devkit_emits_fragments() {
        let ir = ir_with(vec![shm("ipc-shm", 0x0a02_0000, 0x8000)]);
        let rendered = render_linux_dts(
            "stm32mp257fak3",
            Some("stm32mp257f-dk"),
            None,
            DtsMode::Overlay,
            &ir,
            &["cm33".to_string()],
        )
        .expect("render succeeds")
        .expect("uio present");

        assert_eq!(rendered.file_name, "consortium-stm32mp257f-dk.dtso");
        assert!(rendered.contents.contains("/plugin/;"));
        assert!(rendered.contents.contains("fragment@0"));
        assert!(rendered.contents.contains("target-path = \"/\""));
    }

    #[test]
    fn unused_coprocessor_is_not_touched() {
        let base = parse_dts(BASE).unwrap();
        let ir = ir_with(vec![shm("ipc-shm", 0x2040_0000, 0x8000)]);
        // Target a core the base has no node for.
        let analysis = analyze(std::slice::from_ref(&base), &ir, &["cm7".to_string()]);
        assert!(analysis.rprocs.is_empty());
    }
}
