pub use usbd_smart_keyboard::app_prelude::*;

pub use stm32f4xx_hal as hal;

pub use hal::{
    gpio,
    gpio::GpioExt,
    otg_fs::{UsbBusType, USB},
    pac,
    prelude::*,
    rcc::RccExt,
    timer,
    timer::TimerExt,
};

pub use crate::app_init;
pub use crate::common::{UsbClass, UsbDevice};
