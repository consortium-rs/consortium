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

pub trait Codec: core::marker::Sized {
    type Error: core::fmt::Debug;
}

/// Marker trait for types that are safe to transmit as cross-core IPC proto messages.
///
/// Implementors guarantee all field values are:
/// - Sized and ABI-stable
/// - Free of raw pointers, references, and function pointers (which embed address-space
///   assumptions that are meaningless across separate processor cores)
/// - Free of platform-dependent integer widths (`usize`, `isize`)
///
/// Use `#[derive(IpcProto)]` from `consortium-codec-proc-macros` to automatically enforce
/// these constraints at compile time. The derive rejects forbidden types anywhere in the
/// field's type tree — including inside generics like `Vec<usize>` or `Option<*const u8>`.
///
/// Manual `impl IpcProto` is `unsafe` so that out-of-derive impls require the author to
/// acknowledge the ABI contract explicitly.
///
/// # Example
///
/// ```rust,ignore
/// use consortium_codec_proc_macros::IpcProto;
///
/// #[derive(IpcProto)]
/// struct MotorCommand {
///     motor_id: u8,
///     target_rpm: i32,
///     torque_limit: f32,
/// }
/// ```
// SAFETY: implementors must ensure the type has a fixed, platform-independent layout
// with no address-space-local constructs.
pub unsafe trait IpcProto {}

// Blanket impls for primitive fixed-width, ABI-stable types.
// SAFETY: all listed types have a fixed, platform-independent size and layout.
unsafe impl IpcProto for u8 {}
unsafe impl IpcProto for u16 {}
unsafe impl IpcProto for u32 {}
unsafe impl IpcProto for u64 {}
unsafe impl IpcProto for u128 {}
unsafe impl IpcProto for i8 {}
unsafe impl IpcProto for i16 {}
unsafe impl IpcProto for i32 {}
unsafe impl IpcProto for i64 {}
unsafe impl IpcProto for i128 {}
unsafe impl IpcProto for f32 {}
unsafe impl IpcProto for f64 {}
unsafe impl IpcProto for bool {}
unsafe impl IpcProto for () {}

// SAFETY: fixed-size array of an IpcProto element is itself fixed-size and portable.
unsafe impl<T: IpcProto, const N: usize> IpcProto for [T; N] {}

// SAFETY: tuples of IpcProto elements have a fixed layout.
unsafe impl<T0: IpcProto> IpcProto for (T0,) {}
unsafe impl<T0: IpcProto, T1: IpcProto> IpcProto for (T0, T1) {}
unsafe impl<T0: IpcProto, T1: IpcProto, T2: IpcProto> IpcProto for (T0, T1, T2) {}
unsafe impl<T0: IpcProto, T1: IpcProto, T2: IpcProto, T3: IpcProto> IpcProto for (T0, T1, T2, T3) {}

pub trait CodecFor<T>: Codec {
    type Decoded<'buf>: 'buf
    where
        T: 'buf;

    fn encode(msg: &T, buf: &mut [u8]) -> core::result::Result<usize, Self::Error>;

    fn decode<'buf>(buf: &'buf [u8]) -> core::result::Result<Self::Decoded<'buf>, Self::Error>
    where
        T: 'buf;
}
