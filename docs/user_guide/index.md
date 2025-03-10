# User Guide

In this guide, we will cover the most used features, operations and settings of Bongonet.

## Running Bongonet servers
* [Start and stop](start_stop.md)
* [Graceful restart and graceful shutdown](graceful.md)
* [Configuration](conf.md)
* [Daemonization](daemon.md)
* [Systemd integration](systemd.md)
* [Handling panics](panic.md)
* [Error logging](error_log.md)
* [Prometheus](prom.md)

## Building HTTP proxies
* [Life of a request: `bongonet-proxy` phases and filters](phase.md)
* [`Peer`: how to connect to upstream](peer.md)
* [Sharing state across phases with `CTX`](ctx.md)
* [How to return errors](errors.md)
* [Examples: take control of the request](modify_filter.md)
* [Connection pooling and reuse](pooling.md)
* [Handling failures and failover](failover.md)
* [RateLimiter quickstart](rate_limiter.md)

## Advanced topics (WIP)
* [Bongonet internals](internals.md)
* Using BoringSSL
* User defined configuration
* Bongonet async runtime and threading model
* Background Service
* Blocking code in async context
* Tracing
