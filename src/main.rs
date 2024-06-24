#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use alloc::boxed::Box;
use embassy_sync::mutex::Mutex;
use esp_backtrace as _;
use esp_hal::{clock::ClockControl, delay::Delay, peripherals::Peripherals, prelude::*};
use esp_hal::{
    embassy::{self, executor::Executor},
    gpio::IO,
    timer::TimerGroup,
};
use esp_wifi::wifi::WifiStaDevice;

extern crate alloc;
use core::mem::MaybeUninit;

mod net;
mod ultrasonic;

#[global_allocator]
static ALLOCATOR: esp_alloc::EspHeap = esp_alloc::EspHeap::empty();

fn init_heap() {
    const HEAP_SIZE: usize = 32 * 1024;
    static mut HEAP: MaybeUninit<[u8; HEAP_SIZE]> = MaybeUninit::uninit();

    unsafe {
        ALLOCATOR.init(HEAP.as_mut_ptr() as *mut u8, HEAP_SIZE);
    }
}

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();

    let clocks = ClockControl::max(system.clock_control).freeze();
    esp_println::logger::init_logger_from_env();
    let _delay = Delay::new(&clocks);
    init_heap();

    let timer = esp_hal::timer::TimerGroup::new(peripherals.TIMG1, &clocks).timer0;
    let init = esp_wifi::initialize(
        esp_wifi::EspWifiInitFor::Wifi,
        timer,
        esp_hal::rng::Rng::new(peripherals.RNG),
        system.radio_clock_control,
        &clocks,
    )
    .unwrap();

    // Setting up wifi
    let wifi = peripherals.WIFI;
    let (device, controller) = esp_wifi::wifi::new_with_mode(&init, wifi, WifiStaDevice).unwrap();
    let dhcpconfig = embassy_net::Config::dhcpv4(Default::default());
    let stack_resources = Box::leak(Box::new(embassy_net::StackResources::<5>::new()));
    let stack = embassy_net::Stack::new(device, dhcpconfig, stack_resources, 34895);

    let stack = Box::leak(Box::new(stack));

    let executor = Box::leak(Box::new(Executor::new()));
    let timer_group = TimerGroup::new(peripherals.TIMG0, &clocks);
    embassy::init(&clocks, timer_group);

    //Setting up ultrasonic sensor
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);
    let trigger = io.pins.gpio4.into_push_pull_output();
    let echo = io.pins.gpio5.into_pull_up_input();
    let ultrasonic = Box::leak(Box::new(Mutex::new(ultrasonic::Ultrasonic::new(
        trigger, echo,
    ))));

    //Execution
    executor.run(|spawner| {
        spawner.spawn(net::connect(controller)).unwrap();
        spawner.spawn(net::run_network(stack)).unwrap();
        spawner.spawn(net::net_state(stack)).unwrap();
        spawner.spawn(ultrasonic::read_sensor(ultrasonic)).unwrap();
    });
}
