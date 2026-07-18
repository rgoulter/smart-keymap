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
    pub const TAP_DANCE_MAX_DEFINITIONS: usize = 0;

    const KEYBOARD: usize = 0;

    /// Per-keymap composite key system (generated; only families used by this keymap).
    pub mod key_system {
        use crate as smart_keymap;
        use smart_keymap::key;
        use smart_keymap::keymap;

        /// Aggregate key reference.
        #[derive(serde::Deserialize, Debug, Clone, Copy, PartialEq)]
        pub enum Ref {
            /// [smart_keymap::key::keyboard] variant.
            Keyboard(smart_keymap::key::keyboard::Ref),
        }

        /// Aggregate config (no configurable families in this keymap).
        #[derive(serde::Deserialize, Debug, Clone, Copy, PartialEq)]
        pub struct Config {}
        impl Default for Config {
            fn default() -> Self {
                Self::new()
            }
        }
        impl Config {
            /// Constructs a new [Config] with defaults.
            pub const fn new() -> Self {
                Self {}
            }
        }

        /// Aggregate context.
        #[derive(Debug, Clone, Copy)]
        pub struct Context {
            keymap_context: smart_keymap::keymap::KeymapContext,
            keyboard: smart_keymap::key::keyboard::Context,
        }

        impl Context {
            /// Constructs a [Context] from the given [Config].
            pub const fn from_config(config: Config) -> Self {
                let _ = &config;
                Self {
                    keymap_context: smart_keymap::keymap::KeymapContext::new(),
                    keyboard: smart_keymap::key::keyboard::Context,
                }
            }
        }

        impl Default for Context {
            fn default() -> Self {
                Self::from_config(Config::new())
            }
        }

        impl key::Context for Context {
            type Event = Event;

            fn handle_event(
                &mut self,
                _event: key::Event<Self::Event>,
            ) -> key::KeyEvents<Self::Event> {
                let mut pke = key::KeyEvents::no_events();

                pke
            }
        }

        impl keymap::SetKeymapContext for Context {
            fn set_keymap_context(&mut self, context: keymap::KeymapContext) {
                self.keymap_context = context;
            }
        }

        /// Aggregate event.
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum Event {
            /// [smart_keymap::key::keyboard] variant.
            Keyboard(smart_keymap::key::keyboard::Event),
        }

        impl From<smart_keymap::key::keyboard::Event> for Event {
            fn from(v: smart_keymap::key::keyboard::Event) -> Self {
                Event::Keyboard(v)
            }
        }
        impl TryFrom<Event> for smart_keymap::key::keyboard::Event {
            type Error = smart_keymap::key::EventError;
            fn try_from(v: Event) -> Result<Self, Self::Error> {
                match v {
                    Event::Keyboard(v) => Ok(v),
                    _ => Err(smart_keymap::key::EventError::UnmappableEvent),
                }
            }
        }

        /// Aggregate pending key state.
        #[derive(Debug, Clone, PartialEq)]
        #[allow(clippy::large_enum_variant)]
        pub enum PendingKeyState {
            /// [smart_keymap::key::keyboard] variant.
            Keyboard(smart_keymap::key::keyboard::PendingKeyState),
        }

        impl From<smart_keymap::key::keyboard::PendingKeyState> for PendingKeyState {
            fn from(pks: smart_keymap::key::keyboard::PendingKeyState) -> Self {
                PendingKeyState::Keyboard(pks)
            }
        }

        /// Aggregate key state.
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum KeyState {
            /// No-op key state (e.g. auxiliary chorded keys).
            NoOp,
            /// [smart_keymap::key::keyboard] key state.
            Keyboard(smart_keymap::key::keyboard::KeyState),
        }

        impl From<key::NoOpKeyState> for KeyState {
            fn from(_: key::NoOpKeyState) -> Self {
                KeyState::NoOp
            }
        }

        impl From<smart_keymap::key::keyboard::KeyState> for KeyState {
            fn from(ks: smart_keymap::key::keyboard::KeyState) -> Self {
                KeyState::Keyboard(ks)
            }
        }

        /// Aggregate [key::System] for this keymap.
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct System {
            keyboard: smart_keymap::key::keyboard::System<
                Ref,
                [smart_keymap::key::keyboard::Key; { crate::init::KEYBOARD }],
            >,
        }

        impl System {
            /// Constructs the system from data-carrying subsystems.
            #[allow(clippy::too_many_arguments)]
            pub const fn new(
                keyboard: smart_keymap::key::keyboard::System<
                    Ref,
                    [smart_keymap::key::keyboard::Key; { crate::init::KEYBOARD }],
                >,
            ) -> Self {
                Self { keyboard }
            }
        }

        impl key::System<Ref> for System {
            type Ref = Ref;
            type Context = Context;
            type Event = Event;
            type PendingKeyState = PendingKeyState;
            type KeyState = KeyState;

            fn new_pressed_key(
                &self,
                keymap_index: u16,
                context: &Self::Context,
                key_ref: Ref,
            ) -> (
                key::PressedKeyResult<Ref, Self::PendingKeyState, Self::KeyState>,
                key::KeyEvents<Self::Event>,
            ) {
                match key_ref {
                    Ref::Keyboard(key_ref) => {
                        let (pkr, pke) =
                            self.keyboard
                                .new_pressed_key(keymap_index, &context.keyboard, key_ref);
                        (pkr.into_result(), pke.into_events())
                    }
                }
            }

            fn update_pending_state(
                &self,
                pending_state: &mut Self::PendingKeyState,
                keymap_index: u16,
                context: &Self::Context,
                key_ref: Ref,
                event: key::Event<Self::Event>,
            ) -> (Option<key::NewPressedKey<Ref>>, key::KeyEvents<Self::Event>) {
                match (key_ref, pending_state) {
                    _ => panic!("no pending key systems in this key_system"),
                }
            }

            fn update_state(
                &self,
                key_state: &mut Self::KeyState,
                key_ref: &Self::Ref,
                context: &Self::Context,
                keymap_index: u16,
                event: key::Event<Self::Event>,
            ) -> key::KeyEvents<Self::Event> {
                match (key_ref, key_state) {
                    (Ref::Keyboard(key_ref), KeyState::Keyboard(key_state)) => {
                        if let Ok(event) = event.try_into_key_event() {
                            self.keyboard
                                .update_state(
                                    key_state,
                                    key_ref,
                                    &context.keyboard,
                                    keymap_index,
                                    event,
                                )
                                .into_events()
                        } else {
                            smart_keymap::key::KeyEvents::no_events()
                        }
                    }
                    (_, _) => smart_keymap::key::KeyEvents::no_events(),
                }
            }

            fn key_output(
                &self,
                key_ref: &Self::Ref,
                key_state: &Self::KeyState,
            ) -> Option<key::KeyOutput> {
                match (key_ref, key_state) {
                    (Ref::Keyboard(r), KeyState::Keyboard(ks)) => self.keyboard.key_output(r, ks),
                    (_, _) => None,
                }
            }
        }
    }

    pub use key_system::Context;
    pub use key_system::Event;
    pub use key_system::KeyState;
    pub use key_system::PendingKeyState;
    pub use key_system::Ref;
    pub use key_system::System;

    /// The number of keys in the keymap.
    pub const KEY_COUNT: usize = 60;

    /// The key references.
    pub const KEY_REFS: [Ref; KEY_COUNT] = [
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(53)),
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(30)),
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(31)),
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(32)),
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(33)),
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(34)),
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(35)),
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(36)),
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(37)),
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(38)),
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(39)),
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(76)),
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(43)),
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(52)),
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(54)),
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(55)),
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(19)),
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(28)),
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(9)),
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(10)),
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(6)),
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(21)),
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(15)),
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(42)),
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(41)),
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(4)),
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(18)),
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(8)),
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(24)),
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(12)),
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(7)),
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(11)),
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(23)),
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(17)),
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(22)),
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(40)),
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::Modifiers(2)),
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(51)),
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(20)),
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(13)),
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(14)),
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(27)),
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(5)),
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(16)),
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(26)),
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(25)),
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(29)),
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::Modifiers(32)),
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::Modifiers(1)),
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::Modifiers(8)),
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::Modifiers(4)),
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(43)),
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(41)),
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(44)),
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(42)),
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(40)),
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(76)),
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::Modifiers(64)),
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::Modifiers(128)),
        key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::Modifiers(16)),
    ];

    /// The keymap config.
    pub const CONFIG: key_system::Config = key_system::Config::new();

    /// Initial [Context] value.
    pub const CONTEXT: Context = key_system::Context::from_config(key_system::Config::new());

    /// The key system.
    pub const SYSTEM: System =
        key_system::System::new(smart_keymap::key::keyboard::System::new([]));

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
