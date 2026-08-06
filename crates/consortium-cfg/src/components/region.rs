//! One shared-memory window's construction expression.
//!
//! [`Region`] emits the code that gives a core its handle on a single carveout,
//! resolving the address through [`Address`].
//!
//! The one asymmetry worth knowing: **the AP clears the window on first touch**
//! (`mmap_zeroed`) while controller cores do not. A carveout is `no-map` reserved
//! memory that survives a warm reboot, so stale descriptor and slot bytes from a
//! previous run could otherwise read as valid `CIPC` state. The AP is the primary in
//! the descriptor handshake, so it is the right side to own that first touch;
//! controller cores embed a physical base and leave zeroing to the handshake.
//!
//! [`Address`]: super::address::Address

use consortium_cfg_common::{config::VisibleCore, database::MemoryRegionType};
use syn::Ident;

use crate::components::address::Address;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Region {
    /// Hardware memory type from the chip database. This remains a physical
    /// fact for diagnostics, but no longer selects the generated mapping type.
    pub r#type: MemoryRegionType,
    /// Whether the controller-side mapping retains the core's cacheable default.
    pub cache: bool,
    /// address of the region
    pub address: Address,
    /// length of the region
    pub length: u32,
}

impl Region {
    pub fn initialize(&self, core: VisibleCore, uio_ident: &Ident) -> proc_macro2::TokenStream {
        // The AP owns the first touch of the shared-memory carveout and clears it
        // (`mmap_zeroed`) so a warm reboot's stale descriptor/slot bytes never
        // read as valid; controller cores embed a physical base and leave the
        // handshake to zero what it needs.
        let address = match core {
            VisibleCore::Ap => {
                let uio_id = self.address.ap.uio_id;
                let mmap_id = self.address.ap.mmap_id;
                quote::quote! { #uio_ident[#uio_id].mmap_zeroed(#mmap_id)? }
            }
            _ => self.address.memory_lookup(core, uio_ident),
        };
        let length = proc_macro2::Literal::usize_suffixed(self.length as usize);
        let region_identifier = match (core, self.cache) {
            (VisibleCore::Ap, _) | (_, false) => {
                quote::quote! { ::consortium_shared_memory::regions::UncachedRegion }
            }
            (_, true) => quote::quote! { ::consortium_shared_memory::regions::CachedRegion },
        };
        quote::quote! {
            unsafe { #region_identifier::new((#address) as *mut u8, #length) }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::components::address::ApAddress;
    use quote::format_ident;

    fn ap_region() -> Region {
        Region {
            r#type: MemoryRegionType::Sram,
            cache: false,
            address: Address {
                ap: ApAddress {
                    uio_id: 0,
                    mmap_id: 1,
                },
                cp: HashMap::new(),
            },
            length: 0x8000,
        }
    }

    #[test]
    fn uncached_region_wraps_ap_lookup_in_unsafe_new() {
        let uio = format_ident!("uio");
        let tokens = ap_region().initialize(VisibleCore::Ap, &uio).to_string();

        assert!(tokens.contains("unsafe"));
        assert!(tokens.contains(":: consortium_shared_memory :: regions :: UncachedRegion :: new"));
        // base is cast to a raw pointer; length is a usize literal.
        assert!(tokens.contains("as * mut u8"));
        assert!(tokens.contains("32768usize"));
        // address resolves through the AP UIO mmap window, zeroing it first.
        assert!(tokens.contains("uio [0usize] . mmap_zeroed (1usize) ?"));
    }

    #[test]
    fn cached_region_is_controller_only() {
        let uio = format_ident!("uio");
        let mut region = ap_region();
        let cm7 = VisibleCore::Mcore(consortium_cfg_common::config::McoreName::Cm7);
        region.cache = true;
        region.address.cp.insert(cm7, 0x3400_0000);

        let controller = region.initialize(cm7, &uio).to_string();
        let ap = region.initialize(VisibleCore::Ap, &uio).to_string();
        assert!(controller.contains(":: consortium_shared_memory :: regions :: CachedRegion"));
        assert!(ap.contains(":: consortium_shared_memory :: regions :: UncachedRegion"));
    }
}
