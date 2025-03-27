use smart_keymap::input::Event;

/// Length of the buffer used to serialize [LayoutMessage]s.
pub const BUFFER_LENGTH: usize = 3;

/// Messages for the RTIC task which manages the Keyberon layout.
#[derive(Debug)]
pub enum LayoutMessage {
    /// Update the layout with this event.
    Event(Event),
    /// Tick the layout (and write report to the USB class).
    Tick,
}

/// Deserialise a slice of bytes into a keyberon Event.
///
/// The serialisation format must be compatible with
/// the serialisation format in `ser`.
#[allow(clippy::result_unit_err)]
pub fn de(bytes: &[u8]) -> Result<Event, ()> {
    match *bytes {
        [b'P', i, b'\n'] => Ok(Event::Press {
            keymap_index: i as u16,
        }),
        [b'R', i, b'\n'] => Ok(Event::Release {
            keymap_index: i as u16,
        }),
        _ => Err(()),
    }
}

/// Serialise a keyberon event into an array of bytes.
///
/// The serialisation format must be compatible with
/// the serialisation format in `de`.
pub fn ser(e: Event) -> [u8; BUFFER_LENGTH] {
    match e {
        Event::Press { keymap_index } => [b'P', keymap_index as u8, b'\n'],
        Event::Release { keymap_index } => [b'R', keymap_index as u8, b'\n'],
    }
}

/// Deserialise an array of bytes into maybe a Keyberon Event.
pub fn receive_byte(buf: &mut [u8; BUFFER_LENGTH], b: u8) -> Option<Event> {
    buf.rotate_left(1);
    buf[BUFFER_LENGTH - 1] = b;

    if buf[BUFFER_LENGTH - 1] == b'\n' {
        de(&buf[..]).ok()
    } else {
        None
    }
}
