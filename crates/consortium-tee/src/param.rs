use crate::error::{TeeError, TeeResult};
use optee_teec::{ParamNone, ParamTmpRef, ParamType, ParamValue};

/// Intermediate representation of a single TEEC parameter.
/// Insulates the rest of the crate (and generated macro code)
/// from raw optee-teec types.
pub enum TeeParam<'a> {
    TmpRefInput(&'a [u8]),
    TmpRefOutput(Vec<u8>), // internally managed output buffer
    ValueInput(u32, u32),  // (a, b)
    ValueInout(u32, u32),
    ValueOutput,
    None,
}

// ── IntoTeeParam ────────────────────────────────────────────────────────────

/// Convert a Rust type into a `TeeParam` for passing into a TEE command.
pub trait IntoTeeParam {
    fn into_tee_param(&self) -> TeeParam<'_>;
}

impl IntoTeeParam for &[u8] {
    fn into_tee_param(&self) -> TeeParam<'_> {
        TeeParam::TmpRefInput(self)
    }
}

impl IntoTeeParam for Vec<u8> {
    fn into_tee_param(&self) -> TeeParam<'_> {
        TeeParam::TmpRefInput(self.as_slice())
    }
}

impl IntoTeeParam for u32 {
    fn into_tee_param(&self) -> TeeParam<'_> {
        TeeParam::ValueInput(*self, 0)
    }
}

impl IntoTeeParam for u64 {
    /// Splits u64 across (a, b) value pair — high word in a, low word in b.
    fn into_tee_param(&self) -> TeeParam<'_> {
        TeeParam::ValueInput((*self >> 32) as u32, *self as u32)
    }
}

impl IntoTeeParam for bool {
    fn into_tee_param(&self) -> TeeParam<'_> {
        TeeParam::ValueInput(*self as u32, 0)
    }
}

// ── FromTeeParam ─────────────────────────────────────────────────────────────

/// Extract a Rust type from a `TeeParam` returned by a TEE command.
pub trait FromTeeParam: Sized {
    fn from_tee_param(param: TeeParam<'_>) -> TeeResult<Self>;
}

impl FromTeeParam for Vec<u8> {
    fn from_tee_param(param: TeeParam<'_>) -> TeeResult<Self> {
        match param {
            TeeParam::TmpRefOutput(buf) => Ok(buf),
            _ => Err(TeeError::ParamMismatch),
        }
    }
}

impl FromTeeParam for u32 {
    /// Reads from the b-slot of a ValueInout — a-slot carries the input,
    /// b-slot carries the TA's return value.
    fn from_tee_param(param: TeeParam<'_>) -> TeeResult<Self> {
        match param {
            TeeParam::ValueInout(_, b) => Ok(b),
            TeeParam::ValueOutput => Err(TeeError::ParamMismatch),
            _ => Err(TeeError::ParamMismatch),
        }
    }
}

impl FromTeeParam for u64 {
    /// Reassembles from (a, b) — inverse of IntoTeeParam for u64.
    fn from_tee_param(param: TeeParam<'_>) -> TeeResult<Self> {
        match param {
            TeeParam::ValueInout(a, b) => Ok((a as u64) << 32 | b as u64),
            _ => Err(TeeError::ParamMismatch),
        }
    }
}

impl FromTeeParam for bool {
    fn from_tee_param(param: TeeParam<'_>) -> TeeResult<Self> {
        match param {
            TeeParam::ValueInout(_, b) => Ok(b != 0),
            _ => Err(TeeError::ParamMismatch),
        }
    }
}

// ── Raw TEEC materialisation ─────────────────────────────────────────────────

/// Internal: materialise a slice of `TeeParam` into a raw TEEC `Operation`.
/// Called only from `session.rs` — not part of the public API.
pub(crate) struct MaterialisedParams {
    // Holds the four TEEC parameter slots as a fixed array.
    // optee-teec requires exactly four params per operation.
    pub slots: [RawParamSlot; 4],
}

pub(crate) enum RawParamSlot {
    TmpRefInput { ptr: *const u8, len: usize },
    TmpRefOutput { buf: Vec<u8> },
    Value { a: u32, b: u32, kind: ParamType },
    None,
}

impl MaterialisedParams {
    pub(crate) fn from_params(params: Vec<TeeParam<'_>>) -> TeeResult<Self> {
        if params.len() > 4 {
            return Err(TeeError::ParamMismatch);
        }

        let mut slots = [
            RawParamSlot::None,
            RawParamSlot::None,
            RawParamSlot::None,
            RawParamSlot::None,
        ];

        for (i, p) in params.into_iter().enumerate() {
            slots[i] = match p {
                TeeParam::TmpRefInput(s) => RawParamSlot::TmpRefInput {
                    ptr: s.as_ptr(),
                    len: s.len(),
                },
                TeeParam::TmpRefOutput(buf) => RawParamSlot::TmpRefOutput { buf },
                TeeParam::ValueInput(a, b) => RawParamSlot::Value {
                    a,
                    b,
                    kind: ParamType::ValueInput,
                },
                TeeParam::ValueInout(a, b) => RawParamSlot::Value {
                    a,
                    b,
                    kind: ParamType::ValueInout,
                },
                TeeParam::ValueOutput => RawParamSlot::Value {
                    a: 0,
                    b: 0,
                    kind: ParamType::ValueOutput,
                },
                TeeParam::None => RawParamSlot::None,
            };
        }

        Ok(Self { slots })
    }

    /// Extracts output params back into `TeeParam` after invocation,
    /// so `FromTeeParam` impls can consume them.
    pub(crate) fn extract_outputs(self) -> Vec<TeeParam<'static>> {
        self.slots
            .into_iter()
            .map(|slot| match slot {
                RawParamSlot::TmpRefOutput { buf } => TeeParam::TmpRefOutput(buf),
                RawParamSlot::Value {
                    a,
                    b,
                    kind: ParamType::ValueInout,
                }
                | RawParamSlot::Value {
                    a,
                    b,
                    kind: ParamType::ValueOutput,
                } => TeeParam::ValueInout(a, b),
                _ => TeeParam::None,
            })
            .collect()
    }
}
