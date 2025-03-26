use frunk::hlist::Selector;
use frunk::HList;

use usb_device::bus::UsbBus;
use usb_device::device::UsbDevice;
use usb_device::UsbError;

use usbd_human_interface_device::device::consumer::ConsumerControl;
use usbd_human_interface_device::device::keyboard::NKROBootKeyboard;
use usbd_human_interface_device::device::DeviceHList;
use usbd_human_interface_device::usb_class::UsbHidClass;

#[cfg(feature = "usbd-serial")]
use usbd_serial::SerialPort;

/// A USB Vendor ID.
pub const VID: u16 = 0xcafe;

/// A [usb_device::class::UsbClass] impl. with HID Keyboard, HID consumer devices.
pub type UsbClass<B> =
    UsbHidClass<'static, B, HList!(ConsumerControl<'static, B>, NKROBootKeyboard<'static, B>,)>;

/// Polls the given [UsbDevice] with the [UsbHidClass] that has a [NKROBootKeyboard]. (e.g. [UsbClass]).
pub fn usb_poll<B, D, Index>(
    usb_dev: &mut UsbDevice<'static, B>,
    #[cfg(feature = "usbd-serial")] usb_serial: &mut SerialPort<'static, B>,
    keyboard: &mut UsbHidClass<'static, B, D>,
) where
    B: UsbBus,
    D: DeviceHList<'static> + Selector<NKROBootKeyboard<'static, B>, Index>,
{
    if usb_dev.poll(&mut [
        keyboard,
        #[cfg(feature = "usbd-serial")]
        usb_serial,
    ]) {
        #[cfg(feature = "usbd-serial")]
        {
            let mut buf = [0u8; 64];
            match usb_serial.read(&mut buf) {
                Err(_e) => {}
                Ok(_count) => {}
            }
        }

        let interface = keyboard.device::<NKROBootKeyboard<'static, B>, _>();
        match interface.read_report() {
            Err(UsbError::WouldBlock) => {}
            Err(e) => {
                core::panic!("Failed to read keyboard report: {:?}", e)
            }
            Ok(_leds) => {}
        }
    }
}
