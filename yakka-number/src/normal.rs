//! Normal number type in the `[1, 0]` range.

/// A normal number type in the `[1, 0]` range.
///
/// This is represented in '1 / [`u8::MAX`]' increments.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct Normal(u8);

impl Normal {
    /// The `1` value of the [`Normal`] type.
    pub const ONE: Self = Self(u8::MAX);

    /// The `0` value of the [`Normal`] type.
    pub const ZERO: Self = Self(u8::MIN);
}

impl Normal {
    /// Instantiate a new [`Normal`] number from a count of increments of '1 / [`u8::MAX`]'.
    #[inline]
    pub const fn raw(target_count: u8) -> Self {
        Self(target_count)
    }
}

/// An interval between two values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Interval {
    start: usize,
    end: usize,
}

impl Interval {
    /// Determine the start value of the interval.
    #[inline]
    pub const fn start(&self) -> usize {
        let &Self { start, .. } = self;

        start
    }

    /// Determine the end of the interval.
    #[inline]
    pub const fn end(&self) -> usize {
        let &Self { end, .. } = self;

        end
    }
}
