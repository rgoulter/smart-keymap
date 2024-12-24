#[derive(Debug, Clone, Copy)]
pub struct Key(pub u8);

impl Key {
    pub fn key_code(&self) -> u8 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct Event();

#[derive(Debug, Clone, Copy)]
pub struct PressedKey {}

impl PressedKey {
    pub fn new() -> Self {
        Self {}
    }

    pub fn key_code(&self, key_def: &Key) -> u8 {
        key_def.key_code()
    }
}

impl Default for PressedKey {
    fn default() -> Self {
        Self::new()
    }
}
