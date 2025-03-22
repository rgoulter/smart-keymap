#![doc = include_str!("doc_de_chorded.md")]

use core::fmt::Debug;

use serde::Deserialize;

use crate::{input, key};

pub use crate::init::MAX_CHORDS;

/// The maximum number of keys in a chord.
const MAX_CHORD_SIZE: usize = 2;

/// Chords are defined by an (unordered) set of indices into the keymap.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "std", serde(untagged))]
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

    /// Returns whether the chord is satisfied by the given indices.
    pub fn is_satisfied_by(&self, indices: &[u16]) -> bool {
        match self {
            ChordIndices::Chord2(i0, i1) => indices.contains(i0) && indices.contains(i1),
        }
    }
}

/// Chord definitions.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Config {
    /// The timeout (in number of ticks) for a chorded key to resolve.
    ///
    /// (Resolves as passthrough key if no chord is satisfied).
    #[serde(default = "default_timeout")]
    pub timeout: u16,

    /// The keymap chords.
    #[serde(default = "default_chords")]
    #[serde(deserialize_with = "deserialize_chords")]
    pub chords: [Option<ChordIndices>; MAX_CHORDS],
}

fn default_timeout() -> u16 {
    DEFAULT_CONFIG.timeout
}

fn default_chords() -> [Option<ChordIndices>; MAX_CHORDS] {
    DEFAULT_CONFIG.chords
}

/// Deserialize chords for [Config].
fn deserialize_chords<'de, D>(
    deserializer: D,
) -> Result<[Option<ChordIndices>; MAX_CHORDS], D::Error>
where
    D: serde::Deserializer<'de>,
{
    let mut v: heapless::Vec<Option<ChordIndices>, MAX_CHORDS> =
        Deserialize::deserialize(deserializer)?;

    while !v.is_full() {
        v.push(None).unwrap();
    }

    v.into_array()
        .map_err(|_| serde::de::Error::custom("unable to deserialize"))
}

/// Default config.
pub const DEFAULT_CONFIG: Config = Config {
    timeout: 200,
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

    pressed_indices: [Option<u16>; MAX_CHORD_SIZE * MAX_CHORDS],
}

/// Default context.
pub const DEFAULT_CONTEXT: Context = Context::from_config(DEFAULT_CONFIG);

impl Context {
    /// Constructs a context from the given config
    pub const fn from_config(config: Config) -> Context {
        let pressed_indices = [None; MAX_CHORD_SIZE * MAX_CHORDS];
        Context {
            config,
            pressed_indices,
        }
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

    // All the indices (including the given index) from chords which
    //  include the given index.
    //
    // e.g. for chords {01, 12},
    //  sibling_indices(0) -> [0, 1]
    //  sibling_indices(1) -> [0, 1, 2]
    fn sibling_indices(&self, index: u16) -> heapless::Vec<u16, { MAX_CHORD_SIZE * MAX_CHORDS }> {
        let mut res: heapless::Vec<u16, { MAX_CHORD_SIZE * MAX_CHORDS }> = heapless::Vec::new();

        let chords = self.chords_for_indices(&[index]);

        chords.iter().for_each(|ch| match ch {
            ChordIndices::Chord2(i0, i1) => {
                if let Err(pos) = res.binary_search(i0) {
                    res.insert(pos, *i0).unwrap();
                }
                if let Err(pos) = res.binary_search(i1) {
                    res.insert(pos, *i1).unwrap();
                }
            }
        });

        res
    }

    fn insert_pressed_index(&mut self, pos: usize, index: u16) {
        if self.pressed_indices.is_empty() {
            return;
        }

        let mut i = self.pressed_indices.len() - 1;
        while i > pos {
            self.pressed_indices[i] = self.pressed_indices[i - 1];
            i -= 1;
        }

        self.pressed_indices[pos] = Some(index);
    }

    fn remove_pressed_index(&mut self, pos: usize) {
        if self.pressed_indices.is_empty() {
            return;
        }

        let mut i = pos;
        while i < self.pressed_indices.len() - 1 {
            self.pressed_indices[i] = self.pressed_indices[i + 1];
            i += 1;
        }

        self.pressed_indices[self.pressed_indices.len() - 1] = None;
    }

    fn press_index(&mut self, index: u16) {
        match self
            .pressed_indices
            .binary_search_by_key(&index, |&k| k.unwrap_or(u16::MAX))
        {
            Ok(_) => {}
            Err(pos) => self.insert_pressed_index(pos, index),
        }
    }

    fn release_index(&mut self, index: u16) {
        if let Ok(pos) = self
            .pressed_indices
            .binary_search_by_key(&index, |&k| k.unwrap_or(u16::MAX))
        {
            self.remove_pressed_index(pos)
        }
    }

    fn pressed_indices(&self) -> heapless::Vec<u16, { MAX_CHORD_SIZE * MAX_CHORDS }> {
        self.pressed_indices.iter().filter_map(|&i| i).collect()
    }

    /// Updates the context for the given key event.
    pub fn handle_event(&mut self, event: key::Event<Event>) {
        match event {
            key::Event::Input(input::Event::Press { keymap_index }) => {
                self.press_index(keymap_index);
            }
            key::Event::Input(input::Event::Release { keymap_index }) => {
                self.release_index(keymap_index);
            }
            key::Event::Key {
                keymap_index,
                key_event: Event::ChordResolved(false),
            } => self.release_index(keymap_index),
            _ => {}
        }
    }
}

/// Primary Chorded key (with a passthrough key).
///
/// The primary key is the key with the lowest index in the chord,
///  and has the key used for the resolved chord.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Key<K> {
    /// The chorded key
    pub chord: K,
    /// The passthrough key
    pub passthrough: K,
}

impl<K: key::composite::ChordedNestable> Key<K> {
    /// Constructs new pressed key.
    pub fn new_pressed_key(
        &self,
        context: K::Context,
        key_path: key::KeyPath,
    ) -> (
        key::PressedKeyResult<K::PendingKeyState, K::KeyState>,
        key::PressedKeyEvents<K::Event>,
    ) {
        let keymap_index: u16 = key_path[0];
        let pks = PendingKeyState::new(context.into(), keymap_index);

        let chord_resolution = pks.check_resolution(context.into());

        if let Some(resolution) = chord_resolution {
            let (i, key) = match resolution {
                ChordResolution::Chord => (1, &self.chord),
                ChordResolution::Passthrough => (0, &self.passthrough),
            };

            let (pkr, pke) = key.new_pressed_key(context, key_path);
            // PRESSED KEY PATH: add Chord (0 = passthrough, 1 = chord)
            (pkr.add_path_item(i), pke)
        } else {
            let pk = key::PressedKeyResult::Pending(
                key_path,
                key::composite::PendingKeyState::Chorded(pks),
            );

            let timeout_ev = Event::Timeout;
            let ctx: Context = context.into();
            let sch_ev = key::ScheduledEvent::after(
                ctx.config.timeout,
                key::Event::key_event(keymap_index, timeout_ev),
            );
            let pke = key::PressedKeyEvents::scheduled_event(sch_ev.into_scheduled_event());

            (pk, pke)
        }
    }
}

impl<
        K: key::Key<
                PendingKeyState = key::composite::PendingKeyState,
                KeyState = key::composite::KeyState,
            > + Copy,
    > Key<K>
where
    K::Context: Into<Context>,
    K::Event: TryInto<Event>,
    K::Event: From<Event>,
{
    /// Constructs new chorded key.
    pub const fn new(chord: K, passthrough: K) -> Self {
        Key { chord, passthrough }
    }

    /// Maps the Key of the Key into a new type.
    pub fn map_key<T: key::Key + Copy>(self, f: fn(K) -> T) -> Key<T> {
        let Key { chord, passthrough } = self;
        Key {
            chord: f(chord),
            passthrough: f(passthrough),
        }
    }

    /// Maps the Key of the Key into a new type.
    pub fn into_key<T: key::Key + Copy>(self) -> Key<T>
    where
        K: Into<T>,
    {
        self.map_key(|k| k.into())
    }
}

/// Auxiliary chorded key (with a passthrough key).
///
/// The auxiliary keys are chorded keys,
///  but don't store the resolved chord key.
/// (i.e. After te primary chorded key, the remaining keys
///  in the chord are defined with auxiliary chorded keys).
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct AuxiliaryKey<K> {
    /// The passthrough key
    pub passthrough: K,
}

impl<K: key::composite::ChordedNestable> AuxiliaryKey<K> {
    /// Constructs new pressed key.
    pub fn new_pressed_key(
        &self,
        context: K::Context,
        key_path: key::KeyPath,
    ) -> (
        key::PressedKeyResult<K::PendingKeyState, K::KeyState>,
        key::PressedKeyEvents<K::Event>,
    ) {
        let keymap_index: u16 = key_path[0];
        let pks = PendingKeyState::new(context.into(), keymap_index);

        let chord_resolution = pks.check_resolution(context.into());

        if let Some(resolution) = chord_resolution {
            match resolution {
                ChordResolution::Chord => {
                    let pk = key::PressedKeyResult::Resolved(key::composite::KeyState::NoOp);
                    let pke = key::PressedKeyEvents::no_events();

                    (pk, pke)
                }
                // n.b. no need to add to key path; chorded aux_key only nests the passthrough key.
                ChordResolution::Passthrough => self.passthrough.new_pressed_key(context, key_path),
            }
        } else {
            let pk = key::PressedKeyResult::Pending(
                key_path,
                key::composite::PendingKeyState::Chorded(pks),
            );

            let timeout_ev = Event::Timeout;
            let ctx: Context = context.into();
            let sch_ev = key::ScheduledEvent::after(
                ctx.config.timeout,
                key::Event::key_event(keymap_index, timeout_ev),
            );
            let pke = key::PressedKeyEvents::scheduled_event(sch_ev.into_scheduled_event());

            (pk, pke)
        }
    }
}

impl<
        K: key::Key<
                PendingKeyState = key::composite::PendingKeyState,
                KeyState = key::composite::KeyState,
            > + Copy,
    > AuxiliaryKey<K>
where
    K::Context: Into<Context>,
    K::Event: TryInto<Event>,
    K::Event: From<Event>,
{
    /// Constructs new auxiliary chorded key.
    pub const fn new(passthrough: K) -> Self {
        AuxiliaryKey { passthrough }
    }
    /// Maps the Key of the Key into a new type.
    pub fn map_key<T: key::Key + Copy>(self, f: fn(K) -> T) -> AuxiliaryKey<T> {
        let AuxiliaryKey { passthrough } = self;
        AuxiliaryKey {
            passthrough: f(passthrough),
        }
    }

    /// Maps the Key of the Key into a new type.
    pub fn into_key<T: key::Key + Copy>(self) -> AuxiliaryKey<T>
    where
        K: Into<T>,
    {
        self.map_key(|k| k.into())
    }
}

/// Events for chorded keys.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum Event {
    /// The chorded key was resolved. (true if chord, false if passthrough)
    ChordResolved(bool),

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
pub enum ChordResolution {
    /// Resolved as chord.
    Chord,
    /// Resolved as passthrough key.
    Passthrough,
}

/// State for pressed keys.
#[derive(Debug, Clone, PartialEq)]
pub struct PendingKeyState {
    /// The keymap indices which have been pressed.
    pressed_indices: heapless::Vec<u16, { MAX_CHORD_SIZE }>,
    /// Whether the chord has been satisfied.
    satisfaction: ChordSatisfaction,
}

impl PendingKeyState {
    /// Constructs a new [PressedKeyState].
    pub fn new(context: Context, keymap_index: u16) -> Self {
        let sibling_indices = context.sibling_indices(keymap_index);
        let pressed_indices: heapless::Vec<u16, MAX_CHORD_SIZE> = context
            .pressed_indices()
            .iter()
            .filter(|i| sibling_indices.contains(i))
            .copied()
            .collect();

        Self {
            pressed_indices,
            satisfaction: ChordSatisfaction::Unsatisfied,
        }
    }

    fn check_resolution(&self, context: Context) -> Option<ChordResolution> {
        let chords = context.chords_for_indices(self.pressed_indices.as_slice());
        match chords.as_slice() {
            [ch] if ch.is_satisfied_by(&self.pressed_indices) => {
                // Only one chord is satisfied by pressed indices.
                //
                // This resolves the aux key.
                Some(ChordResolution::Chord)
            }
            [] => {
                // Otherwise, this key state resolves to "Passthrough",
                //  since it has been interrupted by an unrelated key press.
                Some(ChordResolution::Passthrough)
            }
            _ => {
                // Overlapping chords.
                None
            }
        }
    }

    /// Handle PKS for primary chorded key.
    pub fn handle_event(
        &mut self,
        context: Context,
        keymap_index: u16,
        event: key::Event<Event>,
    ) -> Option<ChordResolution> {
        match event {
            key::Event::Key {
                keymap_index: _ev_idx,
                key_event: Event::Timeout,
            } => {
                // Timed out before chord unambiguously resolved.
                //  So, the key behaves as the passthrough key.
                Some(ChordResolution::Passthrough)
            }
            key::Event::Input(input::Event::Press {
                keymap_index: pressed_keymap_index,
            }) => {
                // Another key was pressed.
                // Check if the other key belongs to this key's chord indices,

                let pos = self
                    .pressed_indices
                    .binary_search(&keymap_index)
                    .unwrap_or_else(|e| e);

                let push_res = self.pressed_indices.insert(pos, pressed_keymap_index);

                // pressed_indices has capacity of MAX_CHORD_SIZE.
                // pressed_indices will only be full without resolving
                // if multiple chords with max chord size
                //  having the same indices.
                if push_res.is_err() {
                    panic!();
                }

                self.check_resolution(context)
            }
            key::Event::Input(input::Event::Release {
                keymap_index: released_keymap_index,
            }) => {
                if released_keymap_index == keymap_index {
                    // This key state resolves to "Passthrough",
                    //  since it has been released before resolving as chord.
                    Some(ChordResolution::Passthrough)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use key::composite;
    use key::keyboard;

    use key::Context as _;

    #[test]
    fn test_timeout_resolves_unsatisfied_aux_state_as_passthrough_key() {
        // Assemble: an Auxilary chorded key, and its PKS.
        let context = key::composite::Context::default();
        let expected_key = keyboard::Key::new(0x04);
        let _chorded_key = AuxiliaryKey {
            passthrough: expected_key,
        };
        let keymap_index: u16 = 0;
        let mut pks: PendingKeyState = PendingKeyState::new(context.into(), keymap_index);

        // Act: handle a timeout ev.
        let timeout_ev = key::Event::key_event(keymap_index, Event::Timeout).into_key_event();
        let actual_res = pks.handle_event(context.into(), keymap_index, timeout_ev);

        // Assert
        let expected_res = Some(ChordResolution::Passthrough);
        assert_eq!(expected_res, actual_res);
    }

    // #[test]
    // fn test_timeout_resolves_satisfied_key_state_as_chord() {}

    #[test]
    fn test_press_non_chorded_key_resolves_aux_state_as_interrupted() {
        // Assemble: an Auxilary chorded key, and its PKS.
        let context = key::composite::Context::default();
        let expected_key = keyboard::Key::new(0x04);
        let _chorded_key = AuxiliaryKey {
            passthrough: expected_key,
        };
        let keymap_index: u16 = 0;
        let mut pks: PendingKeyState = PendingKeyState::new(context.into(), keymap_index);

        // Act: handle a key press, for an index that's not part of any chord.
        let non_chord_press = input::Event::Press { keymap_index: 9 }.into();
        let actual_res = pks.handle_event(context.into(), keymap_index, non_chord_press);

        // Assert
        let expected_res = Some(ChordResolution::Passthrough);
        assert_eq!(expected_res, actual_res);
    }

    // "unambiguous" in the sense that the chord
    // is not overlapped by another chord.
    // e.g. chord "01" is overlapped by chord "012",
    //  and "pressed {0, 1}" would be 'ambiguous';
    //  wheres "pressed {0, 1, 2}" would be 'unambiguous'.

    #[test]
    fn test_press_chorded_key_resolves_unambiguous_aux_state_as_chord() {
        // Assemble: an Auxilary chorded key, and its PKS, with chord 01.
        let mut context = key::composite::Context {
            chorded_context: Context::from_config(Config {
                chords: [Some(ChordIndices::Chord2(0, 1)), None, None, None],
                ..DEFAULT_CONFIG
            }),
            ..composite::DEFAULT_CONTEXT
        };
        let passthrough = keyboard::Key::new(0x04);
        let _chorded_key = AuxiliaryKey { passthrough };
        let keymap_index: u16 = 0;
        context.handle_event(key::Event::Input(input::Event::Press { keymap_index: 0 }));
        let mut pks: PendingKeyState = PendingKeyState::new(context.into(), keymap_index);

        // Act: handle a key press, for an index that completes (satisfies unambiguously) the chord.
        let chord_press = input::Event::Press { keymap_index: 1 }.into();
        let actual_res = pks.handle_event(context.into(), keymap_index, chord_press);

        // Assert: resolved aux key should have no events, should have (resolved) no output.
        let expected_res = Some(ChordResolution::Chord);
        assert_eq!(expected_res, actual_res);
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
        let _chorded_key = AuxiliaryKey {
            passthrough: expected_key,
        };
        let keymap_index: u16 = 0;
        let mut pks: PendingKeyState = PendingKeyState::new(context.into(), keymap_index);

        // Act: handle a key press, for an index that's not part of any chord.
        let chorded_key_release = input::Event::Release { keymap_index }.into();
        let actual_res = pks.handle_event(context.into(), keymap_index, chorded_key_release);

        // Assert
        let expected_res = Some(ChordResolution::Passthrough);
        assert_eq!(expected_res, actual_res);
    }
}
