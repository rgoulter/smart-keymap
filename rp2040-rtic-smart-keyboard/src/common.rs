use rp2040_hal as hal;

use frunk::HList;

use usbd_human_interface_device::device::consumer::ConsumerControl;
use usbd_human_interface_device::device::keyboard::NKROBootKeyboard;
use usbd_human_interface_device::usb_class::UsbHidClass;

use hal::usb::UsbBus;

/// A [usb_device::class::UsbClass] impl. with HID Keyboard, HID consumer devices.
pub type UsbClass = UsbHidClass<
    'static,
    UsbBus,
    HList!(
        ConsumerControl<'static, UsbBus>,
        NKROBootKeyboard<'static, UsbBus>,
    ),
>;

/// Alias for [usb_device::device::UsbDevice] using the RP2040 [UsbBus].
pub type UsbDevice = usb_device::device::UsbDevice<'static, UsbBus>;

/// Alias for [usbd_serial::SerialPort] using the RP2040 [UsbBus].
pub type UsbSerial = usbd_serial::SerialPort<'static, UsbBus>;

/// Enters the bootloader.
pub fn keyboard_reset_to_bootloader() {
    rp2040_hal::rom_data::reset_to_usb_boot(0, 0);
}
