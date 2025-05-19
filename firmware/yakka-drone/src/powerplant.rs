//! The powerplant subsystem for the drone.

use embassy_rp::pwm::{Config, Pwm};

use yakka_motor::{
    actuate::{Actuator, ControlVector},
    driver::Esc,
    prelude::{Quad, Throttle},
};
use yakka_subsystem::Subsystem;

use crate::resource;

/// The powerplant of the aircraft.
///
/// This is a [`Subsystem`] that permits control of the aircraft motors.
pub struct Powerplant(resource::Motor);

impl Powerplant {
    /// The standard `50 Hz` channel frequency for servomotors.
    pub const CHANNEL_FREQUENCY: u32 = 50;

    /// The clock divider for the channel.
    pub const CHANNEL_DIVIDER: u8 = 128;

    pub const CHANNEL_PERIOD_PARTS: u32 = Self::CHANNEL_FREQUENCY * Self::CHANNEL_DIVIDER as u32;
}

impl From<resource::Motor> for Powerplant {
    #[inline]
    fn from(target_resource: resource::Motor) -> Self {
        Self(target_resource)
    }
}

impl Subsystem for Powerplant {
    type Handle = Quad<Esc<Pwm<'static>>>;

    type Context = ();

    #[inline]
    fn subsystem_with_context(self, _: Self::Context) -> impl Future<Output = Self::Handle>
    where
        Self: Sized,
    {
        async move {
            let Self(target_resource, ..) = self;

            let resource::Motor {
                m0,
                s0,
                m1,
                s1,
                m2,
                s2,
                m3,
                s3,
            } = target_resource;

            let clock_hz = embassy_rp::clocks::clk_sys_freq();
            let divider = 128u8;
            let period = (clock_hz / Self::CHANNEL_PERIOD_PARTS) as u16 - 1;

            let mut pwm_config = Config::default();
            pwm_config.top = period;
            pwm_config.divider = divider.into();

            let p0 = Pwm::new_output_a(s0, m0, pwm_config.clone());
            let p1 = Pwm::new_output_a(s1, m1, pwm_config.clone());
            let p2 = Pwm::new_output_a(s2, m2, pwm_config.clone());
            let p3 = Pwm::new_output_a(s3, m3, pwm_config.clone());

            let m0 = Esc::<_, 100>::channel(p0);
            let m1 = Esc::channel(p1);
            let m2 = Esc::channel(p2);
            let m3 = Esc::channel(p3);

            let mut target_quad = Quad::tuple((m0, m1, m2, m3));

            target_quad.control(ControlVector::bare(Throttle::MIN));

            target_quad
        }
    }
}
