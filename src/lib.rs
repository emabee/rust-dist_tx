//! Structs and traits for dealing with distributed transactions, inspired by XA.

#![warn(missing_docs)]

#[macro_use]
extern crate bitflags;
extern crate byteorder;

pub mod rm;
pub mod tm;
