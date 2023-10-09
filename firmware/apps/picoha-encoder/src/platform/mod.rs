use embedded_hal::{
    digital::v2::OutputPin,
    PwmPin,
};

use crate::platform_io::{
    PlatformLed,
    PlatformSleep,
    PlatformPwm,
    PlatformData,
};

use crate::board::Board;

use rp_pico::hal::{
    pwm::{
        Slice,
        SliceId,
        ValidSliceMode,
        ChannelId
    }
};

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

pub struct PicoPwm<I,M> 
where
    I: SliceId,
    M: ValidSliceMode<I>
{
    slice: Slice<I,M>,
}

impl<I,M> PicoPwm<I,M>
where
    I: SliceId,
    M: ValidSliceMode<I>
{
    fn new(slice: Slice<I,M>) -> Self {
        Self {
            slice: slice
        }
    }
}

impl<I,M> PlatformPwm for PicoPwm<I,M>
where
    I: SliceId,
    M: ValidSliceMode<I>,
{
    fn duty_set(&mut self, duty: u16) {
        self.slice.channel_a.set_duty(duty.into());
    }
}

//////////////////////

impl PlatformSleep for cortex_m::delay::Delay {
    fn sleep_ms(&mut self, delay_ms: u32) {
        self.delay_ms(delay_ms);
    }

    fn sleep_us(&mut self, delay_us: u32) {
        self.delay_us(delay_us);
    }
}

//////////////////////

pub struct PlatformPico<LedPin, I,M>
where
    LedPin:     OutputPin,
    I: SliceId,
    M: ValidSliceMode<I>,
{
    pwm:   PicoPwm<I,M>,
    led:   LedPin,
    sleep: cortex_m::delay::Delay,
}

impl<LedPin, I,M> PlatformPico<LedPin, I,M>
where
    LedPin: OutputPin,
    I: SliceId,
    M: ValidSliceMode<I>
{
    pub fn new(
        pwm:    Slice<I,M>,
        led:    LedPin,
        delay:  cortex_m::delay::Delay,
    ) -> Self {
        Self {
            pwm:   PicoPwm::new(pwm),
            led:   led,
            sleep: delay,
        }
    }
}

impl<LedPin, I,M> PlatformData for PlatformPico<LedPin, I,M>
where
    LedPin: OutputPin,
    I: SliceId,
    M: ValidSliceMode<I>
{
    fn get_led(&mut self) -> &mut dyn PlatformLed {
        &mut self.led
    }

    fn get_sleep(&mut self) -> &mut dyn PlatformSleep {
        &mut self.sleep
    }

    fn get_pwm(&mut self) -> &mut dyn PlatformPwm {
        &mut self.pwm
    }
}
