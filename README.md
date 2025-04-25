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

---

## Packages

| Package                   | Description                                              | Version | Docs | Command                                                                      |
| ------------------------- | -------------------------------------------------------- | ------- | ---- | ---------------------------------------------------------------------------- |
| `bongonet`                | Public-facing crate for proxy/networked systems          |         | [![docs.rs](https://docs.rs/bongonet/badge.svg)](https://docs.rs/bongonet) | `cargo add bongonet`                                                           |
| `bongonet-boringssl`      | SSL integration using BoringSSL                          |         | [![docs.rs](https://docs.rs/bongonet-boringssl/badge.svg)](https://docs.rs/bongonet-boringssl) | `cargo add bongonet-boringssl`                                                 |
| `bongonet-cache`          | Generic cache structures (WIP or alias of memory-cache?) |         | [![docs.rs](https://docs.rs/bongonet-cache/badge.svg)](https://docs.rs/bongonet-cache) | `cargo add bongonet-cache`                                                     |
| `bongonet-core`           | Protocols, traits, and core logic                        |         | [![docs.rs](https://docs.rs/bongonet-core/badge.svg)](https://docs.rs/bongonet-core) | `cargo add bongonet-core`                                                      |
| `bongonet-error`          | Shared error types across crates                         |         | [![docs.rs](https://docs.rs/bongonet-error/badge.svg)](https://docs.rs/bongonet-error) | `cargo add bongonet-error`                                                     |
| `bongonet-header-serde`   | Header serialization/deserialization utilities           |         | [![docs.rs](https://docs.rs/bongonet-header-serde/badge.svg)](https://docs.rs/bongonet-header-serde) | `cargo add bongonet-header-serde`                                              |
| `bongonet-http`           | HTTP headers and utilities                               |         | [![docs.rs](https://docs.rs/bongonet-http/badge.svg)](https://docs.rs/bongonet-http) | `cargo add bongonet-http`                                                      |
| `bongonet-ketama`         | Ketama consistent hashing                                |         | [![docs.rs](https://docs.rs/bongonet-ketama/badge.svg)](https://docs.rs/bongonet-ketama) | `cargo add bongonet-ketama`                                                   |
| `bongonet-limits`         | Rate-limiting algorithms                                 |         | [![docs.rs](https://docs.rs/bongonet-limits/badge.svg)](https://docs.rs/bongonet-limits) | `cargo add bongonet-limits`                                                   |
| `bongonet-load-balancing` | Load balancing extensions                                |         | [![docs.rs](https://docs.rs/bongonet-load-balancing/badge.svg)](https://docs.rs/bongonet-load-balancing) | `cargo add bongonet-load-balancing`                                           |
| `bongonet-lru`            | LRU cache strategies for async systems                   |         | [![docs.rs](https://docs.rs/bongonet-lru/badge.svg)](https://docs.rs/bongonet-lru) | `cargo add bongonet-lru`                                                      |
| `bongonet-memory-cache`   | Async memory cache with locking                          |         | [![docs.rs](https://docs.rs/bongonet-memory-cache/badge.svg)](https://docs.rs/bongonet-memory-cache) | `cargo add bongonet-memory-cache`                                              |
| `bongonet-openssl`        | SSL integration using OpenSSL                            |         | [![docs.rs](https://docs.rs/bongonet-openssl/badge.svg)](https://docs.rs/bongonet-openssl) | `cargo add bongonet-openssl`                                                  |
| `bongonet-pool`           | Async connection pool utilities                          |         | [![docs.rs](https://docs.rs/bongonet-pool/badge.svg)](https://docs.rs/bongonet-pool) | `cargo add bongonet-pool`                                                     |
| `bongonet-proxy`          | HTTP proxy logic and APIs                                |         | [![docs.rs](https://docs.rs/bongonet-proxy/badge.svg)](https://docs.rs/bongonet-proxy) | `cargo add bongonet-proxy`                                                    |
| `bongonet-runtime`        | Runtime integration for network workloads                |         | [![docs.rs](https://docs.rs/bongonet-runtime/badge.svg)](https://docs.rs/bongonet-runtime) | `cargo add bongonet-runtime`                                                  |
| `bongonet-rustls`         | TLS integration using Rustls                             |         | [![docs.rs](https://docs.rs/bongonet-rustls/badge.svg)](https://docs.rs/bongonet-rustls) | `cargo add bongonet-rustls`                                                   |
| `bongonet-timeout`        | Efficient async timer utilities                          |         | [![docs.rs](https://docs.rs/bongonet-timeout/badge.svg)](https://docs.rs/bongonet-timeout) | `cargo add bongonet-timeout`                                                  |

---

Let me know if you'd like to add any more details or modify anything!


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

