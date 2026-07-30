//! Where a shared window lives, from each core's point of view.
//!
//! The same carveout has two different names depending on who is looking at it, and
//! [`Address`] holds both so a component does not have to branch on the core in more
//! than one place:
//!
//! - **the AP** reaches it through a UIO device, so its address is an
//!   [`ApAddress`] — a `(uio_id, mmap_id)` pair resolved at runtime by `mmap`ping
//!   `/dev/uioN`;
//! - **a controller core** has no UIO, so its address is a physical base baked into
//!   the generated source as a literal.
//!
//! [`Address::memory_lookup`] emits the right one for a given [`VisibleCore`]. The
//! `uio_id` and `mmap_id` values themselves are assigned once, in
//! [`bridge`](crate::bridge), so the firmware, NVIC, and DTS outputs cannot drift
//! apart.

use std::collections::HashMap;

use consortium_cfg_common::config::VisibleCore;

/// UIO mmap'd address provided for MMIO in application processors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ApAddress {
    /// ID of the UIO device
    pub uio_id: usize,
    /// Mmap ID in the UIO device of the memory window for this channel
    pub mmap_id: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Address {
    /// application processor
    pub ap: ApAddress,
    /// controller processor
    pub cp: HashMap<VisibleCore, u32>,
}

impl Address {
    pub fn memory_lookup(
        &self,
        core: VisibleCore,
        uio_ident: &syn::Ident,
    ) -> proc_macro2::TokenStream {
        match core {
            VisibleCore::Ap => {
                let uio_id = self.ap.uio_id;
                let mmap_id = self.ap.mmap_id;
                quote::quote! { #uio_ident[#uio_id].mmap(#mmap_id)? }
            }
            _ => {
                let address = *self
                    .cp
                    .get(&core)
                    .expect("core must have an address in the config");
                // Emit a `usize` so it matches the AP `mmap` path's type; call
                // sites cast to `*mut u8` where a pointer is needed.
                let address = proc_macro2::Literal::usize_suffixed(address as usize);
                quote::quote! { #address }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use consortium_cfg_common::config::McoreName;
    use quote::format_ident;

    #[test]
    fn ap_lookup_indexes_uio_mmap_window() {
        let address = Address {
            ap: ApAddress {
                uio_id: 3,
                mmap_id: 2,
            },
            cp: HashMap::new(),
        };

        let uio = format_ident!("uio");
        let tokens = address.memory_lookup(VisibleCore::Ap, &uio).to_string();

        // `uio[3].mmap(2)?` — the UIO id selects the device, the mmap id the window.
        assert_eq!(tokens, "uio [3usize] . mmap (2usize) ?");
    }

    #[test]
    fn controller_lookup_emits_raw_address() {
        let core = VisibleCore::Mcore(McoreName::Cm33);
        let address = Address {
            ap: ApAddress {
                uio_id: 0,
                mmap_id: 0,
            },
            cp: HashMap::from([(core, 0x3400_0000)]),
        };

        let uio = format_ident!("uio");
        let tokens = address.memory_lookup(core, &uio).to_string();

        assert_eq!(tokens, "872415232usize");
    }

    #[test]
    #[should_panic(expected = "core must have an address in the config")]
    fn controller_lookup_without_address_panics() {
        let core = VisibleCore::Mcore(McoreName::Cm33);
        let address = Address {
            ap: ApAddress {
                uio_id: 0,
                mmap_id: 0,
            },
            cp: HashMap::new(),
        };

        let uio = format_ident!("uio");
        let _ = address.memory_lookup(core, &uio);
    }
}
