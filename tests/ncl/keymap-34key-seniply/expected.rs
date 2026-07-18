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

    const KEYBOARD: usize = 28;
    const LAYERED: usize = 34;
    const LAYER_MODIFIERS: usize = 8;
    const STICKY: usize = 16;

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
            /// [smart_keymap::key::layered] variant.
            Layered(smart_keymap::key::layered::Ref),
            /// [smart_keymap::key::sticky] variant.
            Sticky(smart_keymap::key::sticky::Ref),
        }

        /// Aggregate config for families used by this keymap.
        #[derive(serde::Deserialize, Debug, Clone, Copy, PartialEq)]
        pub struct Config {
            /// Config for [smart_keymap::key::layered].
            pub layered: smart_keymap::key::layered::Config,
            /// Config for [smart_keymap::key::sticky].
            pub sticky: smart_keymap::key::sticky::Config,
        }
        impl Default for Config {
            fn default() -> Self {
                Self::new()
            }
        }
        impl Config {
            /// Constructs a new [Config] with defaults.
            pub const fn new() -> Self {
                Self {
                    layered: smart_keymap::key::layered::Config::new(),
                    sticky: smart_keymap::key::sticky::Config::new(),
                }
            }
        }

        /// Aggregate context.
        #[derive(Debug, Clone, Copy)]
        pub struct Context {
            keymap_context: smart_keymap::keymap::KeymapContext,
            keyboard: smart_keymap::key::keyboard::Context,
            layered: smart_keymap::key::layered::Context<{ crate::init::LAYERED_LAYER_COUNT }>,
            sticky: smart_keymap::key::sticky::Context,
        }

        impl Context {
            /// Constructs a [Context] from the given [Config].
            pub const fn from_config(config: Config) -> Self {
                let _ = &config;
                Self {
                    keymap_context: smart_keymap::keymap::KeymapContext::new(),
                    keyboard: smart_keymap::key::keyboard::Context,
                    layered: smart_keymap::key::layered::Context::from_config(config.layered),
                    sticky: smart_keymap::key::sticky::Context::from_config(config.sticky),
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
                event: key::Event<Self::Event>,
            ) -> key::KeyEvents<Self::Event> {
                let mut pke = key::KeyEvents::no_events();
                if let Ok(e) = event.try_into_key_event() {
                    pke.extend(self.layered.handle_event(e).into_events());
                }
                if let Ok(e) = event.try_into_key_event() {
                    pke.extend(self.sticky.handle_event(e).into_events());
                }
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
            /// [smart_keymap::key::layered] variant.
            Layered(smart_keymap::key::layered::LayerEvent),
            /// [smart_keymap::key::sticky] variant.
            Sticky(smart_keymap::key::sticky::Event),
        }

        impl From<smart_keymap::key::keyboard::Event> for Event {
            fn from(v: smart_keymap::key::keyboard::Event) -> Self {
                Event::Keyboard(v)
            }
        }
        impl From<smart_keymap::key::layered::LayerEvent> for Event {
            fn from(v: smart_keymap::key::layered::LayerEvent) -> Self {
                Event::Layered(v)
            }
        }
        impl From<smart_keymap::key::sticky::Event> for Event {
            fn from(v: smart_keymap::key::sticky::Event) -> Self {
                Event::Sticky(v)
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
        impl TryFrom<Event> for smart_keymap::key::layered::LayerEvent {
            type Error = smart_keymap::key::EventError;
            fn try_from(v: Event) -> Result<Self, Self::Error> {
                match v {
                    Event::Layered(v) => Ok(v),
                    _ => Err(smart_keymap::key::EventError::UnmappableEvent),
                }
            }
        }
        impl TryFrom<Event> for smart_keymap::key::sticky::Event {
            type Error = smart_keymap::key::EventError;
            fn try_from(v: Event) -> Result<Self, Self::Error> {
                match v {
                    Event::Sticky(v) => Ok(v),
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
            /// [smart_keymap::key::layered] variant.
            Layered(smart_keymap::key::layered::PendingKeyState),
            /// [smart_keymap::key::sticky] variant.
            Sticky(smart_keymap::key::sticky::PendingKeyState),
        }

        impl From<smart_keymap::key::keyboard::PendingKeyState> for PendingKeyState {
            fn from(pks: smart_keymap::key::keyboard::PendingKeyState) -> Self {
                PendingKeyState::Keyboard(pks)
            }
        }
        impl From<smart_keymap::key::layered::PendingKeyState> for PendingKeyState {
            fn from(pks: smart_keymap::key::layered::PendingKeyState) -> Self {
                PendingKeyState::Layered(pks)
            }
        }
        impl From<smart_keymap::key::sticky::PendingKeyState> for PendingKeyState {
            fn from(pks: smart_keymap::key::sticky::PendingKeyState) -> Self {
                PendingKeyState::Sticky(pks)
            }
        }

        /// Aggregate key state.
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum KeyState {
            /// No-op key state (e.g. auxiliary chorded keys).
            NoOp,
            /// [smart_keymap::key::keyboard] key state.
            Keyboard(smart_keymap::key::keyboard::KeyState),
            /// [smart_keymap::key::layered] key state.
            LayerModifier(smart_keymap::key::layered::ModifierKeyState),
            /// [smart_keymap::key::sticky] key state.
            Sticky(smart_keymap::key::sticky::KeyState),
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
        impl From<smart_keymap::key::layered::ModifierKeyState> for KeyState {
            fn from(ks: smart_keymap::key::layered::ModifierKeyState) -> Self {
                KeyState::LayerModifier(ks)
            }
        }
        impl From<smart_keymap::key::sticky::KeyState> for KeyState {
            fn from(ks: smart_keymap::key::sticky::KeyState) -> Self {
                KeyState::Sticky(ks)
            }
        }

        /// Aggregate [key::System] for this keymap.
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct System {
            keyboard: smart_keymap::key::keyboard::System<
                Ref,
                [smart_keymap::key::keyboard::Key; { crate::init::KEYBOARD }],
            >,
            layered: smart_keymap::key::layered::System<
                Ref,
                [smart_keymap::key::layered::ModifierKey; { crate::init::LAYER_MODIFIERS }],
                [smart_keymap::key::layered::LayeredKey<Ref, { crate::init::LAYERED_LAYER_COUNT }>;
                    { crate::init::LAYERED }],
                { crate::init::LAYERED_LAYER_COUNT },
            >,
            sticky: smart_keymap::key::sticky::System<
                Ref,
                [smart_keymap::key::sticky::Key; { crate::init::STICKY }],
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
                layered: smart_keymap::key::layered::System<
                    Ref,
                    [smart_keymap::key::layered::ModifierKey; { crate::init::LAYER_MODIFIERS }],
                    [smart_keymap::key::layered::LayeredKey<
                        Ref,
                        { crate::init::LAYERED_LAYER_COUNT },
                    >; { crate::init::LAYERED }],
                    { crate::init::LAYERED_LAYER_COUNT },
                >,
                sticky: smart_keymap::key::sticky::System<
                    Ref,
                    [smart_keymap::key::sticky::Key; { crate::init::STICKY }],
                >,
            ) -> Self {
                Self {
                    keyboard,
                    layered,
                    sticky,
                }
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
                    Ref::Layered(key_ref) => {
                        let (pkr, pke) =
                            self.layered
                                .new_pressed_key(keymap_index, &context.layered, key_ref);
                        (pkr.into_result(), pke.into_events())
                    }
                    Ref::Sticky(key_ref) => {
                        let (pkr, pke) =
                            self.sticky
                                .new_pressed_key(keymap_index, &context.sticky, key_ref);
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
                    (Ref::Layered(key_ref), KeyState::LayerModifier(key_state)) => {
                        if let Ok(event) = event.try_into_key_event() {
                            self.layered
                                .update_state(
                                    key_state,
                                    key_ref,
                                    &context.layered,
                                    keymap_index,
                                    event,
                                )
                                .into_events()
                        } else {
                            smart_keymap::key::KeyEvents::no_events()
                        }
                    }
                    (Ref::Sticky(key_ref), KeyState::Sticky(key_state)) => {
                        if let Ok(event) = event.try_into_key_event() {
                            self.sticky
                                .update_state(
                                    key_state,
                                    key_ref,
                                    &context.sticky,
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
                    (Ref::Layered(r), KeyState::LayerModifier(ks)) => {
                        self.layered.key_output(r, ks)
                    }
                    (Ref::Sticky(r), KeyState::Sticky(ks)) => self.sticky.key_output(r, ks),
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
    pub const KEY_COUNT: usize = 34;

    /// The key references.
    pub const KEY_REFS: [Ref; KEY_COUNT] = [
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(0)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(1)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(2)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(3)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(4)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(5)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(6)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(7)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(8)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(9)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(10)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(11)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(12)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(13)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(14)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(15)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(16)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(17)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(18)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(19)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(20)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(21)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(22)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(23)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(24)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(25)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(26)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(27)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(28)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(29)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(30)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(31)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(32)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(33)),
    ];

    /// The keymap config.
    pub const CONFIG: key_system::Config = key_system::Config {
        layered: smart_keymap::key::layered::Config::new(),
        sticky: smart_keymap::key::sticky::Config::new(),
    };

    /// Initial [Context] value.
    pub const CONTEXT: Context = key_system::Context::from_config(key_system::Config {
        layered: smart_keymap::key::layered::Config::new(),
        sticky: smart_keymap::key::sticky::Config::new(),
    });

    /// The key system.
    pub const SYSTEM: System = key_system::System::new(
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
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(20)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(20),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(41),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(0),
                        )),
                        None,
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(26)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(26),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(1),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(2),
                        )),
                        None,
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(9)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(8),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(3),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(4),
                        )),
                        None,
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(19)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(21),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(5),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(6),
                        )),
                        None,
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(5)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(23),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(73),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(7),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(83),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(13)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(28),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(75),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(69),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(46),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(46),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(15)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(24),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(74),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(64),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(53),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(36),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(24)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(12),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(82),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(65),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(8),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(37),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(28)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(18),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(77),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(66),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(51),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(38),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(52)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(19),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(57),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(71),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(9),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(10),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(4)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(4),
                        )),
                        Some(key_system::Ref::Sticky(smart_keymap::key::sticky::Ref(0))),
                        Some(key_system::Ref::Sticky(smart_keymap::key::sticky::Ref(1))),
                        Some(key_system::Ref::Sticky(smart_keymap::key::sticky::Ref(2))),
                        Some(key_system::Ref::Sticky(smart_keymap::key::sticky::Ref(3))),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(21)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(22),
                        )),
                        Some(key_system::Ref::Sticky(smart_keymap::key::sticky::Ref(4))),
                        Some(key_system::Ref::Sticky(smart_keymap::key::sticky::Ref(5))),
                        Some(key_system::Ref::Sticky(smart_keymap::key::sticky::Ref(6))),
                        Some(key_system::Ref::Sticky(smart_keymap::key::sticky::Ref(7))),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(22)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(7),
                        )),
                        Some(key_system::Ref::Sticky(smart_keymap::key::sticky::Ref(8))),
                        Some(key_system::Ref::Sticky(smart_keymap::key::sticky::Ref(9))),
                        Some(key_system::Ref::Sticky(smart_keymap::key::sticky::Ref(10))),
                        Some(key_system::Ref::Sticky(smart_keymap::key::sticky::Ref(11))),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(23)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(9),
                        )),
                        Some(key_system::Ref::Sticky(smart_keymap::key::sticky::Ref(12))),
                        Some(key_system::Ref::Sticky(smart_keymap::key::sticky::Ref(13))),
                        Some(key_system::Ref::Sticky(smart_keymap::key::sticky::Ref(14))),
                        Some(key_system::Ref::Sticky(smart_keymap::key::sticky::Ref(15))),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(10)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(10),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::Modifiers(64),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(11),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::Modifiers(64),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(16)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(11),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(78),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(68),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(12),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(13),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(17)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(13),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(80),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(61),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(14),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(33),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(8)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(14),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(81),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(62),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(15),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(34),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(12)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(15),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(79),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(63),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(47),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(35),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(18)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(52),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(76),
                        )),
                        Some(key_system::Ref::Layered(
                            smart_keymap::key::layered::Ref::Modifier(0),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(45),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(45),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(29)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(29),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(16),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        None,
                        None,
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(27)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(27),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(17),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        None,
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(101),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(6)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(6),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(18),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(19),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(49),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(43),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(7)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(25),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::Modifiers(8),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(20),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(42),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(25)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(5),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(21),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(22),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(23),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(40),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(14)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(17),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(40),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(67),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(24),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(39),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(11)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(16),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(42),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(58),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(25),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(30),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(54)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(54),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(43),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(59),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(26),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(31),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(55)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(55),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(101),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(60),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(48),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(32),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(56)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(56),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(70),
                        )),
                        Some(key_system::Ref::Layered(
                            smart_keymap::key::layered::Ref::Modifier(1),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(27),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(56),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Layered(smart_keymap::key::layered::Ref::Modifier(2)),
                    [
                        Some(key_system::Ref::Layered(
                            smart_keymap::key::layered::Ref::Modifier(3),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        None,
                        None,
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::Modifiers(2)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::Modifiers(2),
                        )),
                        None,
                        None,
                        Some(key_system::Ref::Layered(
                            smart_keymap::key::layered::Ref::Modifier(4),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(44)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(44),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(40),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(40),
                        )),
                        None,
                        None,
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Layered(smart_keymap::key::layered::Ref::Modifier(5)),
                    [
                        Some(key_system::Ref::Layered(
                            smart_keymap::key::layered::Ref::Modifier(6),
                        )),
                        Some(key_system::Ref::Layered(
                            smart_keymap::key::layered::Ref::Modifier(7),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        Some(key_system::Ref::Keyboard(
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
