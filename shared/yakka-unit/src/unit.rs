//! The unit trait and its associated types.

use yakka_number::{
    ratio::{Ratio, Transform},
    scalar::Scalar,
};

use crate::dimension::Dimension;

/// A relationship between an [`Unit`] and its respective base unit.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]

pub enum Scale {
    /// A scale that corresponds to a multiple of the base unit.
    Multiple(Ratio),

    /// A scale that corresponds to a fraction of the base unit.
    Fractional(Ratio),

    /// A scale that corresponds to a ratio of the base unit.
    Same,
}

impl Scale {
    /// Determine the factional [`f32`] that corresponds to the target [`Scale`].
    #[inline]
    pub const fn fractional(self: &Self) -> f32 {
        match self {
            &Scale::Multiple(Ratio(target_value)) => target_value.get() as f32,
            &Scale::Fractional(Ratio(target_value)) => 1.0 / target_value.get() as f32,
            Scale::Same => 1.0,
        }
    }
}

impl Scale {
    /// Applies the current scale to a value.
    #[inline]
    pub fn apply<S>(self: &Self, value: S) -> S
    where
        S: Magnitude,
    {
        match self {
            &Scale::Multiple(scale) => Transform::up(value, scale),
            &Scale::Fractional(scale) => Transform::down(value, scale),
            Scale::Same => value,
        }
    }

    /// Applies the current scale to a value inversely.
    #[inline]
    pub fn inversed<S>(self: &Self, value: S) -> S
    where
        S: Magnitude,
    {
        match self {
            &Scale::Multiple(scale) => Transform::down(value, scale),
            &Scale::Fractional(scale) => Transform::up(value, scale),
            Scale::Same => value,
        }
    }
}

/// A physical unit of measure.
pub trait Unit {
    /// The dimension of the unit.  
    type Dimension: Dimension;

    /// The magnitude of the unit.
    type Magnitude: Magnitude;

    /// The base unit of the unit.
    type Base: Unit<Base = Self::Base, Magnitude = Self::Magnitude, Dimension = Self::Dimension>;

    /// The scale factor of the unit respective to [`Unit::Base`].
    const SCALE_TO_BASE: Scale;

    /// Construct a new unit of this type from a bare magnitude scalar.
    fn raw(scalar: Self::Magnitude) -> Self
    where
        Self: Sized;

    /// Convert a value measured in [`Unit::Base`] to the current unit.
    #[inline]
    fn base(value: Self::Base) -> Self
    where
        Self: Sized,
    {
        let scalar_value = Unit::magnitude(value);

        let scalar_value = Scale::inversed(&Self::SCALE_TO_BASE, scalar_value);

        Self::raw(scalar_value)
    }

    /// Convert a value measured in the current unit to one measured in [`Unit::Base`].
    #[inline]
    fn value(self: Self) -> Self::Base
    where
        Self: Sized,
    {
        let scalar_value = Self::magnitude(self);

        let scalar_value = Scale::apply(&Self::SCALE_TO_BASE, scalar_value);

        Self::Base::raw(scalar_value)
    }

    /// Determine the magnitude of the current unit as-is.
    fn magnitude(self: Self) -> Self::Magnitude
    where
        Self: Sized;

    /// Convert a value measured in the current unit to one measured in another unit of the same
    /// dimension.
    #[inline]
    fn to<U>(self: Self) -> U
    where
        U: Unit<Base = Self::Base, Dimension = Self::Dimension>,
        Self: Sized,
    {
        U::base(Self::value(self))
    }

    /// Convert a value measured in another unit of the same dimension to one measured in the
    /// current unit.
    #[inline]
    fn from<U>(self: Self, value: U) -> Self
    where
        U: Unit<Base = Self::Base, Dimension = Self::Dimension>,
        Self: Sized,
    {
        value.to::<Self>()
    }
}

/// A trait for types that can be used as a magnitude of some physical unit.
pub trait Magnitude: Scalar + Transform {}

impl<T> Magnitude for T where T: Scalar + Transform {}
