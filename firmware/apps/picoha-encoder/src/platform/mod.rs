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
    SliceANum: SliceId,
    SliceBNum: SliceId,
> {
    sliceA: Slice<SliceANum, pwm::FreeRunning>,
    sliceB: Slice<SliceBNum, pwm::FreeRunning>,
}

impl<
    SliceANum: SliceId,
    SliceBNum: SliceId,
> PicoDiffEncoder<SliceANum, SliceBNum> {
    pub fn new<PinAP: AnyPin, PinAN: AnyPin, PinBP: AnyPin, PinBN: AnyPin>(
        mut sliceA:   Slice<SliceANum, pwm::FreeRunning>,
        mut sliceB:   Slice<SliceBNum, pwm::FreeRunning>,

        pin_a_p: PinAP,
        pin_a_n: PinAN,

        pin_b_p: PinBP,
        pin_b_n: PinBN,
    ) -> Self
    where
        PinAP::Id: ValidPwmOutputPin<SliceANum, pwm::A>,
        PinAN::Id: ValidPwmOutputPin<SliceANum, pwm::B>,

        PinBP::Id: ValidPwmOutputPin<SliceBNum, pwm::A>,
        PinBN::Id: ValidPwmOutputPin<SliceBNum, pwm::B>,
    {
        sliceA.channel_a.output_to(pin_a_p);
        sliceA.channel_b.output_to(pin_a_n);

        sliceB.channel_a.output_to(pin_b_p);
        sliceB.channel_b.output_to(pin_b_n);

        Self {
            sliceA: sliceA,
            sliceB: sliceB,
        }
    }
}

impl<
    SliceANum: SliceId,
    SliceBNum: SliceId,
> PlatformEncoder for PicoDiffEncoder<SliceANum, SliceBNum> {
    fn configure(&mut self) {
        self.sliceA.channel_b.set_inverted();
        self.sliceB.channel_b.set_inverted();

        self.sliceA.channel_a.set_duty(self.sliceA.get_top()>>1);
        self.sliceA.channel_b.set_duty(self.sliceA.get_top()>>1);
        self.sliceB.channel_a.set_duty(self.sliceB.get_top()>>1);
        self.sliceB.channel_b.set_duty(self.sliceB.get_top()>>1);

        self.sliceA.channel_a.enable();
        self.sliceA.channel_b.enable();
        self.sliceB.channel_a.enable();
        self.sliceB.channel_b.enable();

        //self.sliceB.set_counter(self.sliceB.get_top()>>2);

        self.sliceB.set_counter(0);
        self.sliceA.set_counter(self.sliceB.get_top()>>2);

        self.sliceA.enable();
        self.sliceB.enable();

        //for i in 0..(self.sliceB.get_top()>>2) {
        //    self.sliceB.advance_phase();
        //}
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
