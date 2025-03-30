use hal::usb::UsbBus;
use rp2040_hal as hal;

pub use usbd_smart_keyboard::common::*;

/// Alias for [usbd_smart_keyboard::common::UsbClass] using the RP2040 [UsbBus].
pub type UsbClass = usbd_smart_keyboard::common::UsbClass<UsbBus>;

/// Alias for [usb_device::device::UsbDevice] using the RP2040 [UsbBus].
pub type UsbDevice = usb_device::device::UsbDevice<'static, UsbBus>;

/// Alias for [usbd_serial::SerialPort] using the RP2040 [UsbBus].
pub type UsbSerial = usbd_serial::SerialPort<'static, UsbBus>;

/// Enters the bootloader.
pub fn keyboard_reset_to_bootloader() {
    rp2040_hal::rom_data::reset_to_usb_boot(0, 0);
}
