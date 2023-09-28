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
    platf.get_pins().dir_set(7, PinDir::Output).unwrap();
}

pub fn main_loop(platf: &mut dyn PlatformData){
    platf.get_led().led_on();
    platf.get_sleep().sleep_ms(100);
    platf.get_led().led_off();
    platf.get_sleep().sleep_ms(100);

    //pins.pin_write(7, PinValue::High).unwrap();
    //delay.sleep_ms(100);
    //led.led_off();
    //pins.pin_write(7, PinValue::Low).unwrap();
    //delay.sleep_ms(100);
}
