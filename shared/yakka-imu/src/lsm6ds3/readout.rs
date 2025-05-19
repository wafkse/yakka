//! Controlled readout of sensor data from the Lsm6ds3.

use embedded_hal_async::{
    delay::DelayNs,
    digital::Wait,
    i2c::{I2c, SevenBitAddress},
};

use crate::lsm6ds3::{
    raw::RawAcceleration,
    register::file::{OutXL, OutYL, OutZL},
};

use super::{
    Address,
    config::{AccelConfig, Config, Configure, DataRate, GyroConfig},
    interrupt::Route,
    range::{AccelRange, GyroRange},
    raw::{Error, RawGyro, RawLsm6ds3},
    register::{
        Read, Write,
        file::{Int1Ctrl, Int2Ctrl, OutXG, OutYG, OutZG},
    },
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

/// Parameters for the [`Readout`] handle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct Params {
    /// The data rate of the FIFO buffer and all subsequent components.
    pub data_rate: DataRate,

    /// The preset range of the accelerometer.
    pub accel_range: AccelRange,

    /// The preset range of the gyroscope.
    pub gyro_range: GyroRange,

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

/// A handle to a [`RawLsm6ds3`] inertial measurement unit destined for controlled data readout.
///
/// The primary reason behind a seperate interface is to allow the sensor to go into a low-power mode when no data is required.
///
/// Interpretation of said [`data points`] are left to the user of this structure.
///
/// [`data points`]: DataPoint
#[derive(Debug)]
pub struct Readout<'a, I, P, D, A = SevenBitAddress>(RawLsm6ds3<'a, I, P, D, A>)
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
            .wait_for_high()
            .await
            .map_err(ReadoutError::Pin)?;

        let a_x = OutXL::read(target_device.channel_mut())
            .await
            .map_err(Error::I2c)
            .map_err(ReadoutError::Raw)?;

        let (a1, a0): (u8, u8) = (a_x.xl_msb().output(), a_x.xl_lsb().output());

        let a_y = OutYL::read(target_device.channel_mut())
            .await
            .map_err(Error::I2c)
            .map_err(ReadoutError::Raw)?;

        let (a3, a2): (u8, u8) = (a_y.yl_msb().output(), a_y.yl_lsb().output());

        let a_z = OutZL::read(target_device.channel_mut())
            .await
            .map_err(Error::I2c)
            .map_err(ReadoutError::Raw)?;

        let (a5, a4): (u8, u8) = (a_z.zl_msb().output(), a_z.zl_lsb().output());

        let raw_accel = RawAcceleration::bytes([a0, a1, a2, a3, a4, a5]);

        let g_x = OutXG::read(target_device.channel_mut())
            .await
            .map_err(Error::I2c)
            .map_err(ReadoutError::Raw)?;

        let (g1, g0): (u8, u8) = (g_x.x_msb().output(), g_x.x_lsb().output());

        let g_y = OutYG::read(target_device.channel_mut())
            .await
            .map_err(Error::I2c)
            .map_err(ReadoutError::Raw)?;

        let (g3, g2): (u8, u8) = (g_y.y_msb().output(), g_y.y_lsb().output());

        let g_z = OutZG::read(target_device.channel_mut())
            .await
            .map_err(Error::I2c)
            .map_err(ReadoutError::Raw)?;

        let (g5, g4): (u8, u8) = (g_z.z_msb().output(), g_z.z_lsb().output());

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
        mut target_device: RawLsm6ds3<'a, I, P, D, A>,
        readout_params: Params,
    ) -> Result<Self, Error<I::Error>> {
        let Params {
            data_rate,
            interrupt_route,
            accel_range,
            gyro_range,
        } = readout_params;

        AccelConfig::raw(Config::tuple((accel_range, data_rate)))
            .configure(&mut target_device)
            .await?;

        GyroConfig::raw(Config::tuple((gyro_range, data_rate)))
            .configure(&mut target_device)
            .await?;

        if let Route::Int1 | Route::Simultaneous = interrupt_route {
            let target_value = Int1Ctrl::read(target_device.channel_mut()).await?;

            Int1Ctrl::write(
                target_device.channel_mut(),
                target_value
                    .int1_drdy_xl()
                    .enabled()
                    .int1_drdy_g()
                    .enabled(),
            )
            .await?
        }

        if let Route::Int2 | Route::Simultaneous = interrupt_route {
            let target_value = Int2Ctrl::read(target_device.channel_mut()).await?;

            Int2Ctrl::write(
                target_device.channel_mut(),
                target_value
                    .int2_drdy_xl()
                    .enabled()
                    .int2_drdy_g()
                    .enabled(),
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
    /// Reference the underlying [`RawLsm6ds3`] device controlled by this handle.
    #[inline]
    pub const fn device(&self) -> &RawLsm6ds3<'a, I, P, D, A> {
        let &Self(ref device) = self;

        device
    }

    /// Reference the underlying [`RawLsm6ds3`] device controlled by this handle.
    #[inline]
    pub const fn device_mut(&mut self) -> &mut RawLsm6ds3<'a, I, P, D, A> {
        let &mut Self(ref mut device) = self;

        device
    }
}
