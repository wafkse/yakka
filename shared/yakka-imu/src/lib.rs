#![cfg_attr(not(test), no_std)]
#![forbid(
    missing_docs,
    unused_unsafe,
    clippy::all,
    clippy::pedantic,
    clippy::cargo,
    clippy::nursery,
    clippy::cargo_common_metadata
)]
//! Inertial Measurement Unit support for Yakka.
//!
//! See each respective top-level module for the chip.

pub mod lsm6ds3;

pub mod bmi160;
