//! The lowered IR: one node per thing the manifest asks the system to have, each
//! owning the code or device-tree it generates.
//!
//! [`bridge`](crate::bridge) builds these from a validated
//! [`Session`](crate::Session); this module is where "the manifest declares two IPC
//! channels" becomes actual `Channel::new(..)` expressions, `reserved-memory`
//! carveouts, and NVIC enables.
//!
//! Codegen is deliberately *not* centralised. Each IR type emits its own output —
//! [`ShuHan::initialize`](ipc_shm::ShuHan::initialize),
//! [`BellA::nvic_enable`](bella::BellA::nvic_enable),
//! [`ToUio::to_uio`](uio::ToUio::to_uio) — so a new component adds a module rather
//! than a branch in a shared emitter.
//!
//! | Module            | Emits                                                     |
//! | ----------------- | --------------------------------------------------------- |
//! | [`ipc_shm`]       | Transceiver construction, scratch-buffer statics, `IpcMemoryEndpoints`. |
//! | [`bella`]         | Type-A doorbell interrupt enablement.                      |
//! | [`peripheral`]    | HAL driver handles, interrupt handlers, NVIC enablement.     |
//! | [`time`]          | The sealed Embassy time-driver vector and `defmt::timestamp!`. |
//! | `dbg`             | Nothing — resolves debug ring layout for the builder's env vars. |
//! | [`uio`]           | `generic-uio` DTS nodes and their `reserved-memory` carveouts. |
//! | [`memory_to_uio`] | Groups shared-memory regions onto one UIO device.            |
//! | [`dts_render`]    | The Linux device tree, merged onto a vendor base or as an overlay. |
//! | [`address`]       | Shared address resolution: AP `uioN`/`mmap_id` vs a controller's physical base. |
//! | [`region`]        | One shared-memory window's construction expression.          |
//!
//! Two conventions worth knowing before adding a component:
//!
//! - **Emitted `unsafe` blocks carry their justification as a `///` doc comment,**
//!   not a `//` comment. `proc_macro2` discards `//` comments because they are not
//!   tokens, so [`render()`](crate::render()) rewrites the doc lines back into ordinary
//!   comments after formatting. That is what satisfies
//!   `clippy::undocumented_unsafe_blocks` in the generated crate.
//! - **Items referencing a chip HAL are gated to `target_arch = "arm"`,** because the
//!   host trybuild harness never links one.

pub mod address;
pub mod bella;
pub mod dbg;
pub mod dts_render;
pub mod ipc_shm;
pub mod memory_to_uio;
pub mod peripheral;
pub mod region;
pub mod time;
pub mod uio;
