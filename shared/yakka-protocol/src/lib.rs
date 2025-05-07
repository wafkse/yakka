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
//! Yakka Protocol
//!
//! The protocol specification for controlling control system setpoints remotely.

pub mod packet;

pub mod control;
