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

- [`bongonet`](https://crates.io/crates/bongonet)  
  <img src="https://img.shields.io/crates/v/bongonet.svg" />
  <img src="https://img.shields.io/crates/d/bongonet.svg" />  
  Public-facing crate for proxy/networked systems

- [`bongonet-core`](https://crates.io/crates/bongonet-core)  
  <img src="https://img.shields.io/crates/v/bongonet-core.svg" />
  <img src="https://img.shields.io/crates/d/bongonet-core.svg" />  
  Defines protocols, traits, and core functionalities

- [`bongonet-proxy`](https://crates.io/crates/bongonet-proxy)  
  <img src="https://img.shields.io/crates/v/bongonet-proxy.svg" />
  <img src="https://img.shields.io/crates/d/bongonet-proxy.svg" />  
  Logic and APIs for building HTTP proxies

- [`bongonet-error`](https://crates.io/crates/bongonet-error)  
  <img src="https://img.shields.io/crates/v/bongonet-error.svg" />
  <img src="https://img.shields.io/crates/d/bongonet-error.svg" />  
  Shared error types across crates

- [`bongonet-http`](https://crates.io/crates/bongonet-http)  
  <img src="https://img.shields.io/crates/v/bongonet-http.svg" />
  <img src="https://img.shields.io/crates/d/bongonet-http.svg" />  
  HTTP header types and utilities

- [`bongonet-openssl`](https://crates.io/crates/bongonet-openssl) / [`bongonet-boringssl`](https://crates.io/crates/bongonet-boringssl)  
  <img src="https://img.shields.io/crates/v/bongonet-openssl.svg" />
  <img src="https://img.shields.io/crates/d/bongonet-openssl.svg" />  
  <img src="https://img.shields.io/crates/v/bongonet-boringssl.svg" />
  <img src="https://img.shields.io/crates/d/bongonet-boringssl.svg" />  
  SSL integrations

- [`bongonet-ketama`](https://crates.io/crates/bongonet-ketama)  
  <img src="https://img.shields.io/crates/v/bongonet-ketama.svg" />
  <img src="https://img.shields.io/crates/d/bongonet-ketama.svg" />  
  [Ketama](https://github.com/RJ/ketama) consistent hashing

- [`bongonet-limits`](https://crates.io/crates/bongonet-limits)  
  <img src="https://img.shields.io/crates/v/bongonet-limits.svg" />
  <img src="https://img.shields.io/crates/d/bongonet-limits.svg" />  
  Efficient rate-limiting algorithms

- [`bongonet-load-balancing`](https://crates.io/crates/bongonet-load-balancing)  
  <img src="https://img.shields.io/crates/v/bongonet-load-balancing.svg" />
  <img src="https://img.shields.io/crates/d/bongonet-load-balancing.svg" />  
  Load balancing extensions

- [`bongonet-memory-cache`](https://crates.io/crates/bongonet-memory-cache)  
  <img src="https://img.shields.io/crates/v/bongonet-memory-cache.svg" />
  <img src="https://img.shields.io/crates/d/bongonet-memory-cache.svg" />  
  Async memory caching with cache-lock

- [`bongonet-timeout`](https://crates.io/crates/bongonet-timeout)  
  <img src="https://img.shields.io/crates/v/bongonet-timeout.svg" />
  <img src="https://img.shields.io/crates/d/bongonet-timeout.svg" />  
  Efficient async timers

- [`tinyufo`](https://crates.io/crates/tinyufo)  
  <img src="https://img.shields.io/crates/v/tinyufo.svg" />
  <img src="https://img.shields.io/crates/d/tinyufo.svg" />  
  Underlying caching logic behind `bongonet-memory-cache`


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

