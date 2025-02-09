// Copyright 2025 KhulnaSoft, Ltd
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![warn(clippy::all)]
#![allow(clippy::new_without_default)]
#![allow(clippy::type_complexity)]
#![allow(clippy::match_wild_err_arm)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::upper_case_acronyms)]
// This enables the feature that labels modules that are only available with
// certain bongonet features
#![cfg_attr(docsrs, feature(doc_cfg))]

//! # Bongonet
//!
//! Bongonet is a framework to build fast, reliable and programmable networked systems at Internet scale.
//!
//! # Features
//! - Http 1.x and Http 2
//! - Modern TLS with OpenSSL or BoringSSL (FIPS compatible)
//! - Zero downtime upgrade
//!
//! # Usage
//! This crate provides low level service and protocol implementation and abstraction.
//!
//! If looking to build a (reverse) proxy, see [`bongonet-proxy`](https://docs.rs/bongonet-proxy) crate.
//!
//! # Feature flags
#![cfg_attr(
    feature = "document-features",
    cfg_attr(doc, doc = ::document_features::document_features!())
)]

pub use bongonet_core::*;

/// HTTP header objects that preserve http header cases
pub mod http {
    pub use bongonet_http::*;
}

#[cfg(feature = "cache")]
#[cfg_attr(docsrs, doc(cfg(feature = "cache")))]
/// Caching services and tooling
pub mod cache {
    pub use bongonet_cache::*;
}

#[cfg(feature = "lb")]
#[cfg_attr(docsrs, doc(cfg(feature = "lb")))]
/// Load balancing recipes
pub mod lb {
    pub use bongonet_load_balancing::*;
}

#[cfg(feature = "proxy")]
#[cfg_attr(docsrs, doc(cfg(feature = "proxy")))]
/// Proxying recipes
pub mod proxy {
    pub use bongonet_proxy::*;
}

#[cfg(feature = "time")]
#[cfg_attr(docsrs, doc(cfg(feature = "time")))]
/// Timeouts and other useful time utilities
pub mod time {
    pub use bongonet_timeout::*;
}

/// A useful set of types for getting started
pub mod prelude {
    pub use bongonet_core::prelude::*;
    pub use bongonet_http::prelude::*;
    pub use bongonet_timeout::*;

    #[cfg(feature = "cache")]
    #[cfg_attr(docsrs, doc(cfg(feature = "cache")))]
    pub use bongonet_cache::prelude::*;

    #[cfg(feature = "lb")]
    #[cfg_attr(docsrs, doc(cfg(feature = "lb")))]
    pub use bongonet_load_balancing::prelude::*;

    #[cfg(feature = "proxy")]
    #[cfg_attr(docsrs, doc(cfg(feature = "proxy")))]
    pub use bongonet_proxy::prelude::*;

    #[cfg(feature = "time")]
    #[cfg_attr(docsrs, doc(cfg(feature = "time")))]
    pub use bongonet_timeout::*;
}
