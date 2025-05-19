//! Device configuration.

use core::ops::{Deref, DerefMut};

use embedded_hal_async::{delay::DelayNs, digital::Wait, i2c::I2c};

use super::{
    Address,
    range::{AccelRange, GyroRange, Ranged},
    raw::{Error, RawLsm6ds3},
    register::{
        Read, Write,
        file::{Ctrl1Xl, Ctrl2G},
    },
};

/// A data rate.
///
/// This is shared across all three components: Gyroscope, Accelerometer, and FIFO buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum DataRate {
    /// A `12.5 Hz` sample rate.
    Rate12_5 = 0b0001,

    /// A `26 Hz` sample rate.
    Rate26 = 0b0010,

    /// A `52 Hz` sample rate.
    ///
    /// This is the default data rate.
    #[default]
    Rate52 = 0b0011,

    /// A `104 Hz` sample rate.
    Rate104 = 0b0100,

    /// A `208 Hz` sample rate.
    Rate208 = 0b0101,

    /// A `416 Hz` sample rate.
    Rate416 = 0b0110,

    /// A `883 Hz` sample rate.
    Rate833 = 0b0111,

    /// A `1.66 kHz` sample rate.
    Rate1_66k = 0b1000,

    /// A `3.33 kHz` sample rate.
    Rate3_33k = 0b1001,

    /// A `6.66 kHz` sample rate.
    Rate6_66k = 0b1010,
}

/// A full-fledged component configuration structure.
///
/// This applies to both gyroscope and accelerometer devices.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Config<R>
where
    R: Ranged,
{
    /// The target operational range.
    target_range: R,

    /// The data rate of the device.
    target_rate: DataRate,
}

impl<R> Config<R>
where
    R: Ranged,
{
    /// Construct a new [`Config`] from a 2-tuple of the range and the target data rate.
    #[inline]
    pub const fn tuple((target_range, target_rate): (R, DataRate)) -> Self {
        Self {
            target_range,
            target_rate,
        }
    }

    /// Deconstruct this [`Config`] into a 2-tuple consisting of its raw parts.
    #[inline]
    pub const fn parts(self) -> (R, DataRate) {
        let Self {
            target_range,
            target_rate,
        } = self;

        (target_range, target_rate)
    }
}

impl<R> Config<R>
where
    R: Ranged,
{
    /// Determine the range `R` contained in this [`Config`].
    #[inline]
    pub const fn range(&self) -> R {
        let &Self { target_range, .. } = self;

        target_range
    }

    /// Determine the [`DataRate`] contained in this [`Config`].
    #[inline]
    pub const fn data_rate(&self) -> DataRate {
        let &Self { target_rate, .. } = self;

        target_rate
    }
}

/// A trait for [`Config`]-related structures that can be directly applied to a [`RawLsm6ds3`] device.
#[allow(async_fn_in_trait, reason = "no send/sync contraints in embedded")]
pub trait Configure {
    /// Configure the target [`RawLsm6ds3`] device with the target settings.
    async fn configure<'a, I, P, D, A>(
        &self,
        target_device: &mut RawLsm6ds3<'a, I, P, D, A>,
    ) -> Result<(), Error<I::Error>>
    where
        I: I2c<A>,
        P: Wait,
        D: DelayNs,
        A: Address;
}

/// A trait for [`Config`]-related structures that can be completely disabled altogether.
///
/// In turn, this does not require an existing reference to the structure.
#[allow(async_fn_in_trait, reason = "no send/sync contraints in embedded")]
pub trait Disable {
    /// Disable the component respective to this configuration structure.
    async fn disable<'a, I, P, D, A>(
        target_device: &mut RawLsm6ds3<'a, I, P, D, A>,
    ) -> Result<(), Error<I::Error>>
    where
        I: I2c<A>,
        P: Wait,
        D: DelayNs,
        A: Address;
}

/// A wrapper over a target [`Configure`] `C` that permits the existence of a deactivated sensor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum SensorConfig<C>
where
    C: Configure + Disable,
{
    /// An active sensor, with a target configuration `C`.
    Active(C),

    /// A deactivated sensor.
    #[default]
    Deactivated,
}

impl<C> Configure for SensorConfig<C>
where
    C: Configure + Disable,
{
    #[inline]
    async fn configure<'a, I, P, D, A>(
        &self,
        target_device: &mut RawLsm6ds3<'a, I, P, D, A>,
    ) -> Result<(), Error<I::Error>>
    where
        I: I2c<A>,
        P: Wait,
        D: DelayNs,
        A: Address,
    {
        match self {
            SensorConfig::Active(target_value) => C::configure(target_value, target_device).await,
            SensorConfig::Deactivated => <C as Disable>::disable(target_device).await,
        }
    }
}

/// The accelerometer configuration structure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct AccelConfig(Config<AccelRange>);

impl AccelConfig {
    /// Create a new [`AccelConfig`] from its raw [`Config`] value.
    #[inline]
    pub const fn raw(config: Config<AccelRange>) -> Self {
        Self(config)
    }
}

impl Configure for AccelConfig {
    async fn configure<'a, I, P, D, A>(
        &self,
        target_device: &mut RawLsm6ds3<'a, I, P, D, A>,
    ) -> Result<(), Error<I::Error>>
    where
        I: I2c<A>,
        P: Wait,
        D: DelayNs,
        A: Address,
    {
        let &Self(target_value) = self;

        let Config {
            target_range,
            target_rate,
        } = target_value;

        let target_value = Ctrl1Xl::read(target_device.channel_mut()).await?;

        Ctrl1Xl::write(
            target_device.channel_mut(),
            target_value
                .fs_xl()
                .overwrite(target_range as u8)
                .value()
                .odr_xl()
                .overwrite(target_rate as u8)
                .value(),
        )
        .await
        .map_err(Error::I2c)
    }
}

impl Disable for AccelConfig {
    async fn disable<'a, I, P, D, A>(
        target_device: &mut RawLsm6ds3<'a, I, P, D, A>,
    ) -> Result<(), Error<I::Error>>
    where
        I: I2c<A>,
        P: Wait,
        D: DelayNs,
        A: Address,
    {
        let target_value = Ctrl1Xl::read(target_device.channel_mut()).await?;

        Ctrl1Xl::write(
            target_device.channel_mut(),
            target_value
                .fs_xl()
                .overwrite(AccelRange::G2 as u8)
                .value()
                .odr_xl()
                .overwrite(0b000)
                .value(),
        )
        .await
        .map_err(Error::I2c)
    }
}

impl Deref for AccelConfig {
    type Target = Config<AccelRange>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        let &Self(ref target_value) = self;

        target_value
    }
}

impl DerefMut for AccelConfig {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        let &mut Self(ref mut target_value) = self;

        target_value
    }
}

/// The gyroscope configuration structure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct GyroConfig(Config<GyroRange>);

impl GyroConfig {
    /// Create a new [`GyroConfig`] from its raw [`Config`] value.
    #[inline]
    pub const fn raw(config: Config<GyroRange>) -> Self {
        Self(config)
    }
}

impl Configure for GyroConfig {
    async fn configure<'a, I, P, D, A>(
        &self,
        target_device: &mut RawLsm6ds3<'a, I, P, D, A>,
    ) -> Result<(), Error<I::Error>>
    where
        I: I2c<A>,
        P: Wait,
        D: DelayNs,
        A: Address,
    {
        let &Self(target_value) = self;

        let Config {
            target_range,
            target_rate,
        } = target_value;

        let target_value = Ctrl2G::read(target_device.channel_mut()).await?;

        Ctrl2G::write(
            target_device.channel_mut(),
            target_value
                .fs_125_g()
                .overwrite(target_range as u8)
                .value()
                .odr_g()
                .overwrite(target_rate as u8)
                .value(),
        )
        .await
        .map_err(Error::I2c)
    }
}

impl Disable for GyroConfig {
    async fn disable<'a, I, P, D, A>(
        target_device: &mut RawLsm6ds3<'a, I, P, D, A>,
    ) -> Result<(), Error<I::Error>>
    where
        I: I2c<A>,
        P: Wait,
        D: DelayNs,
        A: Address,
    {
        let target_value = Ctrl2G::read(target_device.channel_mut()).await?;

        Ctrl2G::write(
            target_device.channel_mut(),
            target_value
                .fs_125_g()
                .overwrite(GyroRange::Dps245 as u8)
                .value()
                .odr_g()
                .overwrite(0b000)
                .value(),
        )
        .await
        .map_err(Error::I2c)
    }
}

impl Deref for GyroConfig {
    type Target = Config<GyroRange>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        let &Self(ref target_value) = self;

        target_value
    }
}

impl DerefMut for GyroConfig {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        let &mut Self(ref mut target_value) = self;

        target_value
    }
}
