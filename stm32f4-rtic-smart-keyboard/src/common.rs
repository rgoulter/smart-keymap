use stm32f4xx_hal::otg_fs::UsbBusType;

pub use usbd_smart_keyboard::common::*;

pub type UsbClass = usbd_smart_keyboard::common::UsbClass<UsbBusType>;

pub type UsbDevice = usb_device::device::UsbDevice<'static, UsbBusType>;
