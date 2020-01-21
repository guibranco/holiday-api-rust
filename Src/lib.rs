//! Library to used to access the Holiday API with Rust
#![allow(dead_code)] // Until every starting struct gets used
#![deny(//missing_docs,
        unsafe_code,
        unused_import_braces,
        unused_qualifications)]

#[macro_use]
extern crate error_chain;

#[macro_use]
mod macros;
mod util;

pub mod client;
pub mod errors;
pub mod holidays;
pub mod countries;
pub mod languages;

pub use hyper::{HeaderMap, StatusCode};