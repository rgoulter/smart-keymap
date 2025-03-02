use usbd_human_interface_device::page;
use usbd_human_interface_device::UsbHidError;

use crate::input::HIDReporter;

/// The keyboard "backend", manages the keyboard from the events received
/// (presses/releases of coordinates on a keyboard layout).
/// through to listing HID scancodes to report using HIDs.
#[derive(Debug)]
pub struct KeyboardBackend {
    keymap: smart_keymap::keymap::Keymap<smart_keymap::init::KeyDefinitionsType>,
    pressed_key_codes: heapless::Vec<page::Keyboard, 16>,
}

impl KeyboardBackend {
    /// Constructs a new keyboard backend.
    pub fn new(
        keymap: smart_keymap::keymap::Keymap<smart_keymap::init::KeyDefinitionsType>,
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

        let keymap_output = self.keymap.report_output();
        let pressed_keycodes = keymap_output.pressed_key_codes();
        self.pressed_key_codes = pressed_keycodes.iter().map(|&key| key.into()).collect();
    }

    /// Writes the HID keyboard and consumer reports from the smart keymap.
    pub fn write_reports<R, CE>(&mut self, hid_reporter: &mut R) -> Result<(), UsbHidError>
    where
        CE: core::fmt::Debug, // usb error
        R: HIDReporter<page::Keyboard, page::Consumer, CE>,
    {
        hid_reporter.write_keyboard_report(self.pressed_key_codes.clone())
    }

    pub fn pressed_key_codes(&self) -> &heapless::Vec<page::Keyboard, 16> {
        &self.pressed_key_codes
    }
}

/// Constructs a [smart_keymap::input::Event] from a [keyberon::layout::Event],
///  using a map from row, column to (maybe) keymap index.
pub fn keymap_index_of<const COLS: usize, const ROWS: usize>(
    indices: &[[Option<u16>; COLS]; ROWS],
    ev: keyberon::layout::Event,
) -> Option<smart_keymap::input::Event> {
    match ev {
        keyberon::layout::Event::Press(r, c) => indices[r as usize][c as usize]
            .map(|keymap_index| smart_keymap::input::Event::Press { keymap_index }),
        keyberon::layout::Event::Release(r, c) => indices[r as usize][c as usize]
            .map(|keymap_index| smart_keymap::input::Event::Release { keymap_index }),
    }
}
