/// Types and initial data used for constructing [KEYMAP].
pub mod init {
    use crate as smart_keymap;

    /// Number of layers supported by the [smart_keymap::key::layered] implementation.
    pub const LAYER_COUNT: usize = 0;

    /// The maximum number of keys in a chord.
    pub const MAX_CHORD_SIZE: usize = 2;

    /// The maximum number of chords.
    pub const MAX_CHORDS: usize = 1;

    /// The maximum number of overlapping chords for a chorded key.
    pub const MAX_OVERLAPPING_CHORD_SIZE: usize = 1;

    /// The tap-dance definitions.
    pub const MAX_TAP_DANCE_DEFINITIONS: usize = 0;

    pub use smart_keymap::key::composite::Ref;

    pub use smart_keymap::key::composite::Context;

    pub use smart_keymap::key::composite::Event;

    pub use smart_keymap::key::composite::PendingKeyState;

    pub use smart_keymap::key::composite::KeyState;

    const CALLBACK: usize = 0;
    const CHORDED: usize = 1;
    const CHORDED_AUXILIARY: usize = 1;
    const KEYBOARD: usize = 0;
    const LAYERED: usize = 0;
    const LAYER_MODIFIERS: usize = 0;
    const STICKY: usize = 0;
    const TAP_DANCE: usize = 0;
    const TAP_HOLD: usize = 0;

    /// The System type
    pub type System = smart_keymap::key::composite::System<
        smart_keymap::key::composite::KeyArrays<
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
    pub const KEY_COUNT: usize = 2;

    /// The key references.
    pub const KEY_REFS: [Ref; KEY_COUNT] = [
        smart_keymap::key::composite::Ref::Chorded(smart_keymap::key::chorded::Ref::Chorded(0)),
        smart_keymap::key::composite::Ref::Chorded(smart_keymap::key::chorded::Ref::Auxiliary(0)),
    ];

    /// The keymap config.
    pub const CONFIG: smart_keymap::key::composite::Config = smart_keymap::key::composite::Config {
        chorded: smart_keymap::key::chorded::Config {
            chords: smart_keymap::slice::Slice::from_slice(&[
                smart_keymap::key::chorded::ChordIndices::from_slice(&[0, 1]),
            ]),
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
                chords: smart_keymap::slice::Slice::from_slice(&[
                    smart_keymap::key::chorded::ChordIndices::from_slice(&[0, 1]),
                ]),
                ..smart_keymap::key::chorded::DEFAULT_CONFIG
            },
            sticky: smart_keymap::key::sticky::DEFAULT_CONFIG,
            tap_dance: smart_keymap::key::tap_dance::DEFAULT_CONFIG,
            tap_hold: smart_keymap::key::tap_hold::DEFAULT_CONFIG,
            ..smart_keymap::key::composite::DEFAULT_CONFIG
        });

    /// The key system.
    pub const SYSTEM: System = smart_keymap::key::composite::System::array_based(
        smart_keymap::key::callback::System::new([]),
        smart_keymap::key::chorded::System::new(
            [smart_keymap::key::chorded::Key::new(
                &[(
                    0,
                    smart_keymap::key::composite::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(6),
                    ),
                )],
                smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(4),
                ),
            )],
            [smart_keymap::key::chorded::AuxiliaryKey::new(
                smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(5),
                ),
            )],
        ),
        smart_keymap::key::keyboard::System::new([]),
        smart_keymap::key::layered::System::new([], []),
        smart_keymap::key::sticky::System::new([]),
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
