#![no_std]
#![no_main]

mod board;
mod platform_io;
mod platform;

use defmt::*;
use defmt_rtt   as _;
use panic_probe as _;

use embedded_hal::PwmPin;

use board::Board;

use rp_pico as bsp;
use bsp::entry;

use platform::{
    PlatformPico,
    PicoDiffEncoder,
};
use platform_io::{
    PlatformLed,
    PlatformSleep,
    PlatformData,
    PlatformEncoder,
};


#[entry]
fn main() -> ! {
    let mut board = Board::init();
    //let mut pwms  = board.pwms;

    //let mut pwm  = pwms.pwm4;
    //let mut delay = board.delay;

    //pwm.enable();

    //let mut channel = pwm.channel_b;
    //channel.output_to(board.pins.pwm_out);

    //board.pwms.pwm4.set_ph_correct();
    //board.pwms.pwm4.enable();
    //board.pwms.pwm4.channel_a.output_to(board.pins.pwm_out);

    //board.pwms.pwm4.set_ph_correct();
    //board.pwms.pwm4.channel_b.enable();
    //board.pwms.pwm4.channel_b.output_to(board.pins.pwm_out);
    //board.pwms.pwm4.channel_b.set_duty(10000);
    
    let mut pwm = PicoDiffEncoder::new(
        board.pwms.pwm0,
        board.pins.enc0a_p_out,
        board.pins.enc0a_n_out,
    );

    pwm.configure();

    //let mut platf = PlatformPico::new(
    //    board.pins.led.into_push_pull_output(),
    //    board.delay,
    //);

    const DELAY_US: u32  = 100u32;
    const DUTY_MAX: u16  = 25000u16;
    const DUTY_STEP: u16 = 1u16;

    let mut duty    = 0u16;

    //platf.get_pwm().duty_set(DUTY_MAX);

    loop {
        duty += DUTY_STEP;
        duty %= DUTY_MAX;
        board.pwms.pwm4.channel_b.set_duty(duty);
        //channel.set_duty(duty);
        board.delay.sleep_us(DELAY_US);


        //platf.get_pwm().duty_set(duty);
        //platf.get_sleep().sleep_us(DELAY_US);

        //platf.get_led().led_on();
        //platf.get_sleep().sleep_ms(100);
        //platf.get_led().led_off();
        //platf.get_sleep().sleep_ms(100);
    }
}

