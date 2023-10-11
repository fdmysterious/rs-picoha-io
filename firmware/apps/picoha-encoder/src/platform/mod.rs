use embedded_hal::{
    digital::v2::OutputPin,
    PwmPin,
};

use fugit::HertzU32;
use fixed::{
    FixedU32,
    FixedU64,
    types::extra::{
        U10,
    }
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

    clocks::{
        Clock,
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

const PICO_MAX_TOP: u16 = 0xFFFFu16;

pub struct PicoDiffEncoder<
    SliceANum: SliceId,
    SliceBNum: SliceId,
> {
    sliceA: Slice<SliceANum, pwm::FreeRunning>,
    sliceB: Slice<SliceBNum, pwm::FreeRunning>,

    input_clk_freq: HertzU32,
}

impl<
    SliceANum: SliceId,
    SliceBNum: SliceId,
> PicoDiffEncoder<SliceANum, SliceBNum> {
    pub fn new<
        PinAP: AnyPin,
        PinAN: AnyPin,
        PinBP: AnyPin,
        PinBN: AnyPin,

        Clk: Clock,
    >(
        clk: &Clk,

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
            input_clk_freq: clk.freq(),
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

        self.sliceA.set_top(PICO_MAX_TOP);
        self.sliceB.set_top(PICO_MAX_TOP);

        self.sliceB.set_counter(0);
        self.sliceA.set_counter(self.sliceB.get_top()>>2);

        self.sliceA.enable();
        self.sliceB.enable();

        //for i in 0..(self.sliceB.get_top()>>2) {
        //    self.sliceB.advance_phase();
        //}
    }

    fn freq_set(&mut self, freq_q16: FixedU32<U10>) {
        let in_freq: FixedU64<U10> = freq_q16.into();

        let sys_freq  = FixedU64::<U10>::from_num(self.input_clk_freq.to_Hz());
        let max_count = FixedU64::<U10>::from_num(PICO_MAX_TOP);

        //self.sliceA.disable();
        //self.sliceB.disable();

        // Formula to determine frac div is: sys_freq / (input_freq * max_count)
        if let Some(tmp) = sys_freq.checked_div(in_freq.checked_mul(max_count).unwrap_or(FixedU64::<U10>::from_num(1))) {
            // If divisor is below limit
            if tmp.int() <= (1<<8) {
                let int:     u64 = tmp.int().to_bits();
                let frac:    u64 = tmp.frac().to_bits();

                //self.sliceA.set_div_int(int as u8);
                //self.sliceA.set_div_frac((frac >> (10-4)) as u8); // convert 10bits frac to 4bits

                //self.sliceB.set_div_int(int as u8);
                //self.sliceB.set_div_frac((frac >> (10-4)) as u8); // Convert 10bits frac to 4bits

                //self.sliceA.enable();
                //self.sliceB.enable();
            }

            // TODO: Manage invalid frequency case
        }
        // TODO: Manage invalid frequency case
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
