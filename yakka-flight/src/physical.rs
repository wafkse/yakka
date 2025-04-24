//! Flight physical state as reported by the on-board sensors.
//!
//! Note that this is not direct sensor data, but rather the change of state of the system for some time difference `dt`.

use yakka_motion::{accelerated::Accelerated3, angle::Angular3};
use yakka_number::scalar::Scalar;

/// A self-contained representation of the physical state of an aircraft.
///
/// This structure contains both linear and angular motion data. Aditionally, it can be used to update the state of the aircraft based on a time difference `dt`, and the current state of the aircraft.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Physical<T>
where
    T: Scalar,
{
    /// The motion of the aircraft.
    motion: Accelerated3<T>,

    /// The angular motion of the aircraft.
    angular: Angular3<T>,
}

impl<T> Physical<T>
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
