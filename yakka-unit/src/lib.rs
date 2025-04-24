#![no_std]
#![forbid(
    missing_docs,
    unsafe_code,
    unused_unsafe,
    clippy::all,
    clippy::pedantic,
    clippy::cargo,
    clippy::nursery,
    clippy::cargo_common_metadata
)]
//! # Yakka Unit
//!
//! This crate exposes the [`Unit`] trait, which is the basis for all unit types in the ecosystem.

/// A declarative macro that expands to an unit definition according to some specified scale and its base unit.
#[macro_export]
macro_rules! unit {
    () => {};
    (
        $(
            #[$unit_meta:meta]
        )*
        for $unit_ident:ident use $unit_base:ident where $unit_dimension:ident

        become $unit_scale:expr
    ) => {
        $(
            #[$unit_meta]
        )*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
        #[repr(transparent)]
        pub struct $unit_ident<T>(T)
        where
            T: Magnitude;


        impl<T> Unit for $unit_ident<T>
        where
            T: Magnitude
        {
            type Dimension = $unit_dimension;

            type Magnitude = T;

            type Base = $unit_base<T>;

            const SCALE_TO_BASE: Scale = $unit_scale;

            #[inline]
            fn raw(scalar: T) -> Self {
                Self(scalar)
            }

            #[inline]
            fn magnitude(self: Self) -> T {
                let Self(scalar_value) = self;

                scalar_value
            }
        }
    };
}

pub mod dimension;

pub mod unit;

pub mod prelude {
    //! A prelude for the Yakka Unit crate.
    //!
    //! This module re-exports the most commonly used types and traits from the Yakka Unit crate.

    pub use crate::dimension::Dimension;
    pub use crate::unit::{Magnitude, Scale, Unit};
}
