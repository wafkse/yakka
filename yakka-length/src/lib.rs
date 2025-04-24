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
//! # Yakka Length
//!
//! This crates provides a set of data types to represent lengths and distances.

use yakka_number::ratio::{self, Ratioable};
use yakka_unit::{
    dimension::Length,
    unit::{Magnitude, Scale, Unit},
};

use yakka_unit::unit;

/// The SI unit of length, the meter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Meter<T>(T)
where
    T: Magnitude;

impl<T> Unit for Meter<T>
where
    T: Magnitude,
{
    type Dimension = Length;

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
    /// A decimeter.
    ///
    /// This is a unit of length equal to one tenth of a meter.
    for Decimeter use Meter where Length
    become Scale::Fractional(ratio::Const::<10>::RATIO)
);

unit!(
    /// A centimeter.
    ///
    /// This is a unit of length equal to one hundredth of a meter.
    for Centimeter use Meter where Length
    become Scale::Fractional(ratio::Const::<100>::RATIO)
);

unit!(
    /// A millimeter.
    ///
    /// This is a unit of length equal to one thousandth of a meter.
    for Millimeter use Meter where Length
    become Scale::Fractional(ratio::Const::<1_000>::RATIO)
);

unit!(
    /// A kilometer.
    ///
    /// This is a unit of length equal to one thousand meters.
    for Kilometer use Meter where Length
    become Scale::Multiple(ratio::Const::<1_000>::RATIO)
);

unit!(
    /// An hectometer.
    ///
    /// This is a unit of length equal to one hundred meters.
    for Hectometer use Meter where Length
    become Scale::Multiple(ratio::Const::<100>::RATIO)
);
