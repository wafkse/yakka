//! Interrupt management.

use core::ops::{Deref, DerefMut};

use embedded_hal_async::digital::Wait;

/// An interrupt pin for [`Lsm6ds3`]-related operation.
#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[repr(transparent)]
pub struct Interrupt<P>(P)
where
    P: Wait;

impl<P> Interrupt<P>
where
    P: Wait,
{
    /// Mark the target pin `P` as an interrupt-enabled pin.
    #[inline]
    pub const fn pin(pin: P) -> Self {
        Self(pin)
    }
}

impl<P> Deref for Interrupt<P>
where
    P: Wait,
{
    type Target = P;

    #[inline]
    fn deref(&self) -> &Self::Target {
        let &Self(ref target_value) = self;

        target_value
    }
}

impl<P> DerefMut for Interrupt<P>
where
    P: Wait,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        let &mut Self(ref mut target_value) = self;

        target_value
    }
}

/// The route of an interrupt.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub enum Route {
    /// Route the interrupt only through the `INT1` pin.
    Int1,

    /// Route the interrupt only through the `INT2` pin.
    Int2,

    /// Route the interrupt through both the `INT1` and `INT2` pins at once.
    Simultaneous,
}

/// The configuration of the interrupt generator pins: `INT1` and `INT2`.
pub struct InterruptConfig {}

// TODO: populate interrupt config and setup readout interface.
