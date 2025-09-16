// #![doc = include_str!("doc_de_chorded.md")]

use core::fmt::Debug;
use core::ops::Index;

use serde::Deserialize;

use crate::{input, key, slice::Slice};

pub use crate::init::{MAX_CHORDS, MAX_CHORD_SIZE, MAX_OVERLAPPING_CHORD_SIZE};

/// Reference for a chorded key.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum Ref {
    /// Ref for [Key].
    Chorded(u8),
    /// Ref for [AuxiliaryKey].
    Auxiliary(u8),
}

/// A chord identifier.
pub type ChordId = u8;

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
    pub chords: Slice<ChordIndices, MAX_CHORDS>,
}

fn default_timeout() -> u16 {
    DEFAULT_CONFIG.timeout
}

/// Default config.
pub const DEFAULT_CONFIG: Config = Config {
    timeout: 200,
    chords: Slice::from_slice(&[]),
};

impl Default for Config {
    /// Returns the default context.
    fn default() -> Self {
        DEFAULT_CONFIG
    }
}

/// State for a key chord.
#[derive(Debug, Clone, PartialEq)]
pub struct ChordState {
    /// The chord index in the chorded config.
    pub index: usize,
    /// The chord's indices.
    pub chord: ChordIndices,
    /// Whether the chord is satisfied by the pressed indices.
    pub is_satisfied: bool,
}

/// Chord definitions.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Context {
    config: Config,
    pressed_indices: [Option<u16>; MAX_CHORD_SIZE * MAX_CHORDS],
    pressed_chords: [bool; MAX_CHORDS],
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
            pressed_chords: [false; MAX_CHORDS],
        }
    }

    fn pressed_chord_with_index(&self, keymap_index: u16) -> Option<ChordState> {
        self.pressed_chords
            .iter()
            .enumerate()
            .filter_map(|(index, &is_pressed)| {
                if is_pressed {
                    Some(ChordState {
                        index,
                        chord: self.config.chords[index],
                        is_satisfied: true,
                    })
                } else {
                    None
                }
            })
            .find(|ChordState { chord, .. }| chord.has_index(keymap_index))
    }

    // Span of indices of pressed chords.
    fn pressed_chords_indices_span(&self) -> heapless::Vec<u16, { MAX_CHORD_SIZE * MAX_CHORDS }> {
        let mut res: heapless::Vec<u16, { MAX_CHORD_SIZE * MAX_CHORDS }> = heapless::Vec::new();

        let pressed_chords =
            self.pressed_chords
                .iter()
                .enumerate()
                .filter_map(|(index, &is_pressed)| {
                    if is_pressed {
                        Some(&self.config.chords[index])
                    } else {
                        None
                    }
                });

        pressed_chords.for_each(|&chord| {
            for &i in chord.as_slice() {
                if let Err(pos) = res.binary_search(&i) {
                    res.insert(pos, i).unwrap();
                }
            }
        });

        res
    }

    /// Returns the chords for the given keymap index.
    ///
    /// - If a chord with that index is resolved as active, return a vec with only that chord.
    /// - Otherwise, return a vec with all the chords which include the keymap index
    ///   and could be satisfied. (i.e. chords which do not overlap with resolved active chords).
    pub fn chords_for_keymap_index(
        &self,
        keymap_index: u16,
    ) -> heapless::Vec<ChordState, { MAX_CHORDS }> {
        match self.pressed_chord_with_index(keymap_index) {
            Some(chord_state) => heapless::Vec::from_slice(&[chord_state]).unwrap(),
            None => {
                let chords_indices_span = self.pressed_chords_indices_span();
                self.config
                    .chords
                    .iter()
                    .enumerate()
                    // filter: satisfiable chords
                    .filter(|&(_index, chord)| chord.has_index(keymap_index))
                    .filter(|&(_index, chord)| {
                        // Filter out chords which overlap with resolved active chords.
                        chords_indices_span.is_empty()
                            || chord.indices.iter().all(|&i| {
                                // The chord index is not part of the pressed chords indices span.
                                chords_indices_span.binary_search(&i).is_err()
                            })
                    })
                    .map(|(index, &chord)| ChordState {
                        index,
                        chord,
                        is_satisfied: false,
                    })
                    .collect()
            }
        }
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

    /// Updates the context for the given key event.
    pub fn handle_event(&mut self, event: key::Event<Event>) {
        match event {
            key::Event::Input(input::Event::Press { keymap_index }) => {
                self.press_index(keymap_index);
            }
            key::Event::Input(input::Event::Release { keymap_index }) => {
                self.release_index(keymap_index);

                // Ensure every chord which includes this keymap index
                //  is not marked as 'pressed'.
                self.config
                    .chords
                    .iter()
                    .enumerate()
                    .for_each(|(chord_id, chord_indices)| {
                        if chord_indices.has_index(keymap_index) {
                            self.pressed_chords[chord_id] = false;
                        }
                    });
            }
            key::Event::Key {
                keymap_index: _,
                key_event: Event::ChordResolved(ChordResolution::Chord(chord_id)),
            } => {
                self.pressed_chords[chord_id as usize] = true;
            }
            _ => {}
        }
    }
}

/// Primary Chorded key (with a passthrough key).
///
/// The primary key is the key with the lowest index in the chord,
///  and has the key used for the resolved chord.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Key<R: Copy> {
    /// The chorded key
    pub chords: Slice<(ChordId, R), MAX_OVERLAPPING_CHORD_SIZE>,
    /// The passthrough key
    pub passthrough: R,
}

impl<R: Copy> Key<R> {
    /// Constructs new pressed key.
    pub fn new_pressed_key(
        &self,
        context: &Context,
        keymap_index: u16,
    ) -> (
        key::PressedKeyResult<R, PendingKeyState, KeyState>,
        key::KeyEvents<Event>,
    ) {
        let pks = PendingKeyState::new(context.into(), keymap_index);

        let chord_resolution = pks.check_resolution();

        if let PendingChordState::Resolved(resolution) = chord_resolution {
            let maybe_new_key_ref = match resolution {
                ChordResolution::Chord(resolved_chord_id) => {
                    // Whether the resolved chord is associated with this key.
                    // (i.e. the resolved chord's primary keymap index is this keymap index).
                    if let Some(resolved_chord_indices) =
                        context.config.chords.get(resolved_chord_id as usize)
                    {
                        if resolved_chord_indices.as_slice()[0] == keymap_index {
                            if let Some((_, new_key_ref)) = self
                                .chords
                                .iter()
                                .find(|(ch_id, _)| *ch_id == resolved_chord_id)
                            {
                                Some(*new_key_ref)
                            } else {
                                panic!("check_resolution has invalid chord id")
                            }
                        } else {
                            None
                        }
                    } else {
                        panic!("check_resolution has invalid chord id")
                    }
                }
                ChordResolution::Passthrough => Some(self.passthrough),
            };

            if let Some(new_key_ref) = maybe_new_key_ref {
                let pkr =
                    key::PressedKeyResult::NewPressedKey(key::NewPressedKey::key(new_key_ref));
                let pke = key::KeyEvents::no_events();

                (pkr, pke)
            } else {
                let pkr = key::PressedKeyResult::NewPressedKey(key::NewPressedKey::NoOp);
                let pke = key::KeyEvents::no_events();
                (pkr, pke)
            }
        } else {
            let pkr = key::PressedKeyResult::Pending(pks);

            let timeout_ev = Event::Timeout;
            let ctx: &Context = context.into();
            let sch_ev = key::ScheduledEvent::after(
                ctx.config.timeout,
                key::Event::key_event(keymap_index, timeout_ev),
            );
            let pke = key::KeyEvents::scheduled_event(sch_ev.into_scheduled_event());

            (pkr, pke)
        }
    }
}

impl<R: Copy> Key<R> {
    /// Constructs new chorded key.
    pub const fn new(chords: &[(ChordId, R)], passthrough: R) -> Self {
        let chords = Slice::from_slice(chords);
        Key {
            chords,
            passthrough,
        }
    }

    fn update_pending_state(
        &self,
        pending_state: &mut PendingKeyState,
        keymap_index: u16,
        context: &Context,
        event: key::Event<Event>,
    ) -> (Option<key::NewPressedKey<R>>, key::KeyEvents<Event>) {
        let ch_state = pending_state.handle_event(keymap_index, event);

        // Whether handling the event resulted in a chord resolution.
        if let Some(ch_state) = ch_state {
            let chorded_ctx: &Context = context.into();
            let maybe_new_key_ref = match ch_state {
                ChordResolution::Chord(resolved_chord_id) => {
                    // Whether the resolved chord is associated with this key.
                    // (i.e. the resolved chord's primary keymap index is this keymap index).
                    if let Some(resolved_chord_indices) =
                        chorded_ctx.config.chords.get(resolved_chord_id as usize)
                    {
                        if resolved_chord_indices.as_slice()[0] == keymap_index {
                            if let Some((_, key_ref)) = self
                                .chords
                                .iter()
                                .find(|(ch_id, _)| *ch_id == resolved_chord_id)
                            {
                                Some(*key_ref)
                            } else {
                                panic!("event's chord resolution has invalid chord id")
                            }
                        } else {
                            None
                        }
                    } else {
                        panic!("event's chord resolution has invalid chord id")
                    }
                }
                ChordResolution::Passthrough => Some(self.passthrough),
            };

            let ch_r_ev = Event::ChordResolved(ch_state);
            let sch_ev =
                key::ScheduledEvent::immediate(key::Event::key_event(keymap_index, ch_r_ev.into()));

            if let Some(new_key_ref) = maybe_new_key_ref {
                let pke = key::KeyEvents::scheduled_event(sch_ev);

                (Some(key::NewPressedKey::key(new_key_ref)), pke)
            } else {
                let pke = key::KeyEvents::scheduled_event(sch_ev);
                (Some(key::NewPressedKey::no_op()), pke)
            }
        } else {
            (None, key::KeyEvents::no_events())
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
pub struct AuxiliaryKey<R> {
    /// The passthrough key
    pub passthrough: R,
}

impl<R: Copy> AuxiliaryKey<R> {
    /// Constructs new pressed key.
    pub fn new_pressed_key(
        &self,
        context: &Context,
        keymap_index: u16,
    ) -> (
        key::PressedKeyResult<R, PendingKeyState, KeyState>,
        key::KeyEvents<Event>,
    ) {
        let pks = PendingKeyState::new(context.into(), keymap_index);

        let chord_resolution = pks.check_resolution();

        if let PendingChordState::Resolved(resolution) = chord_resolution {
            match resolution {
                ChordResolution::Chord(_resolved_chord_id) => {
                    let pkr = key::PressedKeyResult::NewPressedKey(key::NewPressedKey::NoOp);
                    let pke = key::KeyEvents::no_events();

                    (pkr, pke)
                }
                ChordResolution::Passthrough => {
                    let new_key_ref = self.passthrough;
                    let pkr =
                        key::PressedKeyResult::NewPressedKey(key::NewPressedKey::key(new_key_ref));
                    let pke = key::KeyEvents::no_events();
                    (pkr, pke)
                }
            }
        } else {
            let pkr = key::PressedKeyResult::Pending(pks);

            let timeout_ev = Event::Timeout;
            let ctx: &Context = context.into();
            let sch_ev = key::ScheduledEvent::after(
                ctx.config.timeout,
                key::Event::key_event(keymap_index, timeout_ev),
            );
            let pke = key::KeyEvents::scheduled_event(sch_ev.into_scheduled_event());

            (pkr, pke)
        }
    }
}

impl<R: Copy> AuxiliaryKey<R> {
    /// Constructs new auxiliary chorded key.
    pub const fn new(passthrough: R) -> Self {
        AuxiliaryKey { passthrough }
    }

    fn update_pending_state(
        &self,
        pending_state: &mut PendingKeyState,
        keymap_index: u16,
        event: key::Event<Event>,
    ) -> (Option<key::NewPressedKey<R>>, key::KeyEvents<Event>) {
        let ch_state = pending_state.handle_event(keymap_index, event);
        if let Some(ChordResolution::Passthrough) = ch_state {
            let ch_r_ev = Event::ChordResolved(ChordResolution::Passthrough);
            let sch_ev =
                key::ScheduledEvent::immediate(key::Event::key_event(keymap_index, ch_r_ev.into()));
            let pke = key::KeyEvents::scheduled_event(sch_ev);

            (Some(key::NewPressedKey::key(self.passthrough)), pke)
        } else if let Some(ChordResolution::Chord(resolved_chord_id)) = ch_state {
            let ch_r_ev = Event::ChordResolved(ChordResolution::Chord(resolved_chord_id));
            let pke = key::KeyEvents::event(key::Event::key_event(keymap_index, ch_r_ev.into()));

            (Some(key::NewPressedKey::no_op()), pke)
        } else {
            (None, key::KeyEvents::no_events())
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

/// Whether the pressed key state has resolved to a chord or not.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum ChordResolution {
    /// Resolved as chord.
    Chord(ChordId),
    /// Resolved as passthrough key.
    Passthrough,
}

/// The resolution state of a chorded key.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum PendingChordState {
    /// The key state is resolved (as chord or as passthrough).
    Resolved(ChordResolution),
    /// The key chord state is pending.
    ///
    /// The chord may be pending with the ID of a satisfied chord.
    Pending(Option<ChordId>),
}

/// State for pressed keys.
#[derive(Debug, Clone, PartialEq)]
pub struct PendingKeyState {
    /// The keymap indices which have been pressed while the key is pending.
    pressed_indices: heapless::Vec<u16, { MAX_CHORD_SIZE }>,
    /// The chords which this pending key could resolve to.
    possible_chords: heapless::Vec<ChordState, { MAX_CHORDS }>,
}

impl PendingKeyState {
    /// Constructs a new [PendingKeyState].
    pub fn new(context: &Context, keymap_index: u16) -> Self {
        let pressed_indices = heapless::Vec::from_slice(&[keymap_index]).unwrap();
        let possible_chords = context.chords_for_keymap_index(keymap_index);

        Self {
            pressed_indices,
            possible_chords,
        }
    }

    /// Finds the chord state amongst possible_chords which is satisfied (if it exists).
    fn satisfied_chord(&self) -> Option<&ChordState> {
        self.possible_chords
            .iter()
            .find(|&ChordState { is_satisfied, .. }| *is_satisfied)
    }

    fn check_resolution(&self) -> PendingChordState {
        match self.possible_chords.as_slice() {
            [ChordState {
                index,
                is_satisfied,
                ..
            }] if *is_satisfied => {
                // Only one chord is satisfied by pressed indices.
                //
                // This resolves the chord.
                PendingChordState::Resolved(ChordResolution::Chord(*index as u8))
            }
            [] => {
                // Otherwise, this key state resolves to "Passthrough",
                //  since it has been interrupted by an unrelated key press.
                PendingChordState::Resolved(ChordResolution::Passthrough)
            }
            satisfiable_chords => {
                // Overlapping chords.
                PendingChordState::Pending(
                    satisfiable_chords
                        .iter()
                        .find(|&ChordState { is_satisfied, .. }| *is_satisfied)
                        .map(|&ChordState { index, .. }| index as u8),
                )
            }
        }
    }

    /// Handle PKS for primary chorded key.
    pub fn handle_event(
        &mut self,
        keymap_index: u16,
        event: key::Event<Event>,
    ) -> Option<ChordResolution> {
        match event {
            key::Event::Key {
                keymap_index: _ev_idx,
                key_event: Event::Timeout,
            } => {
                // Timed out before chord unambiguously resolved.
                let maybe_satisfied_chord_id = self
                    .satisfied_chord()
                    .map(|chord_state| chord_state.index as u8);
                match maybe_satisfied_chord_id {
                    Some(satisfied_chord_id) => Some(ChordResolution::Chord(satisfied_chord_id)),
                    _ => Some(ChordResolution::Passthrough),
                }
            }
            key::Event::Input(input::Event::Press {
                keymap_index: pressed_keymap_index,
            }) => {
                // Another key was pressed.

                let maybe_satisfied_chord_id = self
                    .satisfied_chord()
                    .map(|chord_state| chord_state.index as u8);

                // Update pressed_indices.
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

                // Chords only remain possible if they have the pressed keymap index.
                self.possible_chords
                    .retain(|chord_state| chord_state.chord.has_index(pressed_keymap_index));

                // Re-evaluate the chord satisfaction states.
                for chord in self.possible_chords.iter_mut() {
                    chord.is_satisfied = chord.chord.is_satisfied_by(&self.pressed_indices);
                }

                let resolution = match self.check_resolution() {
                    PendingChordState::Resolved(resolution) => Some(resolution),
                    PendingChordState::Pending(_) => None,
                };

                // If the chord resolution is now passthrough (i.e. no chords satisfiable),
                // then resolve the chord with the satisfied chord.
                match (resolution, maybe_satisfied_chord_id) {
                    (Some(ChordResolution::Passthrough), Some(satisfied_chord_id)) => {
                        Some(ChordResolution::Chord(satisfied_chord_id))
                    }
                    _ => resolution,
                }
            }
            key::Event::Input(input::Event::Release {
                keymap_index: released_keymap_index,
            }) => {
                if released_keymap_index == keymap_index {
                    let maybe_satisfied_chord_id = self
                        .satisfied_chord()
                        .map(|chord_state| chord_state.index as u8);

                    match maybe_satisfied_chord_id {
                        Some(satisfied_chord_id) => {
                            Some(ChordResolution::Chord(satisfied_chord_id))
                        }

                        // This key state resolves to "Passthrough",
                        //  since it has been released before any chord is satisfied.
                        None => Some(ChordResolution::Passthrough),
                    }
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

/// Key state used by [System]. (Chorded keys do not have a key state).
#[derive(Debug, Clone, PartialEq)]
pub struct KeyState;

/// The [key::System] implementation for the chorded key system.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct System<
    R: Copy + Debug + PartialEq,
    Keys: Index<usize, Output = Key<R>>,
    AuxiliaryKeys: Index<usize, Output = AuxiliaryKey<R>>,
> {
    keys: Keys,
    auxiliary_keys: AuxiliaryKeys,
}

impl<
        R: Copy + Debug + PartialEq,
        Keys: Index<usize, Output = Key<R>>,
        AuxiliaryKeys: Index<usize, Output = AuxiliaryKey<R>>,
    > System<R, Keys, AuxiliaryKeys>
{
    /// Constructs a new [System] with the given key data.
    ///
    /// The key data is for keys with both key codes and modifiers.
    pub const fn new(keys: Keys, auxiliary_keys: AuxiliaryKeys) -> Self {
        Self {
            keys,
            auxiliary_keys,
        }
    }
}

impl<
        R: Copy + Debug + PartialEq,
        Keys: Debug + Index<usize, Output = Key<R>>,
        AuxiliaryKeys: Debug + Index<usize, Output = AuxiliaryKey<R>>,
    > key::System<R> for System<R, Keys, AuxiliaryKeys>
{
    type Ref = Ref;
    type Context = Context;
    type Event = Event;
    type PendingKeyState = PendingKeyState;
    type KeyState = KeyState;

    fn new_pressed_key(
        &self,
        keymap_index: u16,
        context: &Self::Context,
        key_ref: Ref,
    ) -> (
        key::PressedKeyResult<R, Self::PendingKeyState, Self::KeyState>,
        key::KeyEvents<Self::Event>,
    ) {
        match key_ref {
            Ref::Chorded(i) => self.keys[i as usize].new_pressed_key(context, keymap_index),
            Ref::Auxiliary(i) => {
                self.auxiliary_keys[i as usize].new_pressed_key(context, keymap_index)
            }
        }
    }

    fn update_pending_state(
        &self,
        pending_state: &mut Self::PendingKeyState,
        keymap_index: u16,
        context: &Self::Context,
        key_ref: Ref,
        event: key::Event<Self::Event>,
    ) -> (Option<key::NewPressedKey<R>>, key::KeyEvents<Self::Event>) {
        match key_ref {
            Ref::Chorded(i) => self.keys[i as usize].update_pending_state(
                pending_state,
                keymap_index,
                context,
                event,
            ),
            Ref::Auxiliary(i) => self.auxiliary_keys[i as usize].update_pending_state(
                pending_state,
                keymap_index,
                event,
            ),
        }
    }

    fn update_state(
        &self,
        _key_state: &mut Self::KeyState,
        _key_ref: &Self::Ref,
        _context: &Self::Context,
        _keymap_index: u16,
        _event: key::Event<Self::Event>,
    ) -> key::KeyEvents<Self::Event> {
        panic!()
    }

    fn key_output(
        &self,
        _key_ref: &Self::Ref,
        _key_state: &Self::KeyState,
    ) -> Option<key::KeyOutput> {
        panic!()
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     use key::composite;
//     use key::keyboard;

//     use key::Context as _;

//     #[test]
//     fn test_timeout_resolves_unsatisfied_aux_state_as_passthrough_key() {
//         // Assemble: an Auxilary chorded key, and its PKS.
//         let context = key::composite::Context::default();
//         let expected_key = keyboard::Key::new(0x04);
//         let _chorded_key = AuxiliaryKey {
//             passthrough: expected_key,
//         };
//         let keymap_index: u16 = 0;
//         let mut pks: PendingKeyState = PendingKeyState::new((&context).into(), keymap_index);

//         // Act: handle a timeout ev.
//         let timeout_ev = key::Event::key_event(keymap_index, Event::Timeout).into_key_event();
//         let actual_res = pks.handle_event(keymap_index, timeout_ev);

//         // Assert
//         let expected_res = Some(ChordResolution::Passthrough);
//         assert_eq!(expected_res, actual_res);
//     }

//     // #[test]
//     // fn test_timeout_resolves_satisfied_key_state_as_chord() {}

//     #[test]
//     fn test_press_non_chorded_key_resolves_aux_state_as_interrupted() {
//         // Assemble: an Auxilary chorded key, and its PKS.
//         let context = key::composite::Context::default();
//         let expected_key = keyboard::Key::new(0x04);
//         let _chorded_key = AuxiliaryKey {
//             passthrough: expected_key,
//         };
//         let keymap_index: u16 = 0;
//         let mut pks: PendingKeyState = PendingKeyState::new((&context).into(), keymap_index);

//         // Act: handle a key press, for an index that's not part of any chord.
//         let non_chord_press = input::Event::Press { keymap_index: 9 }.into();
//         let actual_res = pks.handle_event(keymap_index, non_chord_press);

//         // Assert
//         let expected_res = Some(ChordResolution::Passthrough);
//         assert_eq!(expected_res, actual_res);
//     }

//     // "unambiguous" in the sense that the chord
//     // is not overlapped by another chord.
//     // e.g. chord "01" is overlapped by chord "012",
//     //  and "pressed {0, 1}" would be 'ambiguous';
//     //  wheres "pressed {0, 1, 2}" would be 'unambiguous'.

//     #[test]
//     fn test_press_chorded_key_resolves_unambiguous_aux_state_as_chord() {
//         // Assemble: an Auxilary chorded key, and its PKS, with chord 01.
//         let mut context = key::composite::Context::from_config(composite::Config {
//             chorded: Config {
//                 chords: Slice::from_slice(&[ChordIndices::from_slice(&[0, 1])]),
//                 ..DEFAULT_CONFIG
//             },
//             ..composite::DEFAULT_CONFIG
//         });

//         let passthrough = keyboard::Key::new(0x04);
//         let _chorded_key = AuxiliaryKey { passthrough };
//         let keymap_index: u16 = 0;
//         context.handle_event(key::Event::Input(input::Event::Press { keymap_index: 0 }));
//         let mut pks: PendingKeyState = PendingKeyState::new((&context).into(), keymap_index);

//         // Act: handle a key press, for an index that completes (satisfies unambiguously) the chord.
//         let chord_press = input::Event::Press { keymap_index: 1 }.into();
//         let actual_res = pks.handle_event(keymap_index, chord_press);

//         // Assert: resolved aux key should have no events, should have (resolved) no output.
//         let expected_res = Some(ChordResolution::Chord(0));
//         assert_eq!(expected_res, actual_res);
//     }

//     // #[test]
//     // fn test_release_resolved_chord_state_releases_chord() {}

//     // This is better covered with an integration test.
//     // #[test]
//     // fn test_release_resolved_aux_passthrough_state_releases_passthrough_key() {}

//     #[test]
//     fn test_release_pending_aux_state_resolves_as_tapped_key() {
//         // Assemble: an Auxilary chorded key, and its PKS.
//         let context = key::composite::Context::default();
//         let expected_key = keyboard::Key::new(0x04);
//         let _chorded_key = AuxiliaryKey {
//             passthrough: expected_key,
//         };
//         let keymap_index: u16 = 0;
//         let mut pks: PendingKeyState = PendingKeyState::new((&context).into(), keymap_index);

//         // Act: handle a key press, for an index that's not part of any chord.
//         let chorded_key_release = input::Event::Release { keymap_index }.into();
//         let actual_res = pks.handle_event(keymap_index, chorded_key_release);

//         // Assert
//         let expected_res = Some(ChordResolution::Passthrough);
//         assert_eq!(expected_res, actual_res);
//     }
// }
