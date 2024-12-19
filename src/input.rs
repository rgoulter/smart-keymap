#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub enum Event {
    Press { keymap_index: u16 },
    Release { keymap_index: u16 },
    VirtualKeyPress { keycode: u8 },
    VirtualKeyRelease { keycode: u8 },
    TapHoldTimeout { keymap_index: u16 },
}
