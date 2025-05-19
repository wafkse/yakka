//! The throttle of a motor.

use core::num::NonZero;

use crate::setting::Limit;

/// The throttle output to a motor controller.
///
/// # Representation
///
/// A throttle can be set to two different modes:
///  - `neutral` mode, where no control output is expected.
///  - `active` mode, where the control output is represented **'1 / [`u8::MAX`]'** increments using a single non-zero [`u8`].
///
/// Note that the `neutral` no-throttle and the `active` `0%` throttle representations are not equivalent, as a throttle of `0%` is perfectly achievable during flight.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[repr(transparent)]
pub struct Throttle(Option<NonZero<u8>>);

impl Throttle {
    /// A neutral throttle output.
    pub const NEUTRAL: Self = Self(None);

    /// A maximal throttle output (`100%`)
    pub const MAX: Self = Self(Some(NonZero::<u8>::MAX));

    /// A minimal throttle output.
    pub const MIN: Self = Self(Some(NonZero::<u8>::MIN));
}

impl Throttle {
    /// Take a [`f32`] in the `(0, 1]` range and convert it to a [`Throttle`] output.
    ///
    /// If the specified value happens to be out-of-bounds, it will be clamped back in.
    ///
    /// If the target value is `0.0`, an zero `active` control signal will be returned.
    #[inline]
    pub const fn percent(target_percentage: f32) -> Self {
        let target_value = target_percentage.clamp(0.0_f32.next_up(), 1.0_f32);

        let target_value = target_value * u8::MAX as f32;

        match NonZero::<u8>::new(target_value as u8) {
            target_value @ Some(..) => Self(target_value),
            None => Self::MIN,
        }
    }

    /// Convert this [`Throttle`] back into its [`f32`]-based representation.
    #[inline]
    pub const fn float(&self) -> f32 {
        let &Self(target_value) = self;

        let target_value = match target_value {
            Some(target_value) => target_value,
            None => NonZero::<u8>::MIN,
        };

        let target_value = NonZero::get(target_value) as f32;

        target_value / u8::MAX as f32
    }
}

impl Throttle {
    /// Apply the limit to this throttle output.
    #[inline]
    pub const fn limit(&self, Limit(target_limit): Limit) -> Self {
        let limit_flt = target_limit.float();

        let throttle_flt = self.float();

        let limited_flt = throttle_flt.clamp(0.0_f32.next_up(), limit_flt);

        Self::percent(limited_flt)
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
    fn percent_exactly_zero_is_min_throttle() {
        let throttle = Throttle::percent(0.0);

        assert_eq!(throttle, Throttle::MIN);
    }

    #[test]
    fn percent_maximum_is_correct() {
        let throttle = Throttle::percent(1.0);
        assert!(Throttle::MAX.float() - throttle.float() < 0.01);
    }

    #[test]
    fn percent_minimum_float_is_min() {
        let throttle = Throttle::percent(0.0);
        assert_eq!(throttle, Throttle::MIN);
    }
}
