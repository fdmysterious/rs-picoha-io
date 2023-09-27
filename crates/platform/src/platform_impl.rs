use embedded_time::rate::*;
use embedded_hal::digital::v2::OutputPin;
use embedded_hal::digital::v2::InputPin;
use embedded_hal::digital::v2::PinState;

use panic_probe as _;

use rp_pico as bsp;

use bsp::hal::{
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

use platform_io::*;
use platform_io::gpio_ctrl::{
    GpioCtrlError,
    PinDir,
    PinValue,
    PinIndex,
    GpioCtrl,
};

//////////////////////////////////////////////////////////////////////////

pub struct MyPlatformLed {
    pin: gpio::Pin<gpio::bank0::Gpio25, gpio::PushPullOutput>
}

impl PlatformLed for MyPlatformLed {
    fn led_on(&mut self) {
        self.pin.set_high().unwrap();
    }

    fn led_off(&mut self) {
        self.pin.set_low().unwrap();
    }
}

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

pub struct PlatformPins {
    pins: [DynPin; 10],
    dirs: [PinDir; 10],
}

impl PlatformPins {
    pub fn new(pins: [DynPin;10]) -> Self {
        Self {
            pins: pins,
            dirs: [
                PinDir::Unknown,
                PinDir::Unknown,
                PinDir::Unknown,
                PinDir::Unknown,
                PinDir::Unknown,
                PinDir::Unknown,
                PinDir::Unknown,
                PinDir::Unknown,
                PinDir::Unknown,
                PinDir::Unknown,
            ]
        }
    }
}

impl GpioCtrl for PlatformPins {
    fn init(&mut self) -> Result<(), GpioCtrlError> {
        // Set default directions for pins
        for i in 0..self.pins.len() {
            self.dir_set(i, PinDir::PullDownInput)?;
        }
        Ok(())
    }

    fn dir_set(&mut self, idx: PinIndex, dir: PinDir) -> Result<(),GpioCtrlError>  {
        if idx < self.pins.len() {
            let dpin             = &mut self.pins[idx];
            let mode: DynPinMode = dir.mode_to_pico_dir().ok_or(GpioCtrlError::PinInvalidDir)?;
            dpin.try_into_mode(mode).or(Err(GpioCtrlError::PinHalError))?;

            Ok(())
        }
        else {
            Err(GpioCtrlError::PinInvalidIndex)
        }
    }

    fn pin_write(&mut self, idx: PinIndex, value: PinValue) -> Result<(), GpioCtrlError> {
        if idx < self.pins.len() {
            let pdir = &self.dirs[idx];

            if pdir.is_output_dir() {
                let pdyn = &mut self.pins[idx];
                pdyn.set_state(value.value_to_state()).or(Err(GpioCtrlError::PinHalError))?;
                Ok(())
            }

            else {
                Err(GpioCtrlError::PinMismatchDir)
            }
        }
        
        else {
            Err(GpioCtrlError::PinInvalidIndex)
        }
    }

    fn pin_read(&self, idx: PinIndex) -> Result<PinValue, GpioCtrlError> {
        if idx < self.pins.len() {
            let pdir = &self.dirs[idx];

            if pdir.is_input_dir() {
                let pdyn = &self.pins[idx];

                match pdyn.is_high() {
                    Ok(true)  => Ok(PinValue::High),
                    Ok(false) => Ok(PinValue::Low ),
                    Err(_)    => Err(GpioCtrlError::PinHalError),
                }
            }

            else {
                Err(GpioCtrlError::PinMismatchDir)
            }
        }

        else {
            Err(GpioCtrlError::PinInvalidIndex)
        }
    }
}

//////////////////////////////////////////////////////////////////////////

pub struct Platform {
    pub led: MyPlatformLed,
    pub delay: MyPlatformSleep,
    pub pins: PlatformPins,
}

impl Platform {
    pub fn init() -> Result<Self, PlatformError> {
        let mut      pac = pac::Peripherals::take().unwrap();
        let         core = pac::CorePeripherals::take().unwrap();

        let mut watchdog = Watchdog::new(pac.WATCHDOG);
        let          sio = Sio::new(pac.SIO);


        let external_xtal_freq_hz = 12_000_000u32.Hz();
        let clocks = init_clocks_and_plls(
            external_xtal_freq_hz.integer(),
            pac.XOSC,
            pac.CLOCKS,
            pac.PLL_SYS,
            pac.PLL_USB,
            &mut pac.RESETS,
            &mut watchdog,
        )
            .ok()
            .unwrap();

        let pins = bsp::Pins::new(
            pac.IO_BANK0,
            pac.PADS_BANK0,
            sio.gpio_bank0,
            &mut pac.RESETS,
        );

        let led_pin = pins.led.into_push_pull_output();
        let delay   = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().integer());

        let pins    = PlatformPins::new([
            pins.gpio0.into(),
            pins.gpio1.into(),
            pins.gpio2.into(),
            pins.gpio3.into(),
            pins.gpio4.into(),
            pins.gpio5.into(),
            pins.gpio6.into(),
            pins.gpio7.into(),
            pins.gpio8.into(),
            pins.gpio9.into(),
        ]);

        Ok(Self {
            led:   MyPlatformLed   { pin: led_pin },
            delay: MyPlatformSleep { delay: delay },
            pins: pins
        })
    }
}
