//! The different gain factors of a PID controller.
//!
//! This module exposes various structures to represent each of the gain factors of a PID
//! controller.

use core::ops::{Deref, DerefMut};

use yakka_number::{prelude::Zero, scalar::Scalar};



/// The proportional gain of a PID (Proportional-Integral-Derivative) controller.
///
/// This is a transparent wrapper over the underlying scalar, therefore the use of this structure is
/// only useful to tag the data with its associated use case.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]

#[repr(transparent)]
pub struct Proportional<T>(T)
where
    T: Scalar;

impl<T> Proportional<T>
where
    T: Scalar,
{
    /// Instantiate a new [`Proportional`] gain factor from the target scalar `T`.
    #[inline]
    pub const fn raw(scalar: T) -> Self {
        Self(scalar)
    }
}

impl<T> Deref for Proportional<T>
where
    T: Scalar,
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        let &Self(ref target_scalar) = self;

        target_scalar
    }
}

impl<T> DerefMut for Proportional<T>
where
    T: Scalar,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        let &mut Self(ref mut target_scalar) = self;

        target_scalar
    }
}

/// The integral gain of a PID (Proportional-Integral-Derivative) controller.
///
/// This is a transparent wrapper over the underlying scalar, therefore the use of this structure is
/// only useful to tag the data with its associated use case.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]

#[repr(transparent)]
pub struct Integral<T>(pub T)
where
    T: Scalar;

impl<T> Integral<T>
where
    T: Scalar,
{
    /// Instantiate a new error of `0` from the target scalar `T`.
    #[inline]
    pub fn zero() -> Self
    where
        T: Zero,
    {
        Self(T::zero())
    }
}

impl<T> Integral<T>
where
    T: Scalar,
{
    /// Instantiate a new [`Integral`] gain factor from the target scalar `T`.
    #[inline]
    pub const fn raw(scalar: T) -> Self {
        Self(scalar)
    }

    /// Determine the value of the integral gain factor.
    #[inline]
    pub const fn value(&self) -> T {
        let &Self(target_scalar) = self;

        target_scalar
    }
}

impl<T> Deref for Integral<T>
where
    T: Scalar,
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        let &Self(ref target_scalar) = self;

        target_scalar
    }
}

impl<T> DerefMut for Derivative<T>
where
    T: Scalar,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        let &mut Self(ref mut target_scalar) = self;

        target_scalar
    }
}

/// The derivative gain of a PID (Proportional-Integral-Derivative) controller.
///
/// This is a transparent wrapper over the underlying scalar, therefore the use of this structure is
/// only useful to tag the data with its associated use case.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]

#[repr(transparent)]
pub struct Derivative<T>(T)
where
    T: Scalar;

impl<T> Derivative<T>
where
    T: Scalar,
{
    /// Instantiate a new [`Derivative`] gain factor from the target scalar `T`.
    #[inline]
    pub const fn raw(scalar: T) -> Self {
        Self(scalar)
    }
}

impl<T> Deref for Derivative<T>
where
    T: Scalar,
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        let &Self(ref target_scalar) = self;

        target_scalar
    }
}

/// A structure that encompasses all gain factors for a PID controller.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]

pub struct Gain<T>(Proportional<T>, Integral<T>, Derivative<T>)
where
    T: Scalar;

impl<T> Gain<T>
where
    T: Scalar,
{
    /// Instantiate a new [`Gain`] from the target 3-tuple of raw values.
    #[inline]
    pub const fn tuple((p, i, d): (T, T, T)) -> Self {
        let p = Proportional::raw(p);

        let i = Integral::raw(i);

        let d = Derivative::raw(d);

        Self(p, i, d)
    }
}

impl<T> Gain<T>
where
    T: Scalar,
{
    /// Determine the proportional gain factor of this
    pub const fn p(&self) -> Proportional<T> {
        let &Self(target_value, ..) = self;

        target_value
    }

    /// Determine the integral gain factor
    pub const fn i(&self) -> Integral<T> {
        let &Self(_, target_value, ..) = self;

        target_value
    }

    /// Determine the derivative gain factor
    pub const fn d(&self) -> Derivative<T> {
        let &Self(.., target_value) = self;

        target_value
    }
}
