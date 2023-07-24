#![no_std]

use platform_io::{PlatformLed, PlatformSleep};

pub fn main_loop(led: &mut dyn PlatformLed, delay: &mut dyn PlatformSleep){
    led.led_on();
    delay.sleep_ms(100);
    led.led_off();
    delay.sleep_ms(100);
}
