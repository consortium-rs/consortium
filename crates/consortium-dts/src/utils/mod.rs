//! Helpers that sit beside the AST rather than inside it.
//!
//! | Module         | Provides                                                       |
//! | -------------- | -------------------------------------------------------------- |
//! | [`edit`]       | Read-only analysis over a parsed [`File`].       |
//! | [`irq_method`] | [`IrqMethod`](irq_method::IrqMethod), the DTS interrupt trigger encoding. |
//!
//! [`edit`] is read-*only* by design: a merged device tree is built by appending
//! [`NodeOverlay`] items to a parsed base, not by rewriting its
//! nodes, so parsed AST values stay immutable and the delta stays visible in the output.
//!
//! [`irq_method`] is the one place a device-tree value and an MCU register value have to
//! agree: its discriminants match the Linux
//! `dt-bindings/interrupt-controller/irq.h` constants, so the same enum can be printed
//! into a DTS `interrupts` cell and used to configure an NVIC line.
//!
//! [`File`]: crate::ast::File
//! [`NodeOverlay`]: crate::ast::NodeOverlay

pub mod edit;
pub mod irq_method;
