//! The register file of the `lsm6ds3` Inertial Measurement Device.

use crate::lsm6ds3::register::{Read, Register, Write};

use yakka_bit::{
    handle::{Bit, BitMut},
    many::{Extract, Extracted, ExtractedMut},
    state::State,
    take::{Bits, HasPrimitive},
};

use yakka_register::value::{Register8, Register16};

use embedded_hal_async::i2c::I2c;

/// A helper macro to expand to a sequence of bit-related functions for a single bit.

macro_rules! bit_singleton {
    () => {};
    (
        $(#[$target_meta:meta])*
        $target_vis:vis $target_name:ident = $target_bit:literal
    ) => {
        permafrost::embed! {
            #[doc =
                [<
                    "Retrieve a mutable handle to the `" $target_name "` bit of this register."
                >]:concatenate{string}
            ]
            ///
            $(#[$target_meta])*
            #[inline]
            pub const fn [< $target_name _mut >]:concatenate <'a> (&'a mut self) -> BitMut<'a, Self, { $target_bit }> {

                BitMut::<'a, Self, { $target_bit }>::wrap(self)
            }
        }
    };
    (
        $(#[$target_meta:meta])*
        $target_vis:vis static $target_name:ident = $target_bit:literal
    ) => {
        permafrost::embed! {
            #[doc =
                [<
                    "Retrieve an immutable handle to the `" $target_name "` bit of this register."
                >]:concatenate{string}
            ]
            ///
            $(#[$target_meta])*
            #[inline]
            pub const fn [< $target_name >]:concatenate <'a> (&'a self) -> Bit<'a, Self, { $target_bit }> {

                Bit::<'a, Self, { $target_bit }>::wrap(self)
            }
        }
    };
}

/// A helper macro to expand to a sequence of bit-related functions for a range of bits.

macro_rules! bit_range {
    () => {};
    (
        $(#[$target_meta:meta])*
        $target_vis:vis $target_name:ident = ($target_bit_start:literal, $target_bit_end:literal)
    ) => {
        permafrost::embed! {
            #[doc =
                [<
                    "Retrieve a mutable handle to the `" $target_name "` bit sequence of this register."
                >]:concatenate{string}
            ]
            ///
            $(#[$target_meta])*
            #[inline]
            pub const fn [< $target_name _mut >]:concatenate <'a> (&'a mut self) -> ExtractedMut<'a, $target_bit_start, $target_bit_end, Self> {
                ExtractedMut::<$target_bit_start, $target_bit_end, Self>::wrap(self)
            }
        }
    };
    (
        $(#[$target_meta:meta])*
        $target_vis:vis static $target_name:ident = ($target_bit_start:literal, $target_bit_end:literal)
    ) => {
        permafrost::embed! {
            #[doc =
                [<
                    "Extract the `" $target_name "` bit sequence of this register."
                >]:concatenate{string}
            ]
            ///
            $(#[$target_meta])*
            #[inline]
            pub fn [< $target_name >](&self) -> Extracted<$target_bit_start, $target_bit_end, Self> {
                Extracted::<$target_bit_start, $target_bit_end, Self>::wrap(self)
            }
        }
    };
}

/// A macro to declare a 8-bit register new-type wrapper.
///
/// This allows both semantically and explicitly to define the register's bit layout.
macro_rules! register8 {
    () => {};
    (
        @ impl for $target_name:ident {
            $(,)?
        }
    ) => {};
    (
        @ impl for $target_name:ident {
            $(#[$target_meta:meta])*
            $target_bit:literal => $target_vis:vis $target_bit_name:ident,

            $($target_tt:tt)*
        }
    ) => {
        bit_singleton! {
            $(#[$target_meta])*
            $target_vis $target_bit_name = $target_bit
        }

        register8! {
            @ impl for $target_name {
                $($target_tt)*
            }
        }
    };
    (
        @ impl for $target_name:ident {
            $(#[$target_meta:meta])*
            $target_bit_start:literal ..= $target_bit_end:literal => $target_vis:vis $target_bit_name:ident,

            $($target_tt:tt)*
        }
    ) => {
        bit_range! {
            $(#[$target_meta])*
            $target_vis $target_bit_name = ($target_bit_start, $target_bit_end)
        }

        register8! {
            @ impl for $target_name {
                $($target_tt)*
            }
        }
    };
    (
        @ static impl for $target_name:ident {
            $(,)?
        }
    ) => {};
    (
        @ static impl for $target_name:ident {
            $(#[$target_meta:meta])*
            $target_bit:literal => $target_vis:vis $target_bit_name:ident,

            $($target_tt:tt)*
        }
    ) => {
        bit_singleton! {
            $(#[$target_meta])*
            $target_vis static $target_bit_name = $target_bit
        }

        register8! {
            @ static impl for $target_name {
                $($target_tt)*
            }
        }
    };
    (
        @ static impl for $target_name:ident {
            $(#[$target_meta:meta])*
            $target_bit_start:literal ..= $target_bit_end:literal => $target_vis:vis $target_bit_name:ident,

            $($target_tt:tt)*
        }
    ) => {
        bit_range! {
            $(#[$target_meta])*
            $target_vis static $target_bit_name = ($target_bit_start, $target_bit_end)
        }

        register8! {
            @ static impl for $target_name {
                $($target_tt)*
            }
        }
    };
    (
        $(#[$target_meta:meta])*
        $target_vis:vis $target_name:ident = $target_address:expr;
        $(
            {
                $($target_tt:tt)*
            }
        )?
    ) => {
        permafrost::embed! {
            register8! {
                $(#[$target_meta])*
                $target_vis static $target_name = $target_address;
                $(
                    {
                        $($target_tt)*
                    }
                )?
            }

            impl $target_name {
                $(
                    register8! {
                        @ impl for $target_name {
                            $($target_tt)*
                        }
                    }
                )?
            }


            impl Write for $target_name {
                #[inline]
                async fn write<'a, I, A>(channel: &'a mut $crate::lsm6ds3::Channel<I, A>, value: Self) -> Result<(), I::Error>
                where
                    I: I2c<A>,
                    A: $crate::lsm6ds3::Address
                {
                    let Self(target_register) = value;

                    let r0 = target_register.value();

                    let target_buffer = [
                        <Self as Register>::ADDRESS,
                        r0
                    ];

                    let channel_address = channel.address();

                    channel
                        .bus_mut()
                        .write(channel_address, &target_buffer)
                        .await?;

                    Ok(())
                }
            }
        }
    };
    (
        $(#[$target_meta:meta])*
        $target_vis:vis static $target_name:ident = $target_address:expr;
        $(
            {
                $($target_tt:tt)*
            }
        )?
    ) => {
        permafrost::embed! {
            $(#[$target_meta])*
            #[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
            #[repr(transparent)]
            $target_vis struct $target_name(Register8);

            impl $target_name {
                $(
                    register8! {
                        @ static impl for $target_name {
                            $($target_tt)*
                        }
                    }
                )?
            }

            impl HasPrimitive for $target_name where
                Register8: HasPrimitive,
            {
                type Primitive = <Register8 as HasPrimitive>::Primitive;

                #[inline]
                fn raw(self) -> Self::Primitive {
                    let Self(target_value) = self;

                    <Register8 as HasPrimitive>::raw(target_value)
                }
            }

            impl<const N: usize> Bits<N> for $target_name
            where
            Register8: Bits<N>,
            {
                #[inline]
                fn set(&mut self, target_state: State) -> State {
                    let &mut Self(ref mut target_value) = self;

                    Bits::<N>::set(target_value, target_state)
                }

                #[inline]
                fn get(&self) -> State {
                    let &Self(ref target_value) = self;

                    Bits::<N>::get(target_value)
                }

                #[inline]
                fn single() -> Self {
                    Self(
                        <Register8 as Bits::<N>>::single()
                    )
                }
            }


            impl<const N: usize, const M: usize> Extract<N, M> for $target_name
            where
                Register8: Extract<N, M>,
            {
                type Output = <Register8 as Extract<N, M>>::Output;

                #[inline]
                fn extract(&self) -> Self::Output {
                    let &Self(ref target_value) = self;

                    Extract::<N, M>::extract(target_value)
                }

                #[inline]
                fn fuse(&mut self, target_output: Self::Output) -> Self::Output {
                    let &mut Self(ref mut target_value) = self;

                    Extract::<N, M>::fuse(target_value, target_output)
                }
            }

            impl Register for $target_name {
                const ADDRESS: u8 = $target_address;
            }

            impl Read for $target_name {
                #[inline]
                async fn read<'a, I, A>(channel: &'a mut $crate::lsm6ds3::Channel<I, A>) -> Result<Self, I::Error>
                where
                    I: I2c<A>,
                    A: $crate::lsm6ds3::Address,
                    Self: Sized
                {
                    let mut target_buffer = [0; core::mem::size_of::<Self>()];

                    let channel_address = channel.address();

                    channel
                        .bus_mut()
                        .write_read(channel_address, core::slice::from_ref(&<Self as Register>::ADDRESS), &mut target_buffer)
                        .await?;

                    let [target_byte] = target_buffer;

                    Ok(
                        Register8::raw(target_byte)
                    ).map(Self)
                }
            }
        }
    };
}

/// A macro to declare a 16-bit register new-type wrapper.
///
/// This allows both semantically and explicitly to define the register's bit layout.
macro_rules! register16 {
    () => {};
    (
        @ impl for $target_name:ident {
            $(,)?
        }
    ) => {};
    (
        @ impl for $target_name:ident {
            $target_bit:literal => $target_vis:vis $target_bit_name:ident,

            $($target_tt:tt)*
        }
    ) => {
        bit_singleton! {
            $target_vis $target_bit_name = $target_bit
        }

        register16! {
            @ impl for $target_name {
                $($target_tt)*
            }
        }
    };
    (
        @ impl for $target_name:ident {
            $(#[$target_meta:meta])*
            $target_bit_start:literal ..= $target_bit_end:literal => $target_vis:vis $target_bit_name:ident,

            $($target_tt:tt)*
        }
    ) => {
        bit_range! {
            $(#[$target_meta])*
            $target_vis $target_bit_name = ($target_bit_start, $target_bit_end)
        }

        register16! {
            @ impl for $target_name {
                $($target_tt)*
            }
        }
    };
    (
        @ static impl for $target_name:ident {
            $(,)?
        }
    ) => {};
    (
        @ static impl for $target_name:ident {
            $(#[$target_meta:meta])*
            $target_bit:literal => $target_vis:vis $target_bit_name:ident,

            $($target_tt:tt)*
        }
    ) => {
        bit_singleton! {
            $(#[$target_meta])*
            $target_vis static $target_bit_name = $target_bit
        }

        register16! {
            @ static impl for $target_name {
                $($target_tt)*
            }
        }
    };
    (
        @ static impl for $target_name:ident {
            $(#[$target_meta:meta])*
            $target_bit_start:literal ..= $target_bit_end:literal => $target_vis:vis $target_bit_name:ident,

            $($target_tt:tt)*
        }
    ) => {
        bit_range! {
            $(#[$target_meta])*
            $target_vis static $target_bit_name = ($target_bit_start, $target_bit_end)
        }

        register16! {
            @ static impl for $target_name {
                $($target_tt)*
            }
        }
    };
    (
        $(#[$target_meta:meta])*
        $target_vis:vis $target_name:ident = $target_address:expr;
        $(
            {
                $($target_tt:tt)*
            }
        )?
    ) => {
        permafrost::embed! {
            register16! {
                $(#[$target_meta])*
                $target_vis static $target_name = $target_address;
                $(
                    {
                        $($target_tt)*
                    }
                )?
            }

            impl $target_name {
                $(
                    register16! {
                        @ impl for $target_name {
                            $($target_tt)*
                        }
                    }
                )?
            }

            impl Write for $target_name {
                #[inline]
                async fn write<'a, I, A>(channel: &'a mut $crate::lsm6ds3::Channel<I, A>, value: Self) -> Result<(), I::Error>
                where
                    I: I2c<A>,
                    A: $crate::lsm6ds3::Address
                {
                    let Self(target_register) = value;

                    let [r0, r1] = target_register.value().to_be_bytes();

                    let target_buffer = [
                        <Self as Register>::ADDRESS, /* note: assumes higher half address */
                        r0, r1
                    ];

                    let channel_address = channel.address();

                    channel
                        .bus_mut()
                        .write(channel_address, &target_buffer)
                        .await?;

                    Ok(())
                }
            }
        }
    };
    (
        $(#[$target_meta:meta])*
        $target_vis:vis static $target_name:ident = $target_address:expr;
        $(
            {
                $($target_tt:tt)*
            }
        )?
    ) => {
        permafrost::embed! {
            $(#[$target_meta])*
            ///
            /// # Remarks
            ///
            /// This is a static register, meaning that it is not mutable.
            #[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
            #[repr(transparent)]
            $target_vis struct $target_name(Register16);


            impl HasPrimitive for $target_name where
                Register16: HasPrimitive,
            {
                type Primitive = <Register16 as HasPrimitive>::Primitive;

                #[inline]
                fn raw(self) -> Self::Primitive {
                    let Self(target_value) = self;

                    <Register16 as HasPrimitive>::raw(target_value)
                }
            }


            impl<const N: usize> Bits<N> for $target_name
            where
            Register16: Bits<N>,
            {

                #[inline]
                fn set(&mut self, target_state: State) -> State {
                    let &mut Self(ref mut target_value) = self;

                    Bits::<N>::set(target_value, target_state)
                }

                #[inline]
                fn get(&self) -> State {
                    let &Self(ref target_value) = self;

                    Bits::<N>::get(target_value)
                }

                #[inline]
                fn single() -> Self {
                    Self(
                        <Register16 as Bits::<N>>::single()
                    )
                }
            }


            impl<const N: usize, const M: usize> Extract<N, M> for $target_name
            where
                Register16: Extract<N, M>,
            {
                type Output = <Register16 as Extract<N, M>>::Output;

                #[inline]
                fn extract(&self) -> Self::Output {
                    let &Self(ref target_value) = self;

                    Extract::<N, M>::extract(target_value)
                }

                #[inline]
                fn fuse(&mut self, target_output: Self::Output) -> Self::Output {
                    let &mut Self(ref mut target_value) = self;

                    Extract::<N, M>::fuse(target_value, target_output)
                }
            }

            impl $target_name {
                $(
                    register16! {
                        @ static impl for $target_name {
                            $($target_tt)*
                        }
                    }
                )?
            }

            impl Register for $target_name {
                const ADDRESS: u8 = $target_address;
            }

            impl Read for $target_name {
                #[inline]
                async fn read<'a, I, A>(channel: &'a mut $crate::lsm6ds3::Channel<I, A>) -> Result<Self, I::Error>
                where
                    I: I2c<A>,
                    A: $crate::lsm6ds3::Address,
                    Self: Sized
                {
                    let mut target_buffer = [0; core::mem::size_of::<Self>()];

                    let channel_address = channel.address();

                    channel
                        .bus_mut()
                        .write_read(channel_address, core::slice::from_ref(&<Self as Register>::ADDRESS), &mut target_buffer)
                        .await?;

                    Ok(
                        Register16::bytes(target_buffer)
                    ).map(Self)
                }
            }
        }
    };
}

register8!(
    /// Enable embedded functions register (r/w).
    ///
    /// # Layout
    ///
    /// | Bit Range | Description                                                       |
    /// |-----------|-------------------------------------------------------------------|
    /// | 7         | Enable access to embedded function registers                     |
    pub FuncCfgAccess = 0x01; {
        /// Enable access to the embedded functions configuration registers, ranging from address `0x02` to `0x32`.
        ///
        /// Default value: `0b0`.
        7 => func_cfg_en,
    }
);

register8!(
    /// Sensor synchronization time frame register (r/w).
    ///
    /// # Layout
    ///
    /// | Bit Range | Description                                                       |
    /// |-----------|-------------------------------------------------------------------|
    /// | 0..=7     | Sensor synchronization time frame setting                        |
    pub SensorSyncTimeFrame = 0x04; {
        /// Sensor synchronization time frame with a step of 500 ms and a full range of 5 s.
        ///
        /// Default value: `0b0000_0000`.
        0..=7 => tph,
    }
);

register8!(
    /// FIFO control register (r/w).
    ///
    /// # Layout
    ///
    /// | Bit Range | Description                                                       |
    /// |-----------|-------------------------------------------------------------------|
    /// | 0..=7     | FIFO threshold level setting                                       |
    pub FifoCtrl1 = 0x06; {
        /// FIFO threshold level setting.
        ///
        /// Watermark flag rises when the number of bytes written to FIFO exceeds or equals the threshold level.
        ///
        /// Default value: `0b0000_0000`.
        0..=7 => fth,
    }
);

register8!(
    /// FIFO control register 2 (r/w).
    ///
    /// # Layout
    ///
    /// | Bit Range | Description                                                       |
    /// |-----------|-------------------------------------------------------------------|
    /// | 0..=3     | FIFO threshold level setting                                       |
    pub FifoCtrl2 = 0x07; {
        /// FIFO threshold level setting.
        ///
        /// Default value: `0b0000`.
        0..=3 => fth_11_8,
    }
);

register8!(
    /// FIFO control register 3 (r/w).
    ///
    /// # Layout
    ///
    /// | Bit Range | Description                                                       |
    /// |-----------|-------------------------------------------------------------------|
    /// | 0..=7     | Gyro and accelerometer FIFO decimation settings                    |
    pub FifoCtrl3 = 0x08; {
        /// Gyroscope FIFO decimation setting.
        ///
        /// # Decimation Factor Table
        ///
        /// | Bit Pattern | Decimation Factor    |
        /// |-------------|----------------------|
        /// | `000`       | Gyro not in FIFO     |
        /// | `001`       | No decimation        |
        /// | `010`       | Decimation by 2      |
        /// | `011`       | Decimation by 3      |
        /// | `100`       | Decimation by 4      |
        /// | `101`       | Decimation by 8      |
        /// | `110`       | Decimation by 16     |
        /// | `111`       | Decimation by 32     |
        0..=2 => dec_fifo_gyro,

        /// Accelerometer FIFO decimation setting.
        ///
        /// # Decimation Factor Table
        ///
        /// | Bit Pattern | Decimation Factor    |
        /// |-------------|----------------------|
        /// | `000`       | Accelerometer not in FIFO |
        /// | `001`       | No decimation        |
        /// | `010`       | Decimation by 2      |
        /// | `011`       | Decimation by 3      |
        /// | `100`       | Decimation by 4      |
        /// | `101`       | Decimation by 8      |
        /// | `110`       | Decimation by 16     |
        /// | `111`       | Decimation by 32     |
        3..=5 => dec_fifo_xl,
    }
);

register8!(
    /// FIFO control register 4 (r/w).
    ///
    /// # Layout
    ///
    /// | Bit Range | Description                                                       |
    /// |-----------|-------------------------------------------------------------------|
    /// | 0..=7     | FIFO data set decimation settings                                  |
    pub FifoCtrl4 = 0x09; {
        /// Fourth FIFO data set decimation setting.
        ///
        /// # Decimation Factor Table
        ///
        /// | Bit Pattern | Decimation Factor    |
        /// |-------------|----------------------|
        /// | `000`       | Fourth data set not in FIFO |
        /// | `001`       | No decimation        |
        /// | `010`       | Decimation by 2      |
        /// | `011`       | Decimation by 3      |
        /// | `100`       | Decimation by 4      |
        /// | `101`       | Decimation by 8      |
        /// | `110`       | Decimation by 16     |
        /// | `111`       | Decimation by 32     |
        0..=2 => dec_ds4_fifo,

        /// Third FIFO data set decimation setting.
        ///
        /// # Decimation Factor Table
        ///
        /// | Bit Pattern | Decimation Factor    |
        /// |-------------|----------------------|
        /// | `000`       | Third data set not in FIFO |
        /// | `001`       | No decimation        |
        /// | `010`       | Decimation by 2      |
        /// | `011`       | Decimation by 3      |
        /// | `100`       | Decimation by 4      |
        /// | `101`       | Decimation by 8      |
        /// | `110`       | Decimation by 16     |
        /// | `111`       | Decimation by 32     |
        3..=5 => dec_ds3_fifo,
    }
);

register8!(
    /// FIFO control register 5 (r/w).
    ///
    /// # Layout
    ///
    /// | Bit Range | Description                                                       |
    /// |-----------|-------------------------------------------------------------------|
    /// | 0..=7     | FIFO output data rate (ODR) and mode selection                    |
    pub FifoCtrl5 = 0x0A; {
        /// FIFO output data rate selection and mode setting.
        ///
        /// # ODR Configuration Table
        ///
        /// | Bit Pattern | ODR    | Mode   |
        /// |-------------|--------|--------|
        /// | `0000`      | Disabled  | Bypass |
        /// | `0001`      | 12.5 Hz  | FIFO   |
        /// | `0010`      | 26 Hz    | FIFO   |
        /// | `0011`      | 52 Hz    | FIFO   |
        /// | `0100`      | 104 Hz   | FIFO   |
        /// | `0101`      | 208 Hz   | FIFO   |
        /// | `0110`      | 416 Hz   | FIFO   |
        /// | `0111`      | 833 Hz   | FIFO   |
        /// | `1000`      | 1.66 kHz | FIFO   |
        /// | `1001`      | 3.33 kHz | FIFO   |
        /// | `1010`      | 6.66 kHz | FIFO   |
        0..=3 => odr_fifo,

        /// FIFO mode selection setting.
        ///
        /// # Mode Configuration Table
        ///
        /// | Bit Pattern | Mode        |
        /// |-------------|-------------|
        /// | `000`       | Bypass mode |
        /// | `001`       | FIFO mode   |
        /// | `010`       | Reserved    |
        /// | `011`       | Continuous mode |
        /// | `100`       | Bypass mode until trigger |
        /// | `101`       | Reserved    |
        /// | `110`       | Continuous mode, overwrite |
        /// | `111`       | Reserved    |
        4..=6 => fifo_mode,
    }
);

register8!(
    /// Angular rate sensor sign and orientation register (r/w).
    ///
    /// This register configures the output sign and axis orientation for gyroscope measurements.
    ///
    /// # Layout
    ///
    /// | Bit Range | Description                               |
    /// |-----------|-------------------------------------------|
    /// | 0..=2     | Directional user-orientation configuration |
    /// | 3..=5     | Sign inversion for X, Y, and Z axes        |
    /// | 6..=7     | Reserved                                   |
    ///
    pub OrientCfgG = 0x0B; {
        /// Directional orientation selection for the gyroscope axes.
        ///
        /// This field sets the mapping of pitch, roll, and yaw onto the device axes.
        ///
        /// # Orientation Mapping Table
        ///
        /// | Bit Pattern | Pitch | Roll | Yaw |
        /// |-------------|--------|------|-----|
        /// | `000`       | X      | Y    | Z   |
        /// | `001`       | X      | Z    | Y   |
        /// | `010`       | Y      | X    | Z   |
        /// | `011`       | Y      | Z    | X   |
        /// | `100`       | Z      | X    | Y   |
        /// | `101`       | Z      | Y    | X   |
        ///
        /// Default: `0b000`
        0..=2 => orient,

        /// Sign inversion for the pitch axis (X).
        ///
        /// # Sign Configuration
        ///
        /// | Bit | Description     |
        /// |-----|-----------------|
        /// | `0` | Positive sign   |
        /// | `1` | Negative sign   |
        ///
        /// Default: `0`
        3 => sign_x_g,

        /// Sign inversion for the roll axis (Y).
        ///
        /// # Sign Configuration
        ///
        /// | Bit | Description     |
        /// |-----|-----------------|
        /// | `0` | Positive sign   |
        /// | `1` | Negative sign   |
        ///
        /// Default: `0`
        4 => sign_y_g,

        /// Sign inversion for the yaw axis (Z).
        ///
        /// # Sign Configuration
        ///
        /// | Bit | Description     |
        /// |-----|-----------------|
        /// | `0` | Positive sign   |
        /// | `1` | Negative sign   |
        ///
        /// Default: `0`
        5 => sign_z_g,
    }
);

register8!(
    /// INT1 pad signal routing register (r/w).
    ///
    /// This register enables specific internal signals to be routed to the INT1 pin.
    /// Each bit enables or disables one interrupt condition output.
    ///
    /// # Layout
    ///
    /// | Bit | Description                                |
    /// |-----|--------------------------------------------|
    /// | 0   | Accelerometer data ready interrupt         |
    /// | 1   | Gyroscope data ready interrupt             |
    /// | 2   | Boot status                                |
    /// | 3   | FIFO threshold interrupt                   |
    /// | 4   | FIFO overrun interrupt                     |
    /// | 5   | FIFO full interrupt                        |
    /// | 6   | Significant motion detection interrupt     |
    /// | 7   | Step detector interrupt                    |
    ///
    pub Int1Ctrl = 0x0D; {
        /// Accelerometer data ready routed to INT1.
        ///
        /// # Routing Table
        ///
        /// | Bit | INT1 Signal |
        /// |-----|-------------|
        /// | `0` | Disabled    |
        /// | `1` | Enabled     |
        ///
        /// Default: `0`
        0 => int1_drdy_xl,

        /// Gyroscope data ready routed to INT1.
        ///
        /// Default: `0`
        1 => int1_drdy_g,

        /// Boot status indication routed to INT1.
        ///
        /// Default: `0`
        2 => int1_boot,

        /// FIFO threshold interrupt routed to INT1.
        ///
        /// Default: `0`
        3 => int1_fth,

        /// FIFO overrun interrupt routed to INT1.
        ///
        /// Default: `0`
        4 => int1_fifo_ovr,

        /// FIFO full interrupt routed to INT1.
        ///
        /// Default: `0`
        5 => int1_full_flag,

        /// Significant motion detection interrupt routed to INT1.
        ///
        /// Default: `0`
        6 => int1_sign_mot,

        /// Step detector interrupt routed to INT1.
        ///
        /// Default: `0`
        7 => int1_step_detector,
    }
);

register8!(
    /// INT2 pad signal routing register (r/w).
    ///
    /// This register enables internal signals to be routed to the INT2 pin.
    ///
    /// # Layout
    ///
    /// | Bit | Description                                |
    /// |-----|--------------------------------------------|
    /// | 0   | Accelerometer data ready interrupt         |
    /// | 1   | Gyroscope data ready interrupt             |
    /// | 2   | Temperature sensor data ready interrupt    |
    /// | 3   | FIFO threshold interrupt                   |
    /// | 4   | FIFO overrun interrupt                     |
    /// | 5   | FIFO full interrupt                        |
    /// | 6   | Step counter overflow interrupt            |
    /// | 7   | Step detection delta-time interrupt        |
    ///
    pub Int2Ctrl = 0x0E; {
        /// Accelerometer data ready routed to INT2.
        ///
        /// Default: `0`
        0 => int2_drdy_xl,

        /// Gyroscope data ready routed to INT2.
        ///
        /// Default: `0`
        1 => int2_drdy_g,

        /// Temperature sensor data ready routed to INT2.
        ///
        /// Default: `0`
        2 => int2_drdy_temp,

        /// FIFO threshold interrupt routed to INT2.
        ///
        /// Default: `0`
        3 => int2_fth,

        /// FIFO overrun interrupt routed to INT2.
        ///
        /// Default: `0`
        4 => int2_fifo_ovr,

        /// FIFO full interrupt routed to INT2.
        ///
        /// Default: `0`
        5 => int2_full_flag,

        /// Step counter overflow interrupt routed to INT2.
        ///
        /// Default: `0`
        6 => int2_step_count_ov,

        /// Step detection delta-time interrupt routed to INT2.
        ///
        /// Default: `0`
        7 => int2_step_delta,
    }
);

register8!(
    /// Device identification register (r).
    ///
    /// This register returns the device ID value.
    ///
    /// # Layout
    ///
    /// | Bit Range | Description  |
    /// |-----------|--------------|
    /// | 0..=7     | Fixed value  |
    ///
    /// Default: `0x69`
    pub WhoAmI = 0x0F; {
        /// Device identification value.
        ///
        /// Value is fixed to `0x69`.
        0..=7 => id,
    }
);

register8!(
    /// Accelerometer control register 1 (r/w).
    ///
    /// This register configures the accelerometer’s output data rate, full scale, and analog filter bandwidth.
    ///
    /// # Layout
    ///
    /// | Bit Range | Description                             |
    /// |-----------|-----------------------------------------|
    /// | 7..=4     | Output data rate and power mode         |
    /// | 3..=2     | Full-scale selection                    |
    /// | 1..=0     | Anti-aliasing filter bandwidth setting  |
    ///
    pub Ctrl1Xl = 0x10; {
        /// Accelerometer output data rate and power mode selection.
        ///
        /// # ODR Setting Table
        ///
        /// | Bit Pattern | ODR (XL_HM_MODE=1) | ODR (XL_HM_MODE=0) |
        /// |-------------|--------------------|--------------------|
        /// | `0000`      | Power-down         | Power-down         |
        /// | `0001`      | 12.5 Hz (LP)       | 12.5 Hz (HP)       |
        /// | `0010`      | 26 Hz (LP)         | 26 Hz (HP)         |
        /// | `0011`      | 52 Hz (LP)         | 52 Hz (HP)         |
        /// | `0100`      | 104 Hz (Normal)    | 104 Hz (HP)        |
        /// | `0101`      | 208 Hz (Normal)    | 208 Hz (HP)        |
        /// | `0110`      | 416 Hz (HP)        | 416 Hz (HP)        |
        /// | `0111`      | 833 Hz (HP)        | 833 Hz (HP)        |
        /// | `1000`      | 1.66 kHz (HP)      | 1.66 kHz (HP)      |
        /// | `1001`      | 3.33 kHz (HP)      | 3.33 kHz (HP)      |
        /// | `1010`      | 6.66 kHz (HP)      | 6.66 kHz (HP)      |
        ///
        4..=7 => odr_xl,

        /// Accelerometer full-scale range selection.
        ///
        /// # Full Scale Table
        ///
        /// | Bit Pattern | Full Scale |
        /// |-------------|------------|
        /// | `00`        | ±2 g       |
        /// | `01`        | ±16 g      |
        /// | `10`        | ±4 g       |
        /// | `11`        | ±8 g       |
        ///
        2..=3 => fs_xl,

        /// Anti-aliasing filter bandwidth configuration.
        ///
        /// # Bandwidth Table
        ///
        /// | Bit Pattern | Bandwidth (XL_HM_MODE=0) |
        /// |-------------|--------------------------|
        /// | `00`        | 400 Hz                   |
        /// | `01`        | 200 Hz                   |
        /// | `10`        | 100 Hz                   |
        /// | `11`        | 50 Hz                    |
        ///
        0..=1 => bw_xl,
    }
);

register8!(
    /// Gyroscope control register 2 (r/w).
    ///
    /// This register configures the gyroscope’s output data rate and full-scale selection.
    ///
    /// # Layout
    ///
    /// | Bit Range | Description                          |
    /// |-----------|--------------------------------------|
    /// | 7..=4     | Output data rate                     |
    /// | 3..=2     | Full-scale selection                 |
    /// | 1         | 125 dps full-scale enable            |
    /// | 0         | Reserved, must be kept at `0`        |
    ///
    pub Ctrl2G = 0x11; {
        /// Gyroscope output data rate selection.
        ///
        /// # ODR Setting Table
        ///
        /// | Bit Pattern | ODR (G_HM_MODE=1) | ODR (G_HM_MODE=0) |
        /// |-------------|------------------|------------------|
        /// | `0000`      | Power-down       | Power-down       |
        /// | `0001`      | 12.5 Hz (LP)     | 12.5 Hz (HP)     |
        /// | `0010`      | 26 Hz (LP)       | 26 Hz (HP)       |
        /// | `0011`      | 52 Hz (LP)       | 52 Hz (HP)       |
        /// | `0100`      | 104 Hz (Normal)  | 104 Hz (HP)      |
        /// | `0101`      | 208 Hz (Normal)  | 208 Hz (HP)      |
        /// | `0110`      | 416 Hz (HP)      | 416 Hz (HP)      |
        /// | `0111`      | 833 Hz (HP)      | 833 Hz (HP)      |
        /// | `1000`      | 1.66 kHz (HP)    | 1.66 kHz (HP)    |
        ///
        4..=7 => odr_g,

        /// Gyroscope full-scale range selection.
        ///
        /// # Full Scale Table
        ///
        /// | Bit Pattern | Full Scale |
        /// |-------------|------------|
        /// | `00`        | 250 dps    |
        /// | `01`        | 500 dps    |
        /// | `10`        | 1000 dps   |
        /// | `11`        | 2000 dps   |
        ///
        2..=3 => fs_g,

        /// Gyroscope 125 dps full-scale enable.
        ///
        /// # Enable Table
        ///
        /// | Bit | Description      |
        /// |-----|------------------|
        /// | `0` | Disabled         |
        /// | `1` | Enabled (125 dps)|
        ///
        1 => fs_125,

        /// A virtual field that is a combination of both [`Ctrl2G::fs_125`] and [`Ctrl2G::fs_g`], inline.
        1..=3 => fs_125_g,
    }
);

register8!(
    /// Control register 3 (r/w).
    ///
    /// Provides control over reboot, data update, interrupt mode, interface settings, and software reset.
    ///
    /// # Layout
    ///
    /// | Bit | Description                          |
    /// |-----|--------------------------------------|
    /// | 7   | Software reset                        |
    /// | 6   | Big/Little Endian data selection      |
    /// | 5   | Address auto-increment                |
    /// | 4   | SPI 3-wire mode selection             |
    /// | 3   | INT pin push-pull/open-drain mode     |
    /// | 2   | Interrupt active level                |
    /// | 1   | Block data update                     |
    /// | 0   | Reboot memory content                 |
    ///
    pub Ctrl3C = 0x12; {
        /// Software reset command.
        ///
        /// The device resets when this bit is set. It is automatically cleared after boot.
        ///
        /// # Reset Table
        ///
        /// | Bit | Description  |
        /// |-----|--------------|
        /// | `0` | Normal mode  |
        /// | `1` | Reset device |
        ///
        7 => sw_reset,

        /// Big/Little Endian selection for data output.
        ///
        /// # Endianness Table
        ///
        /// | Bit | Format                |
        /// |-----|------------------------|
        /// | `0` | LSB at lower address   |
        /// | `1` | MSB at lower address   |
        ///
        6 => ble,

        /// Register address auto-increment on multi-byte read/write.
        ///
        /// # Auto-Increment Table
        ///
        /// | Bit | Mode     |
        /// |------|----------|
        /// | `0` | Disabled |
        /// | `1` | Enabled  |
        ///
        5 => if_inc,

        /// SPI serial interface mode selection.
        ///
        /// # SPI Mode Table
        ///
        /// | Bit | SPI Mode     |
        /// |-----|--------------|
        /// | `0` | 4-wire       |
        /// | `1` | 3-wire       |
        ///
        4 => sim,

        /// INT1/INT2 pad output configuration.
        ///
        /// # Output Mode Table
        ///
        /// | Bit | Output Type  |
        /// |-----|--------------|
        /// | `0` | Push-pull    |
        /// | `1` | Open-drain   |
        ///
        3 => pp_od,

        /// Interrupt signal active level.
        ///
        /// # Polarity Table
        ///
        /// | Bit | Active Level |
        /// |-----|--------------|
        /// | `0` | High         |
        /// | `1` | Low          |
        ///
        2 => h_lactive,

        /// Block data update mode.
        ///
        /// # Update Mode Table
        ///
        /// | Bit | Mode              |
        /// |-----|-------------------|
        /// | `0` | Continuous update |
        /// | `1` | Block update      |
        ///
        1 => bdu,

        /// Reboot memory content on next oscillator startup.
        ///
        /// # Reboot Table
        ///
        /// | Bit | Mode         |
        /// |-----|--------------|
        /// | `0` | Normal       |
        /// | `1` | Reboot       |
        ///
        0 => boot,
    }
);

register8!(
    /// Control register 4 (r/w).
    ///
    /// # Layout
    ///
    /// | Bit Range | Description                                      |
    /// |-----------|--------------------------------------------------|
    /// | 0         | Accelerometer bandwidth configuration source     |
    /// | 1         | Gyroscope sleep mode enable                      |
    /// | 2         | All interrupts routed to INT1                    |
    /// | 3         | Enable temperature data in FIFO                  |
    /// | 4         | Data-ready signal masking                        |
    /// | 5         | Disable I²C interface                            |
    /// | 6         | Enable FIFO threshold stop                       |
    /// | 7         | Reserved – must be set to `0`                    |
    ///
    pub Ctrl4C = 0x13; {
        /// Accelerometer bandwidth configuration source.
        ///
        /// # Selection
        ///
        /// | Bit | Source Description                                               |
        /// |-----|------------------------------------------------------------------|
        /// | `0` | Bandwidth depends on ODR (see Table 48)                         |
        /// | `1` | Bandwidth set via `BW_XL[1:0]` in `CTRL1_XL`                    |
        ///
        0 => xl_bw_scal_odr,

        /// Gyroscope sleep mode enable.
        ///
        /// # Sleep Mode
        ///
        /// | Bit | Mode     |
        /// |-----|----------|
        /// | `0` | Disabled |
        /// | `1` | Enabled  |
        ///
        1 => sleep_g,

        /// Route all interrupts to `INT1` pad.
        ///
        /// # Routing Option
        ///
        /// | Bit | Description                                |
        /// |-----|--------------------------------------------|
        /// | `0` | Signals split between `INT1` and `INT2`    |
        /// | `1` | All routed in logic OR to `INT1` only      |
        ///
        2 => int2_on_int1,

        /// Enable temperature data as the 4th FIFO data set.
        ///
        /// Effective only if `TIMER_PEDO_FIFO_EN` in `FIFO_CTRL2` is `0`.
        ///
        /// # Temperature Data Inclusion
        ///
        /// | Bit | Description              |
        /// |-----|--------------------------|
        /// | `0` | Disabled in FIFO         |
        /// | `1` | Enabled in FIFO          |
        ///
        3 => fifo_temp_en,

        /// Data-ready signal mask during mode transitions.
        ///
        /// When enabled, data-ready signals are masked while filters settle.
        ///
        /// # Masking
        ///
        /// | Bit | Mask Description         |
        /// |-----|--------------------------|
        /// | `0` | Disabled                 |
        /// | `1` | Enabled                  |
        ///
        4 => drdy_mask,

        /// Disable I²C interface, use SPI only.
        ///
        /// # I²C Bus Status
        ///
        /// | Bit | Interface Mode     |
        /// |-----|--------------------|
        /// | `0` | I²C + SPI enabled  |
        /// | `1` | Only SPI enabled   |
        ///
        5 => i2c_disable,

        /// Enable FIFO threshold level stop.
        ///
        /// # FIFO Behavior
        ///
        /// | Bit | Threshold Mode         |
        /// |-----|------------------------|
        /// | `0` | Not limited            |
        /// | `1` | Stops at threshold     |
        ///
        6 => stop_on_fth,
    }
);

register8!(
    /// Control register 5 (r/w).
    ///
    /// # Layout
    ///
    /// | Bit Range | Description                                     |
    /// |-----------|-------------------------------------------------|
    /// | 0..=2     | Output register circular burst (rounding mode)  |
    /// | 3..=4     | Gyroscope self-test selection                   |
    /// | 5..=6     | Accelerometer self-test selection               |
    /// | 7         | Reserved – must be set to `0`                   |
    ///
    pub Ctrl5C = 0x14; {
        /// Output register circular burst (rounding) mode.
        ///
        /// # Rounding Mode Table
        ///
        /// | Bit Pattern | Output Content                                                  |
        /// |-------------|------------------------------------------------------------------|
        /// | `000`       | No rounding                                                     |
        /// | `001`       | Accelerometer only                                              |
        /// | `010`       | Gyroscope only                                                  |
        /// | `011`       | Gyroscope + Accelerometer                                       |
        /// | `100`       | SENSORHUB1–SENSORHUB6                                           |
        /// | `101`       | Accelerometer + SENSORHUB1–SENSORHUB6                           |
        /// | `110`       | Gyro + Accel + SENSORHUB1–12                                    |
        /// | `111`       | Gyro + Accel + SENSORHUB1–6                                     |
        ///
        0..=2 => rounding,

        /// Gyroscope self-test mode.
        ///
        /// # Gyro Self-Test Table
        ///
        /// | Bits | Mode                 |
        /// |------|----------------------|
        /// | `00` | Normal               |
        /// | `01` | Positive sign test   |
        /// | `10` | Not valid            |
        /// | `11` | Negative sign test   |
        ///
        3..=4 => st_g,

        /// Accelerometer self-test mode.
        ///
        /// # Accel Self-Test Table
        ///
        /// | Bits | Mode                 |
        /// |------|----------------------|
        /// | `00` | Normal               |
        /// | `01` | Positive sign test   |
        /// | `10` | Negative sign test   |
        /// | `11` | Not valid            |
        ///
        5..=6 => st_xl,
    }
);

register8!(
    /// Linear acceleration sensor control register 8 (r/w).
    ///
    /// This register configures the accelerometer’s low-pass and high-pass filter modes and settings.
    ///
    /// # Layout
    ///
    /// | Bit Range | Description                                     |
    /// |-----------|-------------------------------------------------|
    /// | 0         | Low-pass filter activation for 6D functionality |
    /// | 1         | Slope/high-pass filter enable                   |
    /// | 2..=3     | Slope/high-pass cutoff frequency configuration  |
    /// | 4         | LPF2 filter enable                              |
    ///
    pub Ctrl8Xl = 0x17; {
        /// Low-pass filter activation for 6D functionality.
        ///
        /// # Table
        ///
        /// | Bit | Description                       |
        /// |-----|-----------------------------------|
        /// | `0` | Filter bypassed in 6D detection   |
        /// | `1` | LPF2 enabled for 6D detection     |
        ///
        0 => low_pass_on_6d,

        /// Slope/high-pass filter enable.
        ///
        /// # Table
        ///
        /// | Bit | Description                           |
        /// |-----|---------------------------------------|
        /// | `0` | Filter disabled                        |
        /// | `1` | Slope or high-pass filter enabled      |
        ///
        1 => hp_slope_xl_en,

        /// Slope/high-pass filter cutoff frequency configuration.
        ///
        /// Selects the cutoff frequency for the slope or high-pass filter, and also affects the LPF2 cutoff.
        ///
        /// # Filter Cutoff Table
        ///
        /// | Bit Pattern | Applied Filter | HPF Cutoff    | LPF2 Cutoff    |
        /// |-------------|----------------|---------------|----------------|
        /// | `00`        | Slope          | ODR_XL/4      | ODR_XL/50      |
        /// | `01`        | High-pass      | ODR_XL/100    | ODR_XL/100     |
        /// | `10`        | High-pass      | ODR_XL/9      | ODR_XL/9       |
        /// | `11`        | High-pass      | ODR_XL/400    | ODR_XL/400     |
        ///
        2..=3 => hpcf_xl,

        /// LPF2 filter enable for accelerometer.
        ///
        /// # Table
        ///
        /// | Bit | Description          |
        /// |-----|----------------------|
        /// | `0` | LPF2 filter disabled |
        /// | `1` | LPF2 filter enabled  |
        ///
        4 => lpf2_xl_en,
    }
);

register8!(
    /// Linear acceleration sensor control register 9 (r/w).
    ///
    /// This register enables or disables individual axes of the accelerometer, and controls the soft-iron correction feature.
    ///
    /// # Layout
    ///
    /// | Bit Range | Description                        |
    /// |-----------|------------------------------------|
    /// | 2         | Accelerometer Z-axis enable        |
    /// | 3         | Accelerometer Y-axis enable        |
    /// | 4         | Accelerometer X-axis enable        |
    /// | 5         | Soft-iron correction enable        |
    ///
    pub Ctrl9Xl = 0x18; {
        /// Z-axis accelerometer output enable.
        ///
        /// # Table
        ///
        /// | Bit | Description            |
        /// |-----|------------------------|
        /// | `0` | Z-axis disabled        |
        /// | `1` | Z-axis enabled (default) |
        ///
        2 => zen_xl,

        /// Y-axis accelerometer output enable.
        ///
        /// # Table
        ///
        /// | Bit | Description            |
        /// |-----|------------------------|
        /// | `0` | Y-axis disabled        |
        /// | `1` | Y-axis enabled (default) |
        ///
        3 => yen_xl,

        /// X-axis accelerometer output enable.
        ///
        /// # Table
        ///
        /// | Bit | Description            |
        /// |-----|------------------------|
        /// | `0` | X-axis disabled        |
        /// | `1` | X-axis enabled (default) |
        ///
        4 => xen_xl,

        /// Soft-iron correction enable.
        ///
        /// This field is effective only when `IRON_EN` in `MASTER_CONFIG` is set to `1`.
        ///
        /// # Table
        ///
        /// | Bit | Description                 |
        /// |-----|-----------------------------|
        /// | `0` | Soft-iron correction off    |
        /// | `1` | Soft-iron correction active |
        ///
        5 => soft_en,
    }
);

register8!(
    /// Control register 10 (r/w).
    ///
    /// This register controls the gyroscope axis output and embedded function activation.
    ///
    /// # Layout
    ///
    /// | Bit Range | Description                           |
    /// |-----------|---------------------------------------|
    /// | 2         | Gyroscope Z-axis enable               |
    /// | 3         | Gyroscope Y-axis enable               |
    /// | 4         | Gyroscope X-axis enable               |
    /// | 5         | Embedded functions enable             |
    /// | 6         | Pedometer step counter reset          |
    /// | 7         | Significant motion detection enable   |
    ///
    pub Ctrl10C = 0x19; {
        /// Z-axis gyroscope output enable.
        ///
        /// # Table
        ///
        /// | Bit | Description            |
        /// |-----|------------------------|
        /// | `0` | Z-axis disabled        |
        /// | `1` | Z-axis enabled (default) |
        ///
        2 => zen_g,

        /// Y-axis gyroscope output enable.
        ///
        /// # Table
        ///
        /// | Bit | Description            |
        /// |-----|------------------------|
        /// | `0` | Y-axis disabled        |
        /// | `1` | Y-axis enabled (default) |
        ///
        3 => yen_g,

        /// X-axis gyroscope output enable.
        ///
        /// # Table
        ///
        /// | Bit | Description            |
        /// |-----|------------------------|
        /// | `0` | X-axis disabled        |
        /// | `1` | X-axis enabled (default) |
        ///
        4 => xen_g,

        /// Embedded function and filter enable.
        ///
        /// Enables the pedometer, tilt, significant motion, sensor hub, ironing, and accelerometer filter logic.
        ///
        /// # Table
        ///
        /// | Bit | Description                  |
        /// |-----|------------------------------|
        /// | `0` | Functions and filters off    |
        /// | `1` | Functions and filters active |
        ///
        5 => func_en,

        /// Reset pedometer step counter.
        ///
        /// # Table
        ///
        /// | Bit | Description       |
        /// |-----|-------------------|
        /// | `0` | No reset          |
        /// | `1` | Reset pedometer   |
        ///
        6 => pedo_rst_step,

        /// Enable significant motion detection.
        ///
        /// # Table
        ///
        /// | Bit | Description                  |
        /// |-----|------------------------------|
        /// | `0` | Significant motion off       |
        /// | `1` | Significant motion enabled   |
        ///
        7 => sign_motion_en,
    }
);

register8!(
    /// Master configuration register (r/w).
    ///
    /// Configures the sensor hub master interface and auxiliary functions.
    ///
    /// # Layout
    ///
    /// | Bit Range | Description                                 |
    /// |-----------|---------------------------------------------|
    /// | 0         | DRDY on INT1 enable                         |
    /// | 1         | FIFO data-valid signal selection            |
    /// | 2         | Sensor hub trigger signal source            |
    /// | 3         | Auxiliary I²C internal pull-up enable       |
    /// | 4         | I²C pass-through mode                       |
    /// | 5         | Hard-iron correction enable                 |
    /// | 6         | Sensor hub master enable                    |
    /// | 7         | Reserved, must be `0`                       |
    ///
    pub MasterConfig = 0x1A; {
        /// Enables Master DRDY signal on INT1 pin.
        ///
        /// # Values
        ///
        /// | Bit | Description                          |
        /// |-----|--------------------------------------|
        /// | `0` | DRDY not routed to INT1 (default)    |
        /// | `1` | DRDY routed to INT1                  |
        ///
        0 => drdy_on_int1,

        /// Selects the data-valid signal source for FIFO writes.
        ///
        /// # Values
        ///
        /// | Bit | FIFO Trigger Source                           |
        /// |-----|-----------------------------------------------|
        /// | `0` | XL/Gyro data-ready or step detection (default)|
        /// | `1` | Sensor hub data-ready                         |
        ///
        1 => data_valid_sel_fifo,

        /// Selects the sensor hub start trigger source.
        ///
        /// # Values
        ///
        /// | Bit | Trigger Source                        |
        /// |-----|----------------------------------------|
        /// | `0` | XL/Gyro data-ready (default)          |
        /// | `1` | External signal from INT2             |
        ///
        2 => start_config,

        /// Enables internal pull-up resistors on auxiliary I²C lines.
        ///
        /// # Values
        ///
        /// | Bit | Pull-Up Status              |
        /// |-----|-----------------------------|
        /// | `0` | Disabled (default)          |
        /// | `1` | Enabled                     |
        ///
        3 => pull_up_en,

        /// Enables I²C pass-through mode.
        ///
        /// # Values
        ///
        /// | Bit | Pass-Through Mode           |
        /// |-----|-----------------------------|
        /// | `0` | Disabled (default)          |
        /// | `1` | Enabled                     |
        ///
        4 => pass_through_mode,

        /// Enables hard-iron correction algorithm.
        ///
        /// # Values
        ///
        /// | Bit | Hard-Iron Correction        |
        /// |-----|-----------------------------|
        /// | `0` | Disabled (default)          |
        /// | `1` | Enabled                     |
        ///
        5 => iron_en,

        /// Enables sensor hub I²C master interface.
        ///
        /// # Values
        ///
        /// | Bit | Sensor Hub Master           |
        /// |-----|-----------------------------|
        /// | `0` | Disabled (default)          |
        /// | `1` | Enabled                     |
        ///
        6 => master_on,
    }
);

register8!(
    /// Wake-up interrupt source register (r).
    ///
    /// Reports the wake-up and free-fall detection events.
    ///
    /// # Layout
    ///
    /// | Bit Range | Description                     |
    /// |-----------|---------------------------------|
    /// | 0         | Free-fall event detected        |
    /// | 1         | Sleep state detected            |
    /// | 2         | Wake-up event detected          |
    /// | 3         | X-axis wake-up detected         |
    /// | 4         | Y-axis wake-up detected         |
    /// | 5         | Z-axis wake-up detected         |
    /// | 6..=7     | Reserved, must be `0`           |
    ///
    pub WakeUpSrc = 0x1B; {
        /// Free-fall event status.
        ///
        /// # Values
        ///
        /// | Bit | Event Status         |
        /// |-----|----------------------|
        /// | `0` | No free-fall (default) |
        /// | `1` | Free-fall detected   |
        ///
        0 => ff_ia,

        /// Sleep event status.
        ///
        /// # Values
        ///
        /// | Bit | Sleep Status         |
        /// |-----|----------------------|
        /// | `0` | No event (default)   |
        /// | `1` | Sleep event detected |
        ///
        1 => sleep_state_ia,

        /// Wake-up event status.
        ///
        /// # Values
        ///
        /// | Bit | Wake-Up Event        |
        /// |-----|----------------------|
        /// | `0` | Not detected (default)|
        /// | `1` | Detected             |
        ///
        2 => wu_ia,

        /// X-axis wake-up event.
        ///
        /// # Values
        ///
        /// | Bit | Status               |
        /// |-----|----------------------|
        /// | `0` | Not detected (default)|
        /// | `1` | Detected             |
        ///
        3 => x_wu,

        /// Y-axis wake-up event.
        ///
        /// # Values
        ///
        /// | Bit | Status               |
        /// |-----|----------------------|
        /// | `0` | Not detected (default)|
        /// | `1` | Detected             |
        ///
        4 => y_wu,

        /// Z-axis wake-up event.
        ///
        /// # Values
        ///
        /// | Bit | Status               |
        /// |-----|----------------------|
        /// | `0` | Not detected (default)|
        /// | `1` | Detected             |
        ///
        5 => z_wu,
    }
);

register16!(
    /// Temperature data output register (r).
    ///
    /// The temperature sensor output data is expressed as a 16-bit word in two’s complement.
    ///
    /// The `OUT_TEMP_L` register represents the least significant byte, while `OUT_TEMP_H` represents the most significant byte.
    pub static OutTemp = 0x20; {
        /// Temperature sensor output data (LSbyte).
        ///
        /// This field represents the lower 8 bits of the temperature data.
        0..=7 => temp_lsb,

        /// Temperature sensor output data (MSbyte).
        ///
        /// This field represents the upper 8 bits of the temperature data.
        8..=15 => temp_msb,
    }
);

register16!(
    /// Angular rate sensor pitch axis (X) output register (r).
    ///
    /// The value is expressed as a 16-bit word in two’s complement.
    ///
    /// The `OUTX_L_G` register represents the least significant byte, while `OUTX_H_G` represents the most significant byte.
    pub static OutXG = 0x22; {
        /// Pitch axis (X) angular rate value (LSbyte).
        0..=7 => x_lsb,

        /// Pitch axis (X) angular rate value (MSbyte).
        8..=15 => x_msb,
    }
);

register16!(
    /// Angular rate sensor roll axis (Y) output register (r).
    ///
    /// The value is expressed as a 16-bit word in two’s complement.
    /// The `OUTY_L_G` register represents the least significant byte, while `OUTY_H_G` represents the most significant byte.
    pub static OutYG = 0x24; {
        /// Roll axis (Y) angular rate value (LSbyte).
        0..=7 => y_lsb,

        /// Roll axis (Y) angular rate value (MSbyte).
        8..=15 => y_msb,
    }
);

register16!(
    /// Angular rate sensor yaw axis (Z) output register (r).
    ///
    /// The value is expressed as a 16-bit word in two’s complement.
    /// The `OUTZ_L_G` register represents the least significant byte, while `OUTZ_H_G` represents the most significant byte.
    pub static OutZG = 0x26; {
        /// Yaw axis (Z) angular rate value (LSbyte).
        0..=7 => z_lsb,

        /// Yaw axis (Z) angular rate value (MSbyte).
        8..=15 => z_msb,
    }
);

register16!(
    /// Linear acceleration sensor X-axis output register (r).
    ///
    /// The value is expressed as a 16-bit word in two’s complement.
    ///
    /// The `OUTX_L_XL` register represents the least significant byte, while `OUTX_H_XL` represents the most significant byte.
    pub static OutXL = 0x28; {
        /// X-axis linear acceleration value (LSbyte).
        0..=7 => xl_lsb,

        /// X-axis linear acceleration value (MSbyte).
        8..=15 => xl_msb,
    }
);

register16!(
    /// Linear acceleration sensor Y-axis output register (r).
    ///
    /// The value is expressed as a 16-bit word in two’s complement.
    ///
    /// The `OUTY_L_XL` register represents the least significant byte, while `OUTY_H_XL` represents the most significant byte.
    pub static OutYL = 0x2A; {
        /// Y-axis linear acceleration value (LSbyte).
        0..=7 => yl_lsb,

        /// Y-axis linear acceleration value (MSbyte).
        8..=15 => yl_msb,
    }
);

register16!(
    /// Linear acceleration sensor Z-axis output register (r).
    ///
    /// The value is expressed as a 16-bit word in two’s complement.
    ///
    /// The `OUTZ_L_XL` register represents the least significant byte, while `OUTZ_H_XL` represents the most significant byte.
    pub static OutZL = 0x2C; {
        /// Z-axis linear acceleration value (LSbyte).
        0..=7 => zl_lsb,

        /// Z-axis linear acceleration value (MSbyte).
        8..=15 => zl_msb,
    }
);
