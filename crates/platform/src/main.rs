#![no_std]
#![no_main]

mod usb_config;
mod platform_impl;
use platform_impl::Platform;


use board::Board;
use app::App;

use rp_pico as bsp;
use bsp::entry;

use platform_io::{GpioCtrl};

#[entry]
fn main() -> ! {
    let board        = Board::init();

    let usb_bus      = board.usb_bus;
    let mut platform = Platform::init(
        board.pins,
        board.delay,
        &usb_bus
    ).unwrap();

    platform.pins.init().unwrap();

    let mut app = App::new();
    app.init(&mut platform);

    loop {
        if platform.usb.dev.poll(&mut [&mut platform.usb.serial]) {
            let mut buf = [0u8; 64];

            match platform.usb.serial.read(&mut buf) {
                Err(_) => {},
                Ok(0)  => {},

                Ok(count) => {
                    platform.usb.serial.write(&buf);
                }
            }
        }
        
        app.app_loop(&mut platform);
    }
}
