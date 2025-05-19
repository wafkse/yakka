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
//! # Yakka Flight
//!
//! This crate provides the *flight control functionality* for the Yakka project.

pub mod motion;
