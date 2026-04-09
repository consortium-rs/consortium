//! Messaging Unit (MU) Register Definitions
//! Reference: IMX93RM.pdf, Rev 7, Chapter 20
//!
//! All offsets are **relative to MU instance base address**
//! Usage: BASE_ADDR + register_offset

// ============================================================================
// MU Instance Base Addresses (physical, no_std / bare-metal use only)
//
// On A-core (Linux / std), do NOT use these constants directly.  Instead pass
// the virtual addresses returned by mmap to `MuDoorbell::new()`.
// ============================================================================

/// Physical base addresses for i.MX 95 MU instances (MUA side).
#[cfg(all(feature = "imx95", feature = "mua"))]
pub const MU_BASES: [u32; crate::chan::MU_INSTANCE_COUNT] = [
    0x4422_0000, // MU1
    0x445B_0000, // MU2
    0x445D_0000, // MU3
    0x445F_0000, // MU4
    0x4461_0000, // MU5
    0x4463_0000, // MU6
    0x4243_0000, // MU7
    0x4273_0000, // MU8
];

/// Physical base addresses for i.MX 93 MU instances (MUA side).
#[cfg(all(feature = "imx93", feature = "mua"))]
pub const MU_BASES: [u32; crate::chan::MU_INSTANCE_COUNT] = [
    0x4422_0000, // MU1
    0x4243_0000, // MU2
];

#[cfg(all(feature = "imx95", feature = "mub"))]
pub const MU_BASES: [u32; crate::chan::MU_INSTANCE_COUNT] = [
    0x4423_0000, // MU1
    0x445C_0000, // MU2
    0x445E_0000, // MU3
    0x4460_0000, // MU4
    0x4462_0000, // MU5
    0x4464_0000, // MU6
    0x4244_0000, // MU7
    0x4274_0000, // MU8
];

#[cfg(all(feature = "imx93", feature = "mub"))]
pub const MU_BASES: [u32; crate::chan::MU_INSTANCE_COUNT] = [
    0x4423_0000, // MU1
    0x4244_0000, // MU2
];

// i.MX 95 has 8 MU instances.  Physical base addresses must be taken from the
// i.MX 95 Reference Manual and supplied by the caller via `MuDoorbell::new()`.
// No compile-time constants are provided here to avoid embedding unverified
// addresses in the crate.  Consult the TRM for your board revision.

// ============================================================================
// MU Side A (MUA) Registers (Relative Offsets)
// ============================================================================
#[cfg(feature = "mua")]
pub mod mua {
    // Version ID Register
    pub const VER: u32 = 0x000;
    // Parameter Register
    pub const PAR: u32 = 0x004;
    // Control Register
    pub const CR: u32 = 0x008;
    // Status Register
    pub const SR: u32 = 0x00C;
    // Core Control 0 Register
    pub const CCR0: u32 = 0x010;
    // Core Interrupt Enable 0 Register
    pub const CIER0: u32 = 0x014;
    // Core Sticky Status 0 Register
    pub const CSSR0: u32 = 0x018;
    // Core Status 0 Register
    #[cfg(feature = "imx95")]
    pub const CSR0: u32 = 0x01C;

    // Flag Control Register
    pub const FCR: u32 = 0x100;
    // Flag Status Register
    pub const FSR: u32 = 0x104;

    // General Purpose Interrupt Enable Register
    pub const GIER: u32 = 0x110;
    // General Purpose Control Register
    pub const GCR: u32 = 0x114;
    // General Purpose Status Register
    pub const GSR: u32 = 0x118;

    // Transmit Control Register
    pub const TCR: u32 = 0x120;
    // Transmit Status Register
    pub const TSR: u32 = 0x124;
    // Receive Control Register
    pub const RCR: u32 = 0x128;
    // Receive Status Register
    pub const RSR: u32 = 0x12C;

    // Transmit Registers
    pub const TR0: u32 = 0x200;
    pub const TR1: u32 = 0x204;
    pub const TR2: u32 = 0x208;
    pub const TR3: u32 = 0x20C;

    // Receive Registers
    pub const RR0: u32 = 0x280;
    pub const RR1: u32 = 0x284;
    pub const RR2: u32 = 0x288;
    pub const RR3: u32 = 0x28C;

    pub mod fields {
        // CR
        pub const MU_MUR: u32 = 1 << 0;
        pub const MU_MURIE: u32 = 1 << 1;

        // SR
        pub const MU_MURS: u32 = 1 << 0;
        pub const MU_MURIP: u32 = 1 << 1;
        pub const MU_EP: u32 = 1 << 2;
        pub const MU_FUP: u32 = 1 << 3;
        pub const MU_GIRP: u32 = 1 << 4;
        pub const MU_TEP: u32 = 1 << 5;
        pub const MU_RFP: u32 = 1 << 6;

        // CCR0
        pub const MU_NMI: u32 = 1 << 0;
        pub const MU_HR: u32 = 1 << 1;
        pub const MU_HRM: u32 = 1 << 2;
        pub const MU_CLKE: u32 = 1 << 3;
        pub const MU_RSTH: u32 = 1 << 4;
        pub const MU_BOOT_MASK: u32 = 0x3 << 5;

        // GIER — enable general-purpose interrupt n (n = 0..3)
        pub const fn gier_gie(n: u8) -> u32 { 1 << n }
        // GCR — set general-purpose interrupt request n (sender side, n = 0..3)
        pub const fn gcr_gir(n: u8) -> u32 { 1 << n }
        // GSR — general-purpose interrupt pending n (receiver side, W1C, n = 0..3)
        pub const fn gsr_gip(n: u8) -> u32 { 1 << n }
    }

    /// Number of general-purpose interrupt channels per MU instance.
    pub const GI_COUNT: usize = 4;
}

// ============================================================================
// MU Side B (MUB) Registers (Relative Offsets)
// ============================================================================
#[cfg(feature = "mub")]
pub mod mub {
    // Version ID Register
    pub const VER: u32 = 0x000;
    // Parameter Register
    pub const PAR: u32 = 0x004;
    // Control Register
    pub const CR: u32 = 0x008;
    // Status Register
    pub const SR: u32 = 0x00C;
    // Core Control 0 Register
    pub const CCR0: u32 = 0x010;
    // Core Interrupt Enable 0 Register
    pub const CIER0: u32 = 0x014;
    // Core Sticky Status 0 Register
    pub const CSSR0: u32 = 0x018;

    // Flag Control Register
    pub const FCR: u32 = 0x100;
    // Flag Status Register
    pub const FSR: u32 = 0x104;

    // General Purpose Interrupt Enable Register
    pub const GIER: u32 = 0x110;
    // General Purpose Control Register
    pub const GCR: u32 = 0x114;
    // General Purpose Status Register
    pub const GSR: u32 = 0x118;

    // Transmit Control Register
    pub const TCR: u32 = 0x120;
    // Transmit Status Register
    pub const TSR: u32 = 0x124;
    // Receive Control Register
    pub const RCR: u32 = 0x128;
    // Receive Status Register
    pub const RSR: u32 = 0x12C;

    // Transmit Registers
    pub const TR0: u32 = 0x200;
    pub const TR1: u32 = 0x204;
    pub const TR2: u32 = 0x208;
    pub const TR3: u32 = 0x20C;

    // Receive Registers
    pub const RR0: u32 = 0x280;
    pub const RR1: u32 = 0x284;
    pub const RR2: u32 = 0x288;
    pub const RR3: u32 = 0x28C;

    pub mod fields {
        // CR
        pub const MU_MUR: u32 = 1 << 0;
        pub const MU_MURIE: u32 = 1 << 1;

        // SR
        pub const MU_MURS: u32 = 1 << 0;
        pub const MU_MURIP: u32 = 1 << 1;
        pub const MU_EP: u32 = 1 << 2;
        pub const MU_FUP: u32 = 1 << 3;
        pub const MU_GIRP: u32 = 1 << 4;
        pub const MU_TEP: u32 = 1 << 5;
        pub const MU_RFP: u32 = 1 << 6;

        // CCR0
        pub const MU_NMI: u32 = 1 << 0;
        pub const MU_HR: u32 = 1 << 1;
        pub const MU_HRM: u32 = 1 << 2;
        pub const MU_CLKE: u32 = 1 << 3;
        pub const MU_RSTH: u32 = 1 << 4;
        pub const MU_BOOT_MASK: u32 = 0x3 << 5;

        // GIER — enable general-purpose interrupt n (n = 0..3)
        pub const fn gier_gie(n: u8) -> u32 { 1 << n }
        // GCR — set general-purpose interrupt request n (sender side, n = 0..3)
        pub const fn gcr_gir(n: u8) -> u32 { 1 << n }
        // GSR — general-purpose interrupt pending n (receiver side, W1C, n = 0..3)
        pub const fn gsr_gip(n: u8) -> u32 { 1 << n }
    }

    /// Number of general-purpose interrupt channels per MU instance.
    pub const GI_COUNT: usize = 4;
}

#[cfg(feature = "mua")]
pub use self::mua as side;
#[cfg(feature = "mub")]
pub use self::mub as side;

// ============================================================================
// Reset Values (TRM Verified)
// ============================================================================
pub mod reset {
    pub const VER: u32 = 0x0309_000F;
    pub const PAR: u32 = 0x0304_0404;
    pub const CR: u32 = 0x0000_0000;
    pub const CCR0: u32 = 0x0000_0014;
    pub const CIER0: u32 = 0x0000_0000;
    pub const CSSR0: u32 = 0x0000_0000;
    pub const FCR: u32 = 0x0000_0000;
    pub const FSR: u32 = 0x0000_0000;
    pub const GIER: u32 = 0x0000_0000;
    pub const GCR: u32 = 0x0000_0000;
    pub const GSR: u32 = 0x0000_0000;
    pub const TCR: u32 = 0x0000_0000;
    pub const TSR: u32 = 0x0000_000F;
    pub const RCR: u32 = 0x0000_0000;
    pub const RSR: u32 = 0x0000_0000;
    pub const TR: u32 = 0x0000_0000;
    pub const RR: u32 = 0x0000_0000;
}

// ============================================================================
// MU Parameters
// ============================================================================
pub const TR_COUNT: usize = 4;
pub const RR_COUNT: usize = 4;
