//! Flight physical state as reported by the on-board sensors.
//!
//! Note that this is not direct sensor data, but rather the change of state of the system for some
//! time difference `dt`.

use yakka_motion::{accelerated::Accelerated3, angle::Angular3};
use yakka_number::scalar::Scalar;

/// A structure that encompasses both linear and angular motion of an aircraft.
#[derive(Clone, Copy, PartialEq)]
pub struct Motion<T>
where
    T: Scalar,
{
    /// The motion of the aircraft.
    motion: Accelerated3<T>,

    /// The angular motion of the aircraft.
    angular: Angular3<T>,
}

impl<T> Motion<T>
where
    T: Scalar,
{
    /// Determine the motion of the aircraft.
    pub const fn motion(&self) -> &Accelerated3<T> {
        let &Self { ref motion, .. } = self;

        motion
    }

    /// Determine the motion of the aircraft, mutably.
    pub const fn motion_mut(&mut self) -> &mut Accelerated3<T> {
        let &mut Self { ref mut motion, .. } = self;

        motion
    }

    /// Determine the angular motion of the aircraft.
    pub const fn angular(&self) -> &Angular3<T> {
        let &Self { ref angular, .. } = self;

        angular
    }

    /// Determine the angular motion of the aircraft, mutably.
    pub const fn angular_mut(&mut self) -> &mut Angular3<T> {
        let &mut Self {
            ref mut angular, ..
        } = self;

        angular
    }
}
