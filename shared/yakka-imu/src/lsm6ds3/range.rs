//! Sensor range presets.

use yakka_fixpoint::Fixpoint;

use crate::lsm6ds3::scale::{Scale, Scaled};

/// A marker trait for types that represent a range of values.
pub trait Ranged: Scaled + Copy {}

/// The set range of the gyroscope built that can be configured in.
///
/// # Remarks
///
/// This is as to be interpreted as a bit sequence `FS_G1 FS_G0 FS_125` residing in the `CTRL2_G` register.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
#[repr(u8)]
pub enum GyroRange {
    /// +/- 145 degrees per second.
    ///
    /// Corresponds to a scale of 4.375 mdps/s.
    Dps125 = 0x01,

    /// +/- 245 degrees per second.
    ///
    /// Corresponds to a scale of 8.75 mdps/s.
    ///
    /// This is the default preset scale.
    #[default]
    Dps245 = 0x00,

    /// +/- 500 degrees per second.
    ///
    /// Corresponds to a scale of 17.50 mdps/s.
    Dps500 = 0x02,
    /// +/- 1000 degrees per second.
    ///
    /// Corresponds to a scale of 35 mdps/s.
    Dps1000 = 0x04,

    /// +/- 2000 degrees per second.
    ///
    /// Corresponds to a scale of 70 mdps/s.
    Dps2000 = 0x06,
}

impl Scaled for GyroRange {
    #[inline]
    fn scale(&self) -> Scale {
        match self {
            Self::Dps125 => Scale::fixed(Fixpoint::convert(4.375)),
            Self::Dps245 => Scale::fixed(Fixpoint::convert(8.75)),
            Self::Dps500 => Scale::fixed(Fixpoint::convert(17.5)),
            Self::Dps1000 => Scale::fixed(Fixpoint::convert(35.0)),
            Self::Dps2000 => Scale::fixed(Fixpoint::convert(70.0)),
        }
    }
}

/// The set range of the accelerometer that can be configured in.
///
/// This is a fixed constant multiplied by `g` (9.81 m/s^2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub enum AccelRange {
    /// +/- 2g
    ///
    /// Corresponds to a scale of 0.061 LSB/g.
    #[default]
    G2 = 0x00,

    /// +/- 4g
    ///
    /// Corresponds to a scale of 0.122 LSB/g.
    G4 = 0x02,

    /// +/- 8g
    ///
    /// Corresponds to a scale of 0.244 LSB/g.
    G8 = 0x03,

    /// +/- 16g
    ///
    /// Corresponds to a scale of 0.488 LSB/g.
    G16 = 0x01,
}

impl Scaled for AccelRange {
    #[inline]
    fn scale(&self) -> Scale {
        match self {
            Self::G2 => Scale::fixed(Fixpoint::convert(0.061)),
            Self::G4 => Scale::fixed(Fixpoint::convert(0.122)),
            Self::G8 => Scale::fixed(Fixpoint::convert(0.244)),
            Self::G16 => Scale::fixed(Fixpoint::convert(0.488)),
        }
    }
}

impl<S> Ranged for S where S: Scaled + Copy {}
