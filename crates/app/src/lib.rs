#![no_std]

use platform_io::{
    PlatformData,
    PlatformLed,
    PlatformSleep,
    GpioCtrl,
    PinValue,
    PinDir
};

pub struct App
{
}

impl App
{
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn init<P: PlatformData>(&self, platf: &mut P)
    {
        platf.get_pins().dir_set(25, PinDir::Output).unwrap();
    }

    pub fn app_loop<P: PlatformData>(&self, platf: &mut P)
    {
    }
}
