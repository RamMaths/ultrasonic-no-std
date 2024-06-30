use embassy_executor::task;
use embassy_time::{Duration, Timer};
use esp_hal::gpio::PullUp;
use esp_hal::gpio::PushPull;
use esp_hal::gpio::{GpioPin, Input, Output};
use esp_hal::prelude::_embedded_hal_digital_v2_InputPin;
use esp_hal::prelude::_embedded_hal_digital_v2_OutputPin;
use esp_hal::prelude::_embedded_hal_digital_v2_ToggleableOutputPin;
use esp_hal::systimer::SystemTimer;
use log::info;

const TRIGGER_PIN: u8 = 15;
const ECHO_PIN: u8 = 16;

pub struct Ultrasonic {
    pub trigger: GpioPin<Output<PushPull>, TRIGGER_PIN>,
    pub echo: GpioPin<Input<PullUp>, ECHO_PIN>,
}

impl Ultrasonic {
    pub fn new(
        trigger: GpioPin<Output<PushPull>, TRIGGER_PIN>,
        echo: GpioPin<Input<PullUp>, ECHO_PIN>,
    ) -> Self {
        Self { trigger, echo }
    }

    pub async fn read_sensor(&mut self) {
        //clean pulse
        info!("Cleaning pulse");
        self.trigger.set_low().unwrap();
        Timer::after_millis(5).await;

        info!("Setting up trigger");
        self.trigger.set_high().unwrap();
        Timer::after_millis(10).await;
        self.trigger.set_low().unwrap();

        info!("Sending echo");
        self.wait_echo_for_high().await;

        let echo_start = SystemTimer::now();

        self.wait_echo_for_low().await;

        info!("Echo finished");
        let echo_end = SystemTimer::now();

        let echo_dur = echo_end.wrapping_sub(echo_start);

        let distance_cm: f32 = echo_dur as f32 / 16.0 / 58.0;

        info!("DISTANCE IS: {}", distance_cm)
    }

    async fn wait_echo_for_high(&self) {
        while !self.echo.is_high().unwrap() {
            // info!("Waiting...");
            Timer::after(Duration::from_micros(1)).await;
        }
    }

    // Define a function to wait for the pin to go low
    async fn wait_echo_for_low(&self) {
        while !self.echo.is_low().unwrap() {
            // info!("Waiting...");
            Timer::after(Duration::from_micros(1)).await;
        }
    }
}

#[task]
pub async fn read_sensor(mut ultrasonic: Ultrasonic) {
    loop {
        Timer::after_millis(1000).await;
        ultrasonic.read_sensor().await;
    }
}

#[task]
pub async fn led(mut led: GpioPin<Output<PushPull>, 4>) {
    loop {
        Timer::after_millis(1000).await;
        led.toggle().unwrap();
    }
}
