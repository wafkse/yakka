//! Packet support.

use bincode::{BorrowDecode, Decode, Encode};

use crate::control::ControlBehavior;

/// An unitary protocol packet.
///
/// This is the top-level unit of information that is sent over a communication channel.
///
/// This is receiver-agnostic, so there is no {client,server}-only packet sets.
#[derive(Encode, BorrowDecode)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "debug-impl", derive(Debug))]
pub enum Packet<'a> {
    /// Set the aircraft's armed state.
    ///
    /// An unarmed aircraft will not respond to control inputs.
    Arm(PacketArm),

    /// A control packet of any kind.
    Control(PacketControl),

    /// Transfer a `defmt` log buffer to the peer.
    Debug(&'a [u8]),
}

/// The packet that defines the armed state of the aircraft.
#[derive(Encode, Decode)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "debug-impl", derive(Debug))]
pub enum PacketArm {
    /// Arm the drone.
    Arm,

    /// Disarm the drone.
    Disarm,
}

/// A single control packet.
#[derive(Encode, Decode)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "debug-impl", derive(Debug))]
pub enum PacketControl {
    /// Define the control behavior of the aircraft.
    Behavior(ControlBehavior),
}
