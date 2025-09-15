#![no_std]
#![no_main]

mod powerplant;

mod network;

mod activity;

mod trace;

mod usb;

mod server;

mod resource;

extern crate yakka_log;

use core::net::Ipv4Addr;

use embassy_executor::Spawner;

use embassy_net::{Config, DhcpConfig, Ipv4Cidr, StackResources, StaticConfigV4};
use embassy_rp::{peripherals, usb::Driver};
use embassy_time::Timer;

use heapless::Vec;
use log::{LevelFilter, error, info};
use server::Server;
use static_cell::StaticCell;

use yakka_motor::{
    actuate::{Actuator, ControlVector},
    prelude::Throttle,
};
use yakka_resource::Distributor;
use yakka_subsystem::Subsystem;

use crate::{
    activity::Activity,
    network::{Network, NetworkParameters},
    powerplant::Powerplant,
    resource::Resources,
    trace::{IrqUsb, Tracer},
};

const CONTROL_AP_NAME: &'static str = "Control Access Point";

const CONTROL_AP_PASSWORD: &'static str = "controlme";

const CONTROL_AP_CHANNEL: u8 = 5;

#[embassy_executor::main]
async fn main(target_spawner: Spawner) {
    let _ = target_spawner;

    let target_peripherals = embassy_rp::init(Default::default());

    let Resources {
        usb,
        network,
        motor,
        activity,
    } = Resources::distribute(target_peripherals);

    let _ = Activity::from(activity).subsystem().await;

    #[embassy_executor::task]
    async fn usb_logger(driver: Driver<'static, peripherals::USB>) -> ! {
        embassy_usb_logger::run!(0x400, LevelFilter::Info, driver);
    }

    let _ = target_spawner.spawn(usb_logger(Driver::new(usb.device, IrqUsb)));

    let mut powerplant = Powerplant::from(motor).subsystem().await;

    static NETWORK_STACK: StaticCell<(cyw43::State, StackResources<{ network::MAX_SOCKETS }>)> =
        StaticCell::new();

    let &mut (ref mut target_state, ref mut target_resources) =
        NETWORK_STACK.init((cyw43::State::new(), StackResources::new()));

    let config = Config::ipv4_static(StaticConfigV4 {
        address: Ipv4Cidr::new(Ipv4Addr::new(10, 0, 0, 1), 24),
        gateway: Some(Ipv4Addr::new(10, 0, 0, 1)),
        dns_servers: Vec::new(),
    });

    let mut network_stack = Network::bare(
        network,
        NetworkParameters {
            config,
            ..Default::default()
        },
    )
    .subsystem_with_context((target_state, target_resources))
    .await;

    network_stack
        .control_mut()
        .start_ap_wpa2(CONTROL_AP_NAME, CONTROL_AP_PASSWORD, CONTROL_AP_CHANNEL)
        .await;

    let target_stack = network_stack.stack();

    Server::<1234, 0x1000>
        .subsystem_with_context(target_stack)
        .await;

    Timer::after_secs(10).await;

    powerplant.control(ControlVector::bare(Throttle::percent(0.05)));

    Timer::after_secs(1).await;

    powerplant.control(ControlVector::bare(Throttle::percent(0.10)));

    Timer::after_secs(1).await;

    powerplant.control(ControlVector::bare(Throttle::percent(0.15)));

    Timer::after_secs(1).await;

    powerplant.control(ControlVector::bare(Throttle::percent(0.25)));

    Timer::after_secs(1).await;

    powerplant.control(ControlVector::bare(Throttle::percent(0.35)));

    Timer::after_secs(5).await;

    powerplant.control(ControlVector::bare(Throttle::MIN));

    // TODO: Flight control loop, blocked by missing parts right now

    loop {
        Timer::after_millis(2000).await;
    }
}
