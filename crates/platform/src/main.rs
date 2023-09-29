#![no_std]
#![no_main]

mod usb_config;
mod platform_impl;
use platform_impl::Platform;

use app;

use board::Board;

use rp_pico as bsp;
use bsp::entry;

use platform_io::{GpioCtrl};

#[entry]
fn main() -> ! {
    let board        = Board::init();
    let mut platform = Platform::init(board).unwrap();

    platform.pins.init().unwrap();

    app::main_init(&mut platform);
    loop {
        app::main_loop(&mut platform);
    }
}
