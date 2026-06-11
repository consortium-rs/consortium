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

/// Marker trait for types that are safe to transmit as cross-core IPC messages.
///
/// Implementors guarantee all field values are:
/// - Sized and ABI-stable
/// - Free of raw pointers, references, and function pointers (which embed address-space
///   assumptions that are meaningless across separate processor cores)
/// - Free of platform-dependent integer widths (`usize`, `isize`)
///
/// Use `#[derive(IpcSafe)]` from `consortium-ipc-macros` to automatically enforce
/// these constraints at compile time. The derive rejects forbidden types anywhere in the
/// field's type tree — including inside generics like `Vec<usize>` or `Option<*const u8>`.
///
/// Manual `impl IpcSafe` is `unsafe` so that out-of-derive impls require the author to
/// acknowledge the ABI contract explicitly.
///
/// # Example
///
/// ```rust,ignore
/// use consortium_ipc_macros::IpcSafe;
///
/// #[derive(IpcSafe)]
/// struct MotorCommand {
///     motor_id: u8,
///     target_rpm: i32,
///     torque_limit: f32,
/// }
/// ```
// SAFETY: implementors must ensure the type has a fixed, platform-independent layout
// with no address-space-local constructs.
pub unsafe trait IpcSafe {}

// Blanket impls for primitive fixed-width, ABI-stable types.
// SAFETY: all listed types have a fixed, platform-independent size and layout.
unsafe impl IpcSafe for u8 {}
unsafe impl IpcSafe for u16 {}
unsafe impl IpcSafe for u32 {}
unsafe impl IpcSafe for u64 {}
unsafe impl IpcSafe for u128 {}
unsafe impl IpcSafe for i8 {}
unsafe impl IpcSafe for i16 {}
unsafe impl IpcSafe for i32 {}
unsafe impl IpcSafe for i64 {}
unsafe impl IpcSafe for i128 {}
unsafe impl IpcSafe for f32 {}
unsafe impl IpcSafe for f64 {}
unsafe impl IpcSafe for bool {}
unsafe impl IpcSafe for () {}

// SAFETY: fixed-size array of an IpcSafe element is itself fixed-size and portable.
unsafe impl<T: IpcSafe, const N: usize> IpcSafe for [T; N] {}

// SAFETY: tuples of IpcSafe elements have a fixed layout.
unsafe impl<T0: IpcSafe> IpcSafe for (T0,) {}
unsafe impl<T0: IpcSafe, T1: IpcSafe> IpcSafe for (T0, T1) {}
unsafe impl<T0: IpcSafe, T1: IpcSafe, T2: IpcSafe> IpcSafe for (T0, T1, T2) {}
unsafe impl<T0: IpcSafe, T1: IpcSafe, T2: IpcSafe, T3: IpcSafe> IpcSafe for (T0, T1, T2, T3) {}
