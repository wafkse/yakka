//! Motor driver interface and its respective implementation.

use crate::{setting::Limit, throttle::Throttle};

use embedded_hal::pwm::SetDutyCycle;

/// A trait for motor drivers which can be controller by a singular [`Throttle`] value.
pub trait Motor {
    /// Attempt to set the throttle output to the specified [`Throttle`] value.
    ///
    /// On success, the motor will attempt to continuously output the same throttle output until a new one is set.
    fn control(&mut self, target_throttle: Throttle) -> Option<Throttle>;

    /// Artificially limit any throttle values sent to this motor automatically.
    ///
    /// Returns the already existing limit, if any.
    fn limit(&mut self, target_limit: Limit) -> Option<Limit>;
}

/// A motor driver over single a pulse-width modulated channel.
///
/// # Assumptions
///
/// The other side interprets a duty cycle of `5 %` as the neutral ("no throttle") state.
///
/// The "neutral" state is behaviorally equivalent to `0 %` throttle, but it does not actually present itself with the same intent.
///
/// # Other remarks
///
/// This driver has no control over the underlying pulse period, nevertheless, it assumes a standard `50 Hz` servomotor period.
pub struct Esc<P, const S: usize = 100>
where
    P: SetDutyCycle,
{
    /// The target channel.
    pwm_channel: P,

    /// The bare-bones target throttle for this driver.
    target_throttle: Option<Throttle>,

    /// The artificially-imposed limit.
    imposed_limit: Option<Limit>,
}

impl<P, const S: usize> Esc<P, S>
where
    P: SetDutyCycle,
{
    /// Create a new motor driver over a PWM channel.
    #[inline]
    pub const fn channel(pwm_channel: P) -> Self {
        Self {
            pwm_channel,
            target_throttle: None,
            imposed_limit: None,
        }
    }
}

impl<P, const S: usize> Motor for Esc<P, S>
where
    P: SetDutyCycle,
{
    #[inline]
    fn control(&mut self, target_throttle: Throttle) -> Option<Throttle> {
        let &mut Self {
            ref mut pwm_channel,
            target_throttle: ref mut exist_throttle,
            ref imposed_limit,
            ..
        } = self;

        let limit_throttle = match imposed_limit.as_ref().copied() {
            Some(limit) => target_throttle.limit(limit),
            None => target_throttle,
        };

        let actual_throttle = limit_throttle.float() / 2.0_f32;

        let throttle_percent = actual_throttle * 10.0_f32 * S as f32;

        let throttle_percent = (5 * S as u16) + (throttle_percent as u16);

        let _ = pwm_channel.set_duty_cycle_fraction(throttle_percent, 100 * S as u16);

        exist_throttle.replace(target_throttle)
    }

    #[inline]
    fn limit(&mut self, target_limit: Limit) -> Option<Limit> {
        let &mut Self {
            ref mut imposed_limit,
            ..
        } = self;

        imposed_limit.replace(target_limit)
    }
}
