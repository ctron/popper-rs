# Popper bindings for Rust

[![CI](https://github.com/ctron/popper-rs/workflows/CI/badge.svg)](https://github.com/ctron/popper-rs/actions?query=workflow%3A%22CI%22)
[![GitHub release (latest SemVer)](https://img.shields.io/github/v/tag/ctron/popper-rs?sort=semver)](https://github.com/ctron/popper-rs/releases)
[![crates.io](https://img.shields.io/crates/v/popper-rs.svg)](https://crates.io/crates/popper-rs)
[![docs.rs](https://docs.rs/popper-rs/badge.svg)](https://docs.rs/popper-rs)

Popper is a:

> Tooltip & Popover Positioning Engine

This project provides Rust bindings for WebAssembly.

## Examples

* [Basic](examples/basic) - A basic example with plain WASM
* [Yew](examples/yew) - An example using Yew with hooks

You can run the example using `trunk`:

```shell
trunk serve examples/basic/index.html --open
```