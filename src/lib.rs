///
/// Copyright 2020 New Relic Corporation. All rights reserved.
/// SPDX-License-Identifier: Apache-2.0
///
pub mod attribute;

pub mod span;
pub use span::{Span, SpanBatch};

#[cfg(feature = "client")]
mod client;
#[cfg(feature = "client")]
pub use client::{Client, ClientBuilder};

#[cfg(feature = "blocking")]
pub mod blocking {
    pub use super::client::blocking::Client;
}
