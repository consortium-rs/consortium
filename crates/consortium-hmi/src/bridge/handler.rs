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

use std::{future::Future, sync::Arc};

use super::ResponseFuture;

#[derive(Clone)]
pub(super) enum Handler {
    Sync(Arc<dyn Fn(String) -> String + Send + Sync + 'static>),
    Async(Arc<dyn Fn(String) -> ResponseFuture + Send + Sync + 'static>),
}

impl Handler {
    pub(super) fn sync(f: impl Fn(String) -> String + Send + Sync + 'static) -> Self {
        Self::Sync(Arc::new(f))
    }

    pub(super) fn async_handler<F, Fut>(f: F) -> Self
    where
        F: Fn(String) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = String> + Send + 'static,
    {
        Self::Async(Arc::new(move |payload| Box::pin(f(payload))))
    }

    pub(super) fn call(self, payload: String) -> ResponseFuture {
        match self {
            Self::Sync(f) => Box::pin(async move { f(payload) }),
            Self::Async(f) => Box::pin(async move { f(payload).await }),
        }
    }
}
