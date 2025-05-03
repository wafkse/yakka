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
//! Motor control for the Yakka framework.

pub mod setting;

pub mod run;

pub mod driver;

pub mod prelude {
    //! A prelude module for the `yakka-motor` crate.

    pub use crate::setting::Limit;

    pub use crate::run::Throttle;

    pub use crate::driver::{Esc, Motor};
}
