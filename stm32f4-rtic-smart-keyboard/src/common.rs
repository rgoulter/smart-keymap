use frunk::HList;

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
