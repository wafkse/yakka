//! Machine register access.

use embedded_hal_async::i2c::I2c;

use super::{Address, Channel};

pub mod file;

/// A trait representing a register in the `lsm6ds3` IMU.
pub trait Register {
    /// The address of the register.
    const ADDRESS: u8;
}

/// A trait for registers whose value can be directly read.
#[allow(
    async_fn_in_trait,
    reason = "no sync/send constraints in embedded code"
)]
pub trait Read: Register {
    /// Read from the device of address `A` in the [`I2c`] bus this register's value.
    async fn read<'a, I, A>(channel: &'a mut Channel<I, A>) -> Result<Self, I::Error>
    where
        I: I2c<A>,
        A: Address,
        Self: Sized;
}

/// A trait for registers whose value can be directly written to.
#[allow(
    async_fn_in_trait,
    reason = "no sync/send constraints in embedded code"
)]
pub trait Write: Register {
    /// Write to register located in the device of address `A` in the [`I2c`] bus.
    async fn write<'a, I, A>(channel: &'a mut Channel<I, A>, value: Self) -> Result<(), I::Error>
    where
        I: I2c<A>,
        A: Address;
}
