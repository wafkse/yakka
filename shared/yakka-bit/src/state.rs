//! Bit-wise state.

use core::ops::Not;

/// A bitwise state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum State {
    /// A set state, i.e. a bit is set to `1`.
    Set,

    /// A non-set state, i.e. a bit is set to `0`.
    ///
    /// This is the default state.
    #[default]
    Cleared,
}

impl State {
    /// Convert a [`State`] into a [`bool`].
    #[inline]
    pub const fn bool(self) -> bool {
        match self {
            Self::Set => true,
            Self::Cleared => false,
        }
    }
}

impl Not for State {
    type Output = Self;

    #[inline]
    fn not(self) -> Self::Output {
        match self {
            Self::Set => Self::Cleared,
            Self::Cleared => Self::Set,
        }
    }
}
