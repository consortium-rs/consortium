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

use std::{future::Future, pin::Pin};

pub(super) type BoxFuture = Pin<Box<dyn Future<Output = String> + Send + 'static>>;

pub(super) enum Handler {
    Sync(Box<dyn Fn(String) -> String + Send + 'static>),
    Async(Box<dyn Fn(String) -> BoxFuture + Send + 'static>),
}

impl Handler {
    pub(super) fn sync(f: impl Fn(String) -> String + Send + 'static) -> Self {
        Self::Sync(Box::new(f))
    }

    pub(super) fn async_handler<F, Fut>(f: F) -> Self
    where
        F: Fn(String) -> Fut + Send + 'static,
        Fut: Future<Output = String> + Send + 'static,
    {
        Self::Async(Box::new(move |payload| Box::pin(f(payload))))
    }
}
