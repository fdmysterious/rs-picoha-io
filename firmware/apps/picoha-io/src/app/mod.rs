use crate::platform_io::{
    PlatformData,
    PlatformLed,
    PlatformSleep,
    GpioCtrl,

    gpio_ctrl::{
        PinIndex,
        PinValue,
        PinDir
    }
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

trait PinDirToGpioDir {
    fn into_gpio_dir(&self) -> gpio::GpioDir;
}

trait PinValueToGpioValue {
    fn into_gpio_value(&self) -> gpio::GpioValue;
}

trait GpioDirToPinDir {
    fn into_pin_dir(&self) -> PinDir;
}

trait GpioValueToPinValue {
    fn into_pin_value(&self) -> PinValue;
}

///

impl PinDirToGpioDir for PinDir {
    fn into_gpio_dir(&self) -> gpio::GpioDir {
        match self {
            Self::PullUpInput   => gpio::GpioDir::PullUpInput,
            Self::PullDownInput => gpio::GpioDir::PullDownInput,
            Self::Output        => gpio::GpioDir::Output,
            _                   => gpio::GpioDir::PullUpInput, // TODO // Cannot convert here
        }
    }
}

impl GpioDirToPinDir for gpio::GpioDir {
    fn into_pin_dir(&self) -> PinDir {
        match self {
            Self::PullUpInput   => PinDir::PullUpInput,
            Self::PullDownInput => PinDir::PullDownInput,
            Self::Output        => PinDir::Output,
        }
    }
}


impl PinValueToGpioValue for PinValue {
    fn into_gpio_value(&self) -> gpio::GpioValue {
        match self {
            PinValue::Low  => gpio::GpioValue::Low,
            PinValue::High => gpio::GpioValue::High,
        }
    }
}

impl GpioValueToPinValue for gpio::GpioValue {
    fn into_pin_value(&self) -> PinValue {
        match self {
            Self::Low  => PinValue::Low,
            Self::High => PinValue::High,
        }
    }
}

///////////////////////////////////////

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

    pub fn process_gpio<P: PlatformData>(&mut self, platf: &mut P, frame: ha::MsgFrame) -> Result<ha::MsgFrame, ha::MsgError>
    {
        let req  = gpio::Request::consume_frame(frame)?;
        let resp = match req {
            gpio::Request::GpioDirSet(idx, dir) => {
                match platf.get_pins().dir_set(idx as PinIndex, dir.into_pin_dir()) {
                    Ok(_)  => gpio::Response::Good,
                    Err(x) => gpio::Response::ErrGeneric("Cannot set into desired mode"),
                }
            }

            gpio::Request::GpioDirGet(idx) => {
                match platf.get_pins().dir_get(idx as PinIndex) {
                    Ok(x)  => gpio::Response::GpioDir(idx, x.into_gpio_dir()),
                    Err(x) => gpio::Response::ErrInvalidArgs,
                }
            }

            gpio::Request::GpioWrite(idx, value) => {
                match platf.get_pins().pin_write(idx as PinIndex, value.into_pin_value()) {
                    Ok(_)  => gpio::Response::Good,
                    Err(_) => gpio::Response::ErrGeneric("Cannot write pin value"),
                }
            }

            gpio::Request::GpioRead(idx) => {
                match platf.get_pins().pin_read(idx as PinIndex) {
                    Ok(x)  => gpio::Response::GpioValue(idx, x.into_gpio_value()),
                    Err(_) => gpio::Response::ErrGeneric("Error reading pin"),
                }
            }
        };

        Ok(resp.to_frame())
    }

    pub fn process_frame<P: PlatformData>(&mut self, platf: &mut P, frame: ha::MsgFrame) -> Result<ha::MsgFrame, ha::MsgError>
    {
        match ha::CodeCategory::categorize(&frame.code) {
            ha::CodeCategory::ReqGeneric => self.process_generic(platf, frame),
            ha::CodeCategory::ReqGpio    => self.process_gpio   (platf, frame),
            _                            => Err(ha::MsgError::UnknownCode),
        }
    }
}
