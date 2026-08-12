// Copyright 2026 Ethan Wu
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// SPDX-License-Identifier: Apache-2.0

//! Structured remoteproc memory and GNU linker generation.

use std::collections::HashSet;
use std::fmt::Write as _;

use consortium_cfg_common::config::VisibleCore;
use thiserror::Error;

/// One firmware address window shared by linker, ELF, and DTS generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RemoteprocMemoryWindow {
    pub origin: u64,
    pub length: u64,
    pub reserve: bool,
}

impl RemoteprocMemoryWindow {
    pub fn end(self) -> Option<u64> {
        self.origin.checked_add(self.length)
    }

    pub fn contains(self, start: u64, length: u64) -> bool {
        let Some(end) = start.checked_add(length) else {
            return false;
        };
        self.origin <= start && self.end().is_some_and(|limit| end <= limit)
    }
}

/// One input-to-output section binding for an auxiliary linker region.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkerSectionBinding {
    pub output: String,
    pub inputs: Vec<String>,
    pub keep: bool,
    pub insert_after: String,
}

/// A named vector or user-defined auxiliary region.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoteprocNamedRegion {
    pub name: String,
    pub attributes: LinkerRegionAttributes,
    pub window: RemoteprocMemoryWindow,
    pub sections: Vec<LinkerSectionBinding>,
}

/// Canonical contract for one Linux-managed remote core.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoteprocMemoryLayout {
    pub core: VisibleCore,
    pub compatible: String,
    pub vector_section: String,
    /// Mandatory vector-table window immediately before the code window.
    pub vector: RemoteprocNamedRegion,
    pub flash: RemoteprocMemoryWindow,
    /// Mandatory remoteproc shared memory containing `.resource_table`.
    pub shmem: RemoteprocNamedRegion,
    pub ram: RemoteprocMemoryWindow,
    /// Optional manifest-defined regions.
    pub regions: Vec<RemoteprocNamedRegion>,
}

impl RemoteprocMemoryLayout {
    pub fn validate(&self) -> Result<(), LinkerMemoryError> {
        validate_linker_window("FLASH", self.flash)?;
        validate_linker_window("RAM", self.ram)?;
        validate_linker_window(&self.vector.name, self.vector.window)?;
        validate_linker_window(&self.shmem.name, self.shmem.window)?;
        LinkerRegionName::new(&self.vector.name)?;
        LinkerRegionName::new(&self.shmem.name)?;
        validate_attributes(&self.vector.name, self.vector.attributes)?;
        validate_attributes(&self.shmem.name, self.shmem.attributes)?;

        let mut names = HashSet::from([
            "FLASH".to_string(),
            "RAM".to_string(),
            self.vector.name.clone(),
            self.shmem.name.clone(),
        ]);
        if names.len() != 4 {
            return Err(LinkerMemoryError::Duplicate(
                "mandatory remoteproc region".into(),
            ));
        }
        let mut prior: Vec<(&str, RemoteprocMemoryWindow)> = vec![
            (&self.vector.name, self.vector.window),
            ("FLASH", self.flash),
            (&self.shmem.name, self.shmem.window),
            ("RAM", self.ram),
        ];
        for (index, (left_name, left)) in prior.iter().enumerate() {
            for (right_name, right) in prior.iter().skip(index + 1) {
                if overlaps_window(*left, *right) {
                    return Err(LinkerMemoryError::Overlap {
                        first: (*left_name).into(),
                        second: (*right_name).into(),
                    });
                }
            }
        }
        for region in &self.regions {
            LinkerRegionName::new(&region.name)?;
            if !names.insert(region.name.clone()) {
                return Err(LinkerMemoryError::Duplicate(region.name.clone()));
            }
            validate_attributes(&region.name, region.attributes)?;
            validate_linker_window(&region.name, region.window)?;
            validate_bindings(&region.name, &region.sections)?;
            for (name, window) in &prior {
                if overlaps_window(*window, region.window) {
                    return Err(LinkerMemoryError::Overlap {
                        first: (*name).into(),
                        second: region.name.clone(),
                    });
                }
            }
            prior.push((&region.name, region.window));
        }
        for (name, window) in &prior {
            if window.reserve {
                validate_page_alignment(name, *window)?;
            }
        }
        Ok(())
    }

    pub fn linker_memory(&self) -> Result<LinkerMemory, LinkerMemoryError> {
        self.validate()?;
        let flash = LinkerRegionName::new("FLASH")?;
        let mut regions = vec![
            LinkerMemoryRegion {
                name: LinkerRegionName::new(&self.vector.name)?,
                role: LinkerRegionRole::Vector,
                attributes: self.vector.attributes,
                origin: self.vector.window.origin,
                length: self.vector.window.length,
                sections: Vec::new(),
            },
            LinkerMemoryRegion {
                name: flash,
                role: LinkerRegionRole::Flash,
                attributes: LinkerRegionAttributes::new(true, false, true),
                origin: self.flash.origin,
                length: self.flash.length,
                sections: Vec::new(),
            },
            LinkerMemoryRegion {
                name: LinkerRegionName::new(&self.shmem.name)?,
                role: LinkerRegionRole::Auxiliary,
                attributes: self.shmem.attributes,
                origin: self.shmem.window.origin,
                length: self.shmem.window.length,
                sections: vec![LinkerSectionBinding {
                    output: ".resource_table".into(),
                    inputs: vec![".resource_table".into(), ".resource_table.*".into()],
                    keep: true,
                    insert_after: ".rodata".into(),
                }],
            },
        ];
        regions.extend(self.regions.iter().map(|region| LinkerMemoryRegion {
            name: LinkerRegionName::new(&region.name).expect("layout validated"),
            role: LinkerRegionRole::Auxiliary,
            attributes: region.attributes,
            origin: region.window.origin,
            length: region.window.length,
            sections: region.sections.clone(),
        }));
        regions.push(LinkerMemoryRegion {
            name: LinkerRegionName::new("RAM")?,
            role: LinkerRegionRole::Ram,
            attributes: LinkerRegionAttributes::new(true, true, true),
            origin: self.ram.origin,
            length: self.ram.length,
            sections: Vec::new(),
        });
        LinkerMemory::new(regions)
    }

    /// Every configured window in which an ELF load segment may reside.
    pub fn load_windows(&self) -> impl Iterator<Item = RemoteprocMemoryWindow> + '_ {
        [self.vector.window, self.flash, self.shmem.window, self.ram]
            .into_iter()
            .chain(self.regions.iter().map(|region| region.window))
    }
}

fn validate_linker_window(
    name: &str,
    window: RemoteprocMemoryWindow,
) -> Result<(), LinkerMemoryError> {
    if window.length == 0 {
        return Err(LinkerMemoryError::Empty(name.into()));
    }
    if window.end().is_none() {
        return Err(LinkerMemoryError::Overflow(name.into()));
    }
    if !window.origin.is_multiple_of(4) || !window.length.is_multiple_of(4) {
        return Err(LinkerMemoryError::Unaligned {
            name: name.into(),
            alignment: 4,
        });
    }
    Ok(())
}

fn validate_page_alignment(
    name: &str,
    window: RemoteprocMemoryWindow,
) -> Result<(), LinkerMemoryError> {
    if !window.origin.is_multiple_of(4096) || !window.length.is_multiple_of(4096) {
        return Err(LinkerMemoryError::Unaligned {
            name: name.into(),
            alignment: 4096,
        });
    }
    Ok(())
}

fn overlaps_window(left: RemoteprocMemoryWindow, right: RemoteprocMemoryWindow) -> bool {
    overlaps(
        left.origin,
        left.end().expect("validated"),
        right.origin,
        right.end().expect("validated"),
    )
}

fn overlaps(a_start: u64, a_end: u64, b_start: u64, b_end: u64) -> bool {
    a_start < b_end && b_start < a_end
}

fn validate_attributes(
    name: &str,
    attributes: LinkerRegionAttributes,
) -> Result<(), LinkerMemoryError> {
    if attributes.read || attributes.write || attributes.execute {
        Ok(())
    } else {
        Err(LinkerMemoryError::EmptyAttributes(name.into()))
    }
}

fn validate_bindings(
    region: &str,
    bindings: &[LinkerSectionBinding],
) -> Result<(), LinkerMemoryError> {
    let mut outputs = HashSet::new();
    for binding in bindings {
        const CORTEX_M_RT_OUTPUTS: &[&str] = &[
            ".vector_table",
            ".text",
            ".rodata",
            ".data",
            ".bss",
            ".uninit",
        ];
        if !valid_section_pattern(&binding.output, false)
            || !valid_section_pattern(&binding.insert_after, false)
            || binding.inputs.is_empty()
            || binding
                .inputs
                .iter()
                .any(|input| !valid_section_pattern(input, true))
            || CORTEX_M_RT_OUTPUTS.contains(&binding.output.as_str())
        {
            return Err(LinkerMemoryError::InvalidSection {
                region: region.into(),
                section: binding.output.clone(),
            });
        }
        if !outputs.insert(binding.output.as_str()) {
            return Err(LinkerMemoryError::DuplicateSection(binding.output.clone()));
        }
    }
    Ok(())
}

fn valid_section_pattern(value: &str, wildcard: bool) -> bool {
    value.starts_with('.')
        && value.len() > 1
        && value.chars().all(|character| {
            character.is_ascii_alphanumeric()
                || matches!(character, '.' | '_' | '-')
                || (wildcard && character == '*')
        })
}

/// A typed GNU linker `MEMORY` command plus optional section-placement
/// augmentations for `cortex-m-rt`'s `link.x`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkerMemory {
    regions: Vec<LinkerMemoryRegion>,
}

impl LinkerMemory {
    pub fn new(mut regions: Vec<LinkerMemoryRegion>) -> Result<Self, LinkerMemoryError> {
        regions.sort_by(|left, right| {
            left.role
                .cmp(&right.role)
                .then_with(|| left.name.as_str().cmp(right.name.as_str()))
        });
        let mut names = HashSet::new();
        let mut outputs = HashSet::new();
        for region in &regions {
            let name = region.name.as_str();
            if !names.insert(name) {
                return Err(LinkerMemoryError::Duplicate(name.into()));
            }
            validate_attributes(name, region.attributes)?;
            validate_linker_window(
                name,
                RemoteprocMemoryWindow {
                    origin: region.origin,
                    length: region.length,
                    reserve: false,
                },
            )?;
            validate_bindings(name, &region.sections)?;
            for section in &region.sections {
                if !outputs.insert(section.output.as_str()) {
                    return Err(LinkerMemoryError::DuplicateSection(section.output.clone()));
                }
            }
        }
        for (index, left) in regions.iter().enumerate() {
            let left_end = left.origin + left.length;
            for right in regions.iter().skip(index + 1) {
                let right_end = right.origin + right.length;
                if overlaps(left.origin, left_end, right.origin, right_end) {
                    return Err(LinkerMemoryError::Overlap {
                        first: left.name.as_str().into(),
                        second: right.name.as_str().into(),
                    });
                }
            }
        }
        Ok(Self { regions })
    }

    pub fn regions(&self) -> &[LinkerMemoryRegion] {
        &self.regions
    }

    /// Renders the generated `memory.x`. The `MEMORY` command is followed only
    /// by explicitly configured section bindings; ordinary placement remains
    /// owned by `cortex-m-rt`'s `link.x`.
    pub fn render(&self) -> String {
        let mut out = String::from("/* Auto-generated by Consortium. Do not edit. */\nMEMORY\n{\n");
        for region in &self.regions {
            writeln!(
                out,
                "  {} ({}) : ORIGIN = {:#010x}, LENGTH = {:#x}",
                region.name.as_str(),
                region.attributes.render(),
                region.origin,
                region.length
            )
            .expect("writing a String cannot fail");
        }
        out.push_str("}\n");
        for region in &self.regions {
            for section in &region.sections {
                out.push_str("\nSECTIONS\n{\n");
                writeln!(out, "  {} :", section.output).expect("String write");
                out.push_str("  {\n");
                for input in &section.inputs {
                    if section.keep {
                        writeln!(out, "    KEEP(*({input}))").expect("String write");
                    } else {
                        writeln!(out, "    *({input})").expect("String write");
                    }
                }
                writeln!(out, "  }} > {}", region.name.as_str()).expect("String write");
                writeln!(out, "}} INSERT AFTER {};", section.insert_after).expect("String write");
            }
        }
        out
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LinkerRegionName(String);

impl LinkerRegionName {
    pub fn new(value: impl Into<String>) -> Result<Self, LinkerMemoryError> {
        let value = value.into();
        let mut chars = value.chars();
        let valid = chars
            .next()
            .is_some_and(|first| first.is_ascii_alphabetic() || first == '_')
            && chars.all(|character| character.is_ascii_alphanumeric() || character == '_');
        if valid {
            Ok(Self(value))
        } else {
            Err(LinkerMemoryError::InvalidName(value))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LinkerRegionRole {
    Vector,
    Flash,
    Auxiliary,
    Ram,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LinkerRegionAttributes {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
}

impl LinkerRegionAttributes {
    pub const fn new(read: bool, write: bool, execute: bool) -> Self {
        Self {
            read,
            write,
            execute,
        }
    }

    fn render(self) -> String {
        let mut out = String::new();
        if self.read {
            out.push('r');
        }
        if self.write {
            out.push('w');
        }
        if self.execute {
            out.push('x');
        }
        out
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkerMemoryRegion {
    pub name: LinkerRegionName,
    pub role: LinkerRegionRole,
    pub attributes: LinkerRegionAttributes,
    pub origin: u64,
    pub length: u64,
    pub sections: Vec<LinkerSectionBinding>,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum LinkerMemoryError {
    #[error("linker memory region '{0}' has zero length")]
    Empty(String),
    #[error("linker memory region '{0}' address range overflows")]
    Overflow(String),
    #[error("linker memory region '{name}' must be {alignment}-byte aligned")]
    Unaligned { name: String, alignment: u64 },
    #[error("linker memory region '{0}' has no access attributes")]
    EmptyAttributes(String),
    #[error("linker memory region name '{0}' is not a valid linker identifier")]
    InvalidName(String),
    #[error("linker memory region '{0}' is declared more than once")]
    Duplicate(String),
    #[error("linker output section '{0}' is declared more than once")]
    DuplicateSection(String),
    #[error("linker section '{section}' for region '{region}' is invalid")]
    InvalidSection { region: String, section: String },
    #[error("linker memory regions '{first}' and '{second}' overlap")]
    Overlap { first: String, second: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    fn region(name: &str, role: LinkerRegionRole, origin: u64, length: u64) -> LinkerMemoryRegion {
        LinkerMemoryRegion {
            name: LinkerRegionName::new(name).unwrap(),
            role,
            attributes: LinkerRegionAttributes::new(true, true, false),
            origin,
            length,
            sections: Vec::new(),
        }
    }

    #[test]
    fn renders_contiguous_vector_flash_and_mandatory_shmem() {
        let vector = region(
            "NS_VECTOR_TBL",
            LinkerRegionRole::Vector,
            0x8010_0000,
            0x1000,
        );
        let mut shmem = region("SHMEM", LinkerRegionRole::Auxiliary, 0x812f_8000, 0x8000);
        shmem.sections.push(LinkerSectionBinding {
            output: ".resource_table".into(),
            inputs: vec![".resource_table".into(), ".resource_table.*".into()],
            keep: true,
            insert_after: ".rodata".into(),
        });
        let memory = LinkerMemory::new(vec![
            region("RAM", LinkerRegionRole::Ram, 0x80a0_0000, 0x100000),
            shmem,
            region("FLASH", LinkerRegionRole::Flash, 0x8010_1000, 0x7ff000),
            vector,
        ])
        .unwrap();
        let rendered = memory.render();
        assert!(rendered.starts_with(
            "/* Auto-generated by Consortium. Do not edit. */\nMEMORY\n{\n  NS_VECTOR_TBL (rw)"
        ));
        assert!(rendered.contains("FLASH (rw) : ORIGIN = 0x80101000"));
        assert!(rendered.contains("SHMEM (rw) : ORIGIN = 0x812f8000"));
        assert!(rendered.contains("KEEP(*(.resource_table.*))"));
        assert!(rendered.contains("} INSERT AFTER .rodata;"));
    }

    #[test]
    fn validates_names_alignment_duplicates_and_overlap() {
        assert!(matches!(
            LinkerRegionName::new("bad-name"),
            Err(LinkerMemoryError::InvalidName(_))
        ));
        assert!(matches!(
            LinkerMemory::new(vec![
                region("FLASH", LinkerRegionRole::Flash, 0, 0x1000),
                region("FLASH", LinkerRegionRole::Ram, 0x2000, 0x1000),
            ]),
            Err(LinkerMemoryError::Duplicate(_))
        ));
        assert!(matches!(
            LinkerMemory::new(vec![region("RAM", LinkerRegionRole::Ram, 0, 0)]),
            Err(LinkerMemoryError::Empty(_))
        ));
        assert!(matches!(
            LinkerMemory::new(vec![region("RAM", LinkerRegionRole::Ram, u64::MAX - 3, 8)]),
            Err(LinkerMemoryError::Overflow(_))
        ));
        assert!(matches!(
            LinkerMemory::new(vec![
                region("FLASH", LinkerRegionRole::Flash, 0, 0x2000),
                region("RAM", LinkerRegionRole::Ram, 0x1000, 0x2000),
            ]),
            Err(LinkerMemoryError::Overlap { .. })
        ));
        assert!(matches!(
            LinkerMemory::new(vec![region("FLASH", LinkerRegionRole::Flash, 1, 0x1000)]),
            Err(LinkerMemoryError::Unaligned { .. })
        ));
    }
}
