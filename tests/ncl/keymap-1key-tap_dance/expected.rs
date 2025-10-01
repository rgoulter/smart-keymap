/// Types and initial data used for constructing [KEYMAP].
pub mod init {
    use crate as smart_keymap;

    /// Number of instructions used by the [crate::key::automation] implementation.
    pub const AUTOMATION_INSTRUCTION_COUNT: usize = 0;

    /// Number of layers supported by the [smart_keymap::key::layered] implementation.
    pub const LAYERED_LAYER_COUNT: usize = 0;

    /// The maximum number of keys in a chord.
    pub const CHORDED_MAX_CHORD_SIZE: usize = 0;

    /// The maximum number of chords.
    pub const CHORDED_MAX_CHORDS: usize = 0;

    /// The maximum number of overlapping chords for a chorded key.
    pub const CHORDED_MAX_OVERLAPPING_CHORD_SIZE: usize = 0;

    /// The tap-dance definitions.
    pub const TAP_DANCE_MAX_DEFINITIONS: usize = 3;

    pub use smart_keymap::key::composite::Ref;

    pub use smart_keymap::key::composite::Context;

    pub use smart_keymap::key::composite::Event;

    pub use smart_keymap::key::composite::PendingKeyState;

    pub use smart_keymap::key::composite::KeyState;

    const AUTOMATION: usize = 0;
    const CALLBACK: usize = 0;
    const CHORDED: usize = 0;
    const CHORDED_AUXILIARY: usize = 0;
    const KEYBOARD: usize = 0;
    const LAYERED: usize = 0;
    const LAYER_MODIFIERS: usize = 0;
    const STICKY: usize = 0;
    const TAP_DANCE: usize = 1;
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
    pub const KEY_COUNT: usize = 1;

    /// The key references.
    pub const KEY_REFS: [Ref; KEY_COUNT] = [smart_keymap::key::composite::Ref::TapDance(
        smart_keymap::key::tap_dance::Ref(0),
    )];

    /// The keymap config.
    pub const CONFIG: smart_keymap::key::composite::Config = smart_keymap::key::composite::Config {
        automation: smart_keymap::key::automation::Config::new(),
        chorded: smart_keymap::key::chorded::Config::new(),
        sticky: smart_keymap::key::sticky::Config::new(),
        tap_dance: smart_keymap::key::tap_dance::Config::new(),
        tap_hold: smart_keymap::key::tap_hold::Config::new(),
        ..smart_keymap::key::composite::Config::new()
    };

    /// Initial [Context] value.
    pub const CONTEXT: Context =
        smart_keymap::key::composite::Context::from_config(smart_keymap::key::composite::Config {
            automation: smart_keymap::key::automation::Config::new(),
            chorded: smart_keymap::key::chorded::Config::new(),
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
        smart_keymap::key::keyboard::System::new([]),
        smart_keymap::key::layered::System::new([], []),
        smart_keymap::key::sticky::System::new([]),
        smart_keymap::key::tap_dance::System::new([
            smart_keymap::key::tap_dance::Key::from_definitions(&[
                smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(4),
                ),
                smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(5),
                ),
                smart_keymap::key::composite::Ref::Keyboard(
                    smart_keymap::key::keyboard::Ref::KeyCode(6),
                ),
            ]),
        ]),
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
