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
//! Motor control for Yakka.

pub mod setting;

pub mod throttle;

pub mod driver;

pub mod actuate;

pub mod prelude {
    //! A prelude module for the `yakka-motor` crate.

    pub use crate::setting::Limit;

    pub use crate::throttle::Throttle;

    pub use crate::driver::{Esc, Motor};

    pub use crate::actuate::Quad;
}
