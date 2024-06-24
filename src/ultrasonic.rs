use embassy_executor::task;
use embassy_time::Timer;
use esp_hal::gpio::PullUp;
use esp_hal::gpio::PushPull;
use esp_hal::gpio::{GpioPin, Input, Output};
use esp_hal::prelude::_embedded_hal_digital_v2_InputPin;
use esp_hal::prelude::_embedded_hal_digital_v2_OutputPin;
use esp_hal::systimer::SystemTimer;
use log::info;

const TRIGGER_PIN: u8 = 4;
const ECHO_PIN: u8 = 5;

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
}

#[task]
pub async fn read_sensor(ultrasonic: &'static Ultrasonic) {
    //clean pulse
    ultrasonic.trigger.set_low().unwrap();
    Timer::after_millis(5).await;

    ultrasonic.trigger.set_high().unwrap();
    Timer::after_millis(10).await;

    while !ultrasonic.echo.is_high().unwrap() {}

    let echo_start = SystemTimer::now();

    while !ultrasonic.echo.is_low().unwrap() {}

    let echo_end = SystemTimer::now();

    let echo_dur = echo_end.wrapping_sub(echo_start);

    let distance_cm = echo_dur / 16 / 58;

    info!("DISTANCE IS: {}", distance_cm);
}
