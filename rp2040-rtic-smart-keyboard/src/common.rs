use rp2040_hal as hal;

use frunk::HList;

use usb_device::UsbError;

use usbd_human_interface_device::device::consumer::ConsumerControl;
use usbd_human_interface_device::device::keyboard::NKROBootKeyboard;
use usbd_human_interface_device::usb_class::UsbHidClass;

use usbd_serial::SerialPort;

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

/// A USB Vendor ID.
pub const VID: u16 = 0xcafe;

/// Enters the bootloader.
pub fn keyboard_reset_to_bootloader() {
    rp2040_hal::rom_data::reset_to_usb_boot(0, 0);
}

/// Polls the given [UsbDevice] with the [UsbHidClass] that has a [NKROBootKeyboard]. (e.g. [UsbClass]).
pub fn usb_poll(
    usb_dev: &mut UsbDevice,
    usb_serial: &mut SerialPort<'static, UsbBus>,
    keyboard: &mut UsbClass,
) {
    if usb_dev.poll(&mut [keyboard, usb_serial]) {
        let mut buf = [0u8; 64];
        match usb_serial.read(&mut buf) {
            Err(_e) => {}
            Ok(_count) => {}
        }

        let interface = keyboard.device::<NKROBootKeyboard<'static, UsbBus>, _>();
        match interface.read_report() {
            Err(UsbError::WouldBlock) => {}
            Err(e) => {
                core::panic!("Failed to read keyboard report: {:?}", e)
            }
            Ok(_leds) => {}
        }
    }
}
