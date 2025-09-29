/// Types and initial data used for constructing [KEYMAP].
pub mod init {
    use crate as smart_keymap;

    /// Number of layers supported by the [smart_keymap::key::layered] implementation.
    pub const LAYERED_LAYER_COUNT: usize = 0;

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

    const CALLBACK: usize = 0;
    const CHORDED: usize = 0;
    const CHORDED_AUXILIARY: usize = 0;
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
    pub const KEY_COUNT: usize = 60;

    /// The key references.
    pub const KEY_REFS: [Ref; KEY_COUNT] = [
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(53)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(30)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(31)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(32)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(33)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(34)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(35)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(36)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(37)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(38)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(39)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(76)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(43)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(52)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(54)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(55)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(19)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(28)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(9)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(10)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(6)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(21)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(15)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(42)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(41)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(4)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(18)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(8)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(24)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(12)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(7)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(11)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(23)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(17)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(22)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(40)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::Modifiers(2)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(51)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(20)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(13)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(14)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(27)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(5)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(16)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(26)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(25)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(29)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::Modifiers(
            32,
        )),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::Modifiers(1)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::Modifiers(8)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::Modifiers(4)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(43)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(41)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(44)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(42)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(40)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(76)),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::Modifiers(
            64,
        )),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::Modifiers(
            128,
        )),
        smart_keymap::key::composite::Ref::Keyboard(smart_keymap::key::keyboard::Ref::Modifiers(
            16,
        )),
    ];

    /// The keymap config.
    pub const CONFIG: smart_keymap::key::composite::Config = smart_keymap::key::composite::Config {
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
        smart_keymap::key::callback::System::new([]),
        smart_keymap::key::chorded::System::new([], []),
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
