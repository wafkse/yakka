//! The register file of the `bmi160` Inertial Measurement Device.

use crate::bmi160::register::{Read, Register, Write};

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
                async fn write<'a, I, A>(channel: &'a mut $crate::bmi160::Channel<I, A>, value: Self) -> Result<(), I::Error>
                where
                    I: I2c<A>,
                    A: $crate::bmi160::Address
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
                async fn read<'a, I, A>(channel: &'a mut $crate::bmi160::Channel<I, A>) -> Result<Self, I::Error>
                where
                    I: I2c<A>,
                    A: $crate::bmi160::Address,
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
                async fn write<'a, I, A>(channel: &'a mut $crate::bmi160::Channel<I, A>, value: Self) -> Result<(), I::Error>
                where
                    I: I2c<A>,
                    A: $crate::bmi160::Address
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
                async fn read<'a, I, A>(channel: &'a mut $crate::bmi160::Channel<I, A>) -> Result<Self, I::Error>
                where
                    I: I2c<A>,
                    A: $crate::bmi160::Address,
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

// NOTE: This does not list all registers nor list all register fields exhaustively, as it defines the bare minimum for the driver to operate.

register8!(
    /// The command register.
    ///
    /// Default value: `0b0000_0000`.
    pub Cmd = 0x7E;
    {
        /// The command bits of this register.
        0..=7 => command_bits,
    }
);

register16!(
    /// A virtual 16-bit register that encompasses the value of the X-axis of the gyroscope.
    ///
    /// This is exposed as a combination of both [`GyroX::low`] and [`GyroX::high`].
    pub static GyroX = 0x0C; {
        /// The low byte of the X-axis of the gyroscope.
        0..=7 => low,

        /// The high byte of the X-axis of the gyroscope.
        8..=15 => high,
    }
);

register16!(
    /// A virtual 16-bit register that encompasses the value of the Y-axis of the gyroscope.
    ///
    /// This is exposed as a combination of both [`GyroY::low`] and [`GyroY::high`].
    pub static GyroY = 0x0E; {
        /// The low byte of the Y-axis of the gyroscope.
        0..=7 => low,

        /// The high byte of the Y-axis of the gyroscope.
        8..=15 => high,
    }
);

register16!(
    /// A virtual 16-bit register that encompasses the value of the Z-axis of the gyroscope.
    ///
    /// This is exposed as a combination of both [`GyroZ::low`] and [`GyroZ::high`].
    pub static GyroZ = 0x10; {
        /// The low byte of the Z-axis of the gyroscope.
        0..=7 => low,

        /// The high byte of the Z-axis of the gyroscope.
        8..=15 => high,
    }
);

register16!(
    /// A virtual 16-bit register that encompasses the value of the X-axis of the accelerometer.
    ///
    /// This is exposed as a combination of both [`AccX::low`] and [`AccX::high`].
    pub static AccX = 0x12; {
        /// The low byte of the X-axis of the accelerometer.
        0..=7 => low,

        /// The high byte of the X-axis of the accelerometer.
        8..=15 => high,
    }
);

register16!(
    /// A virtual 16-bit register that encompasses the value of the Y-axis of the accelerometer.
    ///
    /// This is exposed as a combination of both [`AccY::low`] and [`AccY::high`].
    pub static AccY = 0x14; {
        /// The low byte of the Y-axis of the accelerometer.
        0..=7 => low,

        /// The high byte of the Y-axis of the accelerometer.
        8..=15 => high,
    }
);

register16!(
    /// A virtual 16-bit register that encompasses the value of the Z-axis of the accelerometer.
    ///
    /// This is exposed as a combination of both [`AccZ::low`] and [`AccZ::high`].
    pub static AccZ = 0x16; {
        /// The low byte of the Z-axis of the accelerometer.
        0..=7 => low,

        /// The high byte of the Z-axis of the accelerometer.
        8..=15 => high,
    }
);

register8!(
    /// The status register.
    ///
    /// Default value: `0b0000_0000`.
    pub static Status = 0x1B;
    {
        /// The data-ready status for the gyroscope registers.
        6 => gyroscope_data_ready,

        /// The data-ready status for the accelerometer registers.
        7 => accelerometer_data_ready,
    }
);

register8!(
    /// The first byte of the `INT_STATUS[N]` register sequence.
    ///
    /// Default value: `0b0000_0000`.
    pub static IntStatus1 = 0x1D;
    {
        /// Whether any of the sensors have data ready for readout.
        4 => data_ready_interrupt_active,
    }
);

register8!(
    /// The accelerometer configuration register.
    ///
    /// Default value: `0b0010_1000`.
    pub AccConf = 0x40;
    {
        /// The output data rate of the accelerometer.
        0..=3 => data_rate,

        /// The bandwidth mode of the accelerometer.
        4..=6 => bandwidth_mode,

        /// Whether the undersample mode is active for the accelerometer.
        7 => undersample_mode,
    }
);

register8!(
    /// The accelerometer range register.
    ///
    /// Default value: `0bXXXX_0011`, where `X` denotes an unusable or reserved bit.
    pub AccRange = 0x41;
    {
        /// The specified range of the sensor.
        0..=3 => sensor_range,
    }
);

register8!(
    /// The gyroscope configuration register.
    ///
    /// Default value: `0b0010_1000`.
    pub GyroConf = 0x42;
    {
        /// The output data rate of the gyroscope.
        0..=3 => data_rate,

        /// The bandwidth mode of the gyroscope.
        4..=5 => bandwidth_mode,
    }
);

register8!(
    /// The gyroscope range register.
    ///
    /// Default value: `0bXXXX_0011`, where `X` denotes an unusable or reserved bit.
    pub GyroRange = 0x43;
    {
        /// The specified range of the sensor.
        0..=2 => sensor_range,
    }
);

register8!(
    /// The second byte of the `INT_EN[N]` register sequence.
    ///
    /// Default value: `0b0000_0000`.
    pub IntEnable1 = 0x51;
    {
        /// Whether an interrupt should be generated when any of the sensors have data ready for readout.
        4 => data_ready_interrupt_enable,
    }
);

register8!(
    /// The interrupt out control register.
    ///
    /// Default value: `0b0000_0000`.
    pub IntOutCtrl = 0x54;
    {
        /// Whether the `INT2` pin is used for outgoing interrupts.
        7 => interrupt_two_enable,

        /// The electrical behavior of the `INT2` pin: push-pull or open-drain.
        6 => interrupt_two_behavior,

        /// The active logic level for `INT2` pin: high or low.
        5 => interrupt_two_level,

        /// Whether the `INT2` pin is edge-triggered.
        ///
        /// This is only valid if the pin is defined as an input.
        4 => interrupt_two_edge_triggered,

        /// Whether the `INT1` pin is used for outgoing interrupts.
        3 => interrupt_one_enable,

        /// The electrical behavior of the `INT1` pin: push-pull or open-drain.
        2 => interrupt_one_behavior,

        /// The active logic level for `INT1` pin: high or low.
        1 => interrupt_one_level,

        /// Whether the `INT1` pin is edge-triggered.
        ///
        /// This is only valid if the pin is defined as an input.
        0 => interrupt_one_edge_triggered,
    }
);

register8!(
    /// The second byte of the `INT_MAP[N]` register sequence.
    ///
    /// Default value: `0b0000_0000`.
    pub IntMap1 = 0x56;
    {
        /// Whether to map any data-ready interrupt to the `INT1` pin.
        7 => interrupt_one_data_ready,

        /// Whether to map any data-ready interrupt to the `INT2` pin.
        3 => interrupt_two_data_ready,
    }
);
