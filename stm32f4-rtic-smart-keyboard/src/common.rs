use frunk::HList;

use usb_device::UsbError;

use stm32f4xx_hal::otg_fs::UsbBusType;

use usbd_human_interface_device::device::consumer::ConsumerControl;
use usbd_human_interface_device::device::keyboard::NKROBootKeyboard;
use usbd_human_interface_device::usb_class::UsbHidClass;

/// A [usb_device::class::UsbClass] impl. with HID Keyboard, HID consumer devices.
pub type UsbClass = UsbHidClass<
    'static,
    UsbBusType,
    HList!(
        ConsumerControl<'static, UsbBusType>,
        NKROBootKeyboard<'static, UsbBusType>,
    ),
>;

pub type UsbDevice = usb_device::device::UsbDevice<'static, UsbBusType>;

/// Polls the given [UsbDevice] with the [UsbHidClass] that has a [NKROBootKeyboard]. (e.g. [UsbClass]).
pub fn usb_poll(usb_dev: &mut UsbDevice, keyboard: &mut UsbClass) {
    if usb_dev.poll(&mut [keyboard]) {
        let interface = keyboard.device::<NKROBootKeyboard<'static, UsbBusType>, _>();
        match interface.read_report() {
            Err(UsbError::WouldBlock) => {}
            Err(e) => {
                core::panic!("Failed to read keyboard report: {:?}", e)
            }
            Ok(_leds) => {}
        }
    }
}
