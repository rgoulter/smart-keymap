/// Types and initial data used for constructing [KEYMAP].
pub mod init {
    /// Config used by tap-hold keys.
    pub const CONFIG: crate::key::composite::Config = crate::key::composite::Config {
        chorded: crate::key::chorded::Config {
            chords: [],
            ..crate::key::chorded::DEFAULT_CONFIG
        },
        sticky: crate::key::sticky::DEFAULT_CONFIG,
        tap_hold: crate::key::tap_hold::DEFAULT_CONFIG,
        ..crate::key::composite::DEFAULT_CONFIG
    };

    /// Number of layers supported by the [crate::key::layered] implementation.
    pub const LAYER_COUNT: usize = 0;

    /// The maximum number of chords.
    pub const MAX_CHORDS: usize = 0;

    pub use crate::key::composite::Context;

    pub use crate::key::composite::Event;

    pub use crate::key::composite::PendingKeyState;

    pub use crate::key::composite::KeyState;

    pub use crate::key::composite::Key;

    /// Initial [Context] value.
    pub const CONTEXT: Context = crate::key::composite::Context::from_config(CONFIG);

    crate::tuples::define_keys!(60);

    /// Alias for a [tuples] KeysN type, as generated by keymap.ncl.
    pub type KeyDefinitionsType = Keys60<
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
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
    pub const KEY_DEFINITIONS: KeyDefinitionsType = Keys60::new((
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::new(53)),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::new(30)),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::new(31)),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::new(32)),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::new(33)),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::new(34)),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::new(35)),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::new(36)),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::new(37)),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::new(38)),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::new(39)),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::new(76)),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::new(43)),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::new(52)),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::new(54)),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::new(55)),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::new(19)),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::new(28)),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::new(9)),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::new(10)),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::new(6)),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::new(21)),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::new(15)),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::new(42)),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::new(41)),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::new(4)),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::new(18)),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::new(8)),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::new(24)),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::new(12)),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::new(7)),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::new(11)),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::new(23)),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::new(17)),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::new(22)),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::new(40)),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::from_modifiers(
                crate::key::KeyboardModifiers::LEFT_SHIFT
                    .union(&crate::key::KeyboardModifiers::new()),
            )),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::new(51)),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::new(20)),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::new(13)),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::new(14)),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::new(27)),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::new(5)),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::new(16)),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::new(26)),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::new(25)),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::new(29)),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::from_modifiers(
                crate::key::KeyboardModifiers::RIGHT_SHIFT
                    .union(&crate::key::KeyboardModifiers::new()),
            )),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::from_modifiers(
                crate::key::KeyboardModifiers::LEFT_CTRL
                    .union(&crate::key::KeyboardModifiers::new()),
            )),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::from_modifiers(
                crate::key::KeyboardModifiers::LEFT_GUI
                    .union(&crate::key::KeyboardModifiers::new()),
            )),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::from_modifiers(
                crate::key::KeyboardModifiers::LEFT_ALT
                    .union(&crate::key::KeyboardModifiers::new()),
            )),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::new(43)),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::new(41)),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::new(44)),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::new(42)),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::new(40)),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::new(76)),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::from_modifiers(
                crate::key::KeyboardModifiers::RIGHT_ALT
                    .union(&crate::key::KeyboardModifiers::new()),
            )),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::from_modifiers(
                crate::key::KeyboardModifiers::RIGHT_GUI
                    .union(&crate::key::KeyboardModifiers::new()),
            )),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::from_modifiers(
                crate::key::KeyboardModifiers::RIGHT_CTRL
                    .union(&crate::key::KeyboardModifiers::new()),
            )),
        )),
    ));
}
