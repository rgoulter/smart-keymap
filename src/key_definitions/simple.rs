#[derive(Debug, Clone, Copy)]
pub struct KeyDefinition(pub u8);

impl KeyDefinition {
    pub fn key_code(&self) -> u8 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct Event();

#[derive(Debug, Clone, Copy)]
pub struct PressedKey {
    keymap_index: u16,
}

impl PressedKey {
    pub fn new(keymap_index: u16) -> Self {
        Self { keymap_index }
    }

    pub fn keymap_index(&self) -> u16 {
        self.keymap_index
    }

    pub fn key_code(&self, key_def: &KeyDefinition) -> u8 {
        key_def.key_code()
    }
}
