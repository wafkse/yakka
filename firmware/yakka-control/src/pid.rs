//! Proportional Integral Derivative (PID) controller.

use core::{
    num::NonZero,
    ops::{Div, Mul, Sub},
};

use yakka_number::scalar::Scalar;
use yakka_time::Millisecond;
use yakka_unit::unit::{Scale, Unit};



use crate::{
    cycle::Delta,
    system::{Control, Error, Setpoint, Variable},
};

pub mod gain;

use gain::{Gain, Integral};

/// A PID (Proportional-Integral-Derivative) controller that operates on setpoints and process
/// variables of type `T`.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "debug-impl", derive(Debug))]
pub struct Pid<T>
where
    T: Scalar,
{
    /// The threee the gain factors of the PID controller: (K_p, K_i, K_d)
    gain_factor: Gain<T>,

    /// The setpoint of the controller.
    setpoint: Setpoint<T>,

    /// The integral component of this controller.
    integral: Integral<T>,

    /// The measured `SP-PV` error of the previous control cycle.
    previous_error: Error<T>,
}

/// The context passed to a control cycle inside [`Pid`].
#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "debug-impl", derive(Debug))]
#[non_exhaustive]
pub struct PidContext<T /* = Fixpoint<u8, 8> */>
where
    T: Scalar,
{
    /// The process variable as measured from the driver.
    variable: Variable<T>,

    /// The time delta for this control cycle.
    delta_time: Delta<Millisecond<u8 /* Fixpoint<u8, 4> */>>,

    /// The cycle index of the control cycle.
    cycle_index: Option<NonZero<usize>>,
}

impl Control<f32> for Pid<f32> {
    type Context<'a>
        = PidContext<f32>
    where
        Self: 'a;

    fn cycle_with_ctx<'a>(
        &'a mut self,
        PidContext {
            variable,
            delta_time,
            ..
        }: Self::Context<'a>,
    ) -> f32 {
        let &mut Self {
            gain_factor,
            setpoint,
            ref mut previous_error,
            ref mut integral,
            ..
        } = self;

        let target_error = Error::raw(setpoint.value() - variable.value());

        let proportional = gain_factor.p().mul(target_error.value());

        let dt_ms = NonZero::new(delta_time.unit().magnitude()).unwrap_or(NonZero::<u8>::MIN);

        let dt = Scale::fractional(&Millisecond::<u8>::SCALE_TO_BASE) * dt_ms.get() as f32;

        let &mut Integral(ref mut integral_value) = integral;

        *integral_value += target_error.mul(dt);

        let integral = gain_factor.i().value() * *integral_value;

        let derivative = target_error.value().sub(previous_error.value()).div(dt);

        *previous_error = target_error;

        proportional + integral + derivative
    }
}

impl<T> Pid<T>
where
    T: Scalar,
{
    /// Determine the gain factors in use by this controller.
    #[inline]
    pub const fn gain(&self) -> &Gain<T> {
        let &Self {
            ref gain_factor, ..
        } = self;

        gain_factor
    }

    /// Determine the setpoint for this controller.
    #[inline]
    pub const fn setpoint(&self) -> &Setpoint<T> {
        let &Self { ref setpoint, .. } = self;

        setpoint
    }

    /// Determine the setpoint for this controller, mutably.
    #[inline]
    pub const fn setpoint_mut(&mut self) -> &mut Setpoint<T> {
        let &mut Self {
            ref mut setpoint, ..
        } = self;

        setpoint
    }
}
