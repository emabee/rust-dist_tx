# Rust language bindings for [XA Distributed Transactions](https://pubs.opengroup.org/onlinepubs/009680699/toc.pdf)

[![Latest version](https://img.shields.io/crates/v/dist_tx.svg)](https://crates.io/crates/dist_tx)
[![Documentation](https://docs.rs/dist_tx/badge.svg)](https://docs.rs/dist_tx)
[![License](https://img.shields.io/crates/l/dist_tx.svg)](https://github.com/emabee/dist_tx)

[XA Distributed Transactions](https://pubs.opengroup.org/onlinepubs/009680699/toc.pdf)
support transactions _across_ multiple transactional databases.

This library can be used in synchronous or in asynchronous contexts.

## Crate Features

Technically, the features `sync` and `async` are both optional features,
usually you need exactly one of them. Select the feature `sync` or `async` explicitly.

```toml
[dependencies]
dist_tx = { version = "0.3", features = ["async"] }
```

or

```toml
[dependencies]
dist_tx = { version = "0.3", features = ["sync"] }
```
