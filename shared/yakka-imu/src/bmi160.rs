//! Driver implementation for the BMI160 6-DOF sensor driver.

pub mod sensor;

pub mod scale;

pub mod interrupt;

pub mod register;

pub mod raw;

pub mod readout;

pub mod config;

use embedded_hal_async::{
    delay::DelayNs,
    digital::Wait,
    i2c::{AddressMode, I2c, SevenBitAddress},
};
use raw::Error;
use readout::{Params, Readout};

use self::{interrupt::Interrupt, raw::RawBmi160};

mod private {
    /// A private trait to prevent external implementations of any traits this [`Sealed`] trait is a supertrait of.
    pub trait Sealed {}
}

/// Helper trait to determine the default I2C address for a BMI160 device for the target address mode.
pub trait DefaultAddress: private::Sealed {
    /// The default [`I2c`] address for a BMI160 device in the target address mode.
    const DEFAULT: Self;
}

impl private::Sealed for SevenBitAddress {}

impl DefaultAddress for SevenBitAddress {
    const DEFAULT: Self = 0x68;
}

/// An address mode intended for use with the BMI160 device.
pub trait Address: AddressMode + Copy + private::Sealed {}

impl<A> Address for A where A: AddressMode + Copy + private::Sealed {}

/// A communication channel to a [`I2c`] slave device.
///
/// This is a simple combination of [`I2c`] bus and the address of the respective device.
#[derive(Debug, Clone)]
pub struct Channel<I, A>(I, A)
where
    I: I2c<A>,
    A: Address;

impl<I, A> Channel<I, A>
where
    I: I2c<A>,
    A: Address,
{
    /// Construct a new [`Channel`] for the device `A`.
    #[inline]
    pub const fn raw(bus: I, address: A) -> Self {
        Self(bus, address)
    }
}

impl<I, A> Channel<I, A>
where
    I: I2c<A>,
    A: Address,
{
    /// Determine the [`I2c`] address used to communicate over the bus.
    #[inline]
    pub const fn address(&self) -> A {
        let &Self(.., target_value) = self;

        target_value
    }

    /// Determine the [`I2c`] bus used to communicate with the BMI160.
    #[inline]
    pub const fn bus(&self) -> &I {
        let &Self(ref target_value, ..) = self;

        target_value
    }

    /// Determine the [`I2c`] bus used to communicate with the BMI160.
    #[inline]
    pub const fn bus_mut(&mut self) -> &mut I {
        let &mut Self(ref mut target_value, ..) = self;

        target_value
    }
}

/// The driver structure for the BMI160 Inertial Measurement Unit.
///
/// # Remarks
///
/// This is just an abstraction over the [`I2c`] bus the BMI160 is connected to, as it provides no sensor reading capability by itself.
///
/// To read data from the sensor, see [`BMI160::readout`].
#[derive(Debug, Clone)]
pub struct Bmi160<I, P, D, A = SevenBitAddress>(Channel<I, A>, Interrupt<P>, D)
where
    I: I2c<A>,
    P: Wait,
    A: Address;

impl<I, P, D, A> Bmi160<I, P, D, A>
where
    I: I2c<A>,
    P: Wait,
    A: Address,
{
    /// Access the underlying [`Channel`] inside this [`BMI160`] driver, immutably.
    #[inline]
    pub const fn channel(&self) -> &Channel<I, A> {
        let &Self(ref target_channel, ..) = self;

        target_channel
    }

    /// Access the underlying [`Channel`] inside this [`BMI160`] driver, mutably.
    #[inline]
    pub const fn channel_mut(&mut self) -> &mut Channel<I, A> {
        let &mut Self(ref mut target_channel, ..) = self;

        target_channel
    }

    /// Access the underlying [`Interrupt<P>`] inside this [`BMI160`] driver, immutably.
    #[inline]
    pub const fn interrupt(&self) -> &Interrupt<P> {
        let &Self(.., ref target_interrupt, _) = self;

        target_interrupt
    }

    /// Access the underlying [`Interrupt<P>`] inside this [`BMI160`] driver, mutably.
    #[inline]
    pub const fn interrupt_mut(&mut self) -> &mut Interrupt<P> {
        let &mut Self(.., ref mut target_interrupt, _) = self;

        target_interrupt
    }

    /// Access the underlying [`DelayNs`]-based timer inside this [`BMI160`] driver, immutably.
    #[inline]
    pub const fn timer(&self) -> &D {
        let &Self(.., ref target_delay) = self;

        target_delay
    }

    /// Access the underlying [`DelayNs`]-based timer inside this [`BMI160`] driver, mutably.
    #[inline]
    pub const fn timer_mut(&mut self) -> &mut D {
        let &mut Self(.., ref mut target_delay) = self;

        target_delay
    }
}

impl<I, P, D, A> Bmi160<I, P, D, A>
where
    I: I2c<A>,
    P: Wait,
    D: DelayNs,
    A: Address,
{
    /// Instantiate a new [`Readout`] handle for this device.
    #[inline]
    pub async fn readout<'a>(
        &'a mut self,
        read_param: Params,
    ) -> Result<Readout<'a, I, P, D, A>, Error<I::Error>> {
        Readout::raw(Self::raw(self).await?, read_param).await
    }

    /// Instantiate a new [`RawBMI160`] handle for this device.
    #[inline]
    pub async fn raw<'a>(&'a mut self) -> Result<RawBmi160<'a, I, P, D, A>, Error<I::Error>> {
        RawBmi160::handle(self).await
    }
}

impl<I, P, D, A> Bmi160<I, P, D, A>
where
    I: I2c<A>,
    P: Wait,
    D: DelayNs,
    A: Address,
{
    /// Construct a handle to a [`BMI160`] device on the other side of the target channel.
    #[inline]
    pub fn device(channel: Channel<I, A>, interrupt: Interrupt<P>, delay: D) -> Self {
        Self(channel, interrupt, delay)
    }
}
