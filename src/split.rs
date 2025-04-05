use postcard;
use serde::{Deserialize, Serialize};

use crate::input;

/// Size of message buffer for serializing and deserializing messages.
pub const BUFFER_SIZE: usize = 4;

/// Message sent from one split keyboard half to the other.
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
pub struct Message {
    /// The input event.
    pub input_event: input::Event,
}

impl Message {
    /// Create a new message.
    pub fn new(input_event: input::Event) -> Self {
        Self { input_event }
    }

    /// Serialize the message into a bytes.
    pub fn serialize(&self) -> [u8; BUFFER_SIZE] {
        let mut buf = [0u8; BUFFER_SIZE];
        postcard::to_slice_cobs(self, &mut buf).unwrap();
        buf
    }

    /// Serialize the message into a bytes.
    pub fn deserialize(bytes: &[u8]) -> postcard::Result<Message> {
        let mut buf = [0u8; BUFFER_SIZE];
        buf[..BUFFER_SIZE].copy_from_slice(&bytes[..BUFFER_SIZE]);
        postcard::from_bytes_cobs(&mut buf)
    }
}

/// Receives bytes from split transport, deserializes into messages.
/// Adds byte to the buffer and tries to deserialize a message.
pub fn receive_byte(buf: &mut [u8; BUFFER_SIZE], byte: u8) -> postcard::Result<Message> {
    buf.rotate_left(1);
    buf[BUFFER_SIZE - 1] = byte;
    Message::deserialize(buf)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ser_press() {
        // Assemble
        let input_event = input::Event::Press { keymap_index: 4 };
        let msg = Message { input_event };

        // Act
        let actual_ser = msg.serialize();

        // Assert
        assert_eq!(&actual_ser, &[0x01, 0x02, 0x04, 0x00]);
    }

    #[test]
    fn test_ser_release() {
        // Assemble
        let input_event = input::Event::Release { keymap_index: 4 };
        let msg = Message { input_event };

        // Act
        let actual_ser = msg.serialize();

        // Assert
        assert_eq!(&actual_ser, &[0x03, 0x01, 0x04, 0x00]);
    }

    #[test]
    fn test_deser_press() {
        // Assemble
        let ser = &[0x01, 0x02, 0x04, 0x00];

        // Act
        let actual_msg = Message::deserialize(ser).unwrap();

        // Assert
        let input_event = input::Event::Press { keymap_index: 4 };
        let expected_msg = Message { input_event };

        assert_eq!(&actual_msg, &expected_msg);
    }

    #[test]
    fn test_buf_receive_byte() {
        // Assemble
        let input_messages = &[
            Message::new(input::Event::Press { keymap_index: 4 }),
            Message::new(input::Event::Release { keymap_index: 5 }),
        ];
        let mut buf: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];

        // Act
        let mut actual_messages: Vec<Message> = Vec::new();
        for msg in input_messages {
            let ser = msg.serialize();
            for &byte in ser.iter() {
                if let Ok(msg) = receive_byte(&mut buf, byte) {
                    actual_messages.push(msg);
                }
            }
        }

        // Assert
        assert_eq!(input_messages, &actual_messages.as_slice());
    }
}
