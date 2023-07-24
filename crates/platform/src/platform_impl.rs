use embedded_time::rate::*;
use embedded_hal::digital::v2::OutputPin;
use panic_probe as _;

use rp_pico as bsp;

use bsp::hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    sio::Sio,
    watchdog::Watchdog,
    gpio,
};

use platform_io::*;

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


struct PlatformPinCtrlIn {
    target_pin: &DynPin;
}


//////////////////////////////////////////////////////////////////////////

pub struct Platform {
    pub led: MyPlatformLed,
    pub delay: MyPlatformSleep,
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


        Ok(Self {
            led:   MyPlatformLed   { pin: led_pin },
            delay: MyPlatformSleep { delay: delay },
        })
    }
}