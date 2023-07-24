#![no_std]
#![no_main]

mod platform_impl;
use platform_impl::Platform;

use app;

use rp_pico as bsp;
use bsp::entry;

#[entry]
fn main() -> ! {
    let mut platform = Platform::init().unwrap();

    loop {
        app::main_loop(&mut platform.led, &mut platform.delay);
    }
}
