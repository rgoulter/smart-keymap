/// Callbacks for the keymap.
pub struct KeymapCallbacks {
    /// Callback for resetting keyboard state.
    pub reset: Option<fn() -> ()>,
    /// Callback for entering the bootloader.
    pub reset_to_bootloader: Option<fn() -> ()>,
}

/// The keyboard "backend", manages the keyboard from the events received
/// (presses/releases of coordinates on a keyboard layout).
/// through to listing HID scancodes to report using HIDs.
#[derive(Debug)]
pub struct KeyboardBackend {
    keymap: smart_keymap::init::Keymap,
    keymap_output: smart_keymap::keymap::KeymapOutput,
}

impl KeyboardBackend {
    /// Constructs a new keyboard backend.
    pub fn new(keymap: smart_keymap::init::Keymap) -> Self {
        Self {
            keymap,
            keymap_output: smart_keymap::keymap::KeymapOutput::default(),
        }
    }

    /// Set the keymap callbacks.
    pub fn set_callbacks(&mut self, callbacks: KeymapCallbacks) {
        use smart_keymap::keymap::KeymapCallback;
        if let Some(callback_fn) = callbacks.reset {
            self.keymap.set_callback(KeymapCallback::Reset, callback_fn);
        }
        if let Some(callback_fn) = callbacks.reset_to_bootloader {
            self.keymap
                .set_callback(KeymapCallback::ResetToBootloader, callback_fn);
        }
    }

    /// Register a key event.
    pub fn event(&mut self, event: smart_keymap::input::Event) {
        self.keymap.handle_input(event);
    }

    /// A time event.
    ///
    /// This method must be called regularly, typically every millisecond.
    ///
    /// Returns true if the pressed_key_codes have changed.
    pub fn tick(&mut self) -> bool {
        self.keymap.tick();

        let keymap_output = self.keymap.report_output();

        let old_keymap_output = core::mem::replace(&mut self.keymap_output, keymap_output);

        old_keymap_output != self.keymap_output
    }

    pub fn keymap_output(&self) -> &smart_keymap::keymap::KeymapOutput {
        &self.keymap_output
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
