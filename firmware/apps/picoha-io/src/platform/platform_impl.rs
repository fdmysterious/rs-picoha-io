use embedded_time::rate::*;
use embedded_hal::digital::v2::OutputPin;
use embedded_hal::digital::v2::InputPin;
use embedded_hal::digital::v2::PinState;


use usb_device::class_prelude::UsbBusAllocator;
use usb_device::prelude::{
    UsbDevice,
    UsbDeviceBuilder,
    UsbVidPid,
};

use usbd_serial::SerialPort;
use usbd_serial::USB_CLASS_CDC;


use panic_probe as _;

use rp_pico as bsp;

use bsp::hal::{
    self,
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
    gpio::{
        self,
        DynPin,
    },
};
use bsp::hal::gpio::{DYN_PULL_DOWN_INPUT, DYN_PULL_UP_INPUT, DYN_FLOATING_INPUT, DYN_READABLE_OUTPUT, DynPinMode};

use crate::platform_io::*;
use crate::platform_io::gpio_ctrl::{
    GpioCtrlError,
    PinDir,
    PinValue,
    PinIndex,
    GpioCtrl,
};

use crate::platform::usb_config;

use crate::board::Board;

//////////////////////////////////////////////////////////////////////////

//pub struct MyPlatformLed {
//    pin: gpio::Pin<gpio::bank0::Gpio25, gpio::PushPullOutput>
//}
//
//impl PlatformLed for MyPlatformLed {
//    fn led_on(&mut self) {
//        self.pin.set_high().unwrap();
//    }
//
//    fn led_off(&mut self) {
//        self.pin.set_low().unwrap();
//    }
//}

pub struct MyPlatformSleep {
    delay: cortex_m::delay::Delay,
}

impl PlatformSleep for MyPlatformSleep {
    fn sleep_ms(&mut self, delay_ms: u32) {
        self.delay.delay_ms(delay_ms);
    }
}

//////////////////////////////////////////////////////////////////////////

trait PinDirPico {
    fn mode_to_pico_dir(&self) -> Option<DynPinMode>;
}

impl PinDirPico for PinDir {
    fn mode_to_pico_dir(&self) -> Option<DynPinMode> {
        match self {
            Self::Unknown       => None,
            Self::PullUpInput   => Some(DYN_PULL_UP_INPUT  ),
            Self::PullDownInput => Some(DYN_PULL_DOWN_INPUT),
            Self::NoPullInput   => Some(DYN_FLOATING_INPUT ),
            Self::Output        => Some(DYN_READABLE_OUTPUT),
        }
    }
}

trait DynPinModeToPinDir {
    fn from_pico_dir(x: DynPinMode) -> Self;
}


impl DynPinModeToPinDir for PinDir {
    fn from_pico_dir(x: DynPinMode) -> Self {
        match x {
            DYN_PULL_UP_INPUT   => Self::PullUpInput,
            DYN_PULL_DOWN_INPUT => Self::PullDownInput,
            DYN_FLOATING_INPUT  => Self::NoPullInput,
            DYN_READABLE_OUTPUT => Self::Output,
            _                   => Self::Unknown,
        }
    }
}

trait PinValueToState {
    fn value_to_state(&self) -> PinState;
}

impl PinValueToState for PinValue {
    fn value_to_state(&self) -> PinState {
        match self {
            PinValue::High => PinState::High,
            PinValue::Low  => PinState::Low,
        }
    }
}

//////////////////////////////////////////////////////////////////////////

pub struct PlatformPinStatus {
    pub pin: DynPin,
    pub dir: PinDir
}

impl PlatformPinStatus {
    fn new(pin: DynPin) -> Self {
        Self {
            pin: pin,
            dir: PinDir::Unknown,
        }
    }
}

// TODO // Reimplement this using a magic macro to avoid this copypasta nightmare?
pub struct PlatformPinsArray {
    pin0: PlatformPinStatus,
    pin1: PlatformPinStatus,
    pin2: PlatformPinStatus,
    pin3: PlatformPinStatus,
    pin4: PlatformPinStatus,
    pin5: PlatformPinStatus,
    pin6: PlatformPinStatus,
    pin7: PlatformPinStatus,
    pin8: PlatformPinStatus,
    pin9: PlatformPinStatus,
    pin10: PlatformPinStatus,
    pin11: PlatformPinStatus,
    pin12: PlatformPinStatus,
    pin13: PlatformPinStatus,
    pin14: PlatformPinStatus,
    pin15: PlatformPinStatus,
    pin16: PlatformPinStatus,
    pin17: PlatformPinStatus,
    pin18: PlatformPinStatus,
    pin19: PlatformPinStatus,
    pin20: PlatformPinStatus,
    pin21: PlatformPinStatus,
    pin22: PlatformPinStatus,
    pin25: PlatformPinStatus,
}

impl PlatformPinsArray {
    fn from_pico_pins(pins: bsp::Pins) -> Self {
        Self {
            pin0:  PlatformPinStatus::new(pins.gpio0.into()),
            pin1:  PlatformPinStatus::new(pins.gpio1.into()),
            pin2:  PlatformPinStatus::new(pins.gpio2.into()),
            pin3:  PlatformPinStatus::new(pins.gpio3.into()),
            pin4:  PlatformPinStatus::new(pins.gpio4.into()),
            pin5:  PlatformPinStatus::new(pins.gpio5.into()),
            pin6:  PlatformPinStatus::new(pins.gpio6.into()),
            pin7:  PlatformPinStatus::new(pins.gpio7.into()),
            pin8:  PlatformPinStatus::new(pins.gpio8.into()),
            pin9:  PlatformPinStatus::new(pins.gpio9.into()),
            pin10: PlatformPinStatus::new(pins.gpio10.into()),
            pin11: PlatformPinStatus::new(pins.gpio11.into()),
            pin12: PlatformPinStatus::new(pins.gpio12.into()),
            pin13: PlatformPinStatus::new(pins.gpio13.into()),
            pin14: PlatformPinStatus::new(pins.gpio14.into()),
            pin15: PlatformPinStatus::new(pins.gpio15.into()),
            pin16: PlatformPinStatus::new(pins.gpio16.into()),
            pin17: PlatformPinStatus::new(pins.gpio17.into()),
            pin18: PlatformPinStatus::new(pins.gpio18.into()),
            pin19: PlatformPinStatus::new(pins.gpio19.into()),
            pin20: PlatformPinStatus::new(pins.gpio20.into()),
            pin21: PlatformPinStatus::new(pins.gpio21.into()),
            pin22: PlatformPinStatus::new(pins.gpio22.into()),
            pin25: PlatformPinStatus::new(pins.led.into()),
        }
    }

    fn borrow(&self, idx: PinIndex) -> Option<&PlatformPinStatus> {
        match idx {
            0  => Some(&self.pin0 ),
            1  => Some(&self.pin1 ),
            2  => Some(&self.pin2 ),
            3  => Some(&self.pin3 ),
            4  => Some(&self.pin4 ),
            5  => Some(&self.pin5 ),
            6  => Some(&self.pin6 ),
            7  => Some(&self.pin7 ),
            8  => Some(&self.pin8 ),
            9  => Some(&self.pin9 ),
            10 => Some(&self.pin10),
            11 => Some(&self.pin11),
            12 => Some(&self.pin12),
            13 => Some(&self.pin13),
            14 => Some(&self.pin14),
            15 => Some(&self.pin15),
            16 => Some(&self.pin16),
            17 => Some(&self.pin17),
            18 => Some(&self.pin18),
            19 => Some(&self.pin19),
            20 => Some(&self.pin20),
            21 => Some(&self.pin21),
            22 => Some(&self.pin22),
            25 => Some(&self.pin25),
            _  => None
        }
    }

    fn borrow_mutable(&mut self, idx: PinIndex) -> Option<&mut PlatformPinStatus> {
        match idx {
            0  => Some(&mut self.pin0 ),
            1  => Some(&mut self.pin1 ),
            2  => Some(&mut self.pin2 ),
            3  => Some(&mut self.pin3 ),
            4  => Some(&mut self.pin4 ),
            5  => Some(&mut self.pin5 ),
            6  => Some(&mut self.pin6 ),
            7  => Some(&mut self.pin7 ),
            8  => Some(&mut self.pin8 ),
            9  => Some(&mut self.pin9 ),
            10 => Some(&mut self.pin10),
            11 => Some(&mut self.pin11),
            12 => Some(&mut self.pin12),
            13 => Some(&mut self.pin13),
            14 => Some(&mut self.pin14),
            15 => Some(&mut self.pin15),
            16 => Some(&mut self.pin16),
            17 => Some(&mut self.pin17),
            18 => Some(&mut self.pin18),
            19 => Some(&mut self.pin19),
            20 => Some(&mut self.pin20),
            21 => Some(&mut self.pin21),
            22 => Some(&mut self.pin22),
            25 => Some(&mut self.pin25),
            _  => None
        }
    }
}


pub struct PlatformPins {
    pins: PlatformPinsArray,
}

impl PlatformPins {
    pub fn new(bsp_pins: bsp::Pins) -> Self {
        Self {
            pins: PlatformPinsArray::from_pico_pins(bsp_pins),
        }
    }
}

impl GpioCtrl for PlatformPins {
    fn init(&mut self) -> Result<(), GpioCtrlError> {
        // Set default directions for pins
        //for i in 0..self.pins.len() {
        //    self.dir_set(i, PinDir::PullDownInput)?;
        //}
        Ok(())
    }

    fn dir_set(&mut self, idx: PinIndex, dir: PinDir) -> Result<(),GpioCtrlError>  {
        let mut dpin         = self.pins.borrow_mutable(idx).ok_or(GpioCtrlError::PinInvalidIndex)?;
        let mode: DynPinMode = dir.mode_to_pico_dir().ok_or(GpioCtrlError::PinInvalidDir)?;

        dpin.pin.try_into_mode(mode).or(Err(GpioCtrlError::PinHalError))?;
        dpin.dir             = dir;

        Ok(())
    }

    fn dir_get(&self, idx: PinIndex) -> Result<PinDir, GpioCtrlError> {
        let dpin = self.pins.borrow(idx).ok_or(GpioCtrlError::PinInvalidIndex)?;
        Ok(dpin.dir)
    }

    fn pin_write(&mut self, idx: PinIndex, value: PinValue) -> Result<(), GpioCtrlError> {
        let mut dpin = self.pins.borrow_mutable(idx).ok_or(GpioCtrlError::PinInvalidIndex)?;

        if dpin.dir.is_output_dir() {
            dpin.pin.set_state(value.value_to_state()).or(Err(GpioCtrlError::PinHalError))?;
            Ok(())
        }

        else {
            Err(GpioCtrlError::PinMismatchDir)
        }
    }

    fn pin_read(&self, idx: PinIndex) -> Result<PinValue, GpioCtrlError> {
        let dpin = self.pins.borrow(idx).ok_or(GpioCtrlError::PinInvalidIndex)?;

        if dpin.dir.is_input_dir() {
            match dpin.pin.is_high() {
                Ok(true)  => Ok(PinValue::High),
                Ok(false) => Ok(PinValue::Low ),
                Err(_)    => Err(GpioCtrlError::PinHalError),
            }
        }

        else {
            Err(GpioCtrlError::PinMismatchDir)
        }
    }
}

//////////////////////////////////////////////////////////////////////////

pub struct PlatformUsbConfig<'a> {
    manufacturer_id: u16,
    product_id: u16,
    manufacturer_name: &'a str,
    product_name: &'a str,
    serial_number: &'a str,
}

pub struct PlatformUsb<'a> {
    pub dev:    UsbDevice<'a, hal::usb::UsbBus>,
    pub serial: SerialPort<'a, hal::usb::UsbBus>,
}

impl<'a> PlatformUsb<'a> {
    pub fn new(
        bus: &'a UsbBusAllocator<hal::usb::UsbBus>,
    ) -> Self {

        let serial = SerialPort::new(bus);

        let dev = UsbDeviceBuilder::new(
            bus,
            UsbVidPid(usb_config::USB_MANUFACTURER_ID, usb_config::USB_PRODUCT_ID)
        )
            .manufacturer(usb_config::USB_MANUFACTURER_NAME)
            .product(usb_config::USB_PRODUCT_NAME)
            .serial_number(usb_config::USB_SERIAL_NUMBER)
            .device_class(USB_CLASS_CDC)
            .build();
        

        Self {
            dev: dev,
            serial: serial,
        }
        //Self {
        //    dev:       dev,
        //    serial: serial,
        //}
    }
}

//////////////////////////////////////////////////////////////////////////

pub struct Platform<'a> {
    pub sleep: MyPlatformSleep,
    pub pins: PlatformPins,

    pub usb: PlatformUsb<'a>,
}

impl<'a> PlatformData for Platform<'a> {
    //fn get_led(&mut self) -> &mut dyn PlatformLed {
    //    &mut self.led
    //}

    fn get_sleep(&mut self) -> &mut dyn PlatformSleep {
        &mut self.sleep
    }

    fn get_pins(&mut self) -> &mut dyn GpioCtrl {
        &mut self.pins
    }
}

impl<'a> Platform<'a> {
    pub fn init(
        pins: bsp::Pins,
        delay: cortex_m::delay::Delay,
        usb_bus: &'a UsbBusAllocator<hal::usb::UsbBus>,
    ) -> Result<Self, PlatformError> {

        let usb = PlatformUsb::new(
            usb_bus,
        );
        
        let pins = PlatformPins::new(pins);

        Ok(Self {
            sleep: MyPlatformSleep { delay: delay },
            pins: pins,
            usb: usb,
        })
    }
}
