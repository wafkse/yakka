//! Movement that is uniform both {2,3}-dimensional spaces.

use yakka_number::scalar::Scalar;

use crate::{
    position::{Position2, Position3},
    velocity::{Velocity2, Velocity3},
};

/// A struct that represents uniform motion in a 3-dimensional space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Uniform3<T>
where
    T: Scalar,
{
    /// The position of the object.
    pub point_vector: Position3<T>,

    /// The velocity of the object.
    pub velocity_vector: Velocity3<T>,
}

impl<T> Uniform3<T>
where
    T: Scalar,
{
    /// Retrieve the 3-dimensional point of the uniform.
    #[inline]
    pub const fn point(&self) -> &Position3<T> {
        let &Self {
            ref point_vector, ..
        } = self;

        point_vector
    }

    /// Retrieve a mutable reference to the 3-dimensional point of the uniform.
    #[inline]
    pub const fn point_mut(&mut self) -> &mut Position3<T> {
        let &mut Self {
            ref mut point_vector,
            ..
        } = self;

        point_vector
    }

    /// Retrieve the 3-dimensional velocity of the uniform.
    #[inline]
    pub const fn velocity(&self) -> &Velocity3<T> {
        let &Self {
            ref velocity_vector,
            ..
        } = self;

        velocity_vector
    }

    /// Retrieve a mutable reference to the 3-dimensional velocity of the uniform.
    #[inline]
    pub const fn velocity_mut(&mut self) -> &mut Velocity3<T> {
        let &mut Self {
            ref mut velocity_vector,
            ..
        } = self;

        velocity_vector
    }
}

/// A struct that represents uniform motion in a 2-dimensional space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Uniform2<T>
where
    T: Scalar,
{
    /// The position of the object.
    pub point_vector: Position2<T>,

    /// The velocity of the object.
    pub velocity_vector: Velocity2<T>,
}

impl<T> Uniform2<T>
where
    T: Scalar,
{
    /// Retrieve the 2-dimensional point of the uniform.
    #[inline]
    pub const fn point(&self) -> &Position2<T> {
        let &Self {
            ref point_vector, ..
        } = self;

        point_vector
    }

    /// Retrieve a mutable reference to the 2-dimensional point of the uniform.
    #[inline]
    pub const fn point_mut(&mut self) -> &mut Position2<T> {
        let &mut Self {
            ref mut point_vector,
            ..
        } = self;

        point_vector
    }

    /// Retrieve the 2-dimensional velocity of the uniform.
    #[inline]
    pub const fn velocity(&self) -> &Velocity2<T> {
        let &Self {
            ref velocity_vector,
            ..
        } = self;

        velocity_vector
    }

    /// Retrieve a mutable reference to the 2-dimensional velocity of the uniform.
    #[inline]
    pub const fn velocity_mut(&mut self) -> &mut Velocity2<T> {
        let &mut Self {
            ref mut velocity_vector,
            ..
        } = self;

        velocity_vector
    }
}
