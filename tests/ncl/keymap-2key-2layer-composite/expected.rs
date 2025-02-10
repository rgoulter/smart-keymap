/// Types and initial data used for constructing [KEYMAP].
pub mod init {
    /// Config used by tap-hold keys.
    pub const CONFIG: crate::key::composite::Config = crate::key::composite::Config {
        tap_hold: crate::key::tap_hold::Config {
            timeout: 200,
            interrupt_response: crate::key::tap_hold::InterruptResponse::Ignore,
        },
    };

    /// Number of layers supported by the [crate::key::layered] implementation.
    pub const LAYER_COUNT: usize = 1;

    /// Alias for Context type; i.e. [crate::key::composite::Context] with generics.
    pub type Context = crate::key::composite::Context;

    /// Alias for Event type; i.e. [crate::key::composite::Event].
    pub type Event = crate::key::composite::Event;

    /// Alias for PressedKey type; i.e. [crate::key::composite::PressedKey].
    pub type PressedKey = crate::key::composite::PressedKey;

    /// Initial [Context] value.
    pub const CONTEXT: Context = crate::key::composite::Context {
        layer_context: crate::key::layered::Context {
            active_layers: [false; crate::key::layered::LAYER_COUNT],
        },
        tap_hold_context: crate::key::tap_hold::Context::from_config(CONFIG.tap_hold),
    };

    crate::tuples::define_keys!(2);

    /// Alias for a [tuples] KeysN type, as generated by keymap.ncl.
    pub type KeyDefinitionsType = Keys2<
        crate::key::composite::Layered<
            crate::key::composite::TapHoldKey<crate::key::composite::BaseKey>,
        >,
        crate::key::composite::LayeredKey<
            crate::key::composite::TapHold<crate::key::keyboard::Key>,
        >,
        Context,
        Event,
        PressedKey,
    >;

    /// A [tuples] KeysN value with keys, as generated by keymap.ncl.
    pub const KEY_DEFINITIONS: KeyDefinitionsType = Keys2::new((
        crate::key::composite::Layered(crate::key::composite::TapHoldKey::tap_hold(
            crate::key::tap_hold::Key::new(
                crate::key::composite::BaseKey::keyboard(crate::key::keyboard::Key::new(4)),
                crate::key::composite::BaseKey::layer_modifier(
                    crate::key::layered::ModifierKey::Hold(0),
                ),
            ),
        )),
        crate::key::composite::LayeredKey::layered(crate::key::layered::LayeredKey::new(
            crate::key::composite::TapHold(crate::key::keyboard::Key::new(5)),
            [Some(crate::key::composite::TapHold(
                crate::key::keyboard::Key::new(6),
            ))],
        )),
    ));
}
