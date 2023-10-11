use fixed::{
    FixedU32,
    types::extra::U10,
};

pub trait PlatformLed {
    fn led_on(&mut self);
    fn led_off(&mut self);
}

pub trait PlatformSleep {
    fn sleep_ms(&mut self, delay_ms: u32);
    fn sleep_us(&mut self, delay_us: u32);
}

pub enum PlatformEncoderDirection {
    Forward,
    Backward,
}

pub trait PlatformEncoder {
    /// Initialize the encoder output
    fn configure(&mut self);

    /// Set the encoder output frequency. TODO Use some syntax sugar for Q16 number?
    fn freq_set(&mut self, freq_q16: FixedU32<U10>); // Input format is Q16 unsigned.
}



pub trait PlatformData {
    fn get_led  (&mut self) -> &mut dyn PlatformLed;
    fn get_sleep(&mut self) -> &mut dyn PlatformSleep;
}
