use smart_keymap::input::Event;

/// Messages for the RTIC task which manages the Keyberon layout.
#[derive(Debug)]
pub enum LayoutMessage {
    /// Update the layout with this event.
    Event(Event),
    /// Tick the layout (and write report to the USB class).
    Tick,
}
