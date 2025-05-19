//! Tracing capabilities for the drone.

use core::fmt::{self, Write};

use embassy_executor::Spawner;
use embassy_rp::{bind_interrupts, peripherals, usb};

use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, once_lock::OnceLock, pipe::Pipe};

use embassy_usb::{
    Builder, Config, UsbDevice,
    class::cdc_acm::{CdcAcmClass, State},
};

use embassy_usb_logger::MAX_PACKET_SIZE;

use log::{Level, Log};
use static_cell::StaticCell;

use yakka_subsystem::Subsystem;

use crate::resource;

bind_interrupts!(pub struct IrqUsb {
    USBCTRL_IRQ => usb::InterruptHandler<peripherals::USB>;
});

/// The size of the log [`Pipe`]-based buffers.
pub const LOG_BUFFER_SIZE: usize = 0x400;

/// The maximum size of a byte transfer for the USB.
pub const USB_TRANSFER_BUFFER_SIZE: u16 = 0x40;

/// The singleton instance of the [`TracerBackend`].
static TRACER_SINGLETON: OnceLock<TracerBackend<{ LOG_BUFFER_SIZE }>> = OnceLock::new();

/// The cloned pipe sink.
///
/// All logs will be both forwarded to the USB host and to this pipe.
static CLONED_PIPE: Pipe<CriticalSectionRawMutex, 0x400> = Pipe::new();

/// A tracer for logging in the firmware.
///
/// This [`Subsystem`] can only be set up exactly once. Any further attempts will result in a no-op.
#[repr(transparent)]
pub struct Tracer(resource::Usb);

impl Tracer {
    /// Instantiate a new [`TracerBackend`] with a buffer size of `N`.
    #[inline]
    pub const fn backend<const N: usize>() -> TracerBackend<N> {
        TracerBackend(Pipe::new())
    }
}

impl From<resource::Usb> for Tracer {
    #[inline]
    fn from(target_resource: resource::Usb) -> Self {
        Self(target_resource)
    }
}

impl Subsystem for Tracer {
    type Handle = &'static TracerBackend<{ LOG_BUFFER_SIZE }>;

    type Context = ();

    fn subsystem_with_context(self, _: Self::Context) -> impl Future<Output = Self::Handle>
    where
        Self: Sized,
    {
        #[embassy_executor::task]
        async fn backend_driver(
            mut usb_device: UsbDevice<'static, usb::Driver<'static, peripherals::USB>>,
        ) -> ! {
            usb_device.run().await
        }

        #[embassy_executor::task]
        async fn backend_tracer(
            tracer_backend: &'static TracerBackend<{ LOG_BUFFER_SIZE }>,
            mut device_class: CdcAcmClass<'static, usb::Driver<'static, peripherals::USB>>,
        ) -> ! {
            let &TracerBackend(ref target_pipe) = tracer_backend;

            let mut target_buffer: [u8; LOG_BUFFER_SIZE] = [0; LOG_BUFFER_SIZE];

            device_class.wait_connection().await;

            loop {
                let byte_count = target_pipe.read(&mut target_buffer).await;

                let _ = device_class
                    .write_packet(&target_buffer[..byte_count])
                    .await;

                if byte_count as u8 == MAX_PACKET_SIZE {
                    let _ = device_class.write_packet(&[]).await;
                }

                CLONED_PIPE.write_all(&target_buffer[..byte_count]).await;
            }
        }

        async move {
            let Self(target_resource) = self;

            let resource::Usb { device } = target_resource;

            if TRACER_SINGLETON.is_set() {
                TRACER_SINGLETON.get().await
            } else {
                let target_backend = Tracer::backend::<LOG_BUFFER_SIZE>();

                let _ = TRACER_SINGLETON.init(target_backend);

                let tracer_backend = TRACER_SINGLETON.get().await;

                let config = {
                    let mut config = Config::new(0xcafe, 0xbabe);

                    config.manufacturer = Some("Yakka");
                    config.product = Some("Flight Controller");
                    config.serial_number = None;
                    config.max_power = 100;
                    config.max_packet_size_0 = 0x40;

                    config
                };

                let mut device_builder = {
                    static CONFIG_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();

                    static BOS_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();

                    static CONTROL_BUF: StaticCell<[u8; USB_TRANSFER_BUFFER_SIZE as usize]> =
                        StaticCell::new();

                    let driver = usb::Driver::new(device, IrqUsb);

                    let builder = Builder::new(
                        driver,
                        config,
                        CONFIG_DESCRIPTOR.init([0; 256]),
                        BOS_DESCRIPTOR.init([0; 256]),
                        &mut [],
                        CONTROL_BUF.init([0; USB_TRANSFER_BUFFER_SIZE as usize]),
                    );
                    builder
                };

                let device_class = {
                    static CLASS_STATE: StaticCell<State> = StaticCell::new();

                    CdcAcmClass::new(
                        &mut device_builder,
                        CLASS_STATE.init(State::new()),
                        USB_TRANSFER_BUFFER_SIZE,
                    )
                };

                let target_device = device_builder.build();

                let spawner = Spawner::for_current_executor().await;

                let backend_thread = backend_driver(target_device);

                let trace_thread = backend_tracer(tracer_backend, device_class);

                spawner.must_spawn(backend_thread);

                spawner.must_spawn(trace_thread);

                let _ = unsafe { log::set_logger_racy(tracer_backend) };

                TRACER_SINGLETON.get().await
            }
        }
    }
}

/// The backend engine for [`Tracer`].
#[repr(transparent)]
pub struct TracerBackend<const N: usize>(Pipe<CriticalSectionRawMutex, N>);

impl<const N: usize> Log for TracerBackend<N> {
    #[inline]
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &log::Record) {
        let &Self(ref target_pipe) = self;

        if self.enabled(record.metadata()) {
            let _ = writeln!(
                &mut Writer(target_pipe),
                "{} - {}",
                record.level(),
                record.args()
            );
        }
    }

    #[inline]
    fn flush(&self) {}
}

#[repr(transparent)]
struct Writer<'a, const N: usize>(&'a Pipe<CriticalSectionRawMutex, N>);

impl<'a, const N: usize> fmt::Write for Writer<'a, N> {
    #[inline]
    fn write_str(&mut self, target_str: &str) -> fmt::Result {
        let &mut Self(target_pipe) = self;

        let target_bytes = target_str.as_bytes();

        if let Ok(target_count) = target_pipe.try_write(target_bytes) {
            if target_bytes.len() >= target_count {
                let _ = target_pipe.try_write(&target_bytes[target_count..]);
            }
        }

        Ok(())
    }
}
