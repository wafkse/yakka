//! Packets used to control the aircraft.

use bincode::{Decode, Encode};

/// The control behavior of the aircraft.
#[derive(Encode, Decode, Default)]

pub enum ControlBehavior {
    /// Fully-manual control, no automated stabilization.
    Accrobatic,

    /// A control mode where the control outputs are the weighted sum of both manual control and automated control input.
    #[default]
    Managed,
}
