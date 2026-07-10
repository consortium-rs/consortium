// Copyright 2026 Ethan Wu
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// SPDX-License-Identifier: Apache-2.0

//! A tiny logging facade that forwards to [`defmt`], [`tracing`], both, or
//! neither, chosen by Cargo features.
//!
//! Consortium spans `no_std` firmware (which logs with `defmt`) and `std`
//! host-side tooling such as the config codegen (which logs with `tracing`).
//! This crate lets a single call site — `consortium_log::trace!(...)` — compile
//! against whichever backend the consuming crate turns on:
//!
//! - `defmt` routes each macro to the matching `defmt` macro.
//! - `tracing` routes each macro to the matching `tracing` macro. `tracing`
//!   also enables [`alloc`](#features) because the `tracing` dependency needs
//!   it.
//! - With both enabled, every event is emitted to both backends.
//! - With neither enabled, the macros expand to nothing.
//!
//! # Format syntax
//!
//! The macros forward their token stream verbatim, so use only the syntax the
//! enabled backend(s) accept. The portable subset is a format string plus
//! positional arguments — `trace!("channel {} lowered", name)` — where each
//! argument implements the traits both backends require (`defmt::Format` for
//! `defmt`, `core::fmt::Display`/`Debug` for `tracing`). Backend-specific
//! extensions (`defmt`'s `{=u8}` type hints, `tracing`'s structured fields) are
//! only valid when that backend is the sole one enabled.
//!
//! Log arguments must be side-effect free: when no backend is enabled the
//! macros expand to nothing and the arguments are never evaluated.
//!
//! # Features
//!
//! - `defmt` — forward to the `defmt` logging macros. `defmt`'s proc-macros
//!   expand to literal `defmt::…` paths resolved at the call site, so a crate
//!   that enables this backend **must also depend on `defmt` directly** (the
//!   facade cannot supply the name). The workspace's firmware crates already do.
//! - `tracing` — forward to the `tracing` logging macros (implies `alloc`).
//!   `tracing`'s macros are `$crate`-hygienic, so no direct dependency is
//!   needed beyond this crate.
//! - `alloc` — marker capability required by `tracing`; enabling it alone pulls
//!   no backend.

#![no_std]

// Re-export each backend macro *by name* under a hidden alias. Re-exporting the
// individual macros (rather than the whole crate via `pub use defmt;`) is the
// reliable way to make them resolvable through `$crate::…` from a downstream
// crate: a bare crate re-export does not reliably expose a `#[macro_export]`
// macro by path across crates.
#[cfg(feature = "defmt")]
#[doc(hidden)]
pub use defmt::{
    debug as __defmt_debug, error as __defmt_error, info as __defmt_info, trace as __defmt_trace,
    warn as __defmt_warn,
};
#[cfg(feature = "tracing")]
#[doc(hidden)]
pub use tracing::{
    debug as __tracing_debug, error as __tracing_error, info as __tracing_info,
    trace as __tracing_trace, warn as __tracing_warn,
};

// Backend dispatch. `#[cfg]` *inside* an exported macro's body is resolved
// against the *calling* crate's features, not this crate's, so the selection
// must happen where it can see this crate's features: on the definitions of the
// `__defmt`/`__tracing` dispatch macros, gated once each. When a backend is off,
// its dispatch is the no-op arm. The public level macros carry no `#[cfg]`.

/// Routes a level to the aliased `defmt` macro, or nothing when `defmt` is off.
#[cfg(feature = "defmt")]
#[doc(hidden)]
#[macro_export]
macro_rules! __defmt {
    (trace, $($a:tt)*) => { $crate::__defmt_trace!($($a)*) };
    (debug, $($a:tt)*) => { $crate::__defmt_debug!($($a)*) };
    (info,  $($a:tt)*) => { $crate::__defmt_info!($($a)*) };
    (warn,  $($a:tt)*) => { $crate::__defmt_warn!($($a)*) };
    (error, $($a:tt)*) => { $crate::__defmt_error!($($a)*) };
}
#[cfg(not(feature = "defmt"))]
#[doc(hidden)]
#[macro_export]
macro_rules! __defmt {
    ($level:ident, $($a:tt)*) => {{}};
}

/// Routes a level to the aliased `tracing` macro, or nothing when it is off.
#[cfg(feature = "tracing")]
#[doc(hidden)]
#[macro_export]
macro_rules! __tracing {
    (trace, $($a:tt)*) => { $crate::__tracing_trace!($($a)*) };
    (debug, $($a:tt)*) => { $crate::__tracing_debug!($($a)*) };
    (info,  $($a:tt)*) => { $crate::__tracing_info!($($a)*) };
    (warn,  $($a:tt)*) => { $crate::__tracing_warn!($($a)*) };
    (error, $($a:tt)*) => { $crate::__tracing_error!($($a)*) };
}
#[cfg(not(feature = "tracing"))]
#[doc(hidden)]
#[macro_export]
macro_rules! __tracing {
    ($level:ident, $($a:tt)*) => {{}};
}

/// Logs at the `trace` level to every enabled backend. See the
/// [crate docs](crate) for the portable format syntax.
#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {{
        $crate::__defmt!(trace, $($arg)*);
        $crate::__tracing!(trace, $($arg)*);
    }};
}

/// Logs at the `debug` level to every enabled backend.
#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {{
        $crate::__defmt!(debug, $($arg)*);
        $crate::__tracing!(debug, $($arg)*);
    }};
}

/// Logs at the `info` level to every enabled backend.
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {{
        $crate::__defmt!(info, $($arg)*);
        $crate::__tracing!(info, $($arg)*);
    }};
}

/// Logs at the `warn` level to every enabled backend.
#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {{
        $crate::__defmt!(warn, $($arg)*);
        $crate::__tracing!(warn, $($arg)*);
    }};
}

/// Logs at the `error` level to every enabled backend.
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {{
        $crate::__defmt!(error, $($arg)*);
        $crate::__tracing!(error, $($arg)*);
    }};
}

// Exercised with the `tracing` backend only: a `defmt`-enabled test binary would
// need a linked `#[defmt::global_logger]`, which a plain unit test cannot supply.
#[cfg(all(test, feature = "tracing", not(feature = "defmt")))]
mod tests {
    // The macros forward the portable subset (format string + positional args)
    // to the enabled backend; here that is `tracing`. This exercises each level
    // and each supported arity so a broken expansion fails to compile.
    #[test]
    fn every_level_forwards() {
        let name = "cm33";
        let channel = 3u8;

        crate::trace!("no args");
        crate::debug!("one arg {}", name);
        crate::info!("two args {} {}", name, channel);
        crate::warn!("warn {}", channel);
        crate::error!("error {}", name);
    }
}
