#![no_std]
#![forbid(
    missing_docs,
    unused_unsafe,
    clippy::all,
    clippy::pedantic,
    clippy::cargo,
    clippy::nursery,
    clippy::cargo_common_metadata
)]
//! Yakka Scalar
//!
//! This crate provides a set of traits to work with scalar values in a generic fashion.
//!
//! For the central scalar trait, see [`Scalar`].
//!
//! [`Scalar`]: crate::scalar::Scalar

pub mod identity;

pub mod scalar;

pub mod ratio;

pub mod normal;

pub mod prelude {
    //! A prelude for the Yakka Scalar crate.
    //!
    //! This module re-exports the most commonly used items from the crate, so you can use them without having to import them individually.
    //!
    //! ```rust
    //! use yakka_number::prelude::*;
    //! ```

    pub use crate::ratio::Ratio;
    pub use crate::scalar::Scalar;

    pub use crate::identity::{One, Zero};
}
