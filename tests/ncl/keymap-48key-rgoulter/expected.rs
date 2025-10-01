/// Types and initial data used for constructing [KEYMAP].
pub mod init {
    use crate as smart_keymap;

    /// Number of instructions used by the [crate::key::automation] implementation.
    pub const AUTOMATION_INSTRUCTION_COUNT: usize = 0;

    /// Number of layers supported by the [smart_keymap::key::layered] implementation.
    pub const LAYERED_LAYER_COUNT: usize = 8;

    /// The maximum number of keys in a chord.
    pub const CHORDED_MAX_CHORD_SIZE: usize = 2;

    /// The maximum number of chords.
    pub const CHORDED_MAX_CHORDS: usize = 4;

    /// The maximum number of overlapping chords for a chorded key.
    pub const CHORDED_MAX_OVERLAPPING_CHORD_SIZE: usize = 1;

    /// The tap-dance definitions.
    pub const TAP_DANCE_MAX_DEFINITIONS: usize = 2;

    pub use smart_keymap::key::composite::Ref;

    pub use smart_keymap::key::composite::Context;

    pub use smart_keymap::key::composite::Event;

    pub use smart_keymap::key::composite::PendingKeyState;

    pub use smart_keymap::key::composite::KeyState;

    const AUTOMATION: usize = 0;
    const CALLBACK: usize = 7;
    const CHORDED: usize = 4;
    const CHORDED_AUXILIARY: usize = 4;
    const KEYBOARD: usize = 20;
    const LAYERED: usize = 48;
    const LAYER_MODIFIERS: usize = 38;
    const STICKY: usize = 0;
    const TAP_DANCE: usize = 24;
    const TAP_HOLD: usize = 36;

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
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(24)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(25)),
        smart_keymap::key::composite::Ref::Chorded(smart_keymap::key::chorded::Ref::Chorded(0)),
        smart_keymap::key::composite::Ref::Chorded(smart_keymap::key::chorded::Ref::Auxiliary(0)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(28)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(29)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(30)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(31)),
        smart_keymap::key::composite::Ref::Chorded(smart_keymap::key::chorded::Ref::Chorded(1)),
        smart_keymap::key::composite::Ref::Chorded(smart_keymap::key::chorded::Ref::Auxiliary(1)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(34)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(35)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(36)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(37)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(38)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(39)),
        smart_keymap::key::composite::Ref::Chorded(smart_keymap::key::chorded::Ref::Chorded(2)),
        smart_keymap::key::composite::Ref::Chorded(smart_keymap::key::chorded::Ref::Auxiliary(2)),
        smart_keymap::key::composite::Ref::Chorded(smart_keymap::key::chorded::Ref::Chorded(3)),
        smart_keymap::key::composite::Ref::Chorded(smart_keymap::key::chorded::Ref::Auxiliary(3)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(44)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(45)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(46)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(47)),
    ];

    /// The keymap config.
    pub const CONFIG: smart_keymap::key::composite::Config = smart_keymap::key::composite::Config {
        automation: smart_keymap::key::automation::Config::new(),
        chorded: smart_keymap::key::chorded::Config {
            chords: smart_keymap::slice::Slice::from_slice(&[
                smart_keymap::key::chorded::ChordIndices::from_slice(&[26, 27]),
                smart_keymap::key::chorded::ChordIndices::from_slice(&[32, 33]),
                smart_keymap::key::chorded::ChordIndices::from_slice(&[40, 41]),
                smart_keymap::key::chorded::ChordIndices::from_slice(&[42, 43]),
            ]),
            ..smart_keymap::key::chorded::Config::new()
        },
        sticky: smart_keymap::key::sticky::Config::new(),
        tap_dance: smart_keymap::key::tap_dance::Config::new(),
        tap_hold: smart_keymap::key::tap_hold::Config {
            interrupt_response: smart_keymap::key::tap_hold::InterruptResponse::HoldOnKeyTap,

            ..smart_keymap::key::tap_hold::Config::new()
        },
        ..smart_keymap::key::composite::Config::new()
    };

    /// Initial [Context] value.
    pub const CONTEXT: Context =
        smart_keymap::key::composite::Context::from_config(smart_keymap::key::composite::Config {
            automation: smart_keymap::key::automation::Config::new(),
            chorded: smart_keymap::key::chorded::Config {
                chords: smart_keymap::slice::Slice::from_slice(&[
                    smart_keymap::key::chorded::ChordIndices::from_slice(&[26, 27]),
                    smart_keymap::key::chorded::ChordIndices::from_slice(&[32, 33]),
                    smart_keymap::key::chorded::ChordIndices::from_slice(&[40, 41]),
                    smart_keymap::key::chorded::ChordIndices::from_slice(&[42, 43]),
                ]),
                ..smart_keymap::key::chorded::Config::new()
            },
            sticky: smart_keymap::key::sticky::Config::new(),
            tap_dance: smart_keymap::key::tap_dance::Config::new(),
            tap_hold: smart_keymap::key::tap_hold::Config {
                interrupt_response: smart_keymap::key::tap_hold::InterruptResponse::HoldOnKeyTap,

                ..smart_keymap::key::tap_hold::Config::new()
            },
            ..smart_keymap::key::composite::Config::new()
        });

    /// The key system.
    pub const SYSTEM: System = smart_keymap::key::composite::System::array_based(
        smart_keymap::key::automation::System::new([]),
        smart_keymap::key::callback::System::new([
            smart_keymap::key::callback::Key::new(
                smart_keymap::keymap::KeymapCallback::ResetToBootloader,
            ),
            smart_keymap::key::callback::Key::new(
                smart_keymap::keymap::KeymapCallback::ResetToBootloader,
            ),
            smart_keymap::key::callback::Key::new(
                smart_keymap::keymap::KeymapCallback::ResetToBootloader,
            ),
            smart_keymap::key::callback::Key::new(
                smart_keymap::keymap::KeymapCallback::ResetToBootloader,
            ),
            smart_keymap::key::callback::Key::new(
                smart_keymap::keymap::KeymapCallback::ResetToBootloader,
            ),
            smart_keymap::key::callback::Key::new(
                smart_keymap::keymap::KeymapCallback::ResetToBootloader,
            ),
            smart_keymap::key::callback::Key::new(
                smart_keymap::keymap::KeymapCallback::ResetToBootloader,
            ),
        ]),
        smart_keymap::key::chorded::System::new(
            [
                smart_keymap::key::chorded::Key::new(
                    &[(
                        0,
                        smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(12),
                        ),
                    )],
                    smart_keymap::key::composite::Ref::Layered(
                        smart_keymap::key::layered::Ref::Layered(26),
                    ),
                ),
                smart_keymap::key::chorded::Key::new(
                    &[(
                        1,
                        smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(16),
                        ),
                    )],
                    smart_keymap::key::composite::Ref::Layered(
                        smart_keymap::key::layered::Ref::Layered(32),
                    ),
                ),
                smart_keymap::key::chorded::Key::new(
                    &[(
                        2,
                        smart_keymap::key::composite::Ref::TapHold(
                            smart_keymap::key::tap_hold::Ref(19),
                        ),
                    )],
                    smart_keymap::key::composite::Ref::Layered(
                        smart_keymap::key::layered::Ref::Layered(40),
                    ),
                ),
                smart_keymap::key::chorded::Key::new(
                    &[(
                        3,
                        smart_keymap::key::composite::Ref::TapHold(
                            smart_keymap::key::tap_hold::Ref(26),
                        ),
                    )],
                    smart_keymap::key::composite::Ref::Layered(
                        smart_keymap::key::layered::Ref::Layered(42),
                    ),
                ),
            ],
            [
                smart_keymap::key::chorded::AuxiliaryKey::new(
                    smart_keymap::key::composite::Ref::Layered(
                        smart_keymap::key::layered::Ref::Layered(27),
                    ),
                ),
                smart_keymap::key::chorded::AuxiliaryKey::new(
                    smart_keymap::key::composite::Ref::Layered(
                        smart_keymap::key::layered::Ref::Layered(33),
                    ),
                ),
                smart_keymap::key::chorded::AuxiliaryKey::new(
                    smart_keymap::key::composite::Ref::Layered(
                        smart_keymap::key::layered::Ref::Layered(41),
                    ),
                ),
                smart_keymap::key::chorded::AuxiliaryKey::new(
                    smart_keymap::key::composite::Ref::Layered(
                        smart_keymap::key::layered::Ref::Layered(43),
                    ),
                ),
            ],
        ),
        smart_keymap::key::keyboard::System::new([
            smart_keymap::key::keyboard::Key {
                key_code: 47,
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
                key_code: 48,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(2),
            },
            smart_keymap::key::keyboard::Key {
                key_code: 53,
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
                key_code: 46,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(2),
            },
            smart_keymap::key::keyboard::Key {
                key_code: 56,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(2),
            },
            smart_keymap::key::keyboard::Key {
                key_code: 30,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(2),
            },
            smart_keymap::key::keyboard::Key {
                key_code: 75,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(8),
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
                key_code: 49,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(2),
            },
            smart_keymap::key::keyboard::Key {
                key_code: 78,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(8),
            },
            smart_keymap::key::keyboard::Key {
                key_code: 55,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(2),
            },
            smart_keymap::key::keyboard::Key {
                key_code: 39,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(2),
            },
            smart_keymap::key::keyboard::Key {
                key_code: 45,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(2),
            },
        ]),
        smart_keymap::key::layered::System::new(
            [
                smart_keymap::key::layered::ModifierKey::default(2),
                smart_keymap::key::layered::ModifierKey::default(2),
                smart_keymap::key::layered::ModifierKey::default(2),
                smart_keymap::key::layered::ModifierKey::default(1),
                smart_keymap::key::layered::ModifierKey::default(1),
                smart_keymap::key::layered::ModifierKey::default(1),
                smart_keymap::key::layered::ModifierKey::default(0),
                smart_keymap::key::layered::ModifierKey::default(0),
                smart_keymap::key::layered::ModifierKey::default(0),
                smart_keymap::key::layered::ModifierKey::default(0),
                smart_keymap::key::layered::ModifierKey::default(0),
                smart_keymap::key::layered::ModifierKey::default(0),
                smart_keymap::key::layered::ModifierKey::default(1),
                smart_keymap::key::layered::ModifierKey::default(1),
                smart_keymap::key::layered::ModifierKey::default(1),
                smart_keymap::key::layered::ModifierKey::default(2),
                smart_keymap::key::layered::ModifierKey::default(2),
                smart_keymap::key::layered::ModifierKey::default(2),
                smart_keymap::key::layered::ModifierKey::hold(7),
                smart_keymap::key::layered::ModifierKey::hold(7),
                smart_keymap::key::layered::ModifierKey::hold(7),
                smart_keymap::key::layered::ModifierKey::hold(7),
                smart_keymap::key::layered::ModifierKey::hold(8),
                smart_keymap::key::layered::ModifierKey::hold(8),
                smart_keymap::key::layered::ModifierKey::hold(8),
                smart_keymap::key::layered::ModifierKey::hold(6),
                smart_keymap::key::layered::ModifierKey::hold(6),
                smart_keymap::key::layered::ModifierKey::hold(6),
                smart_keymap::key::layered::ModifierKey::hold(5),
                smart_keymap::key::layered::ModifierKey::hold(4),
                smart_keymap::key::layered::ModifierKey::hold(4),
                smart_keymap::key::layered::ModifierKey::hold(4),
                smart_keymap::key::layered::ModifierKey::hold(3),
                smart_keymap::key::layered::ModifierKey::hold(3),
                smart_keymap::key::layered::ModifierKey::hold(3),
                smart_keymap::key::layered::ModifierKey::hold(5),
                smart_keymap::key::layered::ModifierKey::hold(5),
                smart_keymap::key::layered::ModifierKey::hold(5),
            ],
            [
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(52),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(20),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(20),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(47),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(0),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(69),
                        )),
                        Some(smart_keymap::key::composite::Ref::TapDance(
                            smart_keymap::key::tap_dance::Ref(0),
                        )),
                        Some(smart_keymap::key::composite::Ref::TapDance(
                            smart_keymap::key::tap_dance::Ref(1),
                        )),
                        Some(smart_keymap::key::composite::Ref::TapDance(
                            smart_keymap::key::tap_dance::Ref(2),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(54),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(26),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(26),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(36),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(1),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(64),
                        )),
                        Some(smart_keymap::key::composite::Ref::TapDance(
                            smart_keymap::key::tap_dance::Ref(3),
                        )),
                        Some(smart_keymap::key::composite::Ref::TapDance(
                            smart_keymap::key::tap_dance::Ref(4),
                        )),
                        Some(smart_keymap::key::composite::Ref::TapDance(
                            smart_keymap::key::tap_dance::Ref(5),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(55),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(8),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(8),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(37),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(2),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(65),
                        )),
                        Some(smart_keymap::key::composite::Ref::TapDance(
                            smart_keymap::key::tap_dance::Ref(6),
                        )),
                        Some(smart_keymap::key::composite::Ref::TapDance(
                            smart_keymap::key::tap_dance::Ref(7),
                        )),
                        Some(smart_keymap::key::composite::Ref::TapDance(
                            smart_keymap::key::tap_dance::Ref(8),
                        )),
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
                            smart_keymap::key::keyboard::Ref::KeyCode(21),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(38),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(3),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(66),
                        )),
                        Some(smart_keymap::key::composite::Ref::TapDance(
                            smart_keymap::key::tap_dance::Ref(9),
                        )),
                        Some(smart_keymap::key::composite::Ref::TapDance(
                            smart_keymap::key::tap_dance::Ref(10),
                        )),
                        Some(smart_keymap::key::composite::Ref::TapDance(
                            smart_keymap::key::tap_dance::Ref(11),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(28),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(23),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(23),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(48),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(4),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(70),
                        )),
                        None,
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(0),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
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
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
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
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(0),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
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
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
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
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(9),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(28),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(28),
                        )),
                        None,
                        None,
                        None,
                        None,
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(10),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(24),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(24),
                        )),
                        Some(smart_keymap::key::composite::Ref::TapDance(
                            smart_keymap::key::tap_dance::Ref(12),
                        )),
                        Some(smart_keymap::key::composite::Ref::TapDance(
                            smart_keymap::key::tap_dance::Ref(13),
                        )),
                        Some(smart_keymap::key::composite::Ref::TapDance(
                            smart_keymap::key::tap_dance::Ref(14),
                        )),
                        None,
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(6),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(12),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(12),
                        )),
                        Some(smart_keymap::key::composite::Ref::TapDance(
                            smart_keymap::key::tap_dance::Ref(15),
                        )),
                        Some(smart_keymap::key::composite::Ref::TapDance(
                            smart_keymap::key::tap_dance::Ref(16),
                        )),
                        Some(smart_keymap::key::composite::Ref::TapDance(
                            smart_keymap::key::tap_dance::Ref(17),
                        )),
                        None,
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(21),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(18),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(18),
                        )),
                        Some(smart_keymap::key::composite::Ref::TapDance(
                            smart_keymap::key::tap_dance::Ref(18),
                        )),
                        Some(smart_keymap::key::composite::Ref::TapDance(
                            smart_keymap::key::tap_dance::Ref(19),
                        )),
                        Some(smart_keymap::key::composite::Ref::TapDance(
                            smart_keymap::key::tap_dance::Ref(20),
                        )),
                        None,
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(15),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(19),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(19),
                        )),
                        Some(smart_keymap::key::composite::Ref::TapDance(
                            smart_keymap::key::tap_dance::Ref(21),
                        )),
                        Some(smart_keymap::key::composite::Ref::TapDance(
                            smart_keymap::key::tap_dance::Ref(22),
                        )),
                        Some(smart_keymap::key::composite::Ref::TapDance(
                            smart_keymap::key::tap_dance::Ref(23),
                        )),
                        None,
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::TapHold(smart_keymap::key::tap_hold::Ref(0)),
                    [
                        Some(smart_keymap::key::composite::Ref::TapHold(
                            smart_keymap::key::tap_hold::Ref(1),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(4),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(53),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(5),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(68),
                        )),
                        None,
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::TapHold(smart_keymap::key::tap_hold::Ref(2)),
                    [
                        Some(smart_keymap::key::composite::Ref::TapHold(
                            smart_keymap::key::tap_hold::Ref(3),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(22),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(33),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(6),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(61),
                        )),
                        None,
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::TapHold(smart_keymap::key::tap_hold::Ref(4)),
                    [
                        Some(smart_keymap::key::composite::Ref::TapHold(
                            smart_keymap::key::tap_hold::Ref(5),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(7),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(34),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(7),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(62),
                        )),
                        None,
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::TapHold(smart_keymap::key::tap_hold::Ref(6)),
                    [
                        Some(smart_keymap::key::composite::Ref::TapHold(
                            smart_keymap::key::tap_hold::Ref(7),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(9),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(35),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(8),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(63),
                        )),
                        None,
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(12),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(10),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(10),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(46),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(9),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(71),
                        )),
                        None,
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(0),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
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
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
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
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(0),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
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
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
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
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(7),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(11),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(11),
                        )),
                        None,
                        None,
                        None,
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(80),
                        )),
                        Some(smart_keymap::key::composite::Ref::Mouse(
                            smart_keymap::key::mouse::Ref::CursorLeft,
                        )),
                        Some(smart_keymap::key::composite::Ref::Consumer(
                            smart_keymap::key::consumer::Ref::UsageCode(182),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::TapHold(smart_keymap::key::tap_hold::Ref(8)),
                    [
                        Some(smart_keymap::key::composite::Ref::TapHold(
                            smart_keymap::key::tap_hold::Ref(9),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(13),
                        )),
                        None,
                        None,
                        None,
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(81),
                        )),
                        Some(smart_keymap::key::composite::Ref::Mouse(
                            smart_keymap::key::mouse::Ref::CursorDown,
                        )),
                        Some(smart_keymap::key::composite::Ref::Consumer(
                            smart_keymap::key::consumer::Ref::UsageCode(234),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::TapHold(smart_keymap::key::tap_hold::Ref(
                        10,
                    )),
                    [
                        Some(smart_keymap::key::composite::Ref::TapHold(
                            smart_keymap::key::tap_hold::Ref(11),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(14),
                        )),
                        None,
                        None,
                        None,
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(82),
                        )),
                        Some(smart_keymap::key::composite::Ref::Mouse(
                            smart_keymap::key::mouse::Ref::CursorUp,
                        )),
                        Some(smart_keymap::key::composite::Ref::Consumer(
                            smart_keymap::key::consumer::Ref::UsageCode(233),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::TapHold(smart_keymap::key::tap_hold::Ref(
                        12,
                    )),
                    [
                        Some(smart_keymap::key::composite::Ref::TapHold(
                            smart_keymap::key::tap_hold::Ref(13),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(15),
                        )),
                        None,
                        None,
                        None,
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(79),
                        )),
                        Some(smart_keymap::key::composite::Ref::Mouse(
                            smart_keymap::key::mouse::Ref::CursorRight,
                        )),
                        Some(smart_keymap::key::composite::Ref::Consumer(
                            smart_keymap::key::consumer::Ref::UsageCode(181),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::TapHold(smart_keymap::key::tap_hold::Ref(
                        14,
                    )),
                    [
                        Some(smart_keymap::key::composite::Ref::TapHold(
                            smart_keymap::key::tap_hold::Ref(15),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(51),
                        )),
                        None,
                        None,
                        None,
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(57),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(51),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(29),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(29),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(56),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(10),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(67),
                        )),
                        None,
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(20),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(27),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(27),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(30),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(11),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(58),
                        )),
                        None,
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(13),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(6),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(6),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(31),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(13),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(59),
                        )),
                        None,
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(14),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(25),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(25),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(32),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(14),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(60),
                        )),
                        None,
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(27),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(5),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(5),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(49),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(15),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(72),
                        )),
                        None,
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(0),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
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
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
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
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(0),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
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
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
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
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(5),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(17),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(17),
                        )),
                        None,
                        None,
                        None,
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(74),
                        )),
                        Some(smart_keymap::key::composite::Ref::Mouse(
                            smart_keymap::key::mouse::Ref::WheelLeft,
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(16),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(16),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(16),
                        )),
                        None,
                        None,
                        None,
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(78),
                        )),
                        Some(smart_keymap::key::composite::Ref::Mouse(
                            smart_keymap::key::mouse::Ref::WheelDown,
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(26),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(54),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(54),
                        )),
                        None,
                        None,
                        None,
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(75),
                        )),
                        Some(smart_keymap::key::composite::Ref::Mouse(
                            smart_keymap::key::mouse::Ref::WheelUp,
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(25),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(55),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(55),
                        )),
                        None,
                        None,
                        None,
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(77),
                        )),
                        Some(smart_keymap::key::composite::Ref::Mouse(
                            smart_keymap::key::mouse::Ref::WheelRight,
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(29),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(52),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(52),
                        )),
                        None,
                        None,
                        None,
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(73),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        Some(smart_keymap::key::composite::Ref::Callback(
                            smart_keymap::key::callback::Ref(6),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(0),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
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
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
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
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(0),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
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
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
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
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(0),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
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
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
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
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::TapHold(smart_keymap::key::tap_hold::Ref(
                        16,
                    )),
                    [
                        Some(smart_keymap::key::composite::Ref::TapHold(
                            smart_keymap::key::tap_hold::Ref(17),
                        )),
                        Some(smart_keymap::key::composite::Ref::TapHold(
                            smart_keymap::key::tap_hold::Ref(18),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(55),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(17),
                        )),
                        None,
                        None,
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::TapHold(smart_keymap::key::tap_hold::Ref(
                        20,
                    )),
                    [
                        Some(smart_keymap::key::composite::Ref::TapHold(
                            smart_keymap::key::tap_hold::Ref(21),
                        )),
                        Some(smart_keymap::key::composite::Ref::TapHold(
                            smart_keymap::key::tap_hold::Ref(22),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(39),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(18),
                        )),
                        None,
                        None,
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::TapHold(smart_keymap::key::tap_hold::Ref(
                        23,
                    )),
                    [
                        Some(smart_keymap::key::composite::Ref::TapHold(
                            smart_keymap::key::tap_hold::Ref(24),
                        )),
                        Some(smart_keymap::key::composite::Ref::TapHold(
                            smart_keymap::key::tap_hold::Ref(25),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(45),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(19),
                        )),
                        None,
                        None,
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::TapHold(smart_keymap::key::tap_hold::Ref(
                        27,
                    )),
                    [
                        Some(smart_keymap::key::composite::Ref::TapHold(
                            smart_keymap::key::tap_hold::Ref(28),
                        )),
                        Some(smart_keymap::key::composite::Ref::TapHold(
                            smart_keymap::key::tap_hold::Ref(29),
                        )),
                        None,
                        None,
                        None,
                        None,
                        Some(smart_keymap::key::composite::Ref::Mouse(
                            smart_keymap::key::mouse::Ref::Button(1),
                        )),
                        Some(smart_keymap::key::composite::Ref::Consumer(
                            smart_keymap::key::consumer::Ref::UsageCode(205),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::TapHold(smart_keymap::key::tap_hold::Ref(
                        30,
                    )),
                    [
                        Some(smart_keymap::key::composite::Ref::TapHold(
                            smart_keymap::key::tap_hold::Ref(31),
                        )),
                        Some(smart_keymap::key::composite::Ref::TapHold(
                            smart_keymap::key::tap_hold::Ref(32),
                        )),
                        None,
                        None,
                        None,
                        None,
                        Some(smart_keymap::key::composite::Ref::Mouse(
                            smart_keymap::key::mouse::Ref::Button(2),
                        )),
                        Some(smart_keymap::key::composite::Ref::Consumer(
                            smart_keymap::key::consumer::Ref::UsageCode(183),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::TapHold(smart_keymap::key::tap_hold::Ref(
                        33,
                    )),
                    [
                        Some(smart_keymap::key::composite::Ref::TapHold(
                            smart_keymap::key::tap_hold::Ref(34),
                        )),
                        Some(smart_keymap::key::composite::Ref::TapHold(
                            smart_keymap::key::tap_hold::Ref(35),
                        )),
                        None,
                        None,
                        None,
                        None,
                        Some(smart_keymap::key::composite::Ref::Mouse(
                            smart_keymap::key::mouse::Ref::Button(3),
                        )),
                        Some(smart_keymap::key::composite::Ref::Consumer(
                            smart_keymap::key::consumer::Ref::UsageCode(226),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(0),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
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
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
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
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(0),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
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
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
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
                smart_keymap::key::layered::LayeredKey::new(
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(0),
                    ),
                    [
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
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
                        Some(smart_keymap::key::composite::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
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
        smart_keymap::key::sticky::System::new([]),
        smart_keymap::key::tap_dance::System::new([
            smart_keymap::key::tap_dance::Key::from_definitions(&[
                smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(0),
                ),
                smart_keymap::key::composite::Ref::Callback(smart_keymap::key::callback::Ref(0)),
            ]),
            smart_keymap::key::tap_dance::Key::from_definitions(&[
                smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(0),
                ),
                smart_keymap::key::composite::Ref::Callback(smart_keymap::key::callback::Ref(1)),
            ]),
            smart_keymap::key::tap_dance::Key::from_definitions(&[
                smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(0),
                ),
                smart_keymap::key::composite::Ref::Callback(smart_keymap::key::callback::Ref(2)),
            ]),
            smart_keymap::key::tap_dance::Key::from_definitions(&[
                smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(0),
                ),
                smart_keymap::key::composite::Ref::Layered(
                    smart_keymap::key::layered::Ref::Modifier(0),
                ),
            ]),
            smart_keymap::key::tap_dance::Key::from_definitions(&[
                smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(0),
                ),
                smart_keymap::key::composite::Ref::Layered(
                    smart_keymap::key::layered::Ref::Modifier(1),
                ),
            ]),
            smart_keymap::key::tap_dance::Key::from_definitions(&[
                smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(0),
                ),
                smart_keymap::key::composite::Ref::Layered(
                    smart_keymap::key::layered::Ref::Modifier(2),
                ),
            ]),
            smart_keymap::key::tap_dance::Key::from_definitions(&[
                smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(0),
                ),
                smart_keymap::key::composite::Ref::Layered(
                    smart_keymap::key::layered::Ref::Modifier(3),
                ),
            ]),
            smart_keymap::key::tap_dance::Key::from_definitions(&[
                smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(0),
                ),
                smart_keymap::key::composite::Ref::Layered(
                    smart_keymap::key::layered::Ref::Modifier(4),
                ),
            ]),
            smart_keymap::key::tap_dance::Key::from_definitions(&[
                smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(0),
                ),
                smart_keymap::key::composite::Ref::Layered(
                    smart_keymap::key::layered::Ref::Modifier(5),
                ),
            ]),
            smart_keymap::key::tap_dance::Key::from_definitions(&[
                smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(0),
                ),
                smart_keymap::key::composite::Ref::Layered(
                    smart_keymap::key::layered::Ref::Modifier(6),
                ),
            ]),
            smart_keymap::key::tap_dance::Key::from_definitions(&[
                smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(0),
                ),
                smart_keymap::key::composite::Ref::Layered(
                    smart_keymap::key::layered::Ref::Modifier(7),
                ),
            ]),
            smart_keymap::key::tap_dance::Key::from_definitions(&[
                smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(0),
                ),
                smart_keymap::key::composite::Ref::Layered(
                    smart_keymap::key::layered::Ref::Modifier(8),
                ),
            ]),
            smart_keymap::key::tap_dance::Key::from_definitions(&[
                smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(0),
                ),
                smart_keymap::key::composite::Ref::Layered(
                    smart_keymap::key::layered::Ref::Modifier(9),
                ),
            ]),
            smart_keymap::key::tap_dance::Key::from_definitions(&[
                smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(0),
                ),
                smart_keymap::key::composite::Ref::Layered(
                    smart_keymap::key::layered::Ref::Modifier(10),
                ),
            ]),
            smart_keymap::key::tap_dance::Key::from_definitions(&[
                smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(0),
                ),
                smart_keymap::key::composite::Ref::Layered(
                    smart_keymap::key::layered::Ref::Modifier(11),
                ),
            ]),
            smart_keymap::key::tap_dance::Key::from_definitions(&[
                smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(0),
                ),
                smart_keymap::key::composite::Ref::Layered(
                    smart_keymap::key::layered::Ref::Modifier(12),
                ),
            ]),
            smart_keymap::key::tap_dance::Key::from_definitions(&[
                smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(0),
                ),
                smart_keymap::key::composite::Ref::Layered(
                    smart_keymap::key::layered::Ref::Modifier(13),
                ),
            ]),
            smart_keymap::key::tap_dance::Key::from_definitions(&[
                smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(0),
                ),
                smart_keymap::key::composite::Ref::Layered(
                    smart_keymap::key::layered::Ref::Modifier(14),
                ),
            ]),
            smart_keymap::key::tap_dance::Key::from_definitions(&[
                smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(0),
                ),
                smart_keymap::key::composite::Ref::Layered(
                    smart_keymap::key::layered::Ref::Modifier(15),
                ),
            ]),
            smart_keymap::key::tap_dance::Key::from_definitions(&[
                smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(0),
                ),
                smart_keymap::key::composite::Ref::Layered(
                    smart_keymap::key::layered::Ref::Modifier(16),
                ),
            ]),
            smart_keymap::key::tap_dance::Key::from_definitions(&[
                smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(0),
                ),
                smart_keymap::key::composite::Ref::Layered(
                    smart_keymap::key::layered::Ref::Modifier(17),
                ),
            ]),
            smart_keymap::key::tap_dance::Key::from_definitions(&[
                smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(0),
                ),
                smart_keymap::key::composite::Ref::Callback(smart_keymap::key::callback::Ref(3)),
            ]),
            smart_keymap::key::tap_dance::Key::from_definitions(&[
                smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(0),
                ),
                smart_keymap::key::composite::Ref::Callback(smart_keymap::key::callback::Ref(4)),
            ]),
            smart_keymap::key::tap_dance::Key::from_definitions(&[
                smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(0),
                ),
                smart_keymap::key::composite::Ref::Callback(smart_keymap::key::callback::Ref(5)),
            ]),
        ]),
        smart_keymap::key::tap_hold::System::new([
            smart_keymap::key::tap_hold::Key {
                tap: smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(4),
                ),
                hold: smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::Modifiers(4),
                ),
            },
            smart_keymap::key::tap_hold::Key {
                tap: smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(4),
                ),
                hold: smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::Modifiers(4),
                ),
            },
            smart_keymap::key::tap_hold::Key {
                tap: smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(18),
                ),
                hold: smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::Modifiers(8),
                ),
            },
            smart_keymap::key::tap_hold::Key {
                tap: smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(22),
                ),
                hold: smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::Modifiers(8),
                ),
            },
            smart_keymap::key::tap_hold::Key {
                tap: smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(8),
                ),
                hold: smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::Modifiers(1),
                ),
            },
            smart_keymap::key::tap_hold::Key {
                tap: smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(7),
                ),
                hold: smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::Modifiers(1),
                ),
            },
            smart_keymap::key::tap_hold::Key {
                tap: smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(24),
                ),
                hold: smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::Modifiers(2),
                ),
            },
            smart_keymap::key::tap_hold::Key {
                tap: smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(9),
                ),
                hold: smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::Modifiers(2),
                ),
            },
            smart_keymap::key::tap_hold::Key {
                tap: smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(11),
                ),
                hold: smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::Modifiers(32),
                ),
            },
            smart_keymap::key::tap_hold::Key {
                tap: smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(13),
                ),
                hold: smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::Modifiers(32),
                ),
            },
            smart_keymap::key::tap_hold::Key {
                tap: smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(23),
                ),
                hold: smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::Modifiers(1),
                ),
            },
            smart_keymap::key::tap_hold::Key {
                tap: smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(14),
                ),
                hold: smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::Modifiers(1),
                ),
            },
            smart_keymap::key::tap_hold::Key {
                tap: smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(17),
                ),
                hold: smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::Modifiers(128),
                ),
            },
            smart_keymap::key::tap_hold::Key {
                tap: smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(15),
                ),
                hold: smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::Modifiers(128),
                ),
            },
            smart_keymap::key::tap_hold::Key {
                tap: smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(22),
                ),
                hold: smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::Modifiers(4),
                ),
            },
            smart_keymap::key::tap_hold::Key {
                tap: smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(51),
                ),
                hold: smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::Modifiers(4),
                ),
            },
            smart_keymap::key::tap_hold::Key {
                tap: smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(43),
                ),
                hold: smart_keymap::key::composite::Ref::Layered(
                    smart_keymap::key::layered::Ref::Modifier(18),
                ),
            },
            smart_keymap::key::tap_hold::Key {
                tap: smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(43),
                ),
                hold: smart_keymap::key::composite::Ref::Layered(
                    smart_keymap::key::layered::Ref::Modifier(19),
                ),
            },
            smart_keymap::key::tap_hold::Key {
                tap: smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(43),
                ),
                hold: smart_keymap::key::composite::Ref::Layered(
                    smart_keymap::key::layered::Ref::Modifier(20),
                ),
            },
            smart_keymap::key::tap_hold::Key {
                tap: smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(43),
                ),
                hold: smart_keymap::key::composite::Ref::Layered(
                    smart_keymap::key::layered::Ref::Modifier(21),
                ),
            },
            smart_keymap::key::tap_hold::Key {
                tap: smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(41),
                ),
                hold: smart_keymap::key::composite::Ref::Layered(
                    smart_keymap::key::layered::Ref::Modifier(22),
                ),
            },
            smart_keymap::key::tap_hold::Key {
                tap: smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(41),
                ),
                hold: smart_keymap::key::composite::Ref::Layered(
                    smart_keymap::key::layered::Ref::Modifier(23),
                ),
            },
            smart_keymap::key::tap_hold::Key {
                tap: smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(41),
                ),
                hold: smart_keymap::key::composite::Ref::Layered(
                    smart_keymap::key::layered::Ref::Modifier(24),
                ),
            },
            smart_keymap::key::tap_hold::Key {
                tap: smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(44),
                ),
                hold: smart_keymap::key::composite::Ref::Layered(
                    smart_keymap::key::layered::Ref::Modifier(25),
                ),
            },
            smart_keymap::key::tap_hold::Key {
                tap: smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(44),
                ),
                hold: smart_keymap::key::composite::Ref::Layered(
                    smart_keymap::key::layered::Ref::Modifier(26),
                ),
            },
            smart_keymap::key::tap_hold::Key {
                tap: smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(44),
                ),
                hold: smart_keymap::key::composite::Ref::Layered(
                    smart_keymap::key::layered::Ref::Modifier(27),
                ),
            },
            smart_keymap::key::tap_hold::Key {
                tap: smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(76),
                ),
                hold: smart_keymap::key::composite::Ref::Layered(
                    smart_keymap::key::layered::Ref::Modifier(28),
                ),
            },
            smart_keymap::key::tap_hold::Key {
                tap: smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(40),
                ),
                hold: smart_keymap::key::composite::Ref::Layered(
                    smart_keymap::key::layered::Ref::Modifier(29),
                ),
            },
            smart_keymap::key::tap_hold::Key {
                tap: smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(40),
                ),
                hold: smart_keymap::key::composite::Ref::Layered(
                    smart_keymap::key::layered::Ref::Modifier(30),
                ),
            },
            smart_keymap::key::tap_hold::Key {
                tap: smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(40),
                ),
                hold: smart_keymap::key::composite::Ref::Layered(
                    smart_keymap::key::layered::Ref::Modifier(31),
                ),
            },
            smart_keymap::key::tap_hold::Key {
                tap: smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(42),
                ),
                hold: smart_keymap::key::composite::Ref::Layered(
                    smart_keymap::key::layered::Ref::Modifier(32),
                ),
            },
            smart_keymap::key::tap_hold::Key {
                tap: smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(42),
                ),
                hold: smart_keymap::key::composite::Ref::Layered(
                    smart_keymap::key::layered::Ref::Modifier(33),
                ),
            },
            smart_keymap::key::tap_hold::Key {
                tap: smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(42),
                ),
                hold: smart_keymap::key::composite::Ref::Layered(
                    smart_keymap::key::layered::Ref::Modifier(34),
                ),
            },
            smart_keymap::key::tap_hold::Key {
                tap: smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(76),
                ),
                hold: smart_keymap::key::composite::Ref::Layered(
                    smart_keymap::key::layered::Ref::Modifier(35),
                ),
            },
            smart_keymap::key::tap_hold::Key {
                tap: smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(76),
                ),
                hold: smart_keymap::key::composite::Ref::Layered(
                    smart_keymap::key::layered::Ref::Modifier(36),
                ),
            },
            smart_keymap::key::tap_hold::Key {
                tap: smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(76),
                ),
                hold: smart_keymap::key::composite::Ref::Layered(
                    smart_keymap::key::layered::Ref::Modifier(37),
                ),
            },
        ]),
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
