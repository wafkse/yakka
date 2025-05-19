//! The powerplant subsystem for the drone.

use embassy_executor::Spawner;
use embassy_rp::gpio::{Level, Output};
use embassy_time::{Duration, Timer};
use log::info;
use yakka_subsystem::Subsystem;

use crate::resource;

/// An activity indicator.
///
/// This is a [`Subsystem`] that continously flashes an external light to serve as an activity indicator.
#[repr(transparent)]
pub struct Activity(resource::Activity);

impl From<resource::Activity> for Activity {
    #[inline]
    fn from(target_resource: resource::Activity) -> Self {
        Self(target_resource)
    }
}

impl Subsystem for Activity {
    type Handle = ();

    type Context = ();

    #[inline]
    fn subsystem_with_context(self, _: Self::Context) -> impl Future<Output = Self::Handle>
    where
        Self: Sized,
    {
        #[embassy_executor::task]
        async fn led_update(mut led: Output<'static>) {
            loop {
                info!("on");

                led.set_high();

                Timer::after(Duration::from_millis(500)).await;

                info!("off");

                led.set_low();

                Timer::after(Duration::from_millis(500)).await;
            }
        }

        let Self(target_resource) = self;

        let resource::Activity { l0 } = target_resource;

        async move {
            let spawner = Spawner::for_current_executor().await;

            let update_thread = led_update(Output::new(l0, Level::Low));

            spawner.must_spawn(update_thread);
        }
    }
}
