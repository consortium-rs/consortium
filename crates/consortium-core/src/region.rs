/// Describes a contiguous memory area.
/// Does not imply ownership — just describes what exists.
///
/// Implementations:
///   SramRegion  — cache ops are noops
///   DdrRegion   — cache ops are real dcache instructions
///   SerialRegion— no backing memory, frame buffer only
pub trait Region {
    /// Raw pointer to the start of the region.
    ///
    /// # Safety
    /// Caller must ensure no aliasing and correct lifetime.
    unsafe fn as_ptr(&self) -> *mut u8;

    /// Length in bytes.
    fn len(&self) -> usize;

    /// Write back dirty cache lines to memory.
    /// Noop on coherent regions (SRAM).
    fn flush(&self);

    /// Invalidate stale cache lines.
    /// Noop on coherent regions (SRAM).
    fn invalidate(&self);

    /// Returns true if this region is always cache-coherent.
    /// Allows Transport impls to skip cache ops at compile time
    /// via monomorphization rather than runtime branching.
    fn is_coherent(&self) -> bool;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
