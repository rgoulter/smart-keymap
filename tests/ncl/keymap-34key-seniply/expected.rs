/// Types and initial data used for constructing [KEYMAP].
pub mod init {
    use crate as smart_keymap;

    /// Number of instructions used by the [crate::key::automation] implementation.
    pub const AUTOMATION_INSTRUCTION_COUNT: usize = 0;

    /// Number of layers supported by the [smart_keymap::key::layered] implementation.
    pub const LAYERED_LAYER_COUNT: usize = 5;

    /// The maximum number of keys in a chord.
    pub const CHORDED_MAX_CHORD_SIZE: usize = 0;

    /// The maximum number of chords.
    pub const CHORDED_MAX_CHORDS: usize = 0;

    /// The maximum number of overlapping chords for a chorded key.
    pub const CHORDED_MAX_OVERLAPPING_CHORD_SIZE: usize = 0;

    /// The tap-dance definitions.
    pub const TAP_DANCE_MAX_DEFINITIONS: usize = 0;

    pub use smart_keymap::key::composite::Ref;

    pub use smart_keymap::key::composite::Context;

    pub use smart_keymap::key::composite::Event;

    pub use smart_keymap::key::composite::PendingKeyState;

    pub use smart_keymap::key::composite::KeyState;

    const AUTOMATION: usize = 0;
    const CALLBACK: usize = 0;
    const CHORDED: usize = 0;
    const CHORDED_AUXILIARY: usize = 0;
    const KEYBOARD: usize = 28;
    const LAYERED: usize = 34;
    const LAYER_MODIFIERS: usize = 8;
    const STICKY: usize = 16;
    const TAP_DANCE: usize = 0;
    const TAP_HOLD: usize = 0;

    /// The System type
    pub type System = smart_keymap::key::composite::System<
        smart_keymap::key::composite::KeyArrays<
            AUTOMATION,
            CALLBACK,
            CHORDED,
            CHORDED_AUXILIARY,
            KEYBOARD,
            LAYERED,
            LAYER_MODIFIERS,
            STICKY,
            TAP_DANCE,
            TAP_HOLD,
        >,
    >;

    /// The number of keys in the keymap.
    pub const KEY_COUNT: usize = 34;

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
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(24)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(25)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(26)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(27)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(28)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(29)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(30)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(31)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(32)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(33)),
    ];

    /// The keymap config.
    pub const CONFIG: smart_keymap::key::composite::Config = smart_keymap::key::composite::Config {
        automation: smart_keymap::key::automation::Config::new(),
        chorded: smart_keymap::key::chorded::Config {
            chords: smart_keymap::slice::Slice::from_slice(&[]),
            ..smart_keymap::key::chorded::Config::new()
        },
        sticky: smart_keymap::key::sticky::Config::new(),
        tap_dance: smart_keymap::key::tap_dance::Config::new(),
        tap_hold: smart_keymap::key::tap_hold::Config::new(),
        ..smart_keymap::key::composite::Config::new()
    };

    /// Initial [Context] value.
    pub const CONTEXT: Context =
        smart_keymap::key::composite::Context::from_config(smart_keymap::key::composite::Config {
            automation: smart_keymap::key::automation::Config::new(),
            chorded: smart_keymap::key::chorded::Config {
                chords: smart_keymap::slice::Slice::from_slice(&[]),
                ..smart_keymap::key::chorded::Config::new()
            },
            sticky: smart_keymap::key::sticky::Config::new(),
            tap_dance: smart_keymap::key::tap_dance::Config::new(),
            tap_hold: smart_keymap::key::tap_hold::Config::new(),
            ..smart_keymap::key::composite::Config::new()
        });

    /// The key system.
    pub const SYSTEM: System = smart_keymap::key::composite::System::array_based(
        smart_keymap::key::automation::System::new([]),
        smart_keymap::key::callback::System::new([]),
        smart_keymap::key::chorded::System::new([], []),
        smart_keymap::key::keyboard::System::new([
            smart_keymap::key::keyboard::Key {
                key_code: 30,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(2),
            },
            smart_keymap::key::keyboard::Key {
                key_code: 80,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(4),
            },
            smart_keymap::key::keyboard::Key {
                key_code: 31,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(2),
            },
            smart_keymap::key::keyboard::Key {
                key_code: 9,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(1),
            },
            smart_keymap::key::keyboard::Key {
                key_code: 32,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(2),
            },
            smart_keymap::key::keyboard::Key {
                key_code: 79,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(4),
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
                key_code: 51,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(2),
            },
            smart_keymap::key::keyboard::Key {
                key_code: 46,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(2),
            },
            smart_keymap::key::keyboard::Key {
                key_code: 46,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(2),
            },
            smart_keymap::key::keyboard::Key {
                key_code: 35,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(2),
            },
            smart_keymap::key::keyboard::Key {
                key_code: 37,
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
                key_code: 47,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(2),
            },
            smart_keymap::key::keyboard::Key {
                key_code: 29,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(1),
            },
            smart_keymap::key::keyboard::Key {
                key_code: 27,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(1),
            },
            smart_keymap::key::keyboard::Key {
                key_code: 6,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(1),
            },
            smart_keymap::key::keyboard::Key {
                key_code: 6,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(3),
            },
            smart_keymap::key::keyboard::Key {
                key_code: 49,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(2),
            },
            smart_keymap::key::keyboard::Key {
                key_code: 25,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(1),
            },
            smart_keymap::key::keyboard::Key {
                key_code: 25,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(3),
            },
            smart_keymap::key::keyboard::Key {
                key_code: 36,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(2),
            },
            smart_keymap::key::keyboard::Key {
                key_code: 53,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(2),
            },
            smart_keymap::key::keyboard::Key {
                key_code: 39,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(2),
            },
            smart_keymap::key::keyboard::Key {
                key_code: 48,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(2),
            },
            smart_keymap::key::keyboard::Key {
                key_code: 45,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(2),
            },
        ]),
        smart_keymap::key::layered::System::new(
            [
                smart_keymap::key::layered::ModifierKey::default(0),
                smart_keymap::key::layered::ModifierKey::default(1),
                smart_keymap::key::layered::ModifierKey::hold(2),
                smart_keymap::key::layered::ModifierKey::hold(2),
                smart_keymap::key::layered::ModifierKey::hold(5),
                smart_keymap::key::layered::ModifierKey::hold(4),
                smart_keymap::key::layered::ModifierKey::hold(4),
                smart_keymap::key::layered::ModifierKey::hold(3),
            ],
            [
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(20),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(20),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(41),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(0),
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
                            smart_keymap::key::keyboard::Ref::KeyCode(26),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(1),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(2),
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
                            smart_keymap::key::keyboard::Ref::KeyCode(8),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(3),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(4),
                        )),
                        None,
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(19),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(21),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(5),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(6),
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
                            smart_keymap::key::keyboard::Ref::KeyCode(23),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(73),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(7),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(83),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(13),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(28),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(75),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(69),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(46),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(46),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(15),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(24),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(74),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(64),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(53),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(36),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(24),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(12),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(82),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(65),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(8),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(37),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(28),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(18),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(77),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(66),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(51),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(38),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(52),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(19),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(57),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(71),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(9),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(10),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(4),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(4),
                        )),
                        Some(smart_keymap::key::composite::Ref::Sticky(
                            smart_keymap::key::sticky::Ref(0),
                        )),
                        Some(smart_keymap::key::composite::Ref::Sticky(
                            smart_keymap::key::sticky::Ref(1),
                        )),
                        Some(smart_keymap::key::composite::Ref::Sticky(
                            smart_keymap::key::sticky::Ref(2),
                        )),
                        Some(smart_keymap::key::composite::Ref::Sticky(
                            smart_keymap::key::sticky::Ref(3),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(21),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(22),
                        )),
                        Some(smart_keymap::key::composite::Ref::Sticky(
                            smart_keymap::key::sticky::Ref(4),
                        )),
                        Some(smart_keymap::key::composite::Ref::Sticky(
                            smart_keymap::key::sticky::Ref(5),
                        )),
                        Some(smart_keymap::key::composite::Ref::Sticky(
                            smart_keymap::key::sticky::Ref(6),
                        )),
                        Some(smart_keymap::key::composite::Ref::Sticky(
                            smart_keymap::key::sticky::Ref(7),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(22),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(7),
                        )),
                        Some(smart_keymap::key::composite::Ref::Sticky(
                            smart_keymap::key::sticky::Ref(8),
                        )),
                        Some(smart_keymap::key::composite::Ref::Sticky(
                            smart_keymap::key::sticky::Ref(9),
                        )),
                        Some(smart_keymap::key::composite::Ref::Sticky(
                            smart_keymap::key::sticky::Ref(10),
                        )),
                        Some(smart_keymap::key::composite::Ref::Sticky(
                            smart_keymap::key::sticky::Ref(11),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(23),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(9),
                        )),
                        Some(smart_keymap::key::composite::Ref::Sticky(
                            smart_keymap::key::sticky::Ref(12),
                        )),
                        Some(smart_keymap::key::composite::Ref::Sticky(
                            smart_keymap::key::sticky::Ref(13),
                        )),
                        Some(smart_keymap::key::composite::Ref::Sticky(
                            smart_keymap::key::sticky::Ref(14),
                        )),
                        Some(smart_keymap::key::composite::Ref::Sticky(
                            smart_keymap::key::sticky::Ref(15),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(10),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(10),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::Modifiers(64),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(11),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::Modifiers(64),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(16),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(11),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(78),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(68),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(12),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(13),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(17),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(13),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(80),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(61),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(14),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(33),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(8),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(14),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(81),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(62),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(15),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(34),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(12),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(15),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(79),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(63),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(47),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(35),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(18),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(52),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(76),
                        )),
                        Some(smart_keymap::key::composite::Ref::Layered(
                            smart_keymap::key::layered::Ref::Modifier(0),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(45),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(45),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(29),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(29),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(16),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        None,
                        None,
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(27),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(27),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(17),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        None,
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(101),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(6),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(6),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(18),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(19),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(49),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(43),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(7),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(25),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::Modifiers(8),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(20),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(42),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(25),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(5),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(21),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(22),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(23),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(40),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(14),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(17),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(40),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(67),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(24),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(39),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(11),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(16),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(42),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(58),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(25),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(30),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(54),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(54),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(43),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(59),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(26),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(31),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(55),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(55),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(101),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(60),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(48),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(32),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(56),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(56),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(70),
                        )),
                        Some(smart_keymap::key::composite::Ref::Layered(
                            smart_keymap::key::layered::Ref::Modifier(1),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(27),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(56),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Layered(
                        smart_keymap::key::layered::Ref::Modifier(2),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Layered(
                            smart_keymap::key::layered::Ref::Modifier(3),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        None,
                        None,
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::Modifiers(2),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::Modifiers(2),
                        )),
                        None,
                        None,
                        Some(smart_keymap::key::composite::Ref::Layered(
                            smart_keymap::key::layered::Ref::Modifier(4),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(44),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(44),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(40),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(40),
                        )),
                        None,
                        None,
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Layered(
                        smart_keymap::key::layered::Ref::Modifier(5),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Layered(
                            smart_keymap::key::layered::Ref::Modifier(6),
                        )),
                        Some(smart_keymap::key::composite::Ref::Layered(
                            smart_keymap::key::layered::Ref::Modifier(7),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                    ],
                ),
            ],
        ),
        smart_keymap::key::sticky::System::new([
            smart_keymap::key::sticky::Key::new(smart_keymap::key::KeyboardModifiers::from_byte(4)),
            smart_keymap::key::sticky::Key::new(smart_keymap::key::KeyboardModifiers::from_byte(4)),
            smart_keymap::key::sticky::Key::new(smart_keymap::key::KeyboardModifiers::from_byte(4)),
            smart_keymap::key::sticky::Key::new(smart_keymap::key::KeyboardModifiers::from_byte(4)),
            smart_keymap::key::sticky::Key::new(smart_keymap::key::KeyboardModifiers::from_byte(8)),
            smart_keymap::key::sticky::Key::new(smart_keymap::key::KeyboardModifiers::from_byte(8)),
            smart_keymap::key::sticky::Key::new(smart_keymap::key::KeyboardModifiers::from_byte(8)),
            smart_keymap::key::sticky::Key::new(smart_keymap::key::KeyboardModifiers::from_byte(8)),
            smart_keymap::key::sticky::Key::new(smart_keymap::key::KeyboardModifiers::from_byte(2)),
            smart_keymap::key::sticky::Key::new(smart_keymap::key::KeyboardModifiers::from_byte(2)),
            smart_keymap::key::sticky::Key::new(smart_keymap::key::KeyboardModifiers::from_byte(2)),
            smart_keymap::key::sticky::Key::new(smart_keymap::key::KeyboardModifiers::from_byte(2)),
            smart_keymap::key::sticky::Key::new(smart_keymap::key::KeyboardModifiers::from_byte(1)),
            smart_keymap::key::sticky::Key::new(smart_keymap::key::KeyboardModifiers::from_byte(1)),
            smart_keymap::key::sticky::Key::new(smart_keymap::key::KeyboardModifiers::from_byte(1)),
            smart_keymap::key::sticky::Key::new(smart_keymap::key::KeyboardModifiers::from_byte(1)),
        ]),
        smart_keymap::key::tap_dance::System::new([]),
        smart_keymap::key::tap_hold::System::new([]),
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
