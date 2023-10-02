#![no_std]
#![no_main]

mod board;
mod platform_io;
mod platform;

use defmt::*;
use defmt_rtt   as _;
use panic_probe as _;

use board::Board;

use rp_pico as bsp;
use bsp::entry;

use platform::PlatformPico;
use platform_io::{
    PlatformLed,
    PlatformSleep,
    PlatformData,
};


#[entry]
fn main() -> ! {
    let board = Board::init();
    let mut platf = PlatformPico::new(
        board.pins.led.into_push_pull_output(),
        board.delay,
    );

    loop {
        platf.get_led().led_on();
        platf.get_sleep().sleep_ms(100);
        platf.get_led().led_off();
        platf.get_sleep().sleep_ms(100);
    }
}

