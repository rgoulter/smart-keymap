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
pub struct PressedKey {}

impl PressedKey {
    pub fn new() -> Self {
        Self {}
    }

    pub fn key_code(&self, key_def: &KeyDefinition) -> u8 {
        key_def.key_code()
    }
}
