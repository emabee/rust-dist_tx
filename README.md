# Rust language bindings for [XA Distributed Transactions](https://pubs.opengroup.org/onlinepubs/009680699/toc.pdf)

[![Latest version](https://img.shields.io/crates/v/dist_tx.svg)](https://crates.io/crates/dist_tx)
[![Documentation](https://docs.rs/dist_tx/badge.svg)](https://docs.rs/dist_tx)
[![License](https://img.shields.io/crates/l/dist_tx.svg)](https://github.com/emabee/dist_tx)

## Usage

Add `dist_tx` to the dependencies section in your project's `Cargo.toml`:

```toml
[dependencies]
dist_tx = "0.3"
```

## Crate Features

The lib has two features, `sync` and `async`, which are both default features, but usually you only need one of them. To minimize your dependency, disable default features and select the feature you need explicitly, e.g.

```toml
[dependencies]
dist_tx = { version = "0.3", default-features = false, features = ["async"] }
```

or

```toml
[dependencies]
dist_tx = { version = "0.3", default_features = false, features = ["sync"] }
```
