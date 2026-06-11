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

// `TeeParam` and the derive macros are ungated from the Linux restriction - they must be
// available on every target (TA code running in the TEE secure world is not Linux).
// Error types and the TEE runtime (optee-teec) remain Linux-only.
#![no_std]
extern crate alloc;

use alloc::vec::Vec;

/// Error returned by [`TeeParam::into_tee_param`] and [`TeeParam::from_tee_param`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TeeParamError {
    /// Serialization failed (e.g. buffer too small, type mismatch).
    EncodeFailed,
    /// Deserialization from the provided representation failed.
    DecodeFailed,
}

/// Wire representation of a single TEE parameter slot, generic over the byte-buffer type `B`.
///
/// - `Primitive { a, b }` maps to a TEE **Value** slot — two `u32` fields, no serialization.
///   Integers, booleans, and pairs use this variant. A `u16` becomes `Primitive { a: v as u32, b: 0 }`;
///   nothing is forced through `[u8; 2]`.
/// - `Serialized(B)` maps to a TEE **Memref** slot — opaque bytes produced or consumed by a
///   [`consortium_common::codec::CodecFor`] implementation.
///
/// Two type aliases expose the canonical directions:
/// - [`TeeParamRepr`] — owned (`B = Vec<u8>`), produced by [`TeeParam::into_tee_param`].
/// - [`TeeParamReprRef`] — borrowed (`B = &[u8]`), consumed by [`TeeParam::from_tee_param`].
#[derive(Debug, Clone)]
pub enum TeeParamSlot<B> {
    /// Fits in the two `u32` fields of a TEE `Value` parameter slot.
    Primitive { a: u32, b: u32 },
    /// Carried as opaque bytes in a TEE `Memref` parameter slot.
    Serialized(B),
}

/// Owned TEE parameter representation, returned by [`TeeParam::into_tee_param`].
pub type TeeParamRepr = TeeParamSlot<Vec<u8>>;

/// Borrowed TEE parameter representation, passed to [`TeeParam::from_tee_param`].
pub type TeeParamReprRef<'a> = TeeParamSlot<&'a [u8]>;

/// Marker codec type used when no custom codec is needed (primitive-only commands).
///
/// `TeeParam<NoCodec>` is automatically satisfied by all primitive impls; custom types
/// would produce a compile error pointing to the missing `CodecFor<T>` impl if
/// `NoCodec` is used as the codec.
pub struct NoCodec;

/// Trait for types that can be carried through a TEE parameter slot.
///
/// The codec type `C` is threaded through the signature so that:
/// - **Primitive** types (`u8`…`i64`, `bool`, `(u32, u32)`) implement `TeeParam<C>` for
///   *any* `C` — the codec is never invoked; the type maps to a [`TeeParamSlot::Primitive`].
/// - **Serialized** types implement `TeeParam<C>` for a specific codec `C:
///   CodecFor<Self>` — the codec encodes/decodes the value into
///   [`TeeParamSlot::Serialized`] bytes.
///
/// Use `#[derive(TeeParam)]` from `consortium-tee-macros` to automatically
/// generate the serialized impl. The codec is NOT baked into the derive; it is
/// specified once at the call site with `#[tee_command(codec = MyCodec)]`.
///
/// # Example
///
/// ```rust,ignore
/// use consortium_tee::{TeeParam, tee_command};
///
/// #[derive(TeeParam, serde::Serialize, serde::Deserialize)]
/// struct MotorCommand { motor_id: u8, target_rpm: i32 }
///
/// #[tee_command(codec = PostcardCodec)]
/// fn set_motor(cmd: MotorCommand) { /* … */ }
/// ```
///
/// `MAX_SIZE` bounds the Memref buffer pre-allocated on the CA side. The default
/// from `#[derive(TeeParam)]` is 1024 bytes; override with
/// `#[tee(max_size = N)]`.
pub trait TeeParam<C = NoCodec>: Sized {
    /// Maximum serialized size in bytes (only meaningful for `Serialized` variants).
    const MAX_SIZE: usize;

    /// Convert `self` into a [`TeeParamRepr`] suitable for packing into a TEE slot.
    fn into_tee_param(&self) -> Result<TeeParamRepr, TeeParamError>;

    /// Reconstruct `Self` from a [`TeeParamReprRef`] extracted from a TEE slot.
    fn from_tee_param(repr: TeeParamReprRef<'_>) -> Result<Self, TeeParamError>;
}

// ── Primitive blanket impls ──────────────────────────────────────────────────
//
// All primitive types implement TeeParam<C> for *any* C.  The codec type is
// phantom — primitives never serialize through a codec.

macro_rules! impl_tee_param_primitive {
    ($ty:ty, max: $max:expr, a: $a:expr, b: $b:expr, from: $from:expr) => {
        impl<C> TeeParam<C> for $ty {
            const MAX_SIZE: usize = $max;

            fn into_tee_param(&self) -> Result<TeeParamRepr, TeeParamError> {
                let v = *self;
                Ok(TeeParamSlot::Primitive {
                    a: ($a)(v),
                    b: ($b)(v),
                })
            }

            fn from_tee_param(repr: TeeParamReprRef<'_>) -> Result<Self, TeeParamError> {
                match repr {
                    TeeParamSlot::Primitive { a, b } => Ok(($from)(a, b)),
                    TeeParamSlot::Serialized(_) => Err(TeeParamError::DecodeFailed),
                }
            }
        }
    };
}

impl_tee_param_primitive!(u8,   max: 4, a: |v: u8|   v as u32, b: |_| 0u32, from: |a: u32, _: u32| a as u8);
impl_tee_param_primitive!(u16,  max: 4, a: |v: u16|  v as u32, b: |_| 0u32, from: |a: u32, _: u32| a as u16);
impl_tee_param_primitive!(u32,  max: 4, a: |v: u32|  v,        b: |_| 0u32, from: |a: u32, _: u32| a);
impl_tee_param_primitive!(i8,   max: 4, a: |v: i8|   v as u32, b: |_| 0u32, from: |a: u32, _: u32| a as i8);
impl_tee_param_primitive!(i16,  max: 4, a: |v: i16|  v as u32, b: |_| 0u32, from: |a: u32, _: u32| a as i16);
impl_tee_param_primitive!(i32,  max: 4, a: |v: i32|  v as u32, b: |_| 0u32, from: |a: u32, _: u32| a as i32);
impl_tee_param_primitive!(bool, max: 4, a: |v: bool| v as u32, b: |_| 0u32, from: |a: u32, _: u32| a != 0);

impl<C> TeeParam<C> for u64 {
    const MAX_SIZE: usize = 8;
    fn into_tee_param(&self) -> Result<TeeParamRepr, TeeParamError> {
        Ok(TeeParamSlot::Primitive {
            a: *self as u32,
            b: (*self >> 32) as u32,
        })
    }
    fn from_tee_param(repr: TeeParamReprRef<'_>) -> Result<Self, TeeParamError> {
        match repr {
            TeeParamSlot::Primitive { a, b } => Ok((b as u64) << 32 | a as u64),
            TeeParamSlot::Serialized(_) => Err(TeeParamError::DecodeFailed),
        }
    }
}

impl<C> TeeParam<C> for i64 {
    const MAX_SIZE: usize = 8;
    fn into_tee_param(&self) -> Result<TeeParamRepr, TeeParamError> {
        let v = *self as u64;
        Ok(TeeParamSlot::Primitive {
            a: v as u32,
            b: (v >> 32) as u32,
        })
    }
    fn from_tee_param(repr: TeeParamReprRef<'_>) -> Result<Self, TeeParamError> {
        match repr {
            TeeParamSlot::Primitive { a, b } => Ok(((b as u64) << 32 | a as u64) as i64),
            TeeParamSlot::Serialized(_) => Err(TeeParamError::DecodeFailed),
        }
    }
}

impl<C> TeeParam<C> for (u32, u32) {
    const MAX_SIZE: usize = 8;
    fn into_tee_param(&self) -> Result<TeeParamRepr, TeeParamError> {
        Ok(TeeParamSlot::Primitive {
            a: self.0,
            b: self.1,
        })
    }
    fn from_tee_param(repr: TeeParamReprRef<'_>) -> Result<Self, TeeParamError> {
        match repr {
            TeeParamSlot::Primitive { a, b } => Ok((a, b)),
            TeeParamSlot::Serialized(_) => Err(TeeParamError::DecodeFailed),
        }
    }
}

impl<C> TeeParam<C> for Vec<u8> {
    const MAX_SIZE: usize = 4096;
    fn into_tee_param(&self) -> Result<TeeParamRepr, TeeParamError> {
        Ok(TeeParamSlot::Serialized(self.clone()))
    }
    fn from_tee_param(repr: TeeParamReprRef<'_>) -> Result<Self, TeeParamError> {
        match repr {
            TeeParamSlot::Serialized(b) => Ok(b.to_vec()),
            TeeParamSlot::Primitive { .. } => Err(TeeParamError::DecodeFailed),
        }
    }
}

pub use consortium_tee_macros::{TeeParam, tee_command, tee_service};

#[cfg(all(target_os = "linux", feature = "ca"))]
pub mod error;
#[cfg(all(target_os = "linux", feature = "ca"))]
pub use error::{TeeError, TeeErrorOrigin, TeeResult};

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    struct OtherCodec;

    fn assert_primitive<T>(value: T, a: u32, b: u32, max_size: usize)
    where
        T: TeeParam + TeeParam<OtherCodec> + Copy + core::fmt::Debug + PartialEq,
    {
        assert_eq!(<T as TeeParam>::MAX_SIZE, max_size);

        match <T as TeeParam>::into_tee_param(&value).expect("encode primitive") {
            TeeParamSlot::Primitive { a: got_a, b: got_b } => {
                assert_eq!((got_a, got_b), (a, b));
            }
            TeeParamSlot::Serialized(_) => panic!("primitive encoded as serialized slot"),
        }

        let decoded = <T as TeeParam>::from_tee_param(TeeParamSlot::Primitive { a, b })
            .expect("decode primitive");
        assert_eq!(decoded, value);

        let decoded_with_other_codec =
            <T as TeeParam<OtherCodec>>::from_tee_param(TeeParamSlot::Primitive { a, b })
                .expect("decode primitive with arbitrary codec");
        assert_eq!(decoded_with_other_codec, value);

        assert_eq!(
            <T as TeeParam>::from_tee_param(TeeParamSlot::Serialized(&[1, 2, 3])),
            Err(TeeParamError::DecodeFailed)
        );
    }

    #[test]
    fn small_primitives_round_trip_through_primitive_slots() {
        assert_primitive(0xABu8, 0xAB, 0, 4);
        assert_primitive(0xABCDu16, 0xABCD, 0, 4);
        assert_primitive(0xABCD_EF12u32, 0xABCD_EF12, 0, 4);
        assert_primitive(-5i8, (-5i8) as u32, 0, 4);
        assert_primitive(-1234i16, (-1234i16) as u32, 0, 4);
        assert_primitive(-123_456i32, (-123_456i32) as u32, 0, 4);
        assert_primitive(true, 1, 0, 4);
        assert_primitive(false, 0, 0, 4);
    }

    #[test]
    fn wide_primitives_split_low_bits_into_a_and_high_bits_into_b() {
        assert_primitive(0x1122_3344_5566_7788u64, 0x5566_7788, 0x1122_3344, 8);
        assert_primitive(-0x1122_3344_5566_778i64, 0xBAA9_9888, 0xFEED_DCCB, 8);
    }

    #[test]
    fn u32_pair_round_trips_through_both_value_words() {
        let value = (0xCAFE_BABEu32, 0xFEED_FACEu32);

        match <(u32, u32) as TeeParam>::into_tee_param(&value).expect("encode pair") {
            TeeParamSlot::Primitive { a, b } => {
                assert_eq!((a, b), value);
            }
            TeeParamSlot::Serialized(_) => panic!("pair encoded as serialized slot"),
        }

        let decoded = <(u32, u32) as TeeParam>::from_tee_param(TeeParamSlot::Primitive {
            a: value.0,
            b: value.1,
        })
        .expect("decode pair");
        assert_eq!(decoded, value);
        assert_eq!(<(u32, u32) as TeeParam>::MAX_SIZE, 8);
    }

    #[test]
    fn vec_u8_uses_serialized_memref_representation() {
        let value = vec![1, 2, 3, 5, 8, 13];

        match <Vec<u8> as TeeParam>::into_tee_param(&value).expect("encode vec") {
            TeeParamSlot::Serialized(bytes) => {
                assert_eq!(bytes, value);
            }
            TeeParamSlot::Primitive { .. } => panic!("vec encoded as primitive slot"),
        }

        let decoded = <Vec<u8> as TeeParam>::from_tee_param(TeeParamSlot::Serialized(&value))
            .expect("decode vec");
        assert_eq!(decoded, value);
        assert_eq!(<Vec<u8> as TeeParam>::MAX_SIZE, 4096);
    }

    #[test]
    fn wrong_slot_kinds_return_decode_failed() {
        assert_eq!(
            <u32 as TeeParam>::from_tee_param(TeeParamSlot::Serialized(&[0, 1, 2, 3])),
            Err(TeeParamError::DecodeFailed)
        );
        assert_eq!(
            <Vec<u8> as TeeParam>::from_tee_param(TeeParamSlot::Primitive { a: 1, b: 2 }),
            Err(TeeParamError::DecodeFailed)
        );
    }
}
