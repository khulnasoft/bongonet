# Bongonet

![Bongonet banner image](./docs/assets/bongonet_banner.png)

## What is Bongonet
Bongonet is a Rust framework to [build fast, reliable and programmable networked systems](https://blog.khulnasoft.com/bongonet-open-source).

Bongonet is battle tested as it has been serving more than 40 million Internet requests per second for [more than a few years](https://blog.khulnasoft.com/how-we-built-bongonet-the-proxy-that-connects-khulnasoft-to-the-internet).

## Feature highlights
* Async Rust: fast and reliable
* HTTP 1/2 end to end proxy
* TLS over OpenSSL or BoringSSL
* gRPC and websocket proxying
* Graceful reload
* Customizable load balancing and failover strategies
* Support for a variety of observability tools

## Reasons to use Bongonet
* **Security** is your top priority: Bongonet is a more memory safe alternative for services that are written in C/C++
* Your service is **performance-sensitive**: Bongonet is fast and efficient
* Your service requires extensive **customization**: The APIs Bongonet proxy framework provides are highly programmable

# Getting started

See our [quick starting guide](./docs/quick_start.md) to see how easy it is to build a load balancer.

Our [user guide](./docs/user_guide/index.md) covers more topics such as how to configure and run Bongonet servers, as well as how to build custom HTTP servers and proxy logic on top of Bongonet's framework.

API docs are also available for all the crates.

# Notable crates in this workspace
* Bongonet: the "public facing" crate to build networked systems and proxies
* Bongonet-core: this crate defines the protocols, functionalities and basic traits
* Bongonet-proxy: the logic and APIs to build HTTP proxies
* Bongonet-error: the common error type used across Bongonet crates
* Bongonet-http: the HTTP header definitions and APIs
* Bongonet-openssl & bongonet-boringssl: SSL related extensions and APIs
* Bongonet-ketama: the [Ketama](https://github.com/RJ/ketama) consistent algorithm
* Bongonet-limits: efficient counting algorithms
* Bongonet-load-balancing: load balancing algorithm extensions for bongonet-proxy
* Bongonet-memory-cache: Async in-memory caching with cache lock to prevent cache stampede
* Bongonet-timeout: A more efficient async timer system
* TinyUfo: The caching algorithm behind bongonet-memory-cache

# System requirements

## Systems
Linux is our tier 1 environment and main focus.

We will try our best for most code to compile for Unix environments. This is for developers and users to have an easier time developing with Bongonet in Unix-like environments like macOS (though some features might be missing)

Both x86_64 and aarch64 architectures will be supported.

## Rust version

Bongonet keeps a rolling MSRV (minimum supported Rust version) policy of 6 months. This means we will accept PRs that upgrade the MSRV as long as the new Rust version used is at least 6 months old.

Our current MSRV is 1.72.

## Build Requirements

Some of the crates in this repository have dependencies on additional tools and
libraries that must be satisfied in order to build them:

* Make sure that [Clang] is installed on your system (for boringssl)
* Make sure that [Perl 5] is installed on your system (for openssl)

[Clang]:https://clang.llvm.org/
[Perl 5]:https://www.perl.org/

# Contributing
Please see our [contribution guidelines](./.github/CONTRIBUTING.md).

# License
This project is Licensed under [Apache License, Version 2.0](./LICENSE).
