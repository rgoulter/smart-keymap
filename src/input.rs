#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub enum Event {
    Press { keymap_index: u16 },
    Release { keymap_index: u16 },
    VirtualKeyPress { key_code: u8 },
    VirtualKeyRelease { key_code: u8 },
}

#[derive(Debug, Clone, Copy)]
pub enum PressedInput<K> {
    Key { keymap_index: u16, key: K },
    Virtual { key_code: u8 },
}

impl<K> PressedInput<K> {
    pub fn new_pressed_key(keymap_index: u16, key: K) -> Self {
        Self::Key { keymap_index, key }
    }
}
