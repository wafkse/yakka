#![cfg_attr(not(test), no_std)]
#![forbid(
    missing_docs,
    unsafe_code,
    unused_unsafe,
    clippy::all,
    clippy::pedantic,
    clippy::cargo,
    clippy::nursery
)]
//! # Yakka Control
//!
//! This crate provides the *control functionality* for the Yakka project.

pub mod pid;

pub mod system;

pub mod cycle;

pub mod prelude {
    //! A prelude for the Yakka Control crate.

    pub use crate::pid::Pid;

    pub use crate::system::{Error, Setpoint, Variable};

    pub use crate::cycle::Delta;
}
