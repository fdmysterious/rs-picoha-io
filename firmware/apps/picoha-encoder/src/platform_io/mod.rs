pub trait PlatformLed {
    fn led_on(&mut self);
    fn led_off(&mut self);
}

pub trait PlatformSleep {
    fn sleep_ms(&mut self, delay_ms: u32);
    fn sleep_us(&mut self, delay_us: u32);
}

pub trait PlatformPwm {
    fn duty_set(&mut self, duty: u16);
}

pub trait PlatformData {
    fn get_led  (&mut self) -> &mut dyn PlatformLed;
    fn get_sleep(&mut self) -> &mut dyn PlatformSleep;
    fn get_pwm  (&mut self) -> &mut dyn PlatformPwm;
}
