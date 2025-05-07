//! The throttle of a motor.

use core::{cmp, num::NonZero};

use crate::setting::Limit;

/// The throttle output to a motor controller.
///
/// # Representation
///
/// A throttle can be set to two different modes:
///  - `neutral` mode, where no control output is expected.
///  - `active` mode, where the control output is represented **'1 / [`u8::MAX`]'** increments using a single non-zero [`u8`] integer with a bias of `floor([`u8::MAX`] / 2)`.
///
/// Note that the `neutral` no-throttle and the `active` `0%` throttle representations are not equivalent, as a throttle of `0%` is perfectly achievable during flight.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "debug-impl", derive(Debug))]
#[repr(transparent)]
pub struct Throttle(Option<NonZero<u8>>);

impl Throttle {
    /// The raw integer bias of the [`u8`].
    pub const RAW_BIAS: u8 = u8::MAX / 2;

    /// The integer bias of the underlying [`u8`], but with a type-level [`NonZero`] guarantee.
    pub const BIAS: NonZero<u8> = match NonZero::<u8>::new(Self::RAW_BIAS) {
        Some(target_value) => target_value,
        None => unreachable!(),
    };

    /// A neutral throttle output.
    pub const NEUTRAL: Self = Self(None);

    /// A maximal throttle output (`100%`)
    ///
    pub const MAX: Self = Self(Some(NonZero::<u8>::MAX));

    /// A minimal throttle output.
    ///
    /// *NOTE*: This does *NOT* correspond to a throttle of `0%`, but to a throttle immediately rightwards-adjacent to '-100%`!
    ///
    /// For a throttle of zero, see [`Throttle::ZERO`].
    pub const MIN: Self = Self(Some(NonZero::<u8>::MIN));

    /// A nil throttle output. (`0 %`)
    pub const ZERO: Self = Self(Some(Self::BIAS));
}

impl Throttle {
    /// Take a [`f32`] in the `(-1, 1]` range and convert it to a [`Throttle`] output.
    ///
    /// If the specified value happens to be out-of-bounds, it will be clamped back in.
    ///
    /// If the target value is `0.0`, an zero `active` control signal will be returned.
    #[inline]
    pub const fn percent(target_percentage: f32) -> Self {
        let target_value = target_percentage.clamp(-1.0_f32.next_up(), 1.0_f32);

        let target_value = target_value * Self::RAW_BIAS as f32;

        let target_value = target_value + Self::RAW_BIAS as f32;

        match NonZero::<u8>::new(target_value as u8) {
            target_value @ Some(..) => Self(target_value),
            // NOTE: This corresponds to a value of `-1.0_f32`.
            None => Self::MIN,
        }
    }

    /// Convert this [`Throttle`] back into its [`f32`]-based representation.
    #[inline]
    pub const fn float(&self) -> f32 {
        let &Self(target_value) = self;

        let target_value = match target_value {
            Some(target_value) => target_value,
            None => Self::BIAS,
        };

        let target_value = NonZero::get(target_value) as f32;

        let target_value = target_value - Self::RAW_BIAS as f32;

        target_value / Self::RAW_BIAS as f32
    }
}

impl Throttle {
    /// Apply the limit to this throttle output.
    #[inline]
    pub const fn limit(&self, Limit(target_limit): Limit) -> Self {
        let limit_flt = target_limit.float();

        let throttle_flt = self.float();

        let limited_flt = throttle_flt.clamp(-limit_flt, limit_flt);

        Self::percent(limited_flt)
    }
}

impl PartialOrd for Throttle {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Throttle {
    #[inline]
    fn cmp(&self, &Self(rhs): &Self) -> cmp::Ordering {
        let &Self(lhs) = self;

        let (lhs, ref rhs) = (lhs.unwrap_or(Self::BIAS), rhs.unwrap_or(Self::BIAS));

        lhs.cmp(rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn throttle_neutral_is_none() {
        assert_eq!(Throttle::NEUTRAL, Throttle(None));
    }

    #[test]
    fn throttle_max_is_nonzero_max() {
        assert_eq!(Throttle::MAX, Throttle(Some(NonZero::<u8>::MAX)));
    }

    #[test]
    fn throttle_min_is_nonzero_min() {
        assert_eq!(Throttle::MIN, Throttle(Some(NonZero::<u8>::MIN)));
    }

    #[test]
    fn throttle_zero_is_bias_value() {
        assert_eq!(Throttle::ZERO, Throttle(Some(Throttle::BIAS)));
    }

    #[test]
    fn percent_above_max_clamps_to_max() {
        let throttle = Throttle::percent(2.0);

        assert!(Throttle::MAX.float() - throttle.float() < 0.01);
    }

    #[test]
    fn percent_below_min_clamps_to_min() {
        let throttle = Throttle::percent(-2.0);
        assert_eq!(throttle, Throttle::MIN);
    }

    #[test]
    fn percent_exactly_zero_is_zero_throttle() {
        let throttle = Throttle::percent(0.0);
        assert_eq!(throttle, Throttle::ZERO);
    }

    #[test]
    fn percent_positive_halfway() {
        let expected = Throttle(Some(
            NonZero::new(Throttle::RAW_BIAS + (Throttle::RAW_BIAS / 2)).unwrap(),
        ));
        let actual = Throttle::percent(0.5);
        assert_eq!(actual, expected);
    }

    #[test]
    fn percent_negative_halfway() {
        let expected = Throttle(Some(NonZero::new(Throttle::RAW_BIAS / 2).unwrap()));
        let actual = Throttle::percent(-0.5);
        assert_eq!(actual, expected);
    }

    #[test]
    fn percent_maximum_is_correct() {
        let throttle = Throttle::percent(1.0);
        assert!(Throttle::MAX.float() - throttle.float() < 0.01);
    }

    #[test]
    fn percent_minimum_float_is_min() {
        let throttle = Throttle::percent(-1.0);
        assert_eq!(throttle, Throttle::MIN);
    }
}
