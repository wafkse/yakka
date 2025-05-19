//! FIFO Buffer mode selections.

use core::ops::{Deref, DerefMut};

use embedded_hal_async::{delay::DelayNs, digital::Wait, i2c::I2c};
use yakka_bit::many::Extract;

use super::{
    Address,
    config::{Configure, DataRate, Disable},
    raw::{Error, RawLsm6ds3},
    register::{
        Read, Write,
        file::{FifoCtrl1, FifoCtrl2, FifoCtrl3, FifoCtrl5},
    },
};

/// The FIFO mode of the device.
///
/// # Remarks
///
/// Note that this is not exhaustive over all behavior, defer to the device datasheet for a tangible input.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Default)]
pub enum FifoMode {
    /// No data is written to the FIFO buffer, i.e, it is bypassed completely.
    ///
    /// This is the default mode.
    #[default]
    Bypass = 0b000,

    /// Regular FIFO mode, data is written to the FIFO buffer until is is full.
    ///
    /// No data overwrites will happen after such occurence.
    Fifo = 0b001,

    /// The FIFO buffer is continously written to, discarding any older data if it happens to be full.
    ///
    /// This also presents extranous behavior compared to other [`FifoMode`]s.
    ///
    /// Particularly, this supports entabling a "FIFO Threshold", which essentially means a limit where an interrupt can be generated.
    ///
    Continous = 0b110,

    /// A hybrid mode where the active mode is determined by the interrupt bit in one of the device interrupt registers.
    ///
    /// This will operate under [`FifoMode::Fifo`] when said bit is `1`, [`FifoMode::Continous`] otherwise.
    ContinuousToFifo = 0b011,

    /// A hybrid mode where the active mode is determined by the interrupt bit in one of the device interrupt registers.
    ///
    /// This will operate under [`FifoMode::Bypass`] when said bit is `1`, [`FifoMode::Fifo`] otherwise.
    BypassToFifo = 0b100,
}

/// The watermark level of the FIFO.
///
/// # Remarks
///
/// Even though this is represented as an [`u16`], it will be truncated to 14-bits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct Watermark(u16);

impl Watermark {
    /// Construct a new [`Watermark`] level.
    #[inline]
    pub const fn level(target_value: u16) -> Self {
        Self(target_value)
    }
}

impl Deref for Watermark {
    type Target = u16;

    #[inline]
    fn deref(&self) -> &Self::Target {
        let &Self(ref target_value) = self;

        target_value
    }
}

impl DerefMut for Watermark {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        let &mut Self(ref mut target_value) = self;

        target_value
    }
}

/// Whether a component appears in the FIFO buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub enum Appearance {
    /// No, the component is not to appear in the FIFO buffer.
    #[default]
    No,

    /// Yes, the component is to appear in the FIFO buffer.
    ///
    /// The decimation of the target component is also specified.
    Yes(Decimation),
}

/// The decimation factor of a FIFO buffer component.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Default)]
pub enum Decimation {
    /// No decimation at all.
    ///
    /// This is the default value.
    #[default]
    None = 0b001,

    /// `2` decimation factor.
    D2 = 0b010,

    /// `4` decimation factor.
    D4 = 0b011,

    /// `8` decimation factor.
    D8 = 0b100,

    /// `16` decimation factor.
    D16 = 0b101,

    /// `32` decimation factor.
    D32 = 0b110,
}

/// The settings of the FIFO buffer.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Ord, Eq, Hash, Default)]
pub struct FifoSettings {
    /// The mode of the FIFO buffer.
    pub mode: FifoMode,

    /// The data rate of the FIFO buffer.
    pub rate: DataRate,

    /// The FIFO buffer watermark, if any.
    pub watermark: Option<Watermark>,

    /// The appearance of the accerelometer in the FIFO buffer.
    pub accelerometer: Appearance,

    /// The appearance of the gyroscope in the FIFO buffer.
    pub gyroscope: Appearance,
}

impl Configure for FifoSettings {
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
        let &Self {
            mode,
            rate,
            watermark,
            accelerometer,
            gyroscope,
        } = self;

        if let Some(target_watermark) = watermark {
            let target_value = FifoCtrl1::read(target_device.channel_mut()).await?;

            FifoCtrl1::write(
                target_device.channel_mut(),
                target_value
                    .fth()
                    .overwrite(Extract::<0, 7>::extract(target_watermark.deref()))
                    .value(),
            )
            .await?;

            let target_value = FifoCtrl2::read(target_device.channel_mut()).await?;

            FifoCtrl2::write(
                target_device.channel_mut(),
                target_value
                    .fth_11_8()
                    .overwrite(Extract::<8, 11>::extract(target_watermark.deref()))
                    .value(),
            )
            .await?;
        }

        let target_value = FifoCtrl3::read(target_device.channel_mut()).await?;

        let to_bits = |target_value: Appearance| match target_value {
            Appearance::No => 0b000,
            Appearance::Yes(decimation) => decimation as u8,
        };

        FifoCtrl3::write(
            target_device.channel_mut(),
            target_value
                .dec_fifo_gyro()
                .overwrite(to_bits(gyroscope))
                .value()
                .dec_fifo_xl()
                .overwrite(to_bits(accelerometer))
                .value(),
        )
        .await?;

        let target_value = FifoCtrl5::read(target_device.channel_mut()).await?;

        FifoCtrl5::write(
            target_device.channel_mut(),
            target_value
                .fifo_mode()
                .overwrite(mode as u8)
                .value()
                .odr_fifo()
                .overwrite(rate as u8)
                .value(),
        )
        .await
        .map_err(Error::I2c)
    }
}

impl Disable for FifoSettings {
    async fn disable<'a, I, P, D, A>(
        target_device: &mut RawLsm6ds3<'a, I, P, D, A>,
    ) -> Result<(), Error<I::Error>>
    where
        I: I2c<A>,
        P: Wait,
        D: DelayNs,
        A: Address,
    {
        let target_value = FifoCtrl5::read(target_device.channel_mut()).await?;

        FifoCtrl5::write(
            target_device.channel_mut(),
            target_value
                .fifo_mode()
                .overwrite(FifoMode::default() as u8)
                .value()
                .odr_fifo()
                .overwrite(0b000)
                .value(),
        )
        .await
        .map_err(Error::I2c)
    }
}
