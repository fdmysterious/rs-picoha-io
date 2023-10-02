/// GPIO control interface for picoha-io
///
/// - Florian Dupeyron <florian.dupeyron@mugcat.fr>
/// - July 2023

/// Possible errors for GPIO controller
#[derive(Debug)]
pub enum GpioCtrlError {
    InitError,
    PinConfigError,
    PinHalError,


    /// Invalid requested direction
    PinInvalidDir,

    /// Invalid pin index
    PinInvalidIndex,


    /// Requested pin is not in the given dir
    PinMismatchDir,
}

/// Current GPIO state
#[derive(Debug,Copy,Clone)]
pub enum PinDir {
    Unknown,
    PullUpInput,
    PullDownInput,
    NoPullInput,
    Output
}

impl PinDir {
    pub fn is_input_dir(&self) -> bool {
        match self {
            Self::PullUpInput | Self::PullDownInput | Self::NoPullInput => true,
            _                                                           => false
        }
    }

    pub fn is_output_dir(&self) -> bool {
        match self {
            Self::Output => true,
            _            => false
        }
    }
}

/// Pin value
#[derive(Debug,Copy,Clone)]
pub enum PinValue {
    Low,
    High
}


/// Represents pin index
pub type PinIndex = usize;


/// The GPIO controller controls the state of the GPIOs
pub trait GpioCtrl {
    /// Initializes the GPIO control block
    fn init(&mut self)   -> Result<(), GpioCtrlError>;

    /// Configures the requested pin as input or output
    fn dir_set(&mut self, idx: PinIndex, dir: PinDir) -> Result<(), GpioCtrlError>;

    /// Gets the current direction of the requested pin
    fn dir_get(&self, idx: PinIndex) -> Result<PinDir, GpioCtrlError>;

    /// Set the output value of the corresponding pin
    fn pin_write(&mut self, idx: PinIndex, value: PinValue) -> Result<(), GpioCtrlError>;

    /// Reads the corresponding pin
    fn pin_read(&self, idx: PinIndex) -> Result<PinValue, GpioCtrlError>;
}
