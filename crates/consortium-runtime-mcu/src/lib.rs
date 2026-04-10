#![no_std]

pub mod descriptor;
pub mod irq;

#[cfg(feature = "stm32mp2xx")]
pub use consortium_ipc_doorbell_hsem as hsem;

#[cfg(feature = "imx93")]
pub use consortium_ipc_doorbell_mu as mu;

#[cfg(feature = "imx95")]
pub use consortium_ipc_doorbell_mu as mu;

pub use consortium_ipc_transport_mem;

/// Compute the pointer to the Tx slot in a MemTransport slot pair.
///
/// # Arguments
///
/// * `region_base` — pointer to the start of the shared memory region
/// * `slot_idx` — zero-indexed slot pair (0, 1, 2, ...)
/// * `slot_size` — size of each slot (including 8-byte header): `8 + MTU`
///
/// # Returns
///
/// A tuple `(tx_ptr, rx_ptr)` where:
/// - `tx_ptr`: A→M slot (this side sends, other side receives)
/// - `rx_ptr`: M→A slot (this side receives, other side sends)
///
/// # Safety
///
/// Caller must ensure the computed pointers are valid for the slot size.
pub unsafe fn slot_pair_ptrs(
    region_base: *mut u8,
    slot_idx: usize,
    slot_size: usize,
) -> (*mut u8, *mut u8) {
    let tx_offset = slot_idx * 2 * slot_size;
    let rx_offset = slot_idx * 2 * slot_size + slot_size;
    (
        // SAFETY: the caller must ensure that `region_base` points to a valid shared memory region and that the computed offsets are within bounds for the number of slots and their sizes.
        unsafe { region_base.add(tx_offset) },
        // SAFETY: the caller must ensure that `region_base` is valid and that the computed offsets are within bounds of the shared memory region.
        unsafe { region_base.add(rx_offset) },
    )
}

/// Macro for declaring a static scratch buffer with proper alignment.
///
/// # Example
///
/// ```ignore
/// static_buf!(MY_BUF, 1024);  // Declares static mut MY_BUF: [u8; 1024]
/// ```
#[macro_export]
macro_rules! static_buf {
    ($name:ident, $size:expr) => {
        static $name: $crate::static_cell::StaticCell<[u8; $size]> =
            $crate::static_cell::StaticCell::new([0u8; $size]);
    };
}

pub use static_cell;
