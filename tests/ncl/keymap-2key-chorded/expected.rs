/// Types and initial data used for constructing [KEYMAP].
pub mod init {
    /// Config used by tap-hold keys.
    pub const CONFIG: crate::key::composite::Config = crate::key::composite::Config {
        chorded: crate::key::chorded::Config {
            timeout: 200,
            chords: [Some(crate::key::chorded::ChordIndices::Chord2(0, 1))],
        },
        tap_hold: crate::key::tap_hold::Config {
            timeout: 200,
            interrupt_response: crate::key::tap_hold::InterruptResponse::Ignore,
        },
        ..crate::key::composite::DEFAULT_CONFIG
    };

    /// Number of layers supported by the [crate::key::layered] implementation.
    pub const LAYER_COUNT: usize = 0;

    /// The maximum number of chords.
    pub const MAX_CHORDS: usize = 1;

    pub use crate::key::composite::Context;

    pub use crate::key::composite::Event;

    pub use crate::key::composite::PendingKeyState;

    pub use crate::key::composite::KeyState;

    pub use crate::key::composite::Key;

    /// Initial [Context] value.
    pub const CONTEXT: Context = crate::key::composite::Context::from_config(CONFIG);

    crate::tuples::define_keys!(2);

    /// Alias for a [tuples] KeysN type, as generated by keymap.ncl.
    pub type KeyDefinitionsType = Keys2<
        crate::key::chorded::Key<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::chorded::AuxiliaryKey<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        Context,
        Event,
        PendingKeyState,
        KeyState,
    >;

    /// Alias for the [keymap::Keymap] type.
    pub type Keymap =
        crate::keymap::Keymap<Context, Event, PendingKeyState, KeyState, KeyDefinitionsType>;

    /// A [tuples] KeysN value with keys, as generated by keymap.ncl.
    pub const KEY_DEFINITIONS: KeyDefinitionsType = Keys2::new((
        crate::key::chorded::Key::new(
            crate::key::composite::Layered(crate::key::composite::TapHold(
                crate::key::keyboard::Key::new(6),
            )),
            crate::key::composite::Layered(crate::key::composite::TapHold(
                crate::key::keyboard::Key::new(4),
            )),
        ),
        crate::key::chorded::AuxiliaryKey::new(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::new(5)),
        )),
    ));
}
