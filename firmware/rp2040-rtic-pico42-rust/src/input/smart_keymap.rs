use usbd_human_interface_device::page;

use crate::input::HIDReporter;

/// The keyboard "backend", manages the keyboard from the events received
/// (presses/releases of coordinates on a keyboard layout).
/// through to listing HID scancodes to report using HIDs.
pub struct KeyboardBackend {
    keymap: smart_keymap::keymap::Keymap<
        smart_keymap::init::KeyDefinitionsType,
        smart_keymap::init::LayersImpl
    >,
    pressed_key_codes: heapless::Vec<page::Keyboard, 16>,
}

impl KeyboardBackend {
    pub fn new(
        keymap: smart_keymap::keymap::Keymap<
            smart_keymap::init::KeyDefinitionsType,
            smart_keymap::init::LayersImpl
        >
    ) -> Self {
        Self {
            keymap,
            pressed_key_codes: heapless::Vec::new(),
        }
    }

    /// Register a key event.
    pub fn event(&mut self, event: smart_keymap::input::Event) {
        self.keymap.handle_input(event);
    }

    /// A time event.
    ///
    /// This method must be called regularly, typically every millisecond.
    pub fn tick(&mut self) {
        self.keymap.tick();

        let keymap_output = self.keymap.pressed_keys();
        let pressed_keycodes = keymap_output.pressed_key_codes();
        self.pressed_key_codes = pressed_keycodes.iter().map(|&key| key.into()).collect();
    }

    pub fn write_reports<R, KE, CE>(&mut self, hid_reporter: &mut R)
    where
        KE: core::fmt::Debug, // USBHID Keyboard Error
        CE: core::fmt::Debug, // usb error
        R: HIDReporter<page::Keyboard, page::Consumer, KE, CE>,
    {
        let _ = hid_reporter.write_keyboard_report(self.pressed_key_codes.clone());
    }
}
