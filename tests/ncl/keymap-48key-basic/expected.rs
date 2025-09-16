/// Types and initial data used for constructing [KEYMAP].
pub mod init {
    use crate as smart_keymap;

    /// Number of layers supported by the [smart_keymap::key::layered] implementation.
    pub const LAYER_COUNT: usize = 3;

    /// The maximum number of keys in a chord.
    pub const MAX_CHORD_SIZE: usize = 0;

    /// The maximum number of chords.
    pub const MAX_CHORDS: usize = 0;

    /// The maximum number of overlapping chords for a chorded key.
    pub const MAX_OVERLAPPING_CHORD_SIZE: usize = 0;

    /// The tap-dance definitions.
    pub const MAX_TAP_DANCE_DEFINITIONS: usize = 0;

    pub use smart_keymap::key::composite::Ref;

    pub use smart_keymap::key::composite::Context;

    pub use smart_keymap::key::composite::Event;

    pub use smart_keymap::key::composite::PendingKeyState;

    pub use smart_keymap::key::composite::KeyState;

    const KEYBOARD_DATA_LEN: usize = 17;
    const CALLBACK_DATA_LEN: usize = 0;
    const STICKY_DATA_LEN: usize = 0;
    const TAP_DANCE_DATA_LEN: usize = 0;
    const TAP_HOLD_DATA_LEN: usize = 0;
    const LAYER_MODIFIERS_DATA_LEN: usize = 4;
    const LAYERED_DATA_LEN: usize = 40;
    const CHORDED_DATA_LEN: usize = 0;
    const CHORDED_AUXILIARY_DATA_LEN: usize = 0;

    /// The System type
    pub type System = smart_keymap::key::composite::System<
        smart_keymap::key::composite::KeyArrays<
            KEYBOARD_DATA_LEN,
            CALLBACK_DATA_LEN,
            STICKY_DATA_LEN,
            TAP_DANCE_DATA_LEN,
            TAP_HOLD_DATA_LEN,
            LAYER_MODIFIERS_DATA_LEN,
            LAYERED_DATA_LEN,
            CHORDED_DATA_LEN,
            CHORDED_AUXILIARY_DATA_LEN,
        >,
    >;

    /// The number of keys in the keymap.
    pub const KEY_COUNT: usize = 48;

    /// The key references.
    pub const KEY_REFS: [Ref; KEY_COUNT] = [
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(0)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(1)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(2)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(3)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(4)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(5)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(6)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(7)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(8)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(9)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(10)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(11)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(12)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(13)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(14)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(15)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(16)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(17)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(18)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(19)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(20)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(21)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(22)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(23)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::Modifiers(2)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(24)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(25)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(26)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(27)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(28)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(29)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(16)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(30)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(31)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(32)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(33)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::Modifiers(1)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::Modifiers(8)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::Modifiers(4)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(43)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(34)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(44)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(40)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(35)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(36)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(37)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(38)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(39)),
    ];

    /// The keymap config.
    pub const CONFIG: smart_keymap::key::composite::Config = smart_keymap::key::composite::Config {
        chorded: smart_keymap::key::chorded::Config {
            chords: smart_keymap::slice::Slice::from_slice(&[]),
            ..smart_keymap::key::chorded::DEFAULT_CONFIG
        },
        sticky: smart_keymap::key::sticky::DEFAULT_CONFIG,
        tap_dance: smart_keymap::key::tap_dance::DEFAULT_CONFIG,
        tap_hold: smart_keymap::key::tap_hold::DEFAULT_CONFIG,
        ..smart_keymap::key::composite::DEFAULT_CONFIG
    };

    /// Initial [Context] value.
    pub const CONTEXT: Context =
        smart_keymap::key::composite::Context::from_config(smart_keymap::key::composite::Config {
            chorded: smart_keymap::key::chorded::Config {
                chords: smart_keymap::slice::Slice::from_slice(&[]),
                ..smart_keymap::key::chorded::DEFAULT_CONFIG
            },
            sticky: smart_keymap::key::sticky::DEFAULT_CONFIG,
            tap_dance: smart_keymap::key::tap_dance::DEFAULT_CONFIG,
            tap_hold: smart_keymap::key::tap_hold::DEFAULT_CONFIG,
            ..smart_keymap::key::composite::DEFAULT_CONFIG
        });

    /// The key system.
    pub const SYSTEM: System = smart_keymap::key::composite::System::array_based(
        smart_keymap::key::keyboard::System::new([
            smart_keymap::key::keyboard::Key {
                key_code: 53,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(2),
            },
            smart_keymap::key::keyboard::Key {
                key_code: 30,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(2),
            },
            smart_keymap::key::keyboard::Key {
                key_code: 31,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(2),
            },
            smart_keymap::key::keyboard::Key {
                key_code: 32,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(2),
            },
            smart_keymap::key::keyboard::Key {
                key_code: 33,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(2),
            },
            smart_keymap::key::keyboard::Key {
                key_code: 34,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(2),
            },
            smart_keymap::key::keyboard::Key {
                key_code: 35,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(2),
            },
            smart_keymap::key::keyboard::Key {
                key_code: 36,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(2),
            },
            smart_keymap::key::keyboard::Key {
                key_code: 37,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(2),
            },
            smart_keymap::key::keyboard::Key {
                key_code: 38,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(2),
            },
            smart_keymap::key::keyboard::Key {
                key_code: 39,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(2),
            },
            smart_keymap::key::keyboard::Key {
                key_code: 49,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(2),
            },
            smart_keymap::key::keyboard::Key {
                key_code: 45,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(2),
            },
            smart_keymap::key::keyboard::Key {
                key_code: 46,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(2),
            },
            smart_keymap::key::keyboard::Key {
                key_code: 47,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(2),
            },
            smart_keymap::key::keyboard::Key {
                key_code: 48,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(2),
            },
            smart_keymap::key::keyboard::Key {
                key_code: 56,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(2),
            },
        ]),
        smart_keymap::key::callback::System::new([]),
        smart_keymap::key::sticky::System::new([]),
        smart_keymap::key::tap_dance::System::new([]),
        smart_keymap::key::tap_hold::System::new([]),
        smart_keymap::key::layered::System::new(
            [
                smart_keymap::key::layered::ModifierKey::hold(2),
                smart_keymap::key::layered::ModifierKey::hold(3),
                smart_keymap::key::layered::ModifierKey::hold(1),
                smart_keymap::key::layered::ModifierKey::hold(3),
            ],
            [
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(43),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(53),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(0),
                        )),
                        None,
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(20),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(30),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(1),
                        )),
                        None,
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(26),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(31),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(2),
                        )),
                        None,
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(8),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(32),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(3),
                        )),
                        None,
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(21),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(33),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(4),
                        )),
                        None,
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(23),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(34),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(5),
                        )),
                        None,
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(28),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(35),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(6),
                        )),
                        None,
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(24),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(36),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(7),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(70),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(12),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(37),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(8),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(71),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(18),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(38),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(9),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(72),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(19),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(39),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(10),
                        )),
                        None,
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(42),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(49),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(11),
                        )),
                        None,
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(41),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(76),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(73),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(57),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(4),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(58),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(58),
                        )),
                        None,
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(22),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(59),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(59),
                        )),
                        None,
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(7),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(60),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(60),
                        )),
                        None,
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(9),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(61),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(61),
                        )),
                        None,
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(10),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(62),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(62),
                        )),
                        None,
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(11),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(63),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(63),
                        )),
                        None,
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(13),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(45),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(12),
                        )),
                        None,
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(14),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(46),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(13),
                        )),
                        None,
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(15),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(47),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(14),
                        )),
                        None,
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(51),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(48),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(15),
                        )),
                        None,
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(52),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(56),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(16),
                        )),
                        None,
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(29),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(64),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(64),
                        )),
                        None,
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(27),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(65),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(65),
                        )),
                        None,
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(6),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(66),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(66),
                        )),
                        None,
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(25),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(67),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(67),
                        )),
                        None,
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(5),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(68),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(68),
                        )),
                        None,
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(17),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(69),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(69),
                        )),
                        None,
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(54),
                    ),
                    [
                        None,
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(80),
                        )),
                        None,
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(55),
                    ),
                    [
                        None,
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(81),
                        )),
                        None,
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(56),
                    ),
                    [
                        None,
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(82),
                        )),
                        None,
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::Modifiers(32),
                    ),
                    [
                        None,
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(79),
                        )),
                        None,
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Layered(
                        smart_keymap::key::layered::Ref::Modifier(0),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Layered(
                            smart_keymap::key::layered::Ref::Modifier(1),
                        )),
                        None,
                        None,
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Layered(
                        smart_keymap::key::layered::Ref::Modifier(2),
                    ),
                    [
                        None,
                        Some(smart_keymap::key::composite::Ref::Layered(
                            smart_keymap::key::layered::Ref::Modifier(3),
                        )),
                        None,
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(76),
                    ),
                    [
                        None,
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(74),
                        )),
                        None,
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::Modifiers(64),
                    ),
                    [
                        None,
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(78),
                        )),
                        None,
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::Modifiers(128),
                    ),
                    [
                        None,
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(75),
                        )),
                        None,
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::Modifiers(16),
                    ),
                    [
                        None,
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(77),
                        )),
                        None,
                    ],
                ),
            ],
        ),
        smart_keymap::key::chorded::System::new([], []),
    );

    /// Alias for the [keymap::Keymap] type.
    pub type Keymap = smart_keymap::keymap::Keymap<
        [Ref; KEY_COUNT],
        Ref,
        Context,
        Event,
        PendingKeyState,
        KeyState,
        System,
    >;
}
