use embedded_hal::digital::v2::OutputPin;

use crate::platform_io::{
    PlatformLed,
    PlatformSleep,
    PlatformData,
};

use crate::board::Board;


//////////////////////

impl<T: OutputPin> PlatformLed for T {
    fn led_on(&mut self) {
        self.set_high().ok();
    }

    fn led_off(&mut self) {
        self.set_low().ok();
    }
}

//////////////////////

impl PlatformSleep for cortex_m::delay::Delay {
    fn sleep_ms(&mut self, delay_ms: u32) {
        self.delay_ms(delay_ms);
    }
}


//////////////////////

pub struct PlatformPico<LedPin>
where
    LedPin: OutputPin,
{
    led:   LedPin,
    sleep: cortex_m::delay::Delay,
}

impl<LedPin: OutputPin> PlatformPico<LedPin> {
    pub fn new(
        led: LedPin,
        delay: cortex_m::delay::Delay,
    ) -> Self {
        Self {
            led:   led,
            sleep: delay,
        }
    }
}

impl<LedPin: OutputPin> PlatformData for PlatformPico<LedPin> {
    fn get_led(&mut self) -> &mut dyn PlatformLed {
        &mut self.led
    }

    fn get_sleep(&mut self) -> &mut dyn PlatformSleep {
        &mut self.sleep
    }
}
