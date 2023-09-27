#![no_std]
#![no_main]

mod platform_impl;
use platform_impl::Platform;

use app;

use rp_pico as bsp;
use bsp::entry;

use platform_io::{GpioCtrl};

#[entry]
fn main() -> ! {
    let mut platform = Platform::init().unwrap();
    platform.pins.init().unwrap();

    app::main_init(&mut platform.led, &mut platform.delay, &mut platform.pins);
    loop {
        app::main_loop(&mut platform.led, &mut platform.delay, &mut platform.pins);
    }
}
