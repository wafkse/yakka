use yakka_resource::resources;

use embassy_rp::{Peripherals, peripherals};

resources!(
    /// The top-level struct that encompasses the peripheral resources for all subsystems.
    for Peripherals as pub Resources

    /// A struct that represents the required peripherals for the onboard USB port.
    pub use Usb become {
        /// The actual USB peripheral device.
        pub device => peripherals::USB as USB,
    }

    /// A struct that represents the required peripherals to access the onboard `cyw43` chip.
    pub use Network become {
        /// The Programmable Input-Output peripheral `0`, required for PIO-based SPI communication.
        pub pio0 => peripherals::PIO0 as PIO0,

        /// The Direct Memory Access Channel `0`, required for non-busy bidirectional en masse byte transfer.
        pub dma_ch0 => peripherals::DMA_CH0 as DMA_CH0,

        /// The power input to the `cyw43` chip.
        pub power => peripherals::PIN_23 as PIN_23,

        /// The SPI *Chip Select* (otherwise *CS*) pin.
        pub cs => peripherals::PIN_25 as PIN_25,

        /// The *Data Input-Output* pin.
        pub data_io => peripherals::PIN_24 as PIN_24,

        /// The clock line pin.
        pub clock_line => peripherals::PIN_29 as PIN_29,
    }

    /// A struct that represents the PWM-capable pins required for motor control.
    pub use Motor become {
        /// The first motor pin.
        ///
        /// This will be driven by its respective Pulse-Width Modulation peripheral slice, i.e: [`Motor::s0`].
        pub m0 => peripherals::PIN_0 as PIN_0,

        /// Pulse-Width Modulation peripheral slice. Used to drive [`Motor::m0`].
        pub s0 => peripherals::PWM_SLICE0 as PWM_SLICE0,

        /// The second motor pin.
        ///
        /// This will be driven by its respective Pulse-Width Modulation peripheral slice, i.e: [`Motor::s1`].
        pub m1 => peripherals::PIN_2 as PIN_2,

        /// Pulse-Width Modulation peripheral slice. Used to drive [`Motor::m1`].
        pub s1 => peripherals::PWM_SLICE1 as PWM_SLICE1,

        /// The third motor pin.
        ///
        /// This will be driven by its respective Pulse-Width Modulation peripheral slice, i.e: [`Motor::s2`].
        pub m2 => peripherals::PIN_4 as PIN_4,

        /// Pulse-Width Modulation peripheral slice. Used to drive [`Motor::m2`].
        pub s2 => peripherals::PWM_SLICE2 as PWM_SLICE2,

        /// The fourth motor pin.
        ///
        /// This will be driven by its respective Pulse-Width Modulation peripheral slice, i.e: [`Motor::s3`].
        pub m3 => peripherals::PIN_6 as PIN_6,

        /// Pulse-Width Modulation peripheral slice. Used to drive [`Motor::m3`].
        pub s3 => peripherals::PWM_SLICE3 as PWM_SLICE3,

    }

    /// The resources required for the activity subsystem.
    pub use Activity become {
        /// The pin that houses the LED used for activity signaling.
        pub l0 => peripherals::PIN_17 as PIN_17
    }
);
