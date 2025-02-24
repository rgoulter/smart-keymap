/// Chords (also known as "combos") are a smart keymap feature where
/// simultaneously pressing multiple keys results in the behaviour of another
/// key. e.g. pressing 'qw' keys together might send "Backspace"
///
/// In this module,
/// - [Context] stores the [Chord]s of the keymap.
///   - [Chord] is defined in terms of keymap indices.
/// - [Key] describes a key which is part of a chord
///   - This includes its 'passthrough key';
///     the behaviour of the key when the chord
///     didn't succeed.
///     (e.g. 'q' or 'w' for the chord 'qw').
/// - [PressedKeyState] manages chord resolution.
///   - If a timeout event is received for the key:
///     - if the PKS does not have a satisfied chord,
///       the PKS resolves to "Timed out",
///       and behaves as the passthrough key.
///     - if the PKS has a satisfied chord,
///       the PKS resolves to "Chorded key",
///       - and the primary PKS
///         (the PKS with lowest index in the chord)
///         behaves as the chorded key.
///   - When a key press is received for some keymap index:
///     - if the pressed key does not belong to any of  the chords related to
///        that pressed key,
///       the PKS resolves to "Interrupted",
///       and behaves as the passthrough key.
///     - Otherwise,
///        when a key press belonging to the chords related to the pressed key occurs:
///       - If a chord is fully satisfied (and there are no overlapping chords),
///         the PKS resolves to "Chorded key",
///         - and the primary PKS
///           (the PKS with lowest index in the chord)
///           behaves as the chorded key.
///       - If a chord is fully satisfied (and there are overlapping chords),
///         then the pressed key state remains pending,
///       - Otherwise, the PKS remains pending.
///   - When a key release event is received for some keymap index:
///     - If the PKS is resolved:
///       - as chorded: then the chorded key is released (if the PKS was pressing it),
///       - as passthrough key: then the pass through key is released.
///     - If the PKS is pending,
///       - then the passthrough key is 'tapped'.
use core::fmt::Debug;

use serde::Deserialize;

use crate::{input, key};

use key::PressedKey;

pub use crate::init::MAX_CHORDS;

/// The maximum number of keys in a chord.
const MAX_CHORD_SIZE: usize = 2;

/// Chords are defined by an (unordered) set of indices into the keymap.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum ChordIndices {
    /// A chord from two keys.
    Chord2(u16, u16),
}

impl ChordIndices {
    /// Returns whether the given index is part of the chord.
    pub fn has_index(&self, index: u16) -> bool {
        match self {
            ChordIndices::Chord2(i0, i1) => i0 == &index || i1 == &index,
        }
    }
}

/// Chord definitions.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Config {
    /// The keymap chords.
    pub chords: [Option<ChordIndices>; MAX_CHORDS],
}

/// Default config.
pub const DEFAULT_CONFIG: Config = Config {
    chords: [None; MAX_CHORDS],
};

impl Default for Config {
    /// Returns the default context.
    fn default() -> Self {
        DEFAULT_CONFIG
    }
}

/// Chord definitions.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Context {
    /// The config used by the context.
    pub config: Config,
}

/// Default context.
pub const DEFAULT_CONTEXT: Context = Context::from_config(DEFAULT_CONFIG);

impl Context {
    /// Constructs a context from the given config
    pub const fn from_config(config: Config) -> Context {
        Context { config }
    }

    /// Returns the chord indices for the given pressed indices.
    ///
    /// The returned vec is empty if any of the indices are not part of a chord.
    pub fn chords_for_indices(
        &self,
        indices: &[u16],
    ) -> heapless::Vec<ChordIndices, { MAX_CHORDS }> {
        self.config
            .chords
            .iter()
            .filter_map(|&c| c)
            .filter(|c| indices.iter().all(|i| c.has_index(*i)))
            .collect()
    }
}

impl key::Context for Context {
    type Event = Event;

    fn handle_event(&mut self, _event: Self::Event) {}
}

/// Primary Chorded key (with a passthrough key).
///
/// The primary key is the key with the lowest index in the chord,
///  and has the key used for the resolved chord.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Key<K> {
    chord: K,
    passthrough: K,
}

impl<K: key::Key + Copy> Key<K>
where
    K::Context: Into<Context>,
    K::Event: TryInto<Event>,
    K::Event: From<Event>,
{
    /// Constructs new pressed key.
    pub fn new_pressed_key(
        &self,
        keymap_index: u16,
    ) -> input::PressedKey<Self, PressedKeyState<K>> {
        input::PressedKey {
            keymap_index,
            key: *self,
            pressed_key_state: PressedKeyState::new(keymap_index),
        }
    }
}

/// Auxiliary chorded key (with a passthrough key).
///
/// The auxiliary keys are chorded keys,
///  but don't store the resolved chord key.
/// (i.e. After te primary chorded key, the remaining keys
///  in the chord are defined with auxiliary chorded keys).
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct AuxiliaryKey<K>(K);

impl<K: key::Key + Copy> AuxiliaryKey<K>
where
    K::Context: Into<Context>,
    K::Event: TryInto<Event>,
    K::Event: From<Event>,
{
    /// Constructs new pressed key.
    pub fn new_pressed_key(
        &self,
        keymap_index: u16,
    ) -> input::PressedKey<Self, PressedKeyState<K>> {
        input::PressedKey {
            keymap_index,
            key: *self,
            pressed_key_state: PressedKeyState::new(keymap_index),
        }
    }
}

/// Trait for [PressedKeyState].
pub trait ChordedKey<K: key::Key> {
    /// The chorded key's "passthrough" key.
    fn passthrough_key(&self) -> &K;

    /// The chorded key's "chorded" key.
    fn chorded_key(&self) -> Option<&K>;
}

impl<K: key::Key> ChordedKey<K> for AuxiliaryKey<K> {
    fn passthrough_key(&self) -> &K {
        &self.0
    }

    fn chorded_key(&self) -> Option<&K> {
        None
    }
}

/// Events for chorded keys.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum Event {
    /// Timed out waiting for chord to be satisfied.
    Timeout,
}

/// Whether enough keys have been pressed to satisfy a chord.
///
/// In the case of non-overlapping chords,
///  a satisfied chord is a resolved chord.
///
/// In the case of overlapping chords,
///  e.g. "chord 01" and "chord 012",
///  pressed "01" is satisfies "chord 01".
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChordSatisfaction {
    /// Status where not enough keys have been pressed to satisfy a chord.
    Unsatisfied,
    /// Status where enough keys have been pressed to satisfy a chord.
    Satisfied,
}

/// Whether the pressed key state has resolved to a chord or not.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChordResolution<PK> {
    /// Resolved as chord.
    Chord(Option<PK>),
    /// Resolved as passthrough key.
    Passthrough(PK),
}

/// State for pressed keys.
#[derive(Debug, PartialEq)]
pub enum PressedKeyState<K: key::Key> {
    /// Waiting for more [Event]s
    Pending {
        /// The keymap indices which have been pressed.
        pressed_indices: heapless::Vec<u16, { MAX_CHORD_SIZE }>,
        /// Whether the chord has been satisfied.
        satisfaction: ChordSatisfaction,
    },
    /// Chord resolved from [Event]s
    Resolved(ChordResolution<K::PressedKey>),
}

impl<K: key::Key> PressedKeyState<K>
where
    K::Context: Into<Context>,
    K::Event: TryInto<Event>,
    K::Event: From<Event>,
{
    /// Constructs a new [PressedKeyState].
    pub fn new(keymap_index: u16) -> Self {
        let mut pressed_indices = heapless::Vec::new();
        pressed_indices.push(keymap_index).unwrap();

        Self::Pending {
            pressed_indices,
            satisfaction: ChordSatisfaction::Unsatisfied,
        }
    }

    fn resolve_as_passthrough<C: ChordedKey<K>>(
        &mut self,
        context: K::Context,
        keymap_index: u16,
        key: &C,
    ) -> key::PressedKeyEvents<K::Event> {
        let k = key.passthrough_key();
        let (pk, n_pke) = k.new_pressed_key(context, keymap_index);
        *self = Self::Resolved(ChordResolution::Passthrough(pk));
        n_pke
    }

    fn resolve_as_chord<C: ChordedKey<K>>(
        &mut self,
        context: K::Context,
        keymap_index: u16,
        key: &C,
    ) -> key::PressedKeyEvents<K::Event> {
        if let Some(k) = key.chorded_key() {
            let (pk, n_pke) = k.new_pressed_key(context, keymap_index);
            *self = Self::Resolved(ChordResolution::Chord(Some(pk)));
            n_pke
        } else {
            *self = Self::Resolved(ChordResolution::Chord(None));
            key::PressedKeyEvents::no_events()
        }
    }

    /// Handle PKS for primary chorded key.
    pub fn handle_event_for<C: ChordedKey<K>>(
        &mut self,
        context: K::Context,
        keymap_index: u16,
        key: &C,
        event: key::Event<K::Event>,
    ) -> key::PressedKeyEvents<K::Event> {
        let mut pke = key::PressedKeyEvents::no_events();

        match self {
            Self::Pending {
                pressed_indices,
                satisfaction: _,
            } => {
                match event {
                    key::Event::Key {
                        keymap_index: _ev_idx,
                        key_event,
                    } => {
                        if let Ok(ev) = key_event.try_into() {
                            match ev {
                                Event::Timeout => {
                                    // Timed out before chord unambiguously resolved.
                                    //  So, the key behaves as the passthrough key.
                                    let n_pke =
                                        self.resolve_as_passthrough(context, keymap_index, key);
                                    pke.extend(n_pke);
                                }
                            }
                        }
                    }
                    key::Event::Input(input::Event::Press {
                        keymap_index: pressed_keymap_index,
                    }) => {
                        // Another key was pressed.
                        // Check if the other key belongs to this key's chord indices,

                        let pos = pressed_indices
                            .binary_search(&keymap_index)
                            .unwrap_or_else(|e| e);

                        let push_res = pressed_indices.insert(pos, pressed_keymap_index);

                        // pressed_indices has capacity of MAX_CHORD_SIZE.
                        // pressed_indices will only be full without resolving
                        // if multiple chords with max chord size
                        //  having the same indices.
                        if push_res.is_err() {
                            panic!();
                        }

                        let chords = context
                            .into()
                            .chords_for_indices(pressed_indices.as_slice());
                        match chords.as_slice() {
                            [_ch] => {
                                // Only one chord is satisfied by pressed indices.
                                //
                                // This resolves the aux key.
                                let n_pke = self.resolve_as_chord(context, keymap_index, key);
                                pke.extend(n_pke);
                            }
                            [] => {
                                // Otherwise, this key state resolves to "Passthrough",
                                //  since it has been interrupted by an unrelated key press.
                                let n_pke = self.resolve_as_passthrough(context, keymap_index, key);
                                pke.extend(n_pke);
                            }
                            _ => {
                                // Overlapping chords.
                            }
                        }
                    }
                    key::Event::Input(input::Event::Release {
                        keymap_index: released_keymap_index,
                    }) => {
                        if released_keymap_index == keymap_index {
                            // This key state resolves to "Passthrough",
                            //  since it has been released before resolving as chord.
                            let n_pke = self.resolve_as_passthrough(context, keymap_index, key);
                            pke.extend(n_pke);
                        }
                    }
                    _ => {}
                }
            }
            Self::Resolved(chord_res) => match chord_res {
                ChordResolution::Chord(Some(pk)) => {
                    let n_pke = pk.handle_event(context, event);
                    pke.extend(n_pke);
                }
                ChordResolution::Passthrough(pk) => {
                    let n_pke = pk.handle_event(context, event);
                    pke.extend(n_pke);
                }
                _ => {}
            },
        }

        pke
    }

    /// Key output from the pressed key state.
    pub fn key_output(&self) -> key::KeyOutputState {
        use key::PressedKey as _;

        match self {
            Self::Pending { .. } => key::KeyOutputState::pending(),
            Self::Resolved(ChordResolution::Chord(None)) => key::KeyOutputState::no_output(),
            Self::Resolved(ChordResolution::Chord(Some(pk))) => pk.key_output(),
            Self::Resolved(ChordResolution::Passthrough(pk)) => pk.key_output(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use key::composite;
    use key::keyboard;

    use key::PressedKey;

    #[test]
    fn test_timeout_resolves_unsatisfied_aux_state_as_passthrough_key() {
        // Assemble: an Auxilary chorded key, and its PKS.
        let context = key::composite::Context::default();
        let expected_key = keyboard::Key::new(0x04);
        let chorded_key = AuxiliaryKey(expected_key);
        let keymap_index: u16 = 0;
        let mut pks: PressedKeyState<keyboard::Key> = PressedKeyState::new(keymap_index);

        // Act: handle a timeout ev.
        let timeout_ev = key::Event::key_event(keymap_index, Event::Timeout).into_key_event();
        let actual_events = pks.handle_event_for(context, keymap_index, &chorded_key, timeout_ev);
        let actual_output = pks.key_output();

        // Assert: should have same events, and output as the aux's key's passthrough key.
        let (pk, expected_events) = key::Key::new_pressed_key(&expected_key, context, keymap_index);
        assert_eq!(expected_events, actual_events);
        let expected_output = pk.key_output();
        assert_eq!(expected_output, actual_output);
    }

    // #[test]
    // fn test_timeout_resolves_satisfied_key_state_as_chord() {}

    #[test]
    fn test_press_non_chorded_key_resolves_aux_state_as_interrupted() {
        // Assemble: an Auxilary chorded key, and its PKS.
        let context = key::composite::Context::default();
        let expected_key = keyboard::Key::new(0x04);
        let chorded_key = AuxiliaryKey(expected_key);
        let keymap_index: u16 = 0;
        let mut pks: PressedKeyState<keyboard::Key> = PressedKeyState::new(keymap_index);

        // Act: handle a key press, for an index that's not part of any chord.
        let non_chord_press = input::Event::Press { keymap_index: 9 }.into();
        let actual_events =
            pks.handle_event_for(context, keymap_index, &chorded_key, non_chord_press);
        let actual_output = pks.key_output();

        // Assert: should have same events, and output as the aux's key's passthrough key.
        let (pk, expected_events) = key::Key::new_pressed_key(&expected_key, context, keymap_index);
        assert_eq!(expected_events, actual_events);
        let expected_output = pk.key_output();
        assert_eq!(expected_output, actual_output);
    }

    // "unambiguous" in the sense that the chord
    // is not overlapped by another chord.
    // e.g. chord "01" is overlapped by chord "012",
    //  and "pressed {0, 1}" would be 'ambiguous';
    //  wheres "pressed {0, 1, 2}" would be 'unambiguous'.

    #[test]
    fn test_press_chorded_key_resolves_unambiguous_aux_state_as_chord() {
        // Assemble: an Auxilary chorded key, and its PKS, with chord 01.
        let context = key::composite::Context {
            chorded_context: Context::from_config(Config {
                chords: [Some(ChordIndices::Chord2(0, 1)), None, None, None],
                ..DEFAULT_CONFIG
            }),
            ..composite::DEFAULT_CONTEXT
        };
        let kbd_key = keyboard::Key::new(0x04);
        let chorded_key = AuxiliaryKey(kbd_key);
        let keymap_index: u16 = 0;
        let mut pks: PressedKeyState<keyboard::Key> = PressedKeyState::new(keymap_index);

        // Act: handle a key press, for an index that completes (satisfies unambiguously) the chord.
        let chord_press = input::Event::Press { keymap_index: 1 }.into();
        let actual_events = pks.handle_event_for(context, keymap_index, &chorded_key, chord_press);
        let actual_output = pks.key_output();

        // Assert: resolved aux key should have no events, should have (resolved) no output.
        let expected_events = key::PressedKeyEvents::no_events();
        assert_eq!(expected_events, actual_events);
        let expected_output = key::KeyOutputState::no_output();
        assert_eq!(expected_output, actual_output);
    }

    // #[test]
    // fn test_release_resolved_chord_state_releases_chord() {}

    // This is better covered with an integration test.
    // #[test]
    // fn test_release_resolved_aux_passthrough_state_releases_passthrough_key() {}

    #[test]
    fn test_release_pending_aux_state_resolves_as_tapped_key() {
        // Assemble: an Auxilary chorded key, and its PKS.
        let context = key::composite::Context::default();
        let expected_key = keyboard::Key::new(0x04);
        let chorded_key = AuxiliaryKey(expected_key);
        let keymap_index: u16 = 0;
        let mut pks: PressedKeyState<keyboard::Key> = PressedKeyState::new(keymap_index);

        // Act: handle a key press, for an index that's not part of any chord.
        let chorded_key_release = input::Event::Release { keymap_index }.into();
        let actual_events =
            pks.handle_event_for(context, keymap_index, &chorded_key, chorded_key_release);
        let actual_output = pks.key_output();

        // Assert: should have same events, and output as the aux's key's passthrough key.
        let (pk, expected_events) = key::Key::new_pressed_key(&expected_key, context, keymap_index);
        assert_eq!(expected_events, actual_events);
        let expected_output = pk.key_output();
        assert_eq!(expected_output, actual_output);
    }
}
