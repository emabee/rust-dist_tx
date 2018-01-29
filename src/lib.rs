//! Structs and traits for dealing with distributed transactions.
//!
//! This crate is an attempt to provide a reasonable rust language binding for
//! XA Distributed Transactions.
//!
//! So far there is no support for asynchronous operations in resource managers.

#![warn(missing_docs)]

#[macro_use]
extern crate bitflags;
extern crate byteorder;
#[macro_use]
extern crate log;

pub mod rm;
pub mod tm;
