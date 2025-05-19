//! Sensor scales for sensors of any kind.

use yakka_fixpoint::Fixpoint;

/// A trait for sensor ranges that have a preset scale.
pub trait Scaled {
    /// Determine the sensor scale for the target value.
    fn scale(&self) -> Scale;
}

/// A scale for a sensor value.
///
/// This is applied to the raw sensor data to convert it into a meaningful value, as per the setting of the sensor.
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Scale(Fixpoint<u32, 12>);

impl Scale {
    /// Instantiate a new scale from a raw value.
    #[inline]
    pub const fn raw(raw_value: u32) -> Self {
        Self(Fixpoint::raw(raw_value))
    }

    /// Instantiate a new scale from a barebones [`Fixpoint`] value.
    #[inline]
    pub const fn fixed(target_value: Fixpoint<u32, 12>) -> Self {
        Self(target_value)
    }

    /// Unwrap the raw value from this new-type.
    #[inline]
    pub const fn value(self) -> u32 {
        let Self(target_scale) = self;

        target_scale.value()
    }
}
