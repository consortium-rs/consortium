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

#![cfg_attr(not(feature = "std"), no_std)]

//! An SCMI agent (client) for Consortium's platform-management traits.
//!
//! SCMI — Arm's System Control and Management Interface (DEN0056) — is how an
//! agent asks a platform for clocks, pins, power and resets it does not own. On
//! i.MX 95 the platform is the System Manager on the CM33, and the M7 is one of
//! its agents. [`Client`] speaks the agent side and implements
//! `consortium-pmi`'s [`ClockProvider`], [`PinctrlProvider`], [`PowerProvider`]
//! and [`ResetProvider`] on top of it.
//!
//! # Transport
//!
//! The transport is the Shared Memory Transport ([`smt`]): a mailbox window the
//! agent writes a request into, plus a doorbell to tell the platform it is there.
//! Both are `consortium-core` primitives — the window is any [`Region`], the
//! doorbell any [`Doorbell`] — so a chip binding is a choice of region and bell
//! (an i.MX MU, an STM32 IPCC, an Arm MHU), not new protocol code. Completion is
//! either a second doorbell line or polling the channel's free bit
//! ([`Completion`]).
//!
//! [`SmtTransport`] packages the window and doorbell as a Consortium transport.
//! One exchange is `consortium-core`'s non-waiting pair — hand a request over if
//! the channel is free, read the reply once the platform frees it again — so the
//! async wait ([`Client::call`]) and the blocking one ([`Client::blocking_call`],
//! for a synchronous `init()`) are core's `park_until` and `spin_until` over the
//! same attempt, exactly as the IPC shared-memory transport's are.
//!
//! # Layout
//!
//! | Module      | Contents                                                    |
//! | ----------- | ----------------------------------------------------------- |
//! | [`header`]  | The 32-bit message header: message id, type, protocol, token |
//! | [`status`]  | The status codes a response carries                         |
//! | [`smt`]     | The SMT channel layout and its volatile accessors           |
//! | [`transport`] | [`SmtTransport`]: the channel and doorbell as a try transport |
//! | [`client`]  | [`Client`] and the BASE/CLOCK/PINCTRL/POWER/RESET commands   |
//!
//! Only the agent role exists so far. The platform (server) role — a Consortium
//! core acting as the system manager for others — is planned and fits the same
//! header, status and SMT layers.
//!
//! [`ClockProvider`]: consortium_pmi::ClockProvider
//! [`PinctrlProvider`]: consortium_pmi::PinctrlProvider
//! [`PowerProvider`]: consortium_pmi::PowerProvider
//! [`ResetProvider`]: consortium_pmi::ResetProvider
//! [`Region`]: consortium_core::Region
//! [`Doorbell`]: consortium_core::Doorbell
//!
//! # Features
//!
//! - `std` — provider futures are `Send`.
//! - `defmt` — derive `defmt::Format` on the protocol and error types.

pub mod client;
pub mod error;
pub mod header;
pub mod smt;
pub mod status;
pub mod transport;

pub use {
    client::{Client, Completion},
    error::{Error, ProtocolError},
    header::{MessageHeader, MessageType, ProtocolId},
    smt::{Smt, SmtError},
    status::Status,
    transport::SmtTransport,
};
