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

//! Client-Application error types.
//!
//! A TEE call can fail at several different distances from the caller, and telling them
//! apart is most of the diagnostic value. [`TeeError`] separates them by *stage* — context
//! initialization (is `/dev/tee0` even there?), session opening (is the TA installed, does
//! the UUID match?), and invocation (did the TA reject the command?) — because each points
//! at a different thing to go fix.
//!
//! [`TeeErrorOrigin`] adds the orthogonal axis for an invocation failure: *who* produced
//! the error code. A failure originating in the client library, the trusted OS, or the TA
//! itself mean very different things, and OP-TEE reports that as `TEEC_ORIGIN_*`.
//!
//! Both are re-declared here rather than re-exported from `optee-teec`, so raw
//! `optee-teec` types stay out of the public API. The original error is preserved as a
//! `#[source]`, so nothing is lost for callers that want it.
//!
//! This module is gated to `ca` on Linux non-test builds: `optee-teec` links against a
//! client library that needs a real TEE device.

use thiserror::Error;

/// Which component produced a TA-returned error code.
///
/// Mirrors OP-TEE's `TEEC_ORIGIN_*` values without leaking raw `optee-teec` types into
/// the public API. The distinction matters when triaging: a `NotTrustedOs` failure points
/// at the client library or the device node, while `TrustedApplication` means the TA ran
/// and refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TeeErrorOrigin {
    /// The client library or the communication path, not the secure world.
    NotTrustedOs,
    /// The trusted OS itself (OP-TEE core).
    TrustedOs,
    /// The Trusted Application returned the error.
    TrustedApplication,
    /// The TEE Client API implementation.
    Api,
    /// An origin this crate does not model.
    Unknown,
}

impl From<optee_teec::ErrorOrigin> for TeeErrorOrigin {
    fn from(o: optee_teec::ErrorOrigin) -> Self {
        match o {
            optee_teec::ErrorOrigin::COMMS => Self::NotTrustedOs,
            optee_teec::ErrorOrigin::TEE => Self::TrustedOs,
            optee_teec::ErrorOrigin::TA => Self::TrustedApplication,
            optee_teec::ErrorOrigin::API => Self::Api,
            _ => Self::Unknown,
        }
    }
}

/// A failure somewhere in a Client-Application TEE call.
///
/// Variants are separated by the *stage* that failed, because each points at a different
/// thing to go fix. See the [module documentation](self) for the reasoning.
#[derive(Debug, Error)]
pub enum TeeError {
    /// Failed to initialise the TEEC context (e.g. /dev/tee0 not present).
    #[error("failed to initialise TEE context: {0}")]
    Context(#[source] optee_teec::Error),

    /// Failed to open a session with the target TA.
    #[error("failed to open TEE session: {0}")]
    Session(#[source] optee_teec::Error),

    /// A command invocation was rejected or failed inside the TA.
    /// Preserves the TA error code and origin for diagnostics.
    #[error("TEE command failed (code={code:#010x}, origin={origin:?})")]
    Invoke {
        /// Raw `TEEC_Result` code returned for the command.
        code: u32,
        /// Which component produced `code`.
        origin: TeeErrorOrigin,
        #[source]
        source: optee_teec::Error,
    },

    /// A Rust type could not be mapped to or from a TEEC parameter slot.
    #[error("TEE parameter type mismatch")]
    ParamMismatch,

    /// The TA signalled that an output buffer was too small.
    #[error("TEE output buffer too small: need {need} bytes, have {have}")]
    BufferTooSmall {
        /// Bytes the TA reported it needed.
        need: usize,
        /// Bytes the caller had allocated, from `TeeParam::MAX_SIZE`.
        have: usize,
    },

    /// More than 4 parameters were passed to a single command.
    #[error("TEE commands accept at most 4 parameters, got {0}")]
    TooManyParams(usize),

    /// The `spawn_blocking` task panicked or was cancelled.
    #[error("TEE async task failed: {0}")]
    JoinError(#[from] tokio::task::JoinError),

    /// All sessions in the pool are busy and the non-blocking path was taken.
    #[error("all TEE sessions are busy")]
    Busy,
}

impl TeeError {
    /// Returns `true` for errors that are worth retrying transparently.
    /// Used by `SessionPool` before surfacing the error to the caller.
    pub fn is_transient(&self) -> bool {
        match self {
            Self::Busy => true,
            Self::Session(e) | Self::Context(e) => {
                matches!(e.kind(), optee_teec::ErrorKind::Busy)
            }
            _ => false,
        }
    }

    /// Returns the raw TA error code if the error originated in the TA.
    /// Useful for service-level error mapping in generated code.
    pub fn ta_error_code(&self) -> Option<u32> {
        match self {
            Self::Invoke { code, .. } => Some(*code),
            _ => None,
        }
    }
}

/// `Result` alias for Client-Application TEE operations.
pub type TeeResult<T> = Result<T, TeeError>;

impl From<optee_teec::Error> for TeeError {
    fn from(e: optee_teec::Error) -> Self {
        let code = e.raw_code();
        let origin = e
            .origin()
            .map(TeeErrorOrigin::from)
            .unwrap_or(TeeErrorOrigin::Unknown);
        TeeError::Invoke {
            code,
            origin,
            source: e,
        }
    }
}
