use hal::usb::UsbBus;
use rp2040_hal as hal;

pub use usbd_smart_keyboard::common::*;

pub type UsbClass = usbd_smart_keyboard::common::UsbClass<UsbBus>;

pub type UsbDevice = usb_device::device::UsbDevice<'static, UsbBus>;

pub type UsbSerial = usbd_serial::SerialPort<'static, UsbBus>;
