//! Panicking support for logging facades.

use core::panic::PanicInfo;

use cortex_m::interrupt;

/// The panic handler for Yakka.
///
/// # Assumptions
///
/// This assumes that something very, very bad has happened, and thus, expects no functionality except for the one implemented in this function to be available.
///
/// However, this expects all peripheral hardware to be in a sane state.
///
/// # Behavior
///
/// When a panic occurs, this function will:
/// - Attempt to log the panic message.
/// - Disable all interrupts.
/// - Spin for the rest of eternity.
#[panic_handler]
pub fn panic(panic_info: &PanicInfo) -> ! {
    log::error!("panic: {}", panic_info);

    // Do a catch-all disable first.
    interrupt::disable();

    // Spin loop for eternity.
    loop {
        unsafe {
            core::arch::asm!("yield");
        }
    }
}
