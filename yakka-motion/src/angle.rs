//! Angular motion.
//!
//! This module provides a representation of angular motion in 2-dimensional and 3-dimensional space.
//!
//! This module is designed to only provide Euler angles, i.e: pitch, roll, and yaw.

use core::{fmt, marker, ops};

use yakka_number::scalar::Scalar;

/// An angle in that is represented in counts of `1 / 128` of `π rad`.
///
/// The primary reasons for this representation are:
///   - No floating point math, all math done here is done with integers.
///   - Unique representation of angles, i.e: `256` does not fit in an [`u8`] but `2π rad` is exactly the same as `0 rad`.
///   - Precision of `1 / 128` of `π rad` is sufficient for most applications, more specifically, we have up to `(1 / 128) × pi radians ≈ 1.406250000°` degrees of precision.
///   - No need for explicit angle normalization, as the CPU "does" it for us.
#[repr(transparent)]
pub struct Angle<T>(u8, marker::PhantomData<T>)
where
    T: Scalar;

impl<T> Angle<T> where T: Scalar {}

impl<T> Angle<T>
where
    T: Scalar,
{
    /// Instantiate a new [`Angle`] from an [`u8`] representing the angle in counts of `1 / 128` of `π rad`.
    #[inline]
    pub const fn raw(target_increment: u8) -> Self {
        Self(target_increment, marker::PhantomData)
    }
}
impl<T> ops::Add for Angle<T>
where
    T: Scalar,
{
    type Output = Self;

    #[inline]
    fn add(self, Self(b, ..): Self) -> Self::Output {
        let &Self(a, ..) = &self;

        Self::raw(a.wrapping_add(b))
    }
}

impl<T> ops::Sub for Angle<T>
where
    T: Scalar,
{
    type Output = Self;

    #[inline]
    fn sub(self, Self(b, ..): Self) -> Self::Output {
        let &Self(a, ..) = &self;

        Self::raw(a.wrapping_sub(b))
    }
}

impl<T> ops::Mul for Angle<T>
where
    T: Scalar,
{
    type Output = Self;

    #[inline]
    fn mul(self, Self(b, ..): Self) -> Self::Output {
        let &Self(a, ..) = &self;

        Self::raw(a.wrapping_mul(b))
    }
}

impl<T> Clone for Angle<T>
where
    T: Scalar,
{
    #[inline]
    fn clone(&self) -> Self {
        let &Self(a, ..) = self;

        Self(a, marker::PhantomData)
    }
}

impl<T> Copy for Angle<T> where T: Scalar {}

impl<T> PartialEq for Angle<T>
where
    T: Scalar,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        let &Self(ref a, ..) = self;
        let &Self(ref b, ..) = other;

        a == b
    }
}

impl<T> fmt::Debug for Angle<T>
where
    T: Scalar,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let &Self(ref a, ..) = self;

        f.debug_tuple("Angle").field(a).finish_non_exhaustive()
    }
}

/// A struct that represents [Tait–Bryan angles] (Euler angles) in 3-dimensional space.
///
/// [Tait–Bryan angles]: https://en.wikipedia.org/wiki/Euler_angles#Tait%E2%80%93Bryan_angles
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Euler3<T>
where
    T: Scalar,
{
    /// The pitch component of this Euler angle.
    pitch: Angle<T>,

    /// The roll component of this Euler angle.
    roll: Angle<T>,

    /// The yaw component of this Euler angle.
    yaw: Angle<T>,
}

impl<T> Euler3<T>
where
    T: Scalar,
{
    /// Determine the pitch component of this Euler angle.
    #[inline]
    pub const fn pitch(&self) -> &Angle<T> {
        let &Self { ref pitch, .. } = self;

        pitch
    }

    /// Determine the roll component of this Euler angle.
    #[inline]
    pub const fn roll(&self) -> &Angle<T> {
        let &Self { ref roll, .. } = self;

        roll
    }

    /// Determine the yaw component of this Euler angle.
    pub const fn yaw(&self) -> &Angle<T> {
        let &Self { ref yaw, .. } = self;

        yaw
    }

    /// Determine the pitch component of this Euler angle, mutably.
    #[inline]
    pub const fn pitch_mut(&mut self) -> &mut Angle<T> {
        let &mut Self { ref mut pitch, .. } = self;

        pitch
    }

    /// Determine the roll component of this Euler angle, mutably.
    #[inline]
    pub const fn roll_mut(&mut self) -> &mut Angle<T> {
        let &mut Self { ref mut roll, .. } = self;

        roll
    }

    /// Determine the yaw component of this Euler angle, mutably.
    #[inline]
    pub const fn yaw_mut(&mut self) -> &mut Angle<T> {
        let &mut Self { ref mut yaw, .. } = self;

        yaw
    }
}

/// A struct that represents T in 2-dimensional space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Euler2<T>
where
    T: Scalar,
{
    /// The pitch component of this Euler angle.
    pitch: Angle<T>,

    /// The yaw component of this Euler angle.
    yaw: Angle<T>,
}

impl<T> Euler2<T>
where
    T: Scalar,
{
    /// Determine the pitch component of this Euler angle.
    #[inline]
    pub const fn pitch(&self) -> &Angle<T> {
        let &Self { ref pitch, .. } = self;

        pitch
    }

    /// Determine the yaw component of this Euler angle.
    #[inline]
    pub const fn yaw(&self) -> &Angle<T> {
        let &Self { ref yaw, .. } = self;

        yaw
    }

    /// Determine the pitch component of this Euler angle, mutably.
    #[inline]
    pub const fn pitch_mut(&mut self) -> &mut Angle<T> {
        let &mut Self { ref mut pitch, .. } = self;

        pitch
    }

    /// Determine the yaw component of this Euler angle, mutably.
    #[inline]
    pub const fn yaw_mut(&mut self) -> &mut Angle<T> {
        let &mut Self { ref mut yaw, .. } = self;

        yaw
    }
}

/// A structure encompassing the angular motion of a system in 3-dimensional space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Angular3<T>
where
    T: Scalar,
{
    /// The current angle of the system.
    angle: Euler3<T>,

    /// The angular velocity of the system.
    angular_velocity: Euler3<T>,
}

impl<T> Angular3<T>
where
    T: Scalar,
{
    /// Retrieve the current angle of the system.
    #[inline]
    pub const fn angle(&self) -> &Euler3<T> {
        let &Self { ref angle, .. } = self;

        angle
    }

    /// Retrieve a mutable reference to the current angle of the system.
    #[inline]
    pub const fn angle_mut(&mut self) -> &mut Euler3<T> {
        let &mut Self { ref mut angle, .. } = self;

        angle
    }

    /// Retrieve the angular velocity of the system.
    #[inline]
    pub const fn angular_velocity(&self) -> &Euler3<T> {
        let &Self {
            ref angular_velocity,
            ..
        } = self;

        angular_velocity
    }
}

/// A structure encompassing the angular motion of a system in 2-dimensional space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Angular2<T>
where
    T: Scalar,
{
    /// The current angle of the system.
    angle: Euler2<T>,

    /// The angular velocity of the system.
    angular_velocity: Euler2<T>,
}

impl<T> Angular2<T>
where
    T: Scalar,
{
    /// Retrieve the current angle of the system.
    #[inline]
    pub const fn angle(&self) -> &Euler2<T> {
        let &Self { ref angle, .. } = self;

        angle
    }

    /// Retrieve a mutable reference to the current angle of the system.
    #[inline]
    pub const fn angle_mut(&mut self) -> &mut Euler2<T> {
        let &mut Self { ref mut angle, .. } = self;

        angle
    }

    /// Retrieve the angular velocity of the system.
    #[inline]
    pub const fn angular_velocity(&self) -> &Euler2<T> {
        let &Self {
            ref angular_velocity,
            ..
        } = self;

        angular_velocity
    }
}
