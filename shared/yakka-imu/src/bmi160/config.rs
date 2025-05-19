//! Device configuration.

use embedded_hal_async::{delay::DelayNs, digital::Wait, i2c::I2c};

use super::{
    Address,
    raw::{Error, RawBmi160},
    register::{
        self, Read, Write,
        file::{AccConf, AccRange, GyroConf},
    },
    sensor::{AccelRange, AccelRate, Bandwidth, GyroRange, GyroRate},
};

/// A trait for [`Config`]-related structures that can be directly applied to a [`RawBmi160`] device.
#[allow(async_fn_in_trait, reason = "no send/sync contraints in embedded")]
pub trait Configure {
    /// Configure the target [`RawBmi160`] device with the target settings.
    async fn configure<'a, I, P, D, A>(
        &self,
        target_device: &mut RawBmi160<'a, I, P, D, A>,
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
        target_device: &mut RawBmi160<'a, I, P, D, A>,
    ) -> Result<(), Error<I::Error>>
    where
        I: I2c<A>,
        P: Wait,
        D: DelayNs,
        A: Address;
}

/// The accelerometer configuration structure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct AccelConfig {
    /// The data sample rate of the gyroscope.
    pub sample_rate: AccelRate,

    /// The range of the gyroscope.
    pub sensor_range: AccelRange,

    /// The bandwidth and filter parameters of the gyroscope.
    pub bandwidth: Bandwidth,
}

impl Configure for AccelConfig {
    async fn configure<'a, I, P, D, A>(
        &self,
        target_device: &mut RawBmi160<'a, I, P, D, A>,
    ) -> Result<(), Error<I::Error>>
    where
        I: I2c<A>,
        P: Wait,
        D: DelayNs,
        A: Address,
    {
        let &Self {
            sample_rate,
            sensor_range,
            bandwidth,
        } = self;

        let target_value = AccConf::read(target_device.channel_mut()).await?;

        AccConf::write(
            target_device.channel_mut(),
            target_value
                .bandwidth_mode()
                .overwrite(bandwidth as u8)
                .value()
                .data_rate()
                .overwrite(sample_rate as u8)
                .value(),
        )
        .await
        .map_err(Error::I2c)?;

        let target_value = AccRange::read(target_device.channel_mut()).await?;

        AccRange::write(
            target_device.channel_mut(),
            target_value
                .sensor_range()
                .overwrite(sensor_range as u8)
                .value(),
        )
        .await
        .map_err(Error::I2c)
    }
}

/// The gyroscope configuration structure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct GyroConfig {
    /// The data sample rate of the gyroscope.
    pub sample_rate: GyroRate,

    /// The range of the gyroscope.
    pub sensor_range: GyroRange,

    /// The bandwidth and filter parameters of the gyroscope.
    pub bandwidth: Bandwidth,
}

impl Configure for GyroConfig {
    async fn configure<'a, I, P, D, A>(
        &self,
        target_device: &mut RawBmi160<'a, I, P, D, A>,
    ) -> Result<(), Error<I::Error>>
    where
        I: I2c<A>,
        P: Wait,
        D: DelayNs,
        A: Address,
    {
        let &Self {
            sample_rate,
            sensor_range,
            bandwidth,
        } = self;

        let target_value = GyroConf::read(target_device.channel_mut()).await?;

        GyroConf::write(
            target_device.channel_mut(),
            target_value
                .bandwidth_mode()
                .overwrite(bandwidth as u8)
                .value()
                .data_rate()
                .overwrite(sample_rate as u8)
                .value(),
        )
        .await
        .map_err(Error::I2c)?;

        let target_value = register::file::GyroRange::read(target_device.channel_mut()).await?;

        register::file::GyroRange::write(
            target_device.channel_mut(),
            target_value
                .sensor_range()
                .overwrite(sensor_range as u8)
                .value(),
        )
        .await
        .map_err(Error::I2c)
    }
}
