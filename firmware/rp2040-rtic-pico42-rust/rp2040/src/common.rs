use hal::usb::UsbBus;
use rp2040_hal as hal;

pub use rp2040_rtic_pico42_rust::common::*;

pub type UsbClass = rp2040_rtic_pico42_rust::common::UsbClass<UsbBus>;

pub type UsbDevice = usb_device::device::UsbDevice<'static, UsbBus>;

pub type UsbSerial = usbd_serial::SerialPort<'static, UsbBus>;
