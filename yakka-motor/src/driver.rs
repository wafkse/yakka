//! Motor driver interface and its respective implementation.

use crate::{run::Throttle, setting::Limit};

use embedded_hal::pwm::SetDutyCycle;

/// A trait for motor drivers.
pub trait Motor {
    /// Attempt to set the throttle output to the specified [`Throttle`] value.
    ///
    /// On success, the motor will continuously output the same throttle output until a new one is set.
    ///
    /// The actual throttle of the motor can be accessed through [`Motor::throttle`].
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
/// The other side interprets a duty cycle of `50 %` as the neutral state.
///
/// The "neutral" state is equivalent to `0 %` throttle.
///
/// # Other remarks
///
/// This driver has no control over the underlying pulse period.
pub struct Esc<P>
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

impl<P> Motor for Esc<P>
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

        let actual_throttle = (limit_throttle.float() + 1.0f32) / 2.0_f32;

        let throttle_percent = actual_throttle * 100.0f32;

        // note: we always set a valid percentage, so no error handling is needed here
        let _ = pwm_channel.set_duty_cycle_percent(throttle_percent as u8);

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
