//! Raw register values.
//!
//! All register values are in the big-endian byte order.

use yakka_bit::{
    many::Extract,
    state::State,
    take::{Bits, HasPrimitive},
};

/// A structure representing an 8-bit register.
///
/// This is new-type wrapper over an [`u8`] value, but with extra bit-manipulation methods.
///
/// This register layout applies to most registers in the MPU6050.
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Register8(u8);

impl Register8 {
    /// Instantiate a new [`Register8`] struct from a raw [`u8`] value.
    #[inline]
    pub const fn raw(target_value: u8) -> Self {
        Self(target_value)
    }

    /// Deconstruct this [`Register8`] struct into a raw [`u16`] value.
    #[inline]
    pub const fn value(self) -> u8 {
        let Self(target_value) = self;

        target_value
    }
}

impl HasPrimitive for Register8 {
    type Primitive = u8;

    #[inline]
    fn raw(self) -> Self::Primitive {
        let Self(target_value) = self;

        target_value
    }
}

impl<const N: usize> Bits<N> for Register8
where
    u8: Bits<N>,
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
        Register8::raw(Bits::<N>::single())
    }
}

impl<const N: usize, const M: usize> Extract<N, M> for Register8
where
    <Self as HasPrimitive>::Primitive: Extract<N, M>,
{
    type Output = <<Self as HasPrimitive>::Primitive as Extract<N, M>>::Output;

    #[inline]
    fn extract(&self) -> Self::Output {
        let &Self(ref target_value) = self;

        <<Self as HasPrimitive>::Primitive as Extract<N, M>>::extract(target_value)
    }

    #[inline]
    fn fuse(&mut self, target_output: Self::Output) -> Self::Output {
        let &mut Self(ref mut target_value) = self;

        <<Self as HasPrimitive>::Primitive as Extract<N, M>>::fuse(target_value, target_output)
    }
}

/// A structure representing a 16-bit register.
///
/// This is new-type wrapper over an [`u16`] value, but with extra bit-manipulation methods.
///
/// This is useful for multi-byte registers, such as the accelerometer and gyroscope data registers.
#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Register16(u16);

impl Register16 {
    /// Reinterpret a 2-byte array as a [`Register16`].
    ///
    /// # Layout
    ///
    /// ```no_run
    /// ┌────────────┬────────────┐
    /// |    Half    │    Reg16   |
    /// ┼────────────┤────────────┤
    /// │    Low     |    0x00    |
    /// ┼────────────┤────────────┤
    /// │    High    |    0x01    |
    /// └────────────┴────────────┘
    /// ```
    ///
    /// This is otherwise known as a little-endian byte order.
    #[inline]
    pub const fn bytes([b0, b1]: [u8; 2]) -> Self {
        Self::raw(u16::from_le_bytes([b0, b1]))
    }
}

impl Register16 {
    /// Instantiate a new [`Register16`] struct from a raw [`u16`] value.
    #[inline]
    pub const fn raw(target_value: u16) -> Self {
        Self(target_value)
    }

    /// Deconstruct this [`Register16`] struct into a raw [`u16`] value.
    #[inline]
    pub const fn value(self) -> u16 {
        let Self(target_value) = self;

        target_value
    }
}

impl HasPrimitive for Register16 {
    type Primitive = u16;

    #[inline]
    fn raw(self) -> Self::Primitive {
        let Self(target_value) = self;

        target_value
    }
}

impl<const N: usize> Bits<N> for Register16
where
    u16: Bits<N>,
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
        Register16::raw(Bits::<N>::single())
    }
}

impl<const N: usize, const M: usize> Extract<N, M> for Register16
where
    <Self as HasPrimitive>::Primitive: Extract<N, M>,
{
    type Output = <<Self as HasPrimitive>::Primitive as Extract<N, M>>::Output;

    #[inline]
    fn extract(&self) -> Self::Output {
        let &Self(ref target_value) = self;

        <<Self as HasPrimitive>::Primitive as Extract<N, M>>::extract(target_value)
    }

    #[inline]
    fn fuse(&mut self, target_output: Self::Output) -> Self::Output {
        let &mut Self(ref mut target_value) = self;

        <<Self as HasPrimitive>::Primitive as Extract<N, M>>::fuse(target_value, target_output)
    }
}
