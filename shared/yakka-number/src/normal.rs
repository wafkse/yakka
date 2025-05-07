//! Normal number type in the `[0, 1]` range.

/// A number type in the `[0, 1]` range represented in increments of **1 / [`u8::MAX`]**.
///
/// A round-trip is not guaranteed to result in the same exact value. Expect very tiny rounding errors due to the limited range.
///
/// # Layout
///
/// This type is fully transparent over an [`u8`].
#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "debug-impl", derive(Debug))]
#[repr(transparent)]
pub struct Normal(u8);

impl Normal {
    /// The `1` value of the [`Normal`] type.
    pub const ONE: Self = Self(u8::MAX);

    /// The `0` value of the [`Normal`] type.
    pub const ZERO: Self = Self(u8::MIN);
}

impl Normal {
    /// Take a [`f32`] in the `[0, 1]` range and convert it to a [`Normal`] output.
    ///
    /// If the specified value happens to be out-of-bounds, it will be clamped back in.
    #[inline]
    pub const fn percent(target_percent: f32) -> Self {
        let target_value = target_percent.clamp(0.0_f32, 1.0_f32);

        let target_value = (target_value * u8::MAX as f32) + 0.5;

        let target_value = target_value as u8;

        Self(target_value)
    }

    /// Convert this [`Normal`] back into its [`f32`]-based representation.
    #[inline]
    pub const fn float(&self) -> f32 {
        let &Self(target_value) = self;

        let target_value = target_value as f32;

        let target_value = target_value / u8::MAX as f32;

        target_value
    }
}

#[cfg(test)]
mod tests {
    use super::Normal;

    #[test]
    fn test_zero() {
        let zero = Normal::ZERO;

        assert_eq!(zero.float(), 0.0);
    }

    #[test]
    fn test_one() {
        let one = Normal::ONE;

        assert_eq!(one.float(), 1.0);
    }

    #[test]
    fn test_percent_clamp_lower() {
        let percent = Normal::percent(-0.1);

        assert_eq!(percent.float(), 0.0);
    }

    #[test]
    fn test_percent_clamp_upper() {
        let percent = Normal::percent(1.1);

        assert_eq!(percent.float(), 1.0);
    }

    #[test]
    fn test_percent_middle() {
        let percent = Normal::percent(0.5);

        assert!((percent.float() - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_percent_exact_value() {
        let percent = Normal::percent(0.25);

        assert!((percent.float() - 0.25).abs() < 0.01);
    }
}
