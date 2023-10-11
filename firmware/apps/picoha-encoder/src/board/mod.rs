use cortex_m;
use embedded_time::rate::*;
use embedded_hal::digital::v2::OutputPin;
use embedded_hal::PwmPin;

use rp_pico as bsp;

use bsp::{
    hal::{
        self,
        clocks::{init_clocks_and_plls, Clock},
        pac,
        sio::Sio,
        watchdog::Watchdog,
        gpio,

        clocks::{
            SystemClock,
        },

        pwm::{
            Slice,
            SliceId,
            ChannelId,
        }
    },
};

use usb_device::class_prelude::UsbBusAllocator;

hal::bsp_pins! {
    Gpio0 { name: enc0a_p_out, },
    Gpio1 { name: enc0a_n_out, },
    Gpio2 { name: enc0b_p_out, },
    Gpio3 { name: enc0b_n_out, },
    Gpio4 { name: enc1a_p_out, },
    Gpio5 { name: enc1a_n_out, },
    Gpio6 { name: enc1b_p_out, },
    Gpio7 { name: enc1b_n_out, },

    Gpio25 {
        name: led,
    },
}


pub struct Board
{
    pub pins: Pins,
    pub sys_clk: SystemClock,

    pub pwms: hal::pwm::Slices,

    pub delay: cortex_m::delay::Delay,
    pub usb_bus: UsbBusAllocator<hal::usb::UsbBus>,
}

impl Board
{
    pub fn init() -> Self {
        let mut pac = pac::Peripherals::take().unwrap();
        let core    = pac::CorePeripherals::take().unwrap();

        let mut watchdog = Watchdog::new(pac.WATCHDOG);
        let sio          = Sio::new(pac.SIO);

        let external_xtal_freq_hz = 12_000_000u32;
        let clocks = init_clocks_and_plls(
            external_xtal_freq_hz,
            pac.XOSC,
            pac.CLOCKS,
            pac.PLL_SYS,
            pac.PLL_USB,
            &mut pac.RESETS,
            &mut watchdog,
        )
            .ok()
            .unwrap();

        // ---- USB init
        
        let usb = UsbBusAllocator::new(hal::usb::UsbBus::new(
            pac.USBCTRL_REGS,
            pac.USBCTRL_DPRAM,
            clocks.usb_clock,
            true,
            &mut pac.RESETS,
        ));


        // ---- Peripherals init

        let pins = Pins::new(
            pac.IO_BANK0,
            pac.PADS_BANK0,
            sio.gpio_bank0,
            &mut pac.RESETS,
        );

        let delay = cortex_m::delay::Delay::new(
            core.SYST,
            clocks.system_clock.freq().to_Hz()
        );

        let mut pwms = hal::pwm::Slices::new(pac.PWM, &mut pac.RESETS);

        Self {
            pins: pins,
            pwms: pwms,
            sys_clk: clocks.system_clock,

            usb_bus: usb,
            delay:   delay,
        }
    }
}
