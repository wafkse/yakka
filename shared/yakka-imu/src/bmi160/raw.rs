//! Representation for raw sensor data.

/// A new-type wrapper over an [`i16`] value, representing a discretized analog value.
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Analog(i16);

impl Analog {
    /// Wrap a raw [`i16`] value into this new-type.
    #[inline]
    pub const fn raw(raw_value: i16) -> Self {
        Self(raw_value)
    }

    /// Unwrap the raw value from this new-type.
    #[inline]
    pub const fn unwrap(self) -> i16 {
        let Self(raw_value) = self;

        raw_value
    }
}

/// The raw acceleration values reported by the sensor.
///
/// # Remarks
///
/// This does not necesarilly represent an "acceleration" per se, but rather values the sensor can directly understand.
///
/// For instance, this could be an offset value, or a raw ADC-converted value, or whatever, as long as it is acceleration-related.
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct RawAcceleration {
    /// The X-axis acceleration as reported by the sensor.
    pub x: Analog,

    /// The Y-axis acceleration as reported by the sensor.
    pub y: Analog,

    /// The Z-axis acceleration as reported by the sensor.
    pub z: Analog,
}

impl RawAcceleration {
    /// Instantiate a new raw acceleration structure from 3-tuple of raw sensor data.
    #[inline]
    pub const fn tuple((x, y, z): (Analog, Analog, Analog)) -> Self {
        Self { x, y, z }
    }

    /// Interpret a 6-byte array as a raw acceleration structure.
    ///
    /// This is in the exact same order as the raw bytes sent over the wire by the sensor.
    ///
    /// # Layout
    ///
    /// ```no_run
    /// ┌────────────┬────────────┬────────────┬────────────┐
    /// |    Half    │   X-axis   |   Y-axis   |   Z-axis   |
    /// ┼────────────┼────────────┼────────────┼────────────┤
    /// │    Low     |    0x00    |    0x02    |    0x04    |            
    /// ┼────────────┼────────────┼────────────┼────────────┤
    /// │    High    |    0x01    |    0x03    |    0x05    |
    /// └────────────┴────────────┴────────────┴────────────┘
    /// ```
    ///
    /// The input data is expected to be in little-endian format.
    #[inline]
    pub const fn bytes([b0, b1, b2, b3, b4, b5]: [u8; 6]) -> Self {
        let x = Analog::raw(i16::from_le_bytes([b0, b1]));
        let y = Analog::raw(i16::from_le_bytes([b2, b3]));
        let z = Analog::raw(i16::from_le_bytes([b4, b5]));

        Self { x, y, z }
    }

    /// Convert this [`RawAcceleration`] into a 6-byte array.
    ///
    /// This is in the exact same order as the raw bytes sent over the wire by the sensor.
    ///
    /// # Layout
    ///
    /// ```no_run
    /// ┌────────────┬────────────┬────────────┬────────────┐
    /// |    Half    │   X-axis   |   Y-axis   |   Z-axis   |
    /// ┼────────────┼────────────┼────────────┼────────────┤
    /// │    Low     |    0x00    |    0x02    |    0x04    |            
    /// ┼────────────┼────────────┼────────────┼────────────┤
    /// │    High    |    0x01    |    0x03    |    0x05    |
    /// └────────────┴────────────┴────────────┴────────────┘
    /// ```
    #[inline]
    pub const fn to_bytes(self) -> [u8; 6] {
        let Self { x, y, z } = self;

        let [x0, x1] = x.unwrap().to_le_bytes();
        let [y0, y1] = y.unwrap().to_le_bytes();
        let [z0, z1] = z.unwrap().to_le_bytes();

        [x0, x1, y0, y1, z0, z1]
    }
}

/// The raw angular velocity values reported by the sensor.
///
/// # Remarks
///
/// This does not necesarilly represent an "angular velocity" per se, but rather values the sensor can directly understand.
///
/// For instance, this could be an offset value, or a raw ADC-converted value, or whatever, as long as it is rotation-related.
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct RawGyro {
    /// The roll-axis rotation as reported by the sensor.
    pub roll: Analog,

    /// The pitch-axis rotation as reported by the sensor.
    pub pitch: Analog,

    /// The yaw-axis rotation as reported by the sensor.
    pub yaw: Analog,
}

impl RawGyro {
    /// Instantiate a new raw angular velocity structure from 3-tuple of raw sensor data.
    #[inline]
    pub const fn tuple((x, y, z): (Analog, Analog, Analog)) -> Self {
        Self {
            roll: x,
            pitch: y,
            yaw: z,
        }
    }

    /// Interpret a 6-byte array as a [`RawGyro`] structure.
    ///
    /// This is in the exact same order as the raw bytes sent over the wire by the sensor.
    ///
    /// # Layout
    ///
    /// ```no_run
    /// ┌────────────┬────────────┬────────────┬────────────┐
    /// |    Half    │    Roll    |   Pitch    |    Yaw     |
    /// ┼────────────┼────────────┼────────────┼────────────┤
    /// │    Low     |    0x00    |    0x02    |    0x04    |            
    /// ┼────────────┼────────────┼────────────┼────────────┤
    /// │    High    |    0x01    |    0x03    |    0x05    |
    /// └────────────┴────────────┴────────────┴────────────┘
    /// ```
    ///
    /// The input data is expected to be in little-endian format.
    #[inline]
    pub const fn bytes([b0, b1, b2, b3, b4, b5]: [u8; 6]) -> Self {
        let x = Analog::raw(i16::from_le_bytes([b0, b1]));
        let y = Analog::raw(i16::from_le_bytes([b2, b3]));
        let z = Analog::raw(i16::from_le_bytes([b4, b5]));

        Self {
            roll: x,
            pitch: y,
            yaw: z,
        }
    }

    /// Convert this [`RawGyro`] struct into a 6-byte array.
    ///
    /// This is in the exact same order as the raw bytes sent over the wire by the sensor.
    ///
    /// # Layout
    ///
    /// ```no_run
    /// ┌────────────┬────────────┬────────────┬────────────┐
    /// |    Half    │    Roll    |   Pitch    |    Yaw     |
    /// ┼────────────┼────────────┼────────────┼────────────┤
    /// │    Low     |    0x00    |    0x02    |    0x04    |            
    /// ┼────────────┼────────────┼────────────┼────────────┤
    /// │    High    |    0x01    |    0x03    |    0x05    |
    /// └────────────┴────────────┴────────────┴────────────┘
    /// ```
    #[inline]
    pub const fn to_bytes(self) -> [u8; 6] {
        let Self { roll, pitch, yaw } = self;

        let [r0, r1] = roll.unwrap().to_le_bytes();
        let [p0, p1] = pitch.unwrap().to_le_bytes();
        let [y0, y1] = yaw.unwrap().to_le_bytes();

        [r0, r1, p0, p1, y0, y1]
    }
}

/// The raw temperature value reported by the sensor.
///
/// This is a single [`Analog`] value, therefore, the layout of this struct is identical to the layout of the [`Analog`] struct.
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct Temperature(Analog);

impl Temperature {
    /// Instantiate a new [`Temperature`] struct from a raw [`Analog`] reading.
    #[inline]
    pub const fn reading(target_value: Analog) -> Self {
        Self(target_value)
    }

    /// Interpret a 2-byte array as a [`Temperature`] structure.
    ///
    /// This is in the exact same order as the raw bytes sent over the wire by the sensor.
    ///
    /// # Layout
    ///
    /// ```no_run
    /// ┌────────────┬────────────┐
    /// |    Half    │    Temp    |
    /// ┼────────────┤────────────┤
    /// │    Low     |    0x00    |
    /// ┼────────────┤────────────┤
    /// │    High    |    0x01    |
    /// └────────────┴────────────┘
    /// ```
    ///
    /// The byte order is little-endian.
    #[inline]
    pub const fn bytes([b0, b1]: [u8; 2]) -> Self {
        let value = Analog::raw(i16::from_le_bytes([b0, b1]));

        Self(value)
    }

    /// Convert this [`Temperature`] struct into a 2-byte array.
    ///
    /// This is in the exact same order as the raw bytes sent over the wire by the sensor.
    ///
    /// # Layout
    ///
    /// ```no_run
    /// ┌────────────┬────────────┐
    /// |    Half    │    Temp    |
    /// ┼────────────┤────────────┤
    /// │    Low     |    0x00    |
    /// ┼────────────┤────────────┤
    /// │    High    |    0x01    |
    /// └────────────┴────────────┘
    /// ```
    ///
    /// The byte order is little-endian.
    #[inline]
    pub const fn to_bytes(self) -> [u8; 2] {
        let Self(value) = self;

        let [b0, b1] = value.unwrap().to_le_bytes();

        [b0, b1]
    }
}

use core::ops::{Deref, DerefMut};

use embedded_hal_async::{
    delay::DelayNs,
    digital::Wait,
    i2c::{I2c, SevenBitAddress},
};

use crate::bmi160::register::{Read, Write, file::Cmd};

use super::{
    Address, Bmi160,
    config::{AccelConfig, GyroConfig},
};

/// The intended device behavior struct.
///
/// This encompasses various details about the sensor, such as sampling rate, sensibility, etc.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
pub struct Behavior {
    /// The configuration of the accelerometer.
    pub accel_config: AccelConfig,

    /// The configuration of the gyroscope.
    pub gyro_config: GyroConfig,
}

/// An error during [`RawBMI160`] operation.
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum Error<E> {
    /// A bad address was supplied.
    ///
    /// This implies that the device cannot be correctly be communicated with.
    BadAddress,

    /// An error specific to the [`I2c`] bus.
    I2c(E),
}

impl<E> From<E> for Error<E> {
    #[inline]
    fn from(target_error: E) -> Self {
        Self::I2c(target_error)
    }
}

/// A raw handle to a [`BMI160`] inertial measurement unit.
///
/// This exposes the raw sensing capabilities of the device, without further processing.
#[derive(Debug)]
pub struct RawBmi160<'a, I, P, D, A = SevenBitAddress>(&'a mut Bmi160<I, P, D, A>)
where
    I: I2c<A>,
    P: Wait,
    D: DelayNs,
    A: Address;

impl<'a, I, P, D, A> RawBmi160<'a, I, P, D, A>
where
    I: I2c<A>,
    P: Wait,
    D: DelayNs,
    A: Address,
{
    /// Initialize a new [`RawBMI160`] device handle.
    ///
    /// This will completely reset the device, default to its preset settings.
    #[inline]
    pub async fn handle(
        target_device: &'a mut Bmi160<I, P, D, A>,
    ) -> Result<Self, Error<I::Error>> {
        const RESET_DELAY_NS: u32 = 1000;

        const CMD_RESET_SOFTWARE: u8 = 0xB6;

        let command_value = Cmd::read(target_device.channel_mut()).await?;

        Cmd::write(
            target_device.channel_mut(),
            command_value
                .command_bits()
                .overwrite(CMD_RESET_SOFTWARE)
                .value(),
        )
        .await?;

        target_device.timer_mut().delay_ns(RESET_DELAY_NS).await;

        Ok(Self(target_device))
    }
}

impl<'a, I, P, D, A> Deref for RawBmi160<'a, I, P, D, A>
where
    I: I2c<A>,
    P: Wait,
    D: DelayNs,
    A: Address,
{
    type Target = Bmi160<I, P, D, A>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        let &Self(ref target_value) = self;

        target_value
    }
}

impl<'a, I, P, D, A> DerefMut for RawBmi160<'a, I, P, D, A>
where
    I: I2c<A>,
    P: Wait,
    D: DelayNs,
    A: Address,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        let &mut Self(ref mut target_value) = self;

        target_value
    }
}
