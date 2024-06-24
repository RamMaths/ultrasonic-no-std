use embassy_executor::task;
use embassy_time::Timer;
use esp_hal::gpio::PullUp;
use esp_hal::gpio::PushPull;
use esp_hal::gpio::{GpioPin, Input, Output};
use esp_hal::prelude::_embedded_hal_digital_v2_OutputPin;
use esp_hal::systimer::SystemTimer;

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

    pub async fn read(&mut self) {
        self.trigger.set_low().unwrap();
    }
}
