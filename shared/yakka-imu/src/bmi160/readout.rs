//! Controlled readout of sensor data from the BMI160.

use embedded_hal_async::{
    delay::DelayNs,
    digital::Wait,
    i2c::{I2c, SevenBitAddress},
};

use crate::bmi160::raw::RawAcceleration;

use super::{
    Address,
    config::{AccelConfig, Configure, GyroConfig},
    interrupt::Route,
    raw::{Error, RawBmi160, RawGyro},
    register::{
        Read, Write,
        file::{AccX, AccY, AccZ, GyroX, GyroY, GyroZ, IntMap1, IntOutCtrl},
    },
    sensor::{AccelRange, AccelRate, Bandwidth, GyroRange, GyroRate},
};

/// The individual components of the FIFO buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum Components {
    /// The accelerometer component.
    Accelerometer(AccelRange),

    /// The gyroscope component.
    Gyroscope(GyroRange),

    /// Both the accelerometer and the gyroscope.
    Both(AccelRange, GyroRange),
}

/// A sensor-agnostic data rate representation.
///
/// This is simply the enumeration of all shared sample rates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub enum DataRate {
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
}

/// Parameters for the [`Readout`] handle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct Params {
    /// The data rate of the FIFO buffer and all subsequent components.
    pub data_rate: DataRate,

    /// The preset range of the accelerometer.
    pub accel_range: AccelRange,

    /// The preset range of the gyroscope.
    pub gyro_range: GyroRange,

    /// The bandwidth of the sensors.
    pub bandwidth: Bandwidth,

    /// The route used for interrupt-driven readout.
    pub interrupt_route: Route,
}

/// A data point sourced from the [`Readout`] handle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct DataPoint(pub RawAcceleration, pub RawGyro);

/// An error describing a failure possible in the [`Readout`] handle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum ReadoutError<E, P> {
    /// An error due to a faulty interrupt vector.
    Pin(P),

    /// An error internal to a raw handle.
    Raw(Error<E>),
}

/// A handle to a [`RawBmi160`] inertial measurement unit destined for controlled data readout.
///
/// The primary reason behind a seperate interface is to allow the sensor to go into a low-power mode when no data is required.
///
/// Interpretation of said [`data points`] are left to the user of this structure.
///
/// [`data points`]: DataPoint
#[derive(Debug)]
pub struct Readout<'a, I, P, D, A = SevenBitAddress>(RawBmi160<'a, I, P, D, A>)
where
    I: I2c<A>,
    P: Wait,
    D: DelayNs,
    A: Address;

impl<'a, I, P, D, A> Readout<'a, I, P, D, A>
where
    I: I2c<A>,
    P: Wait,
    D: DelayNs,
    A: Address,
{
    /// Await a single interrupt sequence from the target device.
    ///
    /// This will read a singular [`DataPoint`].
    #[inline]
    pub async fn read(&mut self) -> Result<DataPoint, ReadoutError<I::Error, P::Error>> {
        let &mut Self(ref mut target_device) = self;

        target_device
            .interrupt_mut()
            .wait_for_rising_edge()
            .await
            .map_err(ReadoutError::Pin)?;

        let a_x = AccX::read(target_device.channel_mut())
            .await
            .map_err(Error::I2c)
            .map_err(ReadoutError::Raw)?;

        let (a1, a0): (u8, u8) = (a_x.high().output(), a_x.low().output());

        let a_y = AccY::read(target_device.channel_mut())
            .await
            .map_err(Error::I2c)
            .map_err(ReadoutError::Raw)?;

        let (a3, a2): (u8, u8) = (a_y.high().output(), a_y.low().output());

        let a_z = AccZ::read(target_device.channel_mut())
            .await
            .map_err(Error::I2c)
            .map_err(ReadoutError::Raw)?;

        let (a5, a4): (u8, u8) = (a_z.high().output(), a_z.low().output());

        let raw_accel = RawAcceleration::bytes([a0, a1, a2, a3, a4, a5]);

        let g_x = GyroX::read(target_device.channel_mut())
            .await
            .map_err(Error::I2c)
            .map_err(ReadoutError::Raw)?;

        let (g1, g0): (u8, u8) = (g_x.high().output(), g_x.low().output());

        let g_y = GyroY::read(target_device.channel_mut())
            .await
            .map_err(Error::I2c)
            .map_err(ReadoutError::Raw)?;

        let (g3, g2): (u8, u8) = (g_y.high().output(), g_y.low().output());

        let g_z = GyroZ::read(target_device.channel_mut())
            .await
            .map_err(Error::I2c)
            .map_err(ReadoutError::Raw)?;

        let (g5, g4): (u8, u8) = (g_z.high().output(), g_z.low().output());

        let raw_gyro = RawGyro::bytes([g0, g1, g2, g3, g4, g5]);

        Ok(DataPoint(raw_accel, raw_gyro))
    }
}

impl<'a, I, P, D, A> Readout<'a, I, P, D, A>
where
    I: I2c<A>,
    P: Wait,
    D: DelayNs,
    A: Address,
{
    /// Instantiate a raw readout handle for the target device.
    ///
    /// This configures the target device for interrupt-driven data readout.
    #[inline]
    pub async fn raw(
        mut target_device: RawBmi160<'a, I, P, D, A>,
        readout_params: Params,
    ) -> Result<Self, Error<I::Error>> {
        let Params {
            data_rate,
            interrupt_route,
            accel_range,
            gyro_range,
            bandwidth,
        } = readout_params;

        AccelConfig {
            sample_rate: match data_rate {
                DataRate::Rate25 => AccelRate::Rate25,
                DataRate::Rate50 => AccelRate::Rate50,
                DataRate::Rate100 => AccelRate::Rate100,
                DataRate::Rate200 => AccelRate::Rate200,
                DataRate::Rate400 => AccelRate::Rate400,
                DataRate::Rate800 => AccelRate::Rate800,
                DataRate::Rate1600 => AccelRate::Rate1600,
            },
            sensor_range: accel_range,
            bandwidth,
        }
        .configure(&mut target_device)
        .await?;

        GyroConfig {
            sample_rate: match data_rate {
                DataRate::Rate25 => GyroRate::Rate25,
                DataRate::Rate50 => GyroRate::Rate50,
                DataRate::Rate100 => GyroRate::Rate100,
                DataRate::Rate200 => GyroRate::Rate200,
                DataRate::Rate400 => GyroRate::Rate400,
                DataRate::Rate800 => GyroRate::Rate800,
                DataRate::Rate1600 => GyroRate::Rate1600,
            },
            sensor_range: gyro_range,
            bandwidth,
        }
        .configure(&mut target_device)
        .await?;

        if let Route::Int1 | Route::Simultaneous = interrupt_route {
            let target_value = IntOutCtrl::read(target_device.channel_mut()).await?;

            IntOutCtrl::write(
                target_device.channel_mut(),
                target_value
                    .interrupt_one_edge_triggered()
                    .enabled()
                    .interrupt_one_enable()
                    .enabled(),
            )
            .await?;

            let target_value = IntMap1::read(target_device.channel_mut()).await?;

            IntMap1::write(
                target_device.channel_mut(),
                target_value.interrupt_one_data_ready().enabled(),
            )
            .await?
        }

        if let Route::Int2 | Route::Simultaneous = interrupt_route {
            let target_value = IntOutCtrl::read(target_device.channel_mut()).await?;

            IntOutCtrl::write(
                target_device.channel_mut(),
                target_value
                    .interrupt_two_edge_triggered()
                    .enabled()
                    .interrupt_two_enable()
                    .enabled(),
            )
            .await?;

            let target_value = IntMap1::read(target_device.channel_mut()).await?;

            IntMap1::write(
                target_device.channel_mut(),
                target_value.interrupt_two_data_ready().enabled(),
            )
            .await?
        }

        Ok(Self(target_device))
    }
}

impl<'a, I, P, D, A> Readout<'a, I, P, D, A>
where
    I: I2c<A>,
    P: Wait,
    D: DelayNs,
    A: Address,
{
    /// Reference the underlying [`RawBmi160`] device controlled by this handle.
    #[inline]
    pub const fn device(&self) -> &RawBmi160<'a, I, P, D, A> {
        let &Self(ref device) = self;

        device
    }

    /// Reference the underlying [`RawBmi160`] device controlled by this handle.
    #[inline]
    pub const fn device_mut(&mut self) -> &mut RawBmi160<'a, I, P, D, A> {
        let &mut Self(ref mut device) = self;

        device
    }
}
