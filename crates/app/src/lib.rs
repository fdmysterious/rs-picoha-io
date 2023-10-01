#![no_std]

use platform_io::{
    PlatformData,
    PlatformLed,
    PlatformSleep,
    GpioCtrl,
    PinValue,
    PinDir
};

use protocols::slip::{
    Decoder,
    Encoder,
    SlipError,
};


use protocols::ha;

use protocols::gpio;
use protocols::common;

pub struct App
{
}

impl App
{
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn init<P: PlatformData>(&mut self, platf: &mut P)
    {
    }

    pub fn process_frame<P: PlatformData>(&mut self, platf: &mut P, frame: ha::MsgFrame) -> Result<ha::MsgFrame, ha::MsgError>
    {
        Ok(protocols::common::Response::Good.to_frame())
    }
}
