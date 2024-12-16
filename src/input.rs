#[derive(Debug, Clone, Copy)]
pub enum Event {
    Press(u16),
    Release(u16),
}
