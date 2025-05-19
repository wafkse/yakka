//! The over-USB logging facade.

use embassy_usb_logger::{LoggerState, UsbLogger};

/// A log facade for over-USB logging.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Usb<const S: usize = { Usb::<0>::DEFAULT_BUFFER_SIZE }> {}

impl<const S: usize> Usb<S> {
    /// The default buffer size for the USB logger.
    pub const DEFAULT_BUFFER_SIZE: usize = 0x400;
}
