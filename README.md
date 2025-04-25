<h1 align="center">🚀 Bongonet</h1>

<p align="center">
  <strong>Async Rust networking stack for blazing-fast, secure, and programmable proxying</strong>
</p>

<p align="center">
  <a href="https://crates.io/crates/bongonet">
    <img src="https://img.shields.io/crates/v/bongonet.svg" alt="Crates.io version" />
  </a>
  <a href="https://github.com/your-org/bongonet/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/license-Apache%202.0-blue.svg" alt="License: Apache 2.0" />
  </a>
  <img src="https://img.shields.io/badge/rustc-1.72+-orange.svg" alt="Rustc Version 1.72+" />
</p>

---

## ✨ Feature Highlights

- ⚡ **Async Rust**: Fast and reliable
- 🌐 **HTTP/1 & HTTP/2** end-to-end proxy
- 🔐 **TLS** over OpenSSL, BoringSSL or rustls (experimental)
- 🔄 **gRPC and WebSocket** proxying
- ♻️ **Graceful reload**
- 🎯 **Customizable load balancing** and failover strategies
- 📊 **Observability** support: metrics, tracing, and more

---

## 🛡️ Reasons to Use Bongonet

- ✅ **Security-first**: Memory-safe Rust vs legacy C/C++ codebases
- 🚀 **High performance**: Optimized for low latency and high throughput
- 🔧 **Highly customizable**: Extensible APIs for tailor-made proxy logic

---

## 🚀 Getting Started

Start fast with our [Quick Start Guide](./docs/quick_start.md).

Explore more in the [User Guide](./docs/user_guide/index.md) — from running Bongonet servers to building custom proxy logic.

📚 **API Documentation** is available for all crates in the workspace.

---

## 📦 Notable Crates

| Package | Description | Version | Downloads | Docs |
| :------ | :---------- | :------ | :-------- | :--- |
| [`bongonet`](https://crates.io/crates/bongonet) | Public-facing crate for proxy/networked systems | ![Crates.io](https://img.shields.io/crates/v/bongonet?color=blue) | ![Downloads](https://img.shields.io/crates/d/bongonet.svg) | [![Docs.rs](https://docs.rs/bongonet/badge.svg)](https://docs.rs/bongonet) |
| [`bongonet-core`](https://crates.io/crates/bongonet-core) | Defines protocols, traits, and core functionalities | ![Crates.io](https://img.shields.io/crates/v/bongonet-core?color=blue) | ![Downloads](https://img.shields.io/crates/d/bongonet-core.svg) | [![Docs.rs](https://docs.rs/bongonet-core/badge.svg)](https://docs.rs/bongonet-core) |
| [`bongonet-proxy`](https://crates.io/crates/bongonet-proxy) | Logic and APIs for building HTTP proxies | ![Crates.io](https://img.shields.io/crates/v/bongonet-proxy?color=blue) | ![Downloads](https://img.shields.io/crates/d/bongonet-proxy.svg) | [![Docs.rs](https://docs.rs/bongonet-proxy/badge.svg)](https://docs.rs/bongonet-proxy) |
| [`bongonet-error`](https://crates.io/crates/bongonet-error) | Shared error types across crates | ![Crates.io](https://img.shields.io/crates/v/bongonet-error?color=blue) | ![Downloads](https://img.shields.io/crates/d/bongonet-error.svg) | [![Docs.rs](https://docs.rs/bongonet-error/badge.svg)](https://docs.rs/bongonet-error) |
| [`bongonet-http`](https://crates.io/crates/bongonet-http) | HTTP header types and utilities | ![Crates.io](https://img.shields.io/crates/v/bongonet-http?color=blue) | ![Downloads](https://img.shields.io/crates/d/bongonet-http.svg) | [![Docs.rs](https://docs.rs/bongonet-http/badge.svg)](https://docs.rs/bongonet-http) |
| [`bongonet-openssl`](https://crates.io/crates/bongonet-openssl) | SSL integration with OpenSSL | ![Crates.io](https://img.shields.io/crates/v/bongonet-openssl?color=blue) | ![Downloads](https://img.shields.io/crates/d/bongonet-openssl.svg) | [![Docs.rs](https://docs.rs/bongonet-openssl/badge.svg)](https://docs.rs/bongonet-openssl) |
| [`bongonet-boringssl`](https://crates.io/crates/bongonet-boringssl) | SSL integration with BoringSSL | ![Crates.io](https://img.shields.io/crates/v/bongonet-boringssl?color=blue) | ![Downloads](https://img.shields.io/crates/d/bongonet-boringssl.svg) | [![Docs.rs](https://docs.rs/bongonet-boringssl/badge.svg)](https://docs.rs/bongonet-boringssl) |
| [`bongonet-ketama`](https://crates.io/crates/bongonet-ketama) | [Ketama](https://github.com/RJ/ketama) consistent hashing | ![Crates.io](https://img.shields.io/crates/v/bongonet-ketama?color=blue) | ![Downloads](https://img.shields.io/crates/d/bongonet-ketama.svg) | [![Docs.rs](https://docs.rs/bongonet-ketama/badge.svg)](https://docs.rs/bongonet-ketama) |
| [`bongonet-limits`](https://crates.io/crates/bongonet-limits) | Efficient rate-limiting algorithms | ![Crates.io](https://img.shields.io/crates/v/bongonet-limits?color=blue) | ![Downloads](https://img.shields.io/crates/d/bongonet-limits.svg) | [![Docs.rs](https://docs.rs/bongonet-limits/badge.svg)](https://docs.rs/bongonet-limits) |
| [`bongonet-load-balancing`](https://crates.io/crates/bongonet-load-balancing) | Load balancing extensions | ![Crates.io](https://img.shields.io/crates/v/bongonet-load-balancing?color=blue) | ![Downloads](https://img.shields.io/crates/d/bongonet-load-balancing.svg) | [![Docs.rs](https://docs.rs/bongonet-load-balancing/badge.svg)](https://docs.rs/bongonet-load-balancing) |
| [`bongonet-memory-cache`](https://crates.io/crates/bongonet-memory-cache) | Async memory caching with cache-lock | ![Crates.io](https://img.shields.io/crates/v/bongonet-memory-cache?color=blue) | ![Downloads](https://img.shields.io/crates/d/bongonet-memory-cache.svg) | [![Docs.rs](https://docs.rs/bongonet-memory-cache/badge.svg)](https://docs.rs/bongonet-memory-cache) |
| [`bongonet-timeout`](https://crates.io/crates/bongonet-timeout) | Efficient async timers | ![Crates.io](https://img.shields.io/crates/v/bongonet-timeout?color=blue) | ![Downloads](https://img.shields.io/crates/d/bongonet-timeout.svg) | [![Docs.rs](https://docs.rs/bongonet-timeout/badge.svg)](https://docs.rs/bongonet-timeout) |
| [`tinyufo`](https://crates.io/crates/tinyufo) | Underlying caching logic behind `bongonet-memory-cache` | ![Crates.io](https://img.shields.io/crates/v/tinyufo?color=blue) | ![Downloads](https://img.shields.io/crates/d/tinyufo.svg) | [![Docs.rs](https://docs.rs/tinyufo/badge.svg)](https://docs.rs/tinyufo) |

---

## 🧰 System Requirements

### 🖥️ Supported Platforms

- ✅ **Linux** (Tier 1)
- 💻 **macOS** (Partial support)
- 🪟 **Windows** (Best-effort by community)
- Supported Architectures: `x86_64`, `aarch64`

### 🦀 Rust Version

- MSRV: **1.72**
- Rolling 6-month MSRV policy — upgrades allowed if Rust version is ≥ 6 months old

### 🛠️ Build Requirements

Some crates depend on external tools:

- 🔧 [Clang](https://clang.llvm.org/) (for BoringSSL)
- 🐪 [Perl 5](https://www.perl.org/) (for OpenSSL)

---

## 🤝 Contributing

Check out our [Contribution Guidelines](./.github/CONTRIBUTING.md) to get involved.

---

## ⚖️ License

Licensed under the [Apache License, Version 2.0](./LICENSE).

---

