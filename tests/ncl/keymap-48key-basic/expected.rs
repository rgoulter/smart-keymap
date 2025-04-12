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
    pub const LAYER_COUNT: usize = 3;

    /// The maximum number of chords.
    pub const MAX_CHORDS: usize = 0;

    pub use crate::key::composite::Context;

    pub use crate::key::composite::Event;

    pub use crate::key::composite::PendingKeyState;

    pub use crate::key::composite::KeyState;

    pub use crate::key::composite::Key;

    /// Initial [Context] value.
    pub const CONTEXT: Context = crate::key::composite::Context::from_config(CONFIG);

    crate::tuples::define_keys!(48);

    /// Alias for a [tuples] KeysN type, as generated by keymap.ncl.
    pub type KeyDefinitionsType = Keys48<
        crate::key::composite::Chorded<
            crate::key::composite::LayeredKey<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::LayeredKey<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::LayeredKey<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::LayeredKey<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::LayeredKey<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::LayeredKey<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::LayeredKey<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::LayeredKey<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::LayeredKey<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::LayeredKey<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::LayeredKey<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::LayeredKey<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::LayeredKey<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::LayeredKey<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::LayeredKey<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::LayeredKey<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::LayeredKey<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::LayeredKey<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::LayeredKey<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::LayeredKey<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::LayeredKey<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::LayeredKey<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::LayeredKey<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::LayeredKey<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::LayeredKey<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::LayeredKey<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::LayeredKey<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::LayeredKey<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::LayeredKey<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::LayeredKey<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::Layered<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::LayeredKey<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::LayeredKey<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::LayeredKey<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::LayeredKey<
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
            crate::key::composite::LayeredKey<
                crate::key::composite::TapHold<crate::key::layered::ModifierKey>,
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
            crate::key::composite::LayeredKey<
                crate::key::composite::TapHold<crate::key::layered::ModifierKey>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::LayeredKey<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::LayeredKey<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::LayeredKey<
                crate::key::composite::TapHold<crate::key::keyboard::Key>,
            >,
        >,
        crate::key::composite::Chorded<
            crate::key::composite::LayeredKey<
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
    pub const KEY_DEFINITIONS: KeyDefinitionsType = Keys48::new((
        crate::key::composite::Chorded(crate::key::composite::LayeredKey::Layered(
            crate::key::layered::LayeredKey::new(
                crate::key::composite::TapHold(crate::key::keyboard::Key::new(43)),
                [
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new(53),
                    )),
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new_with_modifiers(
                            53,
                            crate::key::KeyboardModifiers::from_byte(2),
                        ),
                    )),
                    None,
                ],
            ),
        )),
        crate::key::composite::Chorded(crate::key::composite::LayeredKey::Layered(
            crate::key::layered::LayeredKey::new(
                crate::key::composite::TapHold(crate::key::keyboard::Key::new(20)),
                [
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new(30),
                    )),
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new_with_modifiers(
                            30,
                            crate::key::KeyboardModifiers::from_byte(2),
                        ),
                    )),
                    None,
                ],
            ),
        )),
        crate::key::composite::Chorded(crate::key::composite::LayeredKey::Layered(
            crate::key::layered::LayeredKey::new(
                crate::key::composite::TapHold(crate::key::keyboard::Key::new(26)),
                [
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new(31),
                    )),
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new_with_modifiers(
                            31,
                            crate::key::KeyboardModifiers::from_byte(2),
                        ),
                    )),
                    None,
                ],
            ),
        )),
        crate::key::composite::Chorded(crate::key::composite::LayeredKey::Layered(
            crate::key::layered::LayeredKey::new(
                crate::key::composite::TapHold(crate::key::keyboard::Key::new(8)),
                [
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new(32),
                    )),
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new_with_modifiers(
                            32,
                            crate::key::KeyboardModifiers::from_byte(2),
                        ),
                    )),
                    None,
                ],
            ),
        )),
        crate::key::composite::Chorded(crate::key::composite::LayeredKey::Layered(
            crate::key::layered::LayeredKey::new(
                crate::key::composite::TapHold(crate::key::keyboard::Key::new(21)),
                [
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new(33),
                    )),
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new_with_modifiers(
                            33,
                            crate::key::KeyboardModifiers::from_byte(2),
                        ),
                    )),
                    None,
                ],
            ),
        )),
        crate::key::composite::Chorded(crate::key::composite::LayeredKey::Layered(
            crate::key::layered::LayeredKey::new(
                crate::key::composite::TapHold(crate::key::keyboard::Key::new(23)),
                [
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new(34),
                    )),
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new_with_modifiers(
                            34,
                            crate::key::KeyboardModifiers::from_byte(2),
                        ),
                    )),
                    None,
                ],
            ),
        )),
        crate::key::composite::Chorded(crate::key::composite::LayeredKey::Layered(
            crate::key::layered::LayeredKey::new(
                crate::key::composite::TapHold(crate::key::keyboard::Key::new(28)),
                [
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new(35),
                    )),
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new_with_modifiers(
                            35,
                            crate::key::KeyboardModifiers::from_byte(2),
                        ),
                    )),
                    None,
                ],
            ),
        )),
        crate::key::composite::Chorded(crate::key::composite::LayeredKey::Layered(
            crate::key::layered::LayeredKey::new(
                crate::key::composite::TapHold(crate::key::keyboard::Key::new(24)),
                [
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new(36),
                    )),
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new_with_modifiers(
                            36,
                            crate::key::KeyboardModifiers::from_byte(2),
                        ),
                    )),
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new(70),
                    )),
                ],
            ),
        )),
        crate::key::composite::Chorded(crate::key::composite::LayeredKey::Layered(
            crate::key::layered::LayeredKey::new(
                crate::key::composite::TapHold(crate::key::keyboard::Key::new(12)),
                [
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new(37),
                    )),
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new_with_modifiers(
                            37,
                            crate::key::KeyboardModifiers::from_byte(2),
                        ),
                    )),
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new(71),
                    )),
                ],
            ),
        )),
        crate::key::composite::Chorded(crate::key::composite::LayeredKey::Layered(
            crate::key::layered::LayeredKey::new(
                crate::key::composite::TapHold(crate::key::keyboard::Key::new(18)),
                [
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new(38),
                    )),
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new_with_modifiers(
                            38,
                            crate::key::KeyboardModifiers::from_byte(2),
                        ),
                    )),
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new(72),
                    )),
                ],
            ),
        )),
        crate::key::composite::Chorded(crate::key::composite::LayeredKey::Layered(
            crate::key::layered::LayeredKey::new(
                crate::key::composite::TapHold(crate::key::keyboard::Key::new(19)),
                [
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new(39),
                    )),
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new_with_modifiers(
                            39,
                            crate::key::KeyboardModifiers::from_byte(2),
                        ),
                    )),
                    None,
                ],
            ),
        )),
        crate::key::composite::Chorded(crate::key::composite::LayeredKey::Layered(
            crate::key::layered::LayeredKey::new(
                crate::key::composite::TapHold(crate::key::keyboard::Key::new(42)),
                [
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new(49),
                    )),
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new_with_modifiers(
                            49,
                            crate::key::KeyboardModifiers::from_byte(2),
                        ),
                    )),
                    None,
                ],
            ),
        )),
        crate::key::composite::Chorded(crate::key::composite::LayeredKey::Layered(
            crate::key::layered::LayeredKey::new(
                crate::key::composite::TapHold(crate::key::keyboard::Key::new(41)),
                [
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new(76),
                    )),
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new(73),
                    )),
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new(57),
                    )),
                ],
            ),
        )),
        crate::key::composite::Chorded(crate::key::composite::LayeredKey::Layered(
            crate::key::layered::LayeredKey::new(
                crate::key::composite::TapHold(crate::key::keyboard::Key::new(4)),
                [
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new(58),
                    )),
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new(58),
                    )),
                    None,
                ],
            ),
        )),
        crate::key::composite::Chorded(crate::key::composite::LayeredKey::Layered(
            crate::key::layered::LayeredKey::new(
                crate::key::composite::TapHold(crate::key::keyboard::Key::new(22)),
                [
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new(59),
                    )),
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new(59),
                    )),
                    None,
                ],
            ),
        )),
        crate::key::composite::Chorded(crate::key::composite::LayeredKey::Layered(
            crate::key::layered::LayeredKey::new(
                crate::key::composite::TapHold(crate::key::keyboard::Key::new(7)),
                [
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new(60),
                    )),
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new(60),
                    )),
                    None,
                ],
            ),
        )),
        crate::key::composite::Chorded(crate::key::composite::LayeredKey::Layered(
            crate::key::layered::LayeredKey::new(
                crate::key::composite::TapHold(crate::key::keyboard::Key::new(9)),
                [
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new(61),
                    )),
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new(61),
                    )),
                    None,
                ],
            ),
        )),
        crate::key::composite::Chorded(crate::key::composite::LayeredKey::Layered(
            crate::key::layered::LayeredKey::new(
                crate::key::composite::TapHold(crate::key::keyboard::Key::new(10)),
                [
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new(62),
                    )),
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new(62),
                    )),
                    None,
                ],
            ),
        )),
        crate::key::composite::Chorded(crate::key::composite::LayeredKey::Layered(
            crate::key::layered::LayeredKey::new(
                crate::key::composite::TapHold(crate::key::keyboard::Key::new(11)),
                [
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new(63),
                    )),
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new(63),
                    )),
                    None,
                ],
            ),
        )),
        crate::key::composite::Chorded(crate::key::composite::LayeredKey::Layered(
            crate::key::layered::LayeredKey::new(
                crate::key::composite::TapHold(crate::key::keyboard::Key::new(13)),
                [
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new(45),
                    )),
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new_with_modifiers(
                            45,
                            crate::key::KeyboardModifiers::from_byte(2),
                        ),
                    )),
                    None,
                ],
            ),
        )),
        crate::key::composite::Chorded(crate::key::composite::LayeredKey::Layered(
            crate::key::layered::LayeredKey::new(
                crate::key::composite::TapHold(crate::key::keyboard::Key::new(14)),
                [
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new(46),
                    )),
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new_with_modifiers(
                            46,
                            crate::key::KeyboardModifiers::from_byte(2),
                        ),
                    )),
                    None,
                ],
            ),
        )),
        crate::key::composite::Chorded(crate::key::composite::LayeredKey::Layered(
            crate::key::layered::LayeredKey::new(
                crate::key::composite::TapHold(crate::key::keyboard::Key::new(15)),
                [
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new(47),
                    )),
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new_with_modifiers(
                            47,
                            crate::key::KeyboardModifiers::from_byte(2),
                        ),
                    )),
                    None,
                ],
            ),
        )),
        crate::key::composite::Chorded(crate::key::composite::LayeredKey::Layered(
            crate::key::layered::LayeredKey::new(
                crate::key::composite::TapHold(crate::key::keyboard::Key::new(51)),
                [
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new(48),
                    )),
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new_with_modifiers(
                            48,
                            crate::key::KeyboardModifiers::from_byte(2),
                        ),
                    )),
                    None,
                ],
            ),
        )),
        crate::key::composite::Chorded(crate::key::composite::LayeredKey::Layered(
            crate::key::layered::LayeredKey::new(
                crate::key::composite::TapHold(crate::key::keyboard::Key::new(52)),
                [
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new(56),
                    )),
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new_with_modifiers(
                            56,
                            crate::key::KeyboardModifiers::from_byte(2),
                        ),
                    )),
                    None,
                ],
            ),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::from_modifiers(
                crate::key::KeyboardModifiers::from_byte(2),
            )),
        )),
        crate::key::composite::Chorded(crate::key::composite::LayeredKey::Layered(
            crate::key::layered::LayeredKey::new(
                crate::key::composite::TapHold(crate::key::keyboard::Key::new(29)),
                [
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new(64),
                    )),
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new(64),
                    )),
                    None,
                ],
            ),
        )),
        crate::key::composite::Chorded(crate::key::composite::LayeredKey::Layered(
            crate::key::layered::LayeredKey::new(
                crate::key::composite::TapHold(crate::key::keyboard::Key::new(27)),
                [
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new(65),
                    )),
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new(65),
                    )),
                    None,
                ],
            ),
        )),
        crate::key::composite::Chorded(crate::key::composite::LayeredKey::Layered(
            crate::key::layered::LayeredKey::new(
                crate::key::composite::TapHold(crate::key::keyboard::Key::new(6)),
                [
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new(66),
                    )),
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new(66),
                    )),
                    None,
                ],
            ),
        )),
        crate::key::composite::Chorded(crate::key::composite::LayeredKey::Layered(
            crate::key::layered::LayeredKey::new(
                crate::key::composite::TapHold(crate::key::keyboard::Key::new(25)),
                [
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new(67),
                    )),
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new(67),
                    )),
                    None,
                ],
            ),
        )),
        crate::key::composite::Chorded(crate::key::composite::LayeredKey::Layered(
            crate::key::layered::LayeredKey::new(
                crate::key::composite::TapHold(crate::key::keyboard::Key::new(5)),
                [
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new(68),
                    )),
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new(68),
                    )),
                    None,
                ],
            ),
        )),
        crate::key::composite::Chorded(crate::key::composite::LayeredKey::Layered(
            crate::key::layered::LayeredKey::new(
                crate::key::composite::TapHold(crate::key::keyboard::Key::new(17)),
                [
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new(69),
                    )),
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new(69),
                    )),
                    None,
                ],
            ),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::new(16)),
        )),
        crate::key::composite::Chorded(crate::key::composite::LayeredKey::Layered(
            crate::key::layered::LayeredKey::new(
                crate::key::composite::TapHold(crate::key::keyboard::Key::new(54)),
                [
                    None,
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new(80),
                    )),
                    None,
                ],
            ),
        )),
        crate::key::composite::Chorded(crate::key::composite::LayeredKey::Layered(
            crate::key::layered::LayeredKey::new(
                crate::key::composite::TapHold(crate::key::keyboard::Key::new(55)),
                [
                    None,
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new(81),
                    )),
                    None,
                ],
            ),
        )),
        crate::key::composite::Chorded(crate::key::composite::LayeredKey::Layered(
            crate::key::layered::LayeredKey::new(
                crate::key::composite::TapHold(crate::key::keyboard::Key::new(56)),
                [
                    None,
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new(82),
                    )),
                    None,
                ],
            ),
        )),
        crate::key::composite::Chorded(crate::key::composite::LayeredKey::Layered(
            crate::key::layered::LayeredKey::new(
                crate::key::composite::TapHold(crate::key::keyboard::Key::from_modifiers(
                    crate::key::KeyboardModifiers::from_byte(32),
                )),
                [
                    None,
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new(79),
                    )),
                    None,
                ],
            ),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::from_modifiers(
                crate::key::KeyboardModifiers::from_byte(1),
            )),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::from_modifiers(
                crate::key::KeyboardModifiers::from_byte(8),
            )),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::from_modifiers(
                crate::key::KeyboardModifiers::from_byte(4),
            )),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::new(43)),
        )),
        crate::key::composite::Chorded(crate::key::composite::LayeredKey::Layered(
            crate::key::layered::LayeredKey::new(
                crate::key::composite::TapHold(crate::key::layered::ModifierKey::hold(2)),
                [
                    Some(crate::key::composite::TapHold(
                        crate::key::layered::ModifierKey::hold(3),
                    )),
                    None,
                    None,
                ],
            ),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::new(44)),
        )),
        crate::key::composite::Chorded(crate::key::composite::Layered(
            crate::key::composite::TapHold(crate::key::keyboard::Key::new(40)),
        )),
        crate::key::composite::Chorded(crate::key::composite::LayeredKey::Layered(
            crate::key::layered::LayeredKey::new(
                crate::key::composite::TapHold(crate::key::layered::ModifierKey::hold(1)),
                [
                    None,
                    Some(crate::key::composite::TapHold(
                        crate::key::layered::ModifierKey::hold(3),
                    )),
                    None,
                ],
            ),
        )),
        crate::key::composite::Chorded(crate::key::composite::LayeredKey::Layered(
            crate::key::layered::LayeredKey::new(
                crate::key::composite::TapHold(crate::key::keyboard::Key::new(76)),
                [
                    None,
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new(74),
                    )),
                    None,
                ],
            ),
        )),
        crate::key::composite::Chorded(crate::key::composite::LayeredKey::Layered(
            crate::key::layered::LayeredKey::new(
                crate::key::composite::TapHold(crate::key::keyboard::Key::from_modifiers(
                    crate::key::KeyboardModifiers::from_byte(64),
                )),
                [
                    None,
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new(78),
                    )),
                    None,
                ],
            ),
        )),
        crate::key::composite::Chorded(crate::key::composite::LayeredKey::Layered(
            crate::key::layered::LayeredKey::new(
                crate::key::composite::TapHold(crate::key::keyboard::Key::from_modifiers(
                    crate::key::KeyboardModifiers::from_byte(128),
                )),
                [
                    None,
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new(75),
                    )),
                    None,
                ],
            ),
        )),
        crate::key::composite::Chorded(crate::key::composite::LayeredKey::Layered(
            crate::key::layered::LayeredKey::new(
                crate::key::composite::TapHold(crate::key::keyboard::Key::from_modifiers(
                    crate::key::KeyboardModifiers::from_byte(16),
                )),
                [
                    None,
                    Some(crate::key::composite::TapHold(
                        crate::key::keyboard::Key::new(77),
                    )),
                    None,
                ],
            ),
        )),
    ));
}
