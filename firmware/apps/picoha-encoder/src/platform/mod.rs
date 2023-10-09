use embedded_hal::{
    digital::v2::OutputPin,
    PwmPin,
};

use crate::platform_io::{
    PlatformLed,
    PlatformSleep,
    PlatformEncoder,
    PlatformData,
};

use crate::board::Board;

use rp_pico::hal::{
    pwm::{
        self,
        Slice,
        SliceId,
        ValidSliceMode,
        ChannelId,

        ValidPwmOutputPin,
    },

    gpio::{
        AnyPin,
        PinId,
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

pub struct PicoDiffEncoder<
    SliceNum: SliceId,
> {
    slice: Slice<SliceNum, pwm::FreeRunning>,
}

impl<
    SliceNum: SliceId,
> PicoDiffEncoder<SliceNum> {
    pub fn new<PinAP: AnyPin, PinAN: AnyPin>(
        mut slice:   Slice<SliceNum, pwm::FreeRunning>,
        pin_a_p: PinAP,
        pin_a_n: PinAN,
    ) -> Self
    where
        PinAP::Id: ValidPwmOutputPin<SliceNum, pwm::A>,
        PinAN::Id: ValidPwmOutputPin<SliceNum, pwm::B>,
    {
        slice.channel_a.output_to(pin_a_p);
        slice.channel_b.output_to(pin_a_n);

        Self {
            slice: slice,
        }
    }
}

impl<
    SliceNum: SliceId
> PlatformEncoder for PicoDiffEncoder<SliceNum> {
    fn configure(&mut self) {
        self.slice.channel_b.set_inverted();

        self.slice.channel_a.set_duty(self.slice.get_top()>>1);
        self.slice.channel_b.set_duty(self.slice.get_top()>>1);

        self.slice.channel_a.enable();
        self.slice.channel_b.enable();
        self.slice.enable();
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

pub struct PlatformPico<
    LedPin,
>
where
    LedPin:     OutputPin,
{
    led:   LedPin,
    sleep: cortex_m::delay::Delay,
}

impl<LedPin> PlatformPico<LedPin>
where
    LedPin: OutputPin,
{
    pub fn new(
        led:    LedPin,
        delay:  cortex_m::delay::Delay,
    ) -> Self {
        Self {
            led:   led,
            sleep: delay,
        }
    }
}

impl<LedPin> PlatformData for PlatformPico<LedPin>
where
    LedPin: OutputPin,
{
    fn get_led(&mut self) -> &mut dyn PlatformLed {
        &mut self.led
    }

    fn get_sleep(&mut self) -> &mut dyn PlatformSleep {
        &mut self.sleep
    }

}
