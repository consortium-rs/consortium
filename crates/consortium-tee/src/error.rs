// consortium-tee/src/error.rs

use thiserror::Error;

/// The origin of a TA-returned error, mirroring TEEC_Origins
/// without leaking raw optee-teec types into the public API.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TeeErrorOrigin {
    NotTrustedOs,
    TrustedOs,
    TrustedApplication,
    Api,
    Unknown,
}

impl From<optee_teec::ErrorOrigin> for TeeErrorOrigin {
    fn from(o: optee_teec::ErrorOrigin) -> Self {
        match o {
            optee_teec::ErrorOrigin::NotTrustedOs => Self::NotTrustedOs,
            optee_teec::ErrorOrigin::TrustedOs => Self::TrustedOs,
            optee_teec::ErrorOrigin::TrustedApplication => Self::TrustedApplication,
            optee_teec::ErrorOrigin::Api => Self::Api,
            _ => Self::Unknown,
        }
    }
}

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
        code: u32,
        origin: TeeErrorOrigin,
        #[source]
        source: optee_teec::Error,
    },

    /// A Rust type could not be mapped to or from a TEEC parameter slot.
    #[error("TEE parameter type mismatch")]
    ParamMismatch,

    /// The TA signalled that an output buffer was too small.
    #[error("TEE output buffer too small: need {need} bytes, have {have}")]
    BufferTooSmall { need: usize, have: usize },

    /// More than 4 parameters were passed to a single command.
    #[error("TEE commands accept at most 4 parameters, got {0}")]
    TooManyParams(usize),

    /// CA-side `postcard` serialization of a `TeeParam` argument failed.
    #[error("failed to encode TEE parameter: {0}")]
    Encode(String),

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

pub type TeeResult<T> = Result<T, TeeError>;
