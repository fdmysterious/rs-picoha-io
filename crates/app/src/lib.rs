#![no_std]

use platform_io::{
    PlatformData,
    PlatformLed,
    PlatformSleep,
    GpioCtrl,
    PinValue,
    PinDir
};

pub fn main_init(platf: &mut dyn PlatformData) {
    platf.get_pins().dir_set(25, PinDir::Output).unwrap();
}

pub fn main_loop(platf: &mut dyn PlatformData){
    platf.get_pins().pin_write(25, PinValue::High).unwrap();
    platf.get_sleep().sleep_ms(100);
    platf.get_pins().pin_write(25, PinValue::Low).unwrap();
    platf.get_sleep().sleep_ms(100);
}
