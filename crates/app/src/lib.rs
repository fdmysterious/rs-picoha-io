#![no_std]

use platform_io::{
    PlatformLed,
    PlatformSleep,
    GpioCtrl,
    PinValue,
    PinDir
};

pub fn main_init(_led: &mut dyn PlatformLed, _delay: &mut dyn PlatformSleep, pins: &mut dyn GpioCtrl) {
    pins.dir_set(7, PinDir::Output).unwrap();
}

pub fn main_loop(led: &mut dyn PlatformLed, delay: &mut dyn PlatformSleep, pins: &mut dyn GpioCtrl){
    led.led_on();
    pins.pin_write(7, PinValue::High).unwrap();
    delay.sleep_ms(100);
    led.led_off();
    pins.pin_write(7, PinValue::Low).unwrap();
    delay.sleep_ms(100);
}
