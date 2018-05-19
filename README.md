# Parser for Cargo's `--build-plan` output

[![crates.io](https://img.shields.io/crates/v/build-plan.svg)](https://crates.io/crates/build-plan)
[![docs.rs](https://docs.rs/build-plan/badge.svg)](https://docs.rs/build-plan/)
[![Build Status](https://travis-ci.org/jonas-schievink/build-plan.svg?branch=master)](https://travis-ci.org/jonas-schievink/build-plan)

Using `cargo build --build-plan` outputs a JSON document with information about
how to compile the crate and its dependencies. The `build-plan` crate provides a
simple way to parse it back into structures without having to depend on the
entirety of Cargo.

Cargo's `--build-plan` functionality is currently *unstable*, so this only works
on **nightly Rust** for now.

## Usage

Add the dependency to your `Cargo.toml`:

```toml
[dependencies]
build-plan = "0.1.1"
```

Add the crate to your Rust code and import the `BuildPlan` type:

```rust
extern crate build_plan;
use build_plan::BuildPlan;
```

Now you can call `BuildPlan::from_cargo_output(<raw build plan>)` to parse a
JSON build plan.

For more info, check the documentation linked above.
