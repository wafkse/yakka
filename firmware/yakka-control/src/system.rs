//! Various structures for representing the state of a control system, be it closed-loop or
//! open-loop.

use core::ops::{Deref, DerefMut};

use yakka_number::{prelude::Zero, scalar::Scalar};

/// A trait that represents a control system of any kind: closed-loop or open-loop, and that
/// operates on scalar values of type `T`
///
/// For open-loop control systems, an unit [context] can be used. Use the [`Control::cycle`] method
/// to perform a control cycle.
///
/// [context]: Control::Context
pub trait Control<T>
where
    T: Scalar,
{
    /// The context of the control system, tied to the lifetime of some specific control cycle.
    type Context<'a>
    where
        Self: 'a;

    /// Perform a singular control cycle using the default context.
    #[inline]
    fn cycle<'a>(&'a mut self) -> T
    where
        Self::Context<'a>: Default,
    {
        let ctx = Self::Context::<'a>::default();

        self.cycle_with_ctx(ctx)
    }

    /// Perform a singular control cycle using the attached context.
    fn cycle_with_ctx<'a>(&'a mut self, ctx: Self::Context<'a>) -> T;
}

/// The setpoint of a closed-loop control system.
///
/// This represents the active target value that the system is trying to achieve.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "debug-impl", derive(Debug))]
#[repr(transparent)]
pub struct Setpoint<T>(T)
where
    T: Scalar;

impl<T> Setpoint<T>
where
    T: Scalar,
{
    /// Instantiate a new [`Setpoint`] from the target scalar `T`.
    #[inline]
    pub const fn raw(scalar: T) -> Self {
        Self(scalar)
    }

    /// Determine the value of the setpoint.
    #[inline]
    pub const fn value(&self) -> T {
        let &Self(target_scalar) = self;

        target_scalar
    }
}

impl<T> Deref for Setpoint<T>
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

/// The process variable of a control system.
///
/// This is the actual value that the system is measuring or controlling.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "debug-impl", derive(Debug))]
#[repr(transparent)]
pub struct Variable<T>(T)
where
    T: Scalar;

impl<T> Variable<T>
where
    T: Scalar,
{
    /// Instantiate a new [`Variable`] from the target scalar `T`.
    #[inline]
    pub const fn raw(scalar: T) -> Self {
        Self(scalar)
    }

    /// Determine the value of the process variable.
    #[inline]
    pub const fn value(&self) -> T {
        let &Self(target_scalar) = self;

        target_scalar
    }
}

impl<T> Deref for Variable<T>
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

/// The `SP-PV` error of a control system.
///
/// Has no explicit meaning by itself, and thus it must be defined the user.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "debug-impl", derive(Debug))]
pub struct Error<T>(T)
where
    T: Scalar;

impl<T> Error<T>
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

impl<T> Error<T>
where
    T: Scalar,
{
    /// Instantiate a new [`Error`] from the target scalar `T`.
    #[inline]
    pub const fn raw(target_error: T) -> Self {
        Self(target_error)
    }

    /// Determine the value of the error.
    #[inline]
    pub const fn value(&self) -> T {
        let &Self(target_value) = self;

        target_value
    }
}

impl<T> Deref for Error<T>
where
    T: Scalar,
{
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        let &Self(ref target_value) = self;

        target_value
    }
}

impl<T> DerefMut for Error<T>
where
    T: Scalar,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        let &mut Self(ref mut target_value) = self;

        target_value
    }
}
