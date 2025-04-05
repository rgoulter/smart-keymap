/// Types and initial data used for constructing [KEYMAP].
pub mod init {
    /// Config used by tap-hold keys.
    pub const CONFIG: crate::key::composite::Config = crate::key::composite::Config {
        chorded: crate::key::chorded::Config {
            timeout: 200,
            chords: [],
        },
        tap_hold: crate::key::tap_hold::Config {
            timeout: 200,
            interrupt_response: crate::key::tap_hold::InterruptResponse::Ignore,
        },
        ..crate::key::composite::DEFAULT_CONFIG
    };

    /// Number of layers supported by the [crate::key::layered] implementation.
    pub const LAYER_COUNT: usize = 1;

    /// The maximum number of chords.
    pub const MAX_CHORDS: usize = 0;

    pub use crate::key::composite::Context;

    pub use crate::key::composite::Event;

    pub use crate::key::composite::PendingKeyState;

    pub use crate::key::composite::KeyState;

    pub use crate::key::composite::Key;

    /// Initial [Context] value.
    pub const CONTEXT: Context = crate::key::composite::Context::from_config(CONFIG);

    crate::tuples::define_keys!(1);

    /// Alias for a [tuples] KeysN type, as generated by keymap.ncl.
    pub type KeyDefinitionsType = Keys1<
        crate::key::composite::Chorded<
            crate::key::composite::LayeredKey<
                crate::key::composite::TapHoldKey<crate::key::composite::BaseKey>,
            >,
        >,
        Context,
        Event,
        PendingKeyState,
        KeyState,
    >;

    /// A [tuples] KeysN value with keys, as generated by keymap.ncl.
    pub const KEY_DEFINITIONS: KeyDefinitionsType = Keys1::new((crate::key::composite::Chorded(
        crate::key::composite::LayeredKey::Layered(crate::key::layered::LayeredKey::new(
            crate::key::composite::TapHoldKey::TapHold(crate::key::tap_hold::Key::new(
                crate::key::composite::BaseKey::Keyboard(crate::key::keyboard::Key::new(4)),
                crate::key::composite::BaseKey::Keyboard(
                    crate::key::keyboard::Key::from_modifiers(
                        crate::key::KeyboardModifiers::LEFT_CTRL
                            .union(&crate::key::KeyboardModifiers::new()),
                    ),
                ),
            )),
            [Some(crate::key::composite::TapHoldKey::Pass(
                crate::key::composite::BaseKey::LayerModifier(
                    crate::key::layered::ModifierKey::Default(1),
                ),
            ))],
        )),
    ),));
}
