//! Control cycles and their management.
//!
//! A control cycle is a distinct period of time during which a control system performs its
//! operations.

use yakka_number::scalar::Scalar;
use yakka_unit::{dimension::Time, unit::Unit};



/// The delta-time of a control cycle. Measured in units of time.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "debug-impl", derive(Debug))]
#[repr(transparent)]
pub struct Delta<U>(U)
where
    U: Unit<Dimension = Time>;

impl<U> Delta<U>
where
    U: Unit<Dimension = Time>,
{
    /// Instantiate a new [`Delta`] from the target unit `U`.
    #[inline]
    pub const fn time(unit: U) -> Self {
        Self(unit)
    }

    /// Determine the value of the delta-time.
    #[inline]
    pub const fn unit(&self) -> &U {
        let &Self(ref target_unit) = self;

        target_unit
    }
}

/// The output of a specific control system that outputs a scalar value of type `T`.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "debug-impl", derive(Debug))]
#[repr(transparent)]
pub struct Output<T>(T)
where
    T: Scalar;

impl<T> Output<T>
where
    T: Scalar,
{
    /// Instantiate a new [`Output`] from the target scalar `T`.
    #[inline]
    pub const fn raw(target_output: T) -> Self {
        Self(target_output)
    }

    /// Take the scalar value of the output.
    #[inline]
    pub const fn take(self) -> T {
        let Self(target_output) = self;

        target_output
    }
}

/// A limit of the output of a control system.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "debug-impl", derive(Debug))]
pub struct Limit<T>
where
    T: Scalar,
{
    /// The minimum value of the limit.
    min: T,

    /// The maximum value of the limit.
    max: T,
}

/// The filtered output of a control system.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "debug-impl", derive(Debug))]
pub struct Filtered<T>
where
    T: Scalar,
{
    limit: Limit<T>,
    output: Output<T>,
}

impl<T> Filtered<T>
where
    T: Scalar,
{
    /// Determine the limit that was applied to the output.
    #[inline]
    pub const fn limit(&self) -> &Limit<T> {
        let &Self { ref limit, .. } = self;

        limit
    }

    /// Determine the unfiltered output of the control system.
    #[inline]
    pub const fn output(&self) -> &Output<T> {
        let &Self { ref output, .. } = self;

        output
    }
}
