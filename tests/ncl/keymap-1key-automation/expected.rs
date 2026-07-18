/// Types and initial data used for constructing [KEYMAP].
pub mod init {
    use crate as smart_keymap;

    /// Number of instructions used by the [crate::key::automation] implementation.
    pub const AUTOMATION_INSTRUCTION_COUNT: usize = 1;

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

    const AUTOMATION: usize = 1;

    /// Per-keymap composite key system (generated; only families used by this keymap).
    pub mod key_system {
        use crate as smart_keymap;
        use smart_keymap::key;
        use smart_keymap::keymap;

        /// Aggregate key reference.
        #[derive(serde::Deserialize, Debug, Clone, Copy, PartialEq)]
        pub enum Ref {
            /// [smart_keymap::key::automation] variant.
            Automation(smart_keymap::key::automation::Ref),
        }

        /// Aggregate config for families used by this keymap.
        #[derive(serde::Deserialize, Debug, Clone, Copy, PartialEq)]
        pub struct Config {
            /// Config for [smart_keymap::key::automation].
            pub automation: smart_keymap::key::automation::Config<
                { crate::init::AUTOMATION_INSTRUCTION_COUNT },
            >,
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
                    automation: smart_keymap::key::automation::Config::new(),
                }
            }
        }

        /// Aggregate context.
        #[derive(Debug, Clone, Copy)]
        pub struct Context {
            keymap_context: smart_keymap::keymap::KeymapContext,
            automation: smart_keymap::key::automation::Context<
                { crate::init::AUTOMATION_INSTRUCTION_COUNT },
            >,
        }

        impl Context {
            /// Constructs a [Context] from the given [Config].
            pub const fn from_config(config: Config) -> Self {
                let _ = &config;
                Self {
                    keymap_context: smart_keymap::keymap::KeymapContext::new(),
                    automation: smart_keymap::key::automation::Context::from_config(
                        config.automation,
                    ),
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
                    pke.extend(self.automation.handle_event(e).into_events());
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
            /// [smart_keymap::key::automation] variant.
            Automation(smart_keymap::key::automation::Event),
        }

        impl From<smart_keymap::key::automation::Event> for Event {
            fn from(v: smart_keymap::key::automation::Event) -> Self {
                Event::Automation(v)
            }
        }
        impl TryFrom<Event> for smart_keymap::key::automation::Event {
            type Error = smart_keymap::key::EventError;
            fn try_from(v: Event) -> Result<Self, Self::Error> {
                match v {
                    Event::Automation(v) => Ok(v),
                    _ => Err(smart_keymap::key::EventError::UnmappableEvent),
                }
            }
        }

        /// Aggregate pending key state.
        #[derive(Debug, Clone, PartialEq)]
        #[allow(clippy::large_enum_variant)]
        pub enum PendingKeyState {
            /// [smart_keymap::key::automation] variant.
            Automation(smart_keymap::key::automation::PendingKeyState),
        }

        impl From<smart_keymap::key::automation::PendingKeyState> for PendingKeyState {
            fn from(pks: smart_keymap::key::automation::PendingKeyState) -> Self {
                PendingKeyState::Automation(pks)
            }
        }

        /// Aggregate key state.
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum KeyState {
            /// No-op key state (e.g. auxiliary chorded keys).
            NoOp,
            /// [smart_keymap::key::automation] key state.
            Automation(smart_keymap::key::automation::KeyState),
        }

        impl From<key::NoOpKeyState> for KeyState {
            fn from(_: key::NoOpKeyState) -> Self {
                KeyState::NoOp
            }
        }

        impl From<smart_keymap::key::automation::KeyState> for KeyState {
            fn from(ks: smart_keymap::key::automation::KeyState) -> Self {
                KeyState::Automation(ks)
            }
        }

        /// Aggregate [key::System] for this keymap.
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct System {
            automation: smart_keymap::key::automation::System<
                Ref,
                [smart_keymap::key::automation::Key; { crate::init::AUTOMATION }],
                { crate::init::AUTOMATION_INSTRUCTION_COUNT },
            >,
        }

        impl System {
            /// Constructs the system from data-carrying subsystems.
            #[allow(clippy::too_many_arguments)]
            pub const fn new(
                automation: smart_keymap::key::automation::System<
                    Ref,
                    [smart_keymap::key::automation::Key; { crate::init::AUTOMATION }],
                    { crate::init::AUTOMATION_INSTRUCTION_COUNT },
                >,
            ) -> Self {
                Self { automation }
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
                    Ref::Automation(key_ref) => {
                        let (pkr, pke) = self.automation.new_pressed_key(
                            keymap_index,
                            &context.automation,
                            key_ref,
                        );
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
                    (Ref::Automation(key_ref), KeyState::Automation(key_state)) => {
                        if let Ok(event) = event.try_into_key_event() {
                            self.automation
                                .update_state(
                                    key_state,
                                    key_ref,
                                    &context.automation,
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
    pub const KEY_COUNT: usize = 1;

    /// The key references.
    pub const KEY_REFS: [Ref; KEY_COUNT] = [key_system::Ref::Automation(
        smart_keymap::key::automation::Ref(0),
    )];

    /// The keymap config.
    pub const CONFIG: key_system::Config = key_system::Config {
        automation: smart_keymap::key::automation::Config {
            instructions: smart_keymap::key::automation::instructions([
                smart_keymap::key::automation::Instruction::Tap(
                    smart_keymap::key::KeyOutput::from_usage(
                        smart_keymap::key::KeyUsage::Keyboard(4),
                    ),
                ),
            ]),
            ..smart_keymap::key::automation::Config::new()
        },
    };

    /// Initial [Context] value.
    pub const CONTEXT: Context = key_system::Context::from_config(key_system::Config {
        automation: smart_keymap::key::automation::Config {
            instructions: smart_keymap::key::automation::instructions([
                smart_keymap::key::automation::Instruction::Tap(
                    smart_keymap::key::KeyOutput::from_usage(
                        smart_keymap::key::KeyUsage::Keyboard(4),
                    ),
                ),
            ]),
            ..smart_keymap::key::automation::Config::new()
        },
    });

    /// The key system.
    pub const SYSTEM: System =
        key_system::System::new(smart_keymap::key::automation::System::new([
            smart_keymap::key::automation::Key {
                automation_instructions: smart_keymap::key::automation::KeyInstructions {
                    on_press: smart_keymap::key::automation::Execution {
                        start: 0,
                        length: 1,
                    },
                    while_pressed: smart_keymap::key::automation::Execution {
                        start: 1,
                        length: 0,
                    },
                    on_release: smart_keymap::key::automation::Execution {
                        start: 1,
                        length: 0,
                    },
                },
            },
        ]));

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
