#![no_std]

pub mod gpio_ctrl;

#[derive(Debug)]
pub enum PlatformError {
    InitError,
}

pub trait PlatformLed {
    fn led_on(&mut self);
    fn led_off(&mut self);
}

pub trait PlatformSleep {
    fn sleep_ms(&mut self, delay_ms: u32);
}
