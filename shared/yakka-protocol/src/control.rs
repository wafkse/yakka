//! Packets used to control the aircraft.

use bincode::{Decode, Encode};

/// The control behavior of the aircraft.
#[derive(Encode, Decode, Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "debug-impl", derive(Debug))]
pub enum ControlBehavior {
    /// Fully-manual control, no automated stabilization.
    Accrobatic,

    /// A control mode where the control outputs are the weighted sum of both manual control and automated control outputs.
    #[default]
    Managed,
}
