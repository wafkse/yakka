//! Spatial positions for distinct motions in {2,3}-dimensional space.

use core::ops::{Deref, DerefMut};

use nalgebra::{Vector2, Vector3};
use yakka_number::scalar::Scalar;



/// A position in 3-dimensional space.
#[derive(Clone, Copy, PartialEq)]

#[repr(transparent)]
pub struct Position3<T>(Vector3<T>)
where
    T: Scalar;

impl<T> Deref for Position3<T>
where
    T: Scalar,
{
    type Target = Vector3<T>;

    fn deref(&self) -> &Self::Target {
        let &Self(ref target_vector) = self;

        target_vector
    }
}

impl<T> DerefMut for Position3<T>
where
    T: Scalar,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        let &mut Self(ref mut target_vector) = self;

        target_vector
    }
}

/// A position in 2-dimensional space.
#[derive(Clone, Copy, PartialEq)]

#[repr(transparent)]
pub struct Position2<T>(Vector2<T>)
where
    T: Scalar;

impl<T> Deref for Position2<T>
where
    T: Scalar,
{
    type Target = Vector2<T>;

    fn deref(&self) -> &Self::Target {
        let &Self(ref target_vector) = self;

        target_vector
    }
}

impl<T> DerefMut for Position2<T>
where
    T: Scalar,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        let &mut Self(ref mut target_vector) = self;

        target_vector
    }
}
