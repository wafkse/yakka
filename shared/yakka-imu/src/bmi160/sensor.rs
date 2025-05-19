//! Sensor range presets.

use yakka_fixpoint::Fixpoint;

use crate::bmi160::scale::{Scale, Scaled};

/// A marker trait for types that represent a range of values.
pub trait Ranged: Scaled + Copy {}

/// The set range of the gyroscope built that can be configured in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
#[repr(u8)]
pub enum GyroRange {
    /// A gyroscope range of `± 125 º/s`.
    ///
    /// This corresponds to a resolution of `262.144` LSB/º/s.
    Dps125,

    /// A gyroscope range of `± 250 º/s`.
    ///
    /// This corresponds to a resolution of `131.072` LSB/º/s.
    Dps250,

    /// A gyroscope range of `± 500 º/s`.
    ///
    /// This corresponds to a resolution of `65.536` LSB/º/s.
    Dps500,

    /// A gyroscope range of `± 1000 º/s`.
    ///
    /// This corresponds to a resolution of `32.768` LSB/º/s.
    Dps1000,

    /// A gyroscope range of `± 2000 º/s`.
    ///
    /// This corresponds to a resolution of `16.384` LSB/º/s.
    ///
    /// This is the default range.
    #[default]
    Dps2000,
}

impl Scaled for GyroRange {
    #[inline]
    fn scale(&self) -> Scale {
        match self {
            GyroRange::Dps125 => Scale::fixed(Fixpoint::convert(262.144)),
            GyroRange::Dps250 => Scale::fixed(Fixpoint::convert(131.072)),
            GyroRange::Dps500 => Scale::fixed(Fixpoint::convert(65.536)),
            GyroRange::Dps1000 => Scale::fixed(Fixpoint::convert(32.768)),
            GyroRange::Dps2000 => Scale::fixed(Fixpoint::convert(16.384)),
        }
    }
}

/// The sample data rate of the gyroscope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub enum GyroRate {
    /// A data rate of `25 Hz`.
    Rate25 = 0b0110,

    /// A data rate of `50 Hz`.
    Rate50 = 0b0111,

    /// A data rate of `100 Hz`.
    #[default]
    Rate100 = 0b1000,

    /// A data rate of `200 Hz`.
    Rate200 = 0b1001,

    /// A data rate of `400 Hz`.
    Rate400 = 0b1010,

    /// A data rate of `800 Hz`.
    Rate800 = 0b1011,

    /// A data rate of `1600 Hz`.
    Rate1600 = 0b1100,

    /// A data rate of `3200 Hz`.
    Rate3200 = 0b1101,
}

/// The set range of the accelerometer that can be configured in.
///``
/// This is a fixed constant multiplied by `g` (9.81 m/s^2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub enum AccelRange {
    /// +/- 2g
    ///
    /// This implies a scaling factor of `16384` LSB/g.
    ///
    /// This is the default value.
    #[default]
    G2 = 0b0011,

    /// +/- 4g
    ///
    /// This implies a scaling factor of `8192` LSB/g.
    G4 = 0b0101,

    /// +/- 8g
    ///
    /// This implies a scaling factor of `4096` LSB/g.
    G8 = 0b1000,

    /// +/- 16g
    ///
    /// This implies a scaling factor of `2048` LSB/g.
    G16 = 0b1100,
}

impl Scaled for AccelRange {
    #[inline]
    fn scale(&self) -> Scale {
        match self {
            Self::G2 => Scale::fixed(Fixpoint::scalar(16384)),
            Self::G4 => Scale::fixed(Fixpoint::scalar(8192)),
            Self::G8 => Scale::fixed(Fixpoint::scalar(4096)),
            Self::G16 => Scale::fixed(Fixpoint::scalar(2048)),
        }
    }
}

impl<S> Ranged for S where S: Scaled + Copy {}

/// The sample data rate of the accelerometer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub enum AccelRate {
    /// A data rate of `25 Hz`.
    Rate25 = 0b0110,

    /// A data rate of `50 Hz`.
    Rate50 = 0b0111,

    /// A data rate of `100 Hz`.
    #[default]
    Rate100 = 0b1000,

    /// A data rate of `200 Hz`.
    Rate200 = 0b1001,

    /// A data rate of `400 Hz`.
    Rate400 = 0b1010,

    /// A data rate of `800 Hz`.
    Rate800 = 0b1011,

    /// A data rate of `1600 Hz`.
    Rate1600 = 0b1100,

    /// A data rate of `3200 Hz`.
    Rate3200 = 0b1101,
}

/// The filter mode setting.
///
/// This is shared across both accelerometer and gyroscope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub enum Bandwidth {
    /// The `OSR4` filter mode.
    Osr4 = 0b000,

    /// The `OSR2` filter mode.
    Osr2 = 0b001,

    /// The default filter mode.
    #[default]
    Normal = 0b010,
}
