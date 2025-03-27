pub use stm32f4xx_hal as hal;

use hal::gpio;

/// An ID-erased PullUp input.
pub type Input = gpio::ErasedPin<gpio::Input>;
