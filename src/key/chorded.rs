#![doc = include_str!("doc_de_chorded.md")]

use core::fmt::Debug;

use serde::Deserialize;

use crate::{input, key, slice::Slice};

pub use crate::init::{MAX_CHORDS, MAX_CHORD_SIZE};

/// Chords are defined by an (unordered) set of keymap indices into the keymap.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
#[serde(from = "heapless::Vec<u16, MAX_CHORD_SIZE>")]
pub struct ChordIndices {
    /// A slice of keymap indices.
    indices: Slice<u16, MAX_CHORD_SIZE>,
}

impl ChordIndices {
    /// Constructs a new [ChordIndices] value from the given slice.
    ///
    /// The given slice must be less than [MAX_CHORD_SIZE] in length.
    pub const fn from_slice(indices: &[u16]) -> ChordIndices {
        ChordIndices {
            indices: Slice::from_slice(indices),
        }
    }

    /// The chord indices as a slice.
    pub const fn as_slice(&self) -> &[u16] {
        self.indices.as_slice()
    }

    /// Whether the given index is part of the chord.
    pub fn has_index(&self, index: u16) -> bool {
        self.as_slice().iter().any(|&i| i == index)
    }

    /// Whether the chord is satisfied by the given indices.
    pub fn is_satisfied_by(&self, indices: &[u16]) -> bool {
        self.as_slice().iter().all(|&i| indices.contains(&i))
    }
}

impl From<heapless::Vec<u16, MAX_CHORD_SIZE>> for ChordIndices {
    fn from(v: heapless::Vec<u16, MAX_CHORD_SIZE>) -> Self {
        ChordIndices::from_slice(&v)
    }
}

/// Chord definitions.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Config {
    /// The timeout (in number of milliseconds) for a chorded key to resolve.
    ///
    /// (Resolves as passthrough key if no chord is satisfied).
    #[serde(default = "default_timeout")]
    pub timeout: u16,

    /// The keymap chords.
    #[serde(deserialize_with = "deserialize_chords")]
    pub chords: [Option<ChordIndices>; MAX_CHORDS],
}

fn default_timeout() -> u16 {
    DEFAULT_CONFIG.timeout
}

/// Deserialize chords for [Config].
fn deserialize_chords<'de, D>(
    deserializer: D,
) -> Result<[Option<ChordIndices>; MAX_CHORDS], D::Error>
where
    D: serde::Deserializer<'de>,
{
    let mut v: heapless::Vec<Option<heapless::Vec<u16, MAX_CHORD_SIZE>>, MAX_CHORDS> =
        Deserialize::deserialize(deserializer)?;

    while !v.is_full() {
        v.push(None).unwrap();
    }

    v.into_array()
        .map(|a| a.map(|ch_op| ch_op.map(|ch| ch.into())))
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
    config: Config,

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
            .filter(|c| indices.iter().all(|&i| c.has_index(i)))
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

        chords.iter().for_each(|&ch| {
            for &i in ch.as_slice() {
                if let Err(pos) = res.binary_search(&i) {
                    res.insert(pos, i).unwrap();
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
                key_event: Event::ChordResolved(ChordResolution::Passthrough),
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

impl<K: key::Key> Key<K>
where
    for<'c> &'c K::Context: Into<&'c Context>,
    K::Event: TryInto<Event>,
    K::Event: From<Event>,
    K::PendingKeyState: From<PendingKeyState>,
    K::KeyState: From<key::NoOpKeyState<K::Context, K::Event>>,
{
    /// Constructs new pressed key.
    pub fn new_pressed_key(
        &self,
        context: &K::Context,
        key_path: key::KeyPath,
    ) -> (
        key::PressedKeyResult<K::PendingKeyState, K::KeyState>,
        key::KeyEvents<K::Event>,
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
            let pk = key::PressedKeyResult::Pending(key_path, pks.into());

            let timeout_ev = Event::Timeout;
            let ctx: &Context = context.into();
            let sch_ev = key::ScheduledEvent::after(
                ctx.config.timeout,
                key::Event::key_event(keymap_index, timeout_ev),
            );
            let pke = key::KeyEvents::scheduled_event(sch_ev.into_scheduled_event());

            (pk, pke)
        }
    }
}

impl<K> Key<K> {
    /// Constructs new chorded key.
    pub const fn new(chord: K, passthrough: K) -> Self {
        Key { chord, passthrough }
    }
}

impl<
        K: key::Key<
            Context = crate::init::Context,
            Event = crate::init::Event,
            PendingKeyState = crate::init::PendingKeyState,
            KeyState = crate::init::KeyState,
        >,
    > key::Key for Key<K>
{
    type Context = crate::init::Context;
    type Event = crate::init::Event;
    type PendingKeyState = crate::init::PendingKeyState;
    type KeyState = crate::init::KeyState;

    fn new_pressed_key(
        &self,
        context: &Self::Context,
        key_path: key::KeyPath,
    ) -> (
        key::PressedKeyResult<Self::PendingKeyState, Self::KeyState>,
        key::KeyEvents<Self::Event>,
    ) {
        self.new_pressed_key(context, key_path)
    }

    fn handle_event(
        &self,
        pending_state: &mut Self::PendingKeyState,
        context: &Self::Context,
        key_path: key::KeyPath,
        event: key::Event<Self::Event>,
    ) -> (
        Option<key::PressedKeyResult<Self::PendingKeyState, Self::KeyState>>,
        key::KeyEvents<Self::Event>,
    ) {
        let keymap_index: u16 = key_path[0];
        let ch_pks_res: Result<&mut PendingKeyState, _> = pending_state.try_into();
        if let Ok(ch_pks) = ch_pks_res {
            if let Ok(ch_ev) = event.try_into_key_event(|e| e.try_into()) {
                let ch_state = ch_pks.handle_event(context.into(), keymap_index, ch_ev);
                if let Some(ch_state) = ch_state {
                    let (i, nk) = match ch_state {
                        key::chorded::ChordResolution::Chord => (1, &self.chord),
                        key::chorded::ChordResolution::Passthrough => (0, &self.passthrough),
                    };
                    let (pkr, mut pke) = nk.new_pressed_key(context, key_path);
                    // PRESSED KEY PATH: add Chord (0 = passthrough, 1 = chord)
                    let pkr = pkr.add_path_item(i);

                    let ch_r_ev = key::chorded::Event::ChordResolved(ch_state);
                    let sch_ev = key::ScheduledEvent::immediate(key::Event::key_event(
                        keymap_index,
                        ch_r_ev.into(),
                    ));
                    pke.add_event(sch_ev);

                    (Some(pkr), pke)
                } else {
                    (None, key::KeyEvents::no_events())
                }
            } else {
                (None, key::KeyEvents::no_events())
            }
        } else {
            (None, key::KeyEvents::no_events())
        }
    }

    fn lookup(
        &self,
        path: &[u16],
    ) -> &dyn key::Key<
        Context = Self::Context,
        Event = Self::Event,
        PendingKeyState = Self::PendingKeyState,
        KeyState = Self::KeyState,
    > {
        match path {
            [] => self,
            // 0 = passthrough, 1 = chord
            [0, path @ ..] => self.passthrough.lookup(path),
            [1, path @ ..] => self.chord.lookup(path),
            _ => panic!(),
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
pub struct AuxiliaryKey<K> {
    /// The passthrough key
    pub passthrough: K,
}

impl<K: key::Key> AuxiliaryKey<K>
where
    for<'c> &'c K::Context: Into<&'c Context>,
    K::Event: TryInto<Event>,
    K::Event: From<Event>,
    K::PendingKeyState: From<PendingKeyState>,
    K::KeyState: From<key::NoOpKeyState<K::Context, K::Event>>,
{
    /// Constructs new pressed key.
    pub fn new_pressed_key(
        &self,
        context: &K::Context,
        key_path: key::KeyPath,
    ) -> (
        key::PressedKeyResult<K::PendingKeyState, K::KeyState>,
        key::KeyEvents<K::Event>,
    ) {
        let keymap_index: u16 = key_path[0];
        let pks = PendingKeyState::new(context.into(), keymap_index);

        let chord_resolution = pks.check_resolution(context.into());

        if let Some(resolution) = chord_resolution {
            match resolution {
                ChordResolution::Chord => {
                    let pk = key::PressedKeyResult::Resolved(key::NoOpKeyState::new().into());
                    let pke = key::KeyEvents::no_events();

                    (pk, pke)
                }
                // n.b. no need to add to key path; chorded aux_key only nests the passthrough key.
                ChordResolution::Passthrough => self.passthrough.new_pressed_key(context, key_path),
            }
        } else {
            let pk = key::PressedKeyResult::Pending(key_path, pks.into());

            let timeout_ev = Event::Timeout;
            let ctx: &Context = context.into();
            let sch_ev = key::ScheduledEvent::after(
                ctx.config.timeout,
                key::Event::key_event(keymap_index, timeout_ev),
            );
            let pke = key::KeyEvents::scheduled_event(sch_ev.into_scheduled_event());

            (pk, pke)
        }
    }
}

impl<K> AuxiliaryKey<K> {
    /// Constructs new auxiliary chorded key.
    pub const fn new(passthrough: K) -> Self {
        AuxiliaryKey { passthrough }
    }
}

impl<
        K: key::Key<
            Context = crate::init::Context,
            Event = crate::init::Event,
            PendingKeyState = crate::init::PendingKeyState,
            KeyState = crate::init::KeyState,
        >,
    > key::Key for AuxiliaryKey<K>
{
    type Context = crate::init::Context;
    type Event = crate::init::Event;
    type PendingKeyState = crate::init::PendingKeyState;
    type KeyState = crate::init::KeyState;

    fn new_pressed_key(
        &self,
        context: &Self::Context,
        key_path: key::KeyPath,
    ) -> (
        key::PressedKeyResult<Self::PendingKeyState, Self::KeyState>,
        key::KeyEvents<Self::Event>,
    ) {
        self.new_pressed_key(context, key_path)
    }

    fn handle_event(
        &self,
        pending_state: &mut Self::PendingKeyState,
        context: &Self::Context,
        key_path: key::KeyPath,
        event: key::Event<Self::Event>,
    ) -> (
        Option<key::PressedKeyResult<Self::PendingKeyState, Self::KeyState>>,
        key::KeyEvents<Self::Event>,
    ) {
        let keymap_index = key_path[0];
        let ch_pks_res: Result<&mut PendingKeyState, _> = pending_state.try_into();
        if let Ok(ch_pks) = ch_pks_res {
            if let Ok(ch_ev) = event.try_into_key_event(|e| e.try_into()) {
                let ch_state = ch_pks.handle_event(context.into(), keymap_index, ch_ev);
                if let Some(key::chorded::ChordResolution::Passthrough) = ch_state {
                    let nk = &self.passthrough;
                    let (pkr, mut pke) = nk.new_pressed_key(context, key_path);

                    // n.b. no need to add to key path; chorded aux_key only nests the passthrough key.

                    let ch_r_ev = key::chorded::Event::ChordResolved(
                        key::chorded::ChordResolution::Passthrough,
                    );
                    let sch_ev = key::ScheduledEvent::immediate(key::Event::key_event(
                        keymap_index,
                        ch_r_ev.into(),
                    ));
                    pke.add_event(sch_ev);

                    (Some(pkr), pke)
                } else if let Some(key::chorded::ChordResolution::Chord) = ch_state {
                    let ch_r_ev =
                        key::chorded::Event::ChordResolved(key::chorded::ChordResolution::Chord);
                    let pke =
                        key::KeyEvents::event(key::Event::key_event(keymap_index, ch_r_ev.into()));

                    (
                        Some(key::PressedKeyResult::Resolved(
                            key::NoOpKeyState::new().into(),
                        )),
                        pke,
                    )
                } else {
                    (None, key::KeyEvents::no_events())
                }
            } else {
                (None, key::KeyEvents::no_events())
            }
        } else {
            (None, key::KeyEvents::no_events())
        }
    }

    fn lookup(
        &self,
        path: &[u16],
    ) -> &dyn key::Key<
        Context = Self::Context,
        Event = Self::Event,
        PendingKeyState = Self::PendingKeyState,
        KeyState = Self::KeyState,
    > {
        match path {
            [] => self,
            _ => self.passthrough.lookup(path),
        }
    }
}

/// Events for chorded keys.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum Event {
    /// The chorded key was resolved.
    ChordResolved(ChordResolution),

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
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
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
    /// Constructs a new [PendingKeyState].
    pub fn new(context: &Context, keymap_index: u16) -> Self {
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

    fn check_resolution(&self, context: &Context) -> Option<ChordResolution> {
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
        context: &Context,
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
        let mut pks: PendingKeyState = PendingKeyState::new((&context).into(), keymap_index);

        // Act: handle a timeout ev.
        let timeout_ev = key::Event::key_event(keymap_index, Event::Timeout).into_key_event();
        let actual_res = pks.handle_event((&context).into(), keymap_index, timeout_ev);

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
        let mut pks: PendingKeyState = PendingKeyState::new((&context).into(), keymap_index);

        // Act: handle a key press, for an index that's not part of any chord.
        let non_chord_press = input::Event::Press { keymap_index: 9 }.into();
        let actual_res = pks.handle_event((&context).into(), keymap_index, non_chord_press);

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
        let mut context = key::composite::Context::from_config(composite::Config {
            chorded: Config {
                chords: [Some(ChordIndices::from_slice(&[0, 1])), None, None, None],
                ..DEFAULT_CONFIG
            },
            ..composite::DEFAULT_CONFIG
        });

        let passthrough = keyboard::Key::new(0x04);
        let _chorded_key = AuxiliaryKey { passthrough };
        let keymap_index: u16 = 0;
        context.handle_event(key::Event::Input(input::Event::Press { keymap_index: 0 }));
        let mut pks: PendingKeyState = PendingKeyState::new((&context).into(), keymap_index);

        // Act: handle a key press, for an index that completes (satisfies unambiguously) the chord.
        let chord_press = input::Event::Press { keymap_index: 1 }.into();
        let actual_res = pks.handle_event((&context).into(), keymap_index, chord_press);

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
        let mut pks: PendingKeyState = PendingKeyState::new((&context).into(), keymap_index);

        // Act: handle a key press, for an index that's not part of any chord.
        let chorded_key_release = input::Event::Release { keymap_index }.into();
        let actual_res = pks.handle_event((&context).into(), keymap_index, chorded_key_release);

        // Assert
        let expected_res = Some(ChordResolution::Passthrough);
        assert_eq!(expected_res, actual_res);
    }
}
