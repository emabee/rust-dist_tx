//! Rust language bindings for
//! [XA Distributed Transactions](https://pubs.opengroup.org/onlinepubs/009680699/toc.pdf).
//!
//! - A synchronous variant is provided in module [`sync`]
//! - An asynchronous variant is provided in module [`a_sync`]
//!
//! See the respective module description for more details.
//!
//! Technically, the features `sync` and `async` are both optional features,
//! usually you need exactly one of them. Select the feature `sync` or `async` explicitly.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![forbid(unsafe_code)]

#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub mod a_sync;

#[cfg(feature = "sync")]
#[cfg_attr(docsrs, doc(cfg(feature = "sync")))]
pub mod sync;

mod error_code;
mod flags;
mod return_code;
mod rm_error;
mod xa_error;
mod xa_transaction_id;

pub use error_code::ErrorCode;
pub use flags::Flags;
pub use return_code::ReturnCode;
pub use rm_error::RmError;
pub use xa_error::XaError;
pub use xa_transaction_id::XaTransactionId;
