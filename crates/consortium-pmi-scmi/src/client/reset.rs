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

//! The RESET protocol (0x16), as [`ResetProvider`].

use consortium_core::{Doorbell, MaybeSend, Region};
use consortium_pmi::{ResetId, ResetProvider};

use super::{Client, words};
use crate::{Error, ProtocolId};

/// `RESET` (the protocol's one action message).
pub const RESET: u8 = 0x4;

/// Flags bit 0: assert then release autonomously.
const AUTONOMOUS: u32 = 1 << 0;
/// Flags bit 1: explicitly assert (cleared: explicitly release).
const EXPLICIT_ASSERT: u32 = 1 << 1;
/// Reset state: architectural cold reset.
const ARCH_COLD_RESET: u32 = 0;

impl<R: Region, D: Doorbell> Client<R, D> {
    async fn reset_domain(&mut self, id: ResetId, flags: u32) -> Result<(), Error<D::Error>> {
        let req = words::<12>(&[id.0, flags, ARCH_COLD_RESET]);
        self.call(ProtocolId::RESET, RESET, &req, &mut []).await?;
        Ok(())
    }
}

impl<R, D> ResetProvider for Client<R, D>
where
    R: Region + MaybeSend,
    D: Doorbell + MaybeSend,
{
    type Error = Error<D::Error>;

    fn assert(&mut self, id: ResetId) -> impl Future<Output = Result<(), Self::Error>> + MaybeSend {
        self.reset_domain(id, EXPLICIT_ASSERT)
    }

    fn deassert(
        &mut self,
        id: ResetId,
    ) -> impl Future<Output = Result<(), Self::Error>> + MaybeSend {
        self.reset_domain(id, 0)
    }

    fn reset(&mut self, id: ResetId) -> impl Future<Output = Result<(), Self::Error>> + MaybeSend {
        self.reset_domain(id, AUTONOMOUS)
    }
}
