//! Motion that is accelerated over time in {2,3}-dimensional space.

use core::ops::{Deref, DerefMut};


use nalgebra::Vector3;
use yakka_number::scalar::Scalar;

use crate::{position::Position3, velocity::Velocity3};

/// An acceleration in 3-dimensional space.
#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "debug-impl", derive(Debug))]
pub struct Acceleration3<T>(Vector3<T>)
where
    T: Scalar;

impl<T> Deref for Acceleration3<T>
where
    T: Scalar,
{
    type Target = Vector3<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        let &Self(ref target_vector) = self;

        target_vector
    }
}

impl<T> DerefMut for Acceleration3<T>
where
    T: Scalar,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        let &mut Self(ref mut target_vector) = self;

        target_vector
    }
}

/// Accelerated motion in 3-dimensional space.
#[derive(Clone, Copy, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "debug-impl", derive(Debug))]
pub struct Accelerated3<T>
where
    T: Scalar,
{
    /// The position of the object.
    pub point_vector: Position3<T>,

    /// The velocity of the object.
    pub velocity_vector: Velocity3<T>,

    /// The acceleration of the object.
    pub acceleration_vector: Acceleration3<T>,
}

impl<T> Accelerated3<T>
where
    T: Scalar,
{
    /// Determine the position of the object.
    #[inline]
    pub const fn position(&self) -> Position3<T> {
        let &Self { point_vector, .. } = self;

        point_vector
    }

    /// Determine the velocity of the object.
    #[inline]
    pub const fn velocity(&self) -> Velocity3<T> {
        let &Self {
            velocity_vector, ..
        } = self;

        velocity_vector
    }

    /// Determine the acceleration of the object.
    #[inline]
    pub const fn acceleration(&self) -> Acceleration3<T> {
        let &Self {
            acceleration_vector,
            ..
        } = self;

        acceleration_vector
    }
}
