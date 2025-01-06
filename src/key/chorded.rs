/// Chords (also known as "combos") are a smart keymap feature where
/// simultaneously pressing multiple keys results in the behaviour of another
/// key. e.g. pressing 'qw' keys together might send "Backspace"
///
/// In this module,
/// - [Context] stores the [Chord]s of the keymap.
///   - [Chord] is defined in terms of keymap indices.
/// - [Key] describes a key which is part of a chord
///   - This includes its 'pass-through key';
///     the behaviour of the key when the chord
///     didn't succeed.
///     (e.g. 'q' or 'w' for the chord 'qw').
/// - [PressedKeyState] manages chord resolution.
///   - If a timeout event is received for the key,
///     the PKS resolves to "Timed out",
///     and behaves as the pass-through key.
///   - If a key press occurs for keys unrelated
///     to the chords related to that pressed key,
///     the PKS resolves to "Interrupted",
///     and behaves as the pass-through key.
///   - Otherwise,
///     when a key press belonging to the chords related to the pressed key occurs:
///     - If the chord is fully satisfied (and there are no overlapping chords),
///       the pressed key state resolves, and behaves
///     - Otherwise, the PKS tracks the pressed key index.
use serde::Deserialize;

use crate::{input, key};

const MAX_CHORDS: usize = 16;

/// Chords are defined by an (unordered) set of indices into the keymap.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum ChordIndices {
    /// A chord from two keys.
    Chord2(u16, u16),
}

/// A chord.
///
/// Pressing all of the keys of the keymap indices
///  acts-as if pressing the key.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Chord {
    /// The indices of the keys that make up the chord.
    keymap_indices: ChordIndices,
    /// The 'key' the chord resolves to.
    key: u8,
}

/// Chord definitions.
///
/// ChN: number of chords
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Context {
    chords: [Chord; MAX_CHORDS],
}

impl key::Context for Context {
    type Event = Event;

    fn handle_event(&mut self, _event: Self::Event) {}
}

/// Chorded key (with a pass-through key).
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Key<K: key::Key>(K);

impl<K: key::Key> key::Key for Key<K> {
    type Context = Context;
    type ContextEvent = Event;
    type Event = Event;
    type PressedKeyState = PressedKeyState;

    fn new_pressed_key(
        &self,
        _context: &Self::Context,
        keymap_index: u16,
    ) -> (
        input::PressedKey<Self, Self::PressedKeyState>,
        key::PressedKeyEvents<Self::Event>,
    ) {
        (
            input::PressedKey {
                keymap_index,
                key: *self,
                pressed_key_state: PressedKeyState::new(keymap_index),
            },
            key::PressedKeyEvents::no_events(),
        )
    }
}

/// Events for chorded keys.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct Event {}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChordSatisfaction {
    Unsatisfied,
    Satisfied,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChordResolution {
    ChordedKey,
    Interrupted,
    TimedOut,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Status {
    /// Waiting for more [Event]s
    Pending(ChordSatisfaction),
    /// Chord resolved from [Event]s
    Resolved(ChordResolution),
}

/// State for pressed keys.
#[derive(Debug, PartialEq)]
pub struct PressedKeyState {
    status: Status,
    pressed_indices: heapless::Vec<u16, 8>,
}

/// Convenience type alias.
pub type PressedKey<K> = input::PressedKey<Key<K>, PressedKeyState>;

impl PressedKeyState {
    /// Constructs a new [PressedKeyState].
    fn new(keymap_index: u16) -> Self {
        let mut pressed_indices = heapless::Vec::new();
        pressed_indices.push(keymap_index).unwrap();
        Self {
            status: Status::Pending(ChordSatisfaction::Unsatisfied),
            pressed_indices,
        }
    }
}

impl<K: key::Key> key::PressedKeyState<Key<K>> for PressedKeyState {
    type Event = Event;

    fn handle_event_for(
        &mut self,
        _keymap_index: u16,
        _key: &Key<K>,
        _event: key::Event<Self::Event>,
    ) -> impl IntoIterator<Item = key::Event<Self::Event>> {
        None
    }

    fn key_output(&self, _key: &Key<K>) -> Option<key::KeyOutput> {
        None
    }
}
