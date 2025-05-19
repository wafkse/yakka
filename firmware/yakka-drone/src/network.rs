//! The network link of the drone.

use core::{
    fmt,
    num::NonZero,
    ops::{Deref, DerefMut},
};

use embassy_executor::Spawner;
use embassy_net::{Config, Runner, Stack, StackResources};
use embassy_rp::{
    bind_interrupts,
    gpio::{Level, Output},
    pac, peripherals,
    pio::{self, Pio},
};

use cyw43::{Control, NetDriver, PowerManagementMode};
use cyw43_pio::{DEFAULT_CLOCK_DIVIDER, PioSpi};

use yakka_subsystem::Subsystem;

use crate::resource;

const FIRMWARE_BYTES: &[u8] = include_bytes!("../../vendor/43439A0.bin");

const FIRMWARE_CLM_BYTES: &[u8] = include_bytes!("../../vendor/43439A0_clm.bin");

bind_interrupts!(
    struct NetworkIrq {
        PIO0_IRQ_0 => pio::InterruptHandler<peripherals::PIO0>;
    }
);

/// The maximum sockets allowed in the network stack.
pub const MAX_SOCKETS: usize = 0x10;

/// A handle that provisions access to the network stack.
pub struct NetworkStack<'a> {
    /// The underlying network stack.
    network_stack: Stack<'a>,

    /// The control handle to the wireless device driver.
    driver_control: Control<'a>,
}

impl<'a> NetworkStack<'a> {
    /// Access the network stack.
    pub const fn stack(&self) -> Stack<'a> {
        let &Self { network_stack, .. } = self;

        network_stack
    }

    /// Access the driver control interface in a mutable manner.
    #[inline]
    pub const fn control_mut(&mut self) -> &mut Control<'a> {
        let &mut Self {
            ref mut driver_control,
            ..
        } = self;

        driver_control
    }
}

impl<'a> Deref for NetworkStack<'a> {
    type Target = Stack<'a>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        let &Self {
            ref network_stack, ..
        } = self;

        network_stack
    }
}

impl<'a> DerefMut for NetworkStack<'a> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        let &mut Self {
            ref mut network_stack,
            ..
        } = self;

        network_stack
    }
}

impl<'a> fmt::Debug for NetworkStack<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NetworkStack").finish_non_exhaustive()
    }
}

/// The parameters for the [`Network`] subsystem.
#[derive(Debug, Clone, Default)]
pub struct NetworkParameters {
    /// The local network configuration.
    pub config: Config,

    /// The random seed for the network stack.
    pub random_seed: Option<NonZero<u64>>,
}

/// The network subsystem.
///
/// This is a [`Subsystem`] that allows access to wireless capabilities of the device.
pub struct Network<const S: usize = { MAX_SOCKETS }> {
    /// The required peripherals to operate the network stack.
    network_resource: resource::Network,

    /// The parameters to the network interface.
    network_parameters: NetworkParameters,
}

impl<const S: usize> Network<S> {
    /// Create a network stack from its peripheral resources and parameters.
    #[inline]
    pub const fn bare(
        network_resource: resource::Network,
        network_parameters: NetworkParameters,
    ) -> Self {
        Self {
            network_resource,
            network_parameters,
        }
    }
}

impl<const S: usize> Subsystem for Network<S> {
    type Handle = NetworkStack<'static>;

    type Context = (&'static mut cyw43::State, &'static mut StackResources<S>);

    fn subsystem_with_context(
        self,
        (driver_state, stack_resources): Self::Context,
    ) -> impl Future<Output = Self::Handle> {
        let Self {
            network_resource,
            network_parameters,
        } = self;

        let resource::Network {
            pio0,
            dma_ch0,
            power,
            cs,
            data_io,
            clock_line,
            ..
        } = network_resource;

        #[embassy_executor::task]
        async fn backend(
            target_runtime: cyw43::Runner<
                'static,
                Output<'static>,
                PioSpi<'static, peripherals::PIO0, 0, peripherals::DMA_CH0>,
            >,
        ) -> ! {
            target_runtime.run().await
        }

        #[embassy_executor::task]
        async fn network_driver(mut target_runner: Runner<'static, NetDriver<'static>>) -> ! {
            target_runner.run().await
        }

        let chip_select = Output::new(cs, Level::High);

        let Pio {
            ref mut common,
            irq0,
            sm0,
            ..
        } = Pio::new(pio0, NetworkIrq);

        let pio_spi = PioSpi::new(
            common,
            sm0,
            DEFAULT_CLOCK_DIVIDER,
            irq0,
            chip_select,
            data_io,
            clock_line,
            dma_ch0,
        );

        let power_out = Output::new(power, Level::Low);

        async move {
            let (target_device, mut driver_control, backend_runtime) =
                cyw43::new(driver_state, power_out, pio_spi, FIRMWARE_BYTES).await;

            let spawner = Spawner::for_current_executor().await;

            let backend_thread = backend(backend_runtime);

            spawner.must_spawn(backend_thread);

            driver_control.init(FIRMWARE_CLM_BYTES).await;

            driver_control
                .set_power_management(PowerManagementMode::Performance)
                .await;

            let NetworkParameters {
                config,
                random_seed,
            } = network_parameters;

            let random_seed = match random_seed {
                Some(target_seed) => NonZero::get(target_seed),
                None => {
                    let target_register = pac::ROSC.random();

                    let high = target_register.read() as u64;
                    let low = target_register.read() as u64;

                    high << 32 | low
                }
            };

            let (network_stack, network_runner) =
                embassy_net::new(target_device, config, stack_resources, random_seed);

            let network_thread = network_driver(network_runner);

            spawner.must_spawn(network_thread);

            NetworkStack {
                network_stack,
                driver_control,
            }
        }
    }
}
