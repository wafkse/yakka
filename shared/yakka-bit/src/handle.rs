//! Handle-like new-types for working directly with individual bits.

use crate::{state::State, take::Bits};

/// The read-only `N`-th bit inside `I`.
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Bit<'a, I, const N: usize>(&'a I)
where
    I: Bits<N>;

impl<'a, I, const N: usize> Bit<'a, I, N>
where
    I: Bits<N>,
{
    /// Wrap the target value's bit in a [`Bit`] struct.
    #[inline]
    pub const fn wrap(target_value: &'a I) -> Self {
        Self(target_value)
    }
}

impl<'a, I, const N: usize> Bit<'a, I, N>
where
    I: Bits<N>,
{
    /// Determine the [`State`] of this [`Bit`].
    #[inline]
    pub fn state(&self) -> State {
        let &Self(target_value) = self;

        Bits::<N>::get(target_value)
    }

    /// Instantiate a new copy of the underlying register but with the target bit cleared.
    #[inline]
    pub fn cleared(&self) -> I {
        let &Self(target_value) = self;

        Bits::<N>::cleared(target_value)
    }

    /// Instantiate a new copy of the underlying register but with the target bit set.
    #[inline]
    pub fn enabled(&self) -> I {
        let &Self(target_value) = self;

        Bits::<N>::enabled(target_value)
    }
}

/// The read-write `N`-th bit inside `I`.
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct BitMut<'a, I, const N: usize>(&'a mut I)
where
    I: Bits<N>;

impl<'a, I, const N: usize> BitMut<'a, I, N>
where
    I: Bits<N>,
{
    /// Wrap the target value's bit in a [`BitMut`] struct.
    #[inline]
    pub const fn wrap(target_value: &'a mut I) -> Self {
        Self(target_value)
    }
}
