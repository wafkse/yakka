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
//! # Yakka Motion
//!
//! This crate provides basic data structures to represent the motion of a vehicle in both 2-dimensional and 3-dimensional space.

pub mod uniform;

pub mod position;

pub mod accelerated;

pub mod velocity;

pub mod angle;
