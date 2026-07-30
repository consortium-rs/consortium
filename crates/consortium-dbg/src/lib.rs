#![cfg_attr(not(feature = "host"), no_std)]

//! `defmt` logging over shared memory: the `CDBG` ring, its firmware-side
//! [`global_logger`], its host-side reader/decoder, and an optional panic handler.
//!
//! [`global_logger`]: https://docs.rs/defmt/latest/defmt/attr.global_logger.html
//!
//! # The problem
//!
//! A coprocessor on a Consortium SoC usually has no UART of its own to log
//! through, or has one already spoken for. But it does share memory with a
//! Linux-capable core that has a filesystem, a clock, and somewhere to put logs.
//! So the debug path runs the same way as everything else here: firmware writes
//! into a shared window, Linux reads it.
//!
//! `defmt` is the right encoder for the firmware end. Format strings and type
//! metadata stay in the ELF and never reach the wire, so a log line costs a few
//! bytes and no formatting code — which is what makes logging affordable on a
//! Cortex-M at all. The cost is that the bytes are meaningless without the ELF
//! they came from, so the host side needs a decoder.
//!
//! # Architecture
//!
//! ```text
//!  firmware (no_std)                        Linux / host
//!  ────────────────────                     ─────────────────────
//!  defmt::info!(..)
//!    └─ logger::ConsortiumLogger            host::HostRing
//!         (global_logger; rzcobs-encodes      (validate CDBG, drain frames)
//!          one frame, wraps it in a             └─ host::DefmtSession
//!          packet, appends to the ring)             (decode against the ELF)
//!                    │                                      │
//!                    └────────  CDBG shared window  ────────┘
//!                          descriptor + circular buffer
//!  panic::(handler)  ─ latches Flags::Panic in the descriptor
//! ```
//!
//! | Module          | Role                                                            |
//! | --------------- | --------------------------------------------------------------- |
//! | [`config`]      | Compile-time region base/length from `CONSORTIUM_DBG_*`.          |
//! | [`descriptor`]  | The `CDBG` descriptor and single-writer/single-reader ring rules.  |
//! | [`packet`]      | Per-frame envelope inside the ring.                              |
//! | `logger`        | Firmware `#[global_logger]`; encodes and appends.                 |
//! | `host`          | Host-side validation, draining, and `defmt` decoding.             |
//! | `panic`         | Optional `#[panic_handler]` that records a crash in the ring.      |
//!
//! # Descriptor discipline
//!
//! The ring has exactly one writer (firmware) and one reader (host), and there is
//! no lock. What keeps it correct is that **each side only mutates the fields it
//! owns**: the writer owns `write_ptr`, `capacity`, and `flags`; the reader owns
//! `read_ptr`; `write_ptr == read_ptr` means empty. Preserve that split when
//! changing anything here — it is the whole synchronization story.
//!
//! # Configuration
//!
//! Layout comes from the builder, not from the firmware source. `csti build`
//! lowers `[dbg.<core>]` in `Consortium.toml` into `CONSORTIUM_DBG_BASE` and
//! `CONSORTIUM_DBG_LENGTH`, which [`config`] reads with `option_env!` and parses
//! in a `const` context.
//!
//! Under a plain `cargo build` those variables are absent, so
//! [`config::CONFIGURED`] is `false` and the logger is a **no-op** rather than a
//! build failure — which is what lets firmware crates be checked without the
//! pipeline. The trade-off is that an image built without the pipeline logs nothing,
//! silently; firmware that must not ship that way should `assert!` on
//! [`config::CONFIGURED`] in a `const` item. See [`config`] for the snippet.
//!
//! # Features
//!
//! Defaults are `host` and `logger`.
//!
//! | Feature         | Effect                                                         |
//! | --------------- | -------------------------------------------------------------- |
//! | `logger`        | Firmware `#[global_logger]` writing into the ring.               |
//! | `host`          | Host-side `HostRing` reader; implies `std`.     |
//! | `decode`        | Adds `defmt-decoder`/`defmt-parser` for `host::DefmtSession`; implies `host`. |
//! | `panic-handler` | Installs `#[panic_handler]`; implies `logger`.                    |
//! | `arch-cortex-m` | Interrupt-disable and halt for Cortex-M. Required by `panic-handler`. |
//! | `arch-riscv`    | Interrupt-disable and halt for RISC-V. Required by `panic-handler`.   |
//! | `doorbell-hsem` / `doorbell-mu` | Optional doorbell integration.                   |
//!
//! `panic-handler` needs **exactly one** arch feature, and the two arch features
//! are mutually exclusive; both conditions are `compile_error!`s below rather than
//! link-time surprises.
//!
//! # Testing
//!
//! ```bash
//! cargo test -p consortium-dbg              # descriptor, ring, packet
//! cargo test -p consortium-dbg --features decode
//! just test dbg host
//! ```
//!
//! Host-side tests cover descriptor initialization, ring read/write behaviour,
//! decoding fixtures, and the overflow/error flags — no target hardware needed.

pub mod config;
pub mod descriptor;
#[cfg(feature = "host")]
pub mod host;

/// Re-export of the defmt stream decoder, so host consumers (e.g.
/// `consortium-runtime-app`) can name `host::DefmtSession` callback types
/// (`Frame`) without a separate `defmt-decoder` dependency.
#[cfg(feature = "decode")]
pub use defmt_decoder;
/// Re-export of `defmt-parser`, whose [`Level`](defmt_parser::Level) is the type
/// returned by `Frame::level()` (defmt-decoder does not re-export it publicly).
#[cfg(feature = "decode")]
pub use defmt_parser;
#[cfg(feature = "logger")]
pub mod logger;
pub mod packet;
#[cfg(feature = "panic-handler")]
pub mod panic;
pub mod utils;

#[cfg(all(
    feature = "panic-handler",
    not(any(feature = "arch-cortex-m", feature = "arch-riscv"))
))]
compile_error!(
    "`panic-handler` requires exactly one arch feature: enable `arch-cortex-m` or `arch-riscv`"
);

#[cfg(all(feature = "arch-cortex-m", feature = "arch-riscv"))]
compile_error!("`arch-cortex-m` and `arch-riscv` are mutually exclusive");
