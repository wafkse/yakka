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
//! # Yakka Time
//!
//! This crate exposes a variety of time-related data types and ways to manipulate them.

use yakka_number::ratio::{self, Ratioable};
use yakka_unit::{
    dimension::Time,
    unit::{Magnitude, Scale, Unit},
};

use yakka_unit::unit;



/// The SI unit of time, the second.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "debug-impl", derive(Debug))]
#[repr(transparent)]
pub struct Second<T>(T)
where
    T: Magnitude;

impl<T> Unit for Second<T>
where
    T: Magnitude,
{
    type Dimension = Time;

    type Magnitude = T;

    type Base = Self;

    const SCALE_TO_BASE: Scale = Scale::Same;

    #[inline]
    fn raw(scalar: T) -> Self
    where
        Self: Sized,
    {
        Self(scalar)
    }

    #[inline]
    fn magnitude(self: Self) -> T
    where
        Self: Sized,
    {
        let Self(scalar_value) = self;

        scalar_value
    }
}

unit!(
    /// A millisecond.
    ///
    /// This is a unit of time equal to one thousandth of a second.
    for Millisecond use Second where Time
    become Scale::Fractional(ratio::Const::<1_000>::RATIO)
);

unit!(
    /// A microsecond.
    ///
    /// This is a unit of time equal to one millionth of a second.
    for Microsecond use Second where Time
    become Scale::Fractional(ratio::Const::<1_000_000>::RATIO)
);

unit!(
    /// A nanosecond.
    ///
    /// This is a unit of time equal to one billionth of a second.
    for Nanosecond use Second where Time
    become Scale::Fractional(ratio::Const::<1_000_000_000>::RATIO)
);

unit!(
    /// A minute.
    ///
    /// This is a unit of time equal to sixty seconds.
    for Minute use Second where Time
    become Scale::Multiple(ratio::Const::<60>::RATIO)
);

unit!(
    /// An hour.
    ///
    /// This is a unit of time equal to sixty minutes.
    for Hour use Second where Time
    become Scale::Multiple(ratio::Const::<{ 60 * 60 }>::RATIO)
);

unit!(
    /// A day.
    ///
    /// This is a unit of time equal to twenty-four hours.
    for Day use Second where Time
    become Scale::Multiple(ratio::Const::<{ 24 * 3600 }>::RATIO)
);
