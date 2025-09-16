/// Types and initial data used for constructing [KEYMAP].
pub mod init {
    use crate as smart_keymap;

    /// Number of layers supported by the [smart_keymap::key::layered] implementation.
    pub const LAYER_COUNT: usize = 1;

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

    const KEYBOARD_DATA_LEN: usize = 0;
    const CALLBACK_DATA_LEN: usize = 0;
    const STICKY_DATA_LEN: usize = 0;
    const TAP_DANCE_DATA_LEN: usize = 0;
    const TAP_HOLD_DATA_LEN: usize = 0;
    const LAYER_MODIFIERS_DATA_LEN: usize = 1;
    const LAYERED_DATA_LEN: usize = 1;
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
    pub const KEY_COUNT: usize = 2;

    /// The key references.
    pub const KEY_REFS: [Ref; KEY_COUNT] = [
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Modifier(0)),
        smart_keymap::key::composite::Ref::Layered(smart_keymap::key::layered::Ref::Layered(0)),
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
        smart_keymap::key::keyboard::System::new([]),
        smart_keymap::key::callback::System::new([]),
        smart_keymap::key::sticky::System::new([]),
        smart_keymap::key::tap_dance::System::new([]),
        smart_keymap::key::tap_hold::System::new([]),
        smart_keymap::key::layered::System::new(
            [smart_keymap::key::layered::ModifierKey::hold(1)],
            [smart_keymap::key::layered::LayeredKey::new(
                smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(4),
                ),
                [Some(smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(5),
                ))],
            )],
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
