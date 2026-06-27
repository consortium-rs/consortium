use std::collections::HashMap;

use consortium_cfg_common::config::VisibleCore;

/// UIO mmap'd address provided for MMIO in application processors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ApAddress {
    /// ID of the UIO device
    pub uio_id: u32,
    /// Mmap ID in the UIO device of the memory window for this channel
    pub mmap_id: u32,
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
        core: &VisibleCore,
        uio_ident: syn::Ident,
    ) -> proc_macro2::TokenStream {
        match core {
            VisibleCore::Ap => {
                let uio_id = self.ap.uio_id;
                let mmap_id = self.ap.mmap_id;
                quote::quote! { #uio_ident[#uio_id].mmap(#mmap_id)? }
            }
            _ => {
                let address = self
                    .cp
                    .get(core)
                    .expect("core must have an address in the config");
                quote::quote! { #address }
            }
        }
    }
}
