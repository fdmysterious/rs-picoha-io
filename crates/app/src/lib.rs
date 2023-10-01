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

use const_random::const_random;

// Constants
#[cfg(debug_assertions)]
const VERSION: &str = env!("GIT_HASH");

#[cfg(not(debug_assertions))]
const VERSION: &str = env!("CARGO_PKG_VERSION");

const ID: [u8;8] = const_random!([u8;8]);

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

    pub fn process_generic<P: PlatformData>(&mut self, platf: &mut P, frame: ha::MsgFrame) -> Result<ha::MsgFrame, ha::MsgError>
    {
        let req  = common::Request::consume_frame(frame)?;
        let resp = match req{
            common::Request::Ping    => common::Response::Good,
            common::Request::ItfType => common::Response::ItfTypeResp(ha::ItfType::Gpio),
            common::Request::Version => common::Response::VersionResp(&VERSION),
            common::Request::IdGet   => common::Response::IdResp(&ID),
        };

        Ok(resp.to_frame())
    }

    pub fn process_frame<P: PlatformData>(&mut self, platf: &mut P, frame: ha::MsgFrame) -> Result<ha::MsgFrame, ha::MsgError>
    {
        match ha::CodeCategory::categorize(&frame.code) {
            ha::CodeCategory::ReqGeneric => self.process_generic(platf, frame),
            //ha::CodeCategory::ReqGpio    => self.process_gpio   (platf, frame)

            _ => Err(ha::MsgError::UnknownCode)
        }
    }
}
