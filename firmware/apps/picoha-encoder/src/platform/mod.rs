use embedded_hal::digital::v2::OutputPin;

use crate::platform_io::{
    PlatformLed,
    PlatformSleep,
    PlatformData,
};

use crate::board::Board;


//////////////////////

pub struct PlatformLedPico<LedPin>
where
    LedPin: OutputPin
{
    pin: LedPin,
}

impl<LedPin: OutputPin> PlatformLedPico<LedPin>
{
    fn new(p: LedPin) -> Self {
        Self {
            pin: p,
        }
    }
}

impl<LedPin: OutputPin> PlatformLed for PlatformLedPico<LedPin>
{
    fn led_on(&mut self) {
        self.pin.set_high().ok();
    }

    fn led_off(&mut self) {
        self.pin.set_low().ok();
    }
}

//////////////////////

pub struct PlatformSleepPico {
    delay: cortex_m::delay::Delay,
}

impl PlatformSleepPico {
    fn new(delay: cortex_m::delay::Delay) -> Self {
        Self {
            delay: delay,
        }
    }
}

impl PlatformSleep for PlatformSleepPico {
    fn sleep_ms(&mut self, delay_ms: u32) {
        self.delay.delay_ms(delay_ms);
    }
}

//////////////////////

pub struct PlatformPico<LedPin>
where
    LedPin: OutputPin,
{
    led: PlatformLedPico<LedPin>,
    sleep: PlatformSleepPico,
}

impl<LedPin: OutputPin> PlatformPico<LedPin> {
    pub fn new(
        led: LedPin,
        delay: cortex_m::delay::Delay,
    ) -> Self {
        Self {
            led:   PlatformLedPico::new(led),
            sleep: PlatformSleepPico::new(delay),
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
