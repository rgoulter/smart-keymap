use smart_keymap::input::Event;

/// Messages for the task-oriented frameworks which manages the
///  keyboard backend.
#[derive(Debug)]
pub enum BackendMessage {
    /// Update the layout with this event.
    Event(Event),
    /// Tick the layout (and write report to the USB class).
    Tick,
}
