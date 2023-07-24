/// GPIO control interface for picoha-io
///
/// - Florian Dupeyron <florian.dupeyron@mugcat.fr>
/// - July 2023

use crate::PlatformError;

/// Possible errors for GPIO controller
#[derive(Debug)]
pub enum GpioCtrlError {
    InitError,
    PinConfigError,

    /// Wanted pin is not in the requested direction
    PinInvalidDir,

    /// Invalid pin index
    PinInvalidIndex,
}

/// Current GPIO state
pub enum PinDir {
    PullUpInput,
    PullDownInput,
    NoPullInput,
    Output
}

/// Pin value
pub enum PinValue {
    Low,
    High
}


/// Represents pin index
type PinIndex = u8;


/// Represents an output controllable pin
pub trait PinCtrlOut {
    /// Set output pin high
    fn high(&mut self) -> Result<(), GpioCtrlError>;

    /// Set output pin low
    fn low(&mut self) -> Result<(), GpioCtrlError>;
}


/// Represents an input controllable pin
pub trait PinCtrlIn {
    /// Read the pin value
    fn read(&self) -> Result<PinValue, GpioCtrlError>;
}


/// The GPIO controller controls the state of the GPIOs
pub trait GpioCtrl {
    /// Initializes the GPIO control block
    fn init(&self)   -> Result<(), GpioCtrlError>;

    /// Configures the requested pin as input or output
    fn dir_set(&mut self, idx: PinIndex, dir: PinDir);

    /// Gets a reference to the reqeusted input. Returns nothing if the
    /// pin is not configured as an output or the index is invalid.
    fn borrow_in(&self) -> Result<&dyn PinCtrlIn, GpioCtrlError>;

    /// Gets a mutable reference the requested output.
    fn borrow_out(&mut self) -> Result<&mut dyn PinCtrlOut, GpioCtrlError>;
}