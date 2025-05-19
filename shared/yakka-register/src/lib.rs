#![no_std]
#![forbid(
    missing_docs,
    unsafe_code,
    unused_unsafe,
    clippy::all,
    clippy::pedantic,
    clippy::cargo,
    clippy::nursery
)]
//! Yakka Register
//!
//! This is a helper crate that defines register with bit extraction facilities.

pub mod value;
