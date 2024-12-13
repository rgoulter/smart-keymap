#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum KeyDefinition {
    Simple(u8),
    TapHold { tap: u8, hold: u8 },
}
