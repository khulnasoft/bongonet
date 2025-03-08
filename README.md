# Bongonet 

🚀 **High-performance, memory-safe proxy framework in Rust**

---

## ✨ Feature Highlights

![Features](https://img.shields.io/badge/High--Performance-%E2%9C%85-blue) ![Security](https://img.shields.io/badge/Secure-%E2%9C%85-green) ![Observability](https://img.shields.io/badge/Observability-%E2%9C%85-purple)

✅ **Async Rust** – Fast and reliable

✅ **Full HTTP 1/2 Proxy** – End-to-end proxying support

✅ **Secure TLS** – Supports OpenSSL, BoringSSL, and experimental Rustls

✅ **gRPC & WebSocket** – Seamless proxying for modern applications

✅ **Graceful Reload** – No downtime on config updates

✅ **Custom Load Balancing & Failover** – Flexible traffic management

✅ **Comprehensive Observability** – Works with various monitoring tools

---

## 🎯 Why Choose Bongonet?

🛡️ **Security First** – A safer alternative to C/C++-based proxies

⚡ **Blazing Fast** – Optimized for performance-sensitive applications

🛠 **Highly Customizable** – Extendable APIs for tailored proxy solutions

---

## 🚀 Getting Started

🔹 Read our [Quick Start Guide](./docs/quick_start.md) to set up a load balancer in minutes.

🔹 Check out the [User Guide](./docs/user_guide/index.md) for detailed configuration and advanced usage.

🔹 Explore our **API documentation** for all Bongonet crates.

---

## 📦 Notable Crates in This Workspace

| Crate | Version | Description | Crates.io |
|--------|---------|-------------|-----------|
| **Bongonet** | 1.0.0 | The core public crate for building networked systems and proxies | [📦](https://crates.io/crates/bongonet) |
| **Bongonet-core** | 1.0.0 | Defines protocols, functionalities, and fundamental traits | [📦](https://crates.io/crates/bongonet-core) |
| **Bongonet-proxy** | 1.0.0 | APIs and logic for building HTTP proxies | [📦](https://crates.io/crates/bongonet-proxy) |
| **Bongonet-error** | 1.0.0 | Common error types shared across Bongonet crates | [📦](https://crates.io/crates/bongonet-error) |
| **Bongonet-http** | 1.0.0 | HTTP header definitions and APIs | [📦](https://crates.io/crates/bongonet-http) |
| **Bongonet-openssl** | 1.0.0 | SSL-related extensions and APIs | [📦](https://crates.io/crates/bongonet-openssl) |
| **Bongonet-boringssl** | 1.0.0 | SSL-related extensions and APIs | [📦](https://crates.io/crates/bongonet-boringssl) |
| **Bongonet-ketama** | 1.0.0 | Consistent hashing using [Ketama](https://github.com/RJ/ketama) | [📦](https://crates.io/crates/bongonet-ketama) |
| **Bongonet-limits** | 1.0.0 | Efficient counting algorithms | [📦](https://crates.io/crates/bongonet-limits) |
| **Bongonet-load-balancing** | 1.0.0 | Load balancing algorithm extensions for Bongonet-proxy | [📦](https://crates.io/crates/bongonet-load-balancing) |
| **Bongonet-memory-cache** | 1.0.0 | Async in-memory caching with cache lock prevention | [📦](https://crates.io/crates/bongonet-memory-cache) |
| **Bongonet-timeout** | 1.0.0 | High-performance async timer system | [📦](https://crates.io/crates/bongonet-timeout) |
| **TinyUfo** | 1.0.0 | Caching algorithm powering Bongonet-memory-cache | [📦](https://crates.io/crates/tinyufo) |

---

## 🖥️ System Requirements

### 🏗️ Supported Platforms
✅ **Linux** (Tier 1 support, primary development focus)

✅ **Unix-like operating systems** (macOS support with some limitations)

✅ **Windows** (Community-supported, experimental)

✅ **Architectures** – Supports **x86_64** & **aarch64**

### 🔧 Rust Version

Bongonet follows a **rolling MSRV policy** (Minimum Supported Rust Version) of **6 months**.

✅ **Current MSRV:** Rust **1.72**

### ⚙️ Build Dependencies

🔹 **[Clang](https://clang.llvm.org/)** – Required for BoringSSL

🔹 **[Perl 5](https://www.perl.org/)** – Required for OpenSSL

---

## 🤝 Contributing

We ❤️ contributions! See our [Contribution Guidelines](./.github/CONTRIBUTING.md) for more info.

---

## 📜 License

Bongonet is licensed under the **[Apache License 2.0](./LICENSE)**.

---

💡 **Stay connected:** Follow updates, report issues, and contribute to making Bongonet even better!

📢 **Join Our Community:** [![Discord](https://img.shields.io/discord/your-discord-id?label=Join%20Discord&logo=discord&color=7289da)](https://discord.gg/khulnasoft) [![Twitter](https://img.shields.io/twitter/follow/your-twitter-handle?style=social)](https://twitter.com/khulnasoft)

