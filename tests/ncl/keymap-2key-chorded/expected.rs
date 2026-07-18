/// Types and initial data used for constructing [KEYMAP].
pub mod init {
    use crate as smart_keymap;

    /// Number of instructions used by the [crate::key::automation] implementation.
    pub const AUTOMATION_INSTRUCTION_COUNT: usize = 0;

    /// Number of layers supported by the [smart_keymap::key::layered] implementation.
    pub const LAYERED_LAYER_COUNT: usize = 0;

    /// The maximum number of keys in a chord.
    pub const CHORDED_MAX_CHORD_SIZE: usize = 2;

    /// The maximum number of chords.
    pub const CHORDED_MAX_CHORDS: usize = 1;

    /// The maximum number of overlapping chords for a chorded key.
    pub const CHORDED_MAX_OVERLAPPING_CHORD_SIZE: usize = 1;

    /// The tap-dance definitions.
    pub const TAP_DANCE_MAX_DEFINITIONS: usize = 0;

    const CHORDED: usize = 1;
    const CHORDED_AUXILIARY: usize = 1;
    const KEYBOARD: usize = 0;

    /// Per-keymap composite key system (generated; only families used by this keymap).
    pub mod key_system {
        use crate as smart_keymap;
        use smart_keymap::key;
        use smart_keymap::keymap;

        const CHORDED_MAX_PRESSED_INDICES: usize = crate::init::CHORDED_MAX_CHORD_SIZE * 2;

        /// Aggregate key reference.
        #[derive(serde::Deserialize, Debug, Clone, Copy, PartialEq)]
        pub enum Ref {
            /// [smart_keymap::key::chorded] variant.
            Chorded(smart_keymap::key::chorded::Ref),
            /// [smart_keymap::key::keyboard] variant.
            Keyboard(smart_keymap::key::keyboard::Ref),
        }

        /// Aggregate config for families used by this keymap.
        #[derive(serde::Deserialize, Debug, Clone, Copy, PartialEq)]
        pub struct Config {
            /// Config for [smart_keymap::key::chorded].
            pub chorded: smart_keymap::key::chorded::Config<
                { crate::init::CHORDED_MAX_CHORDS },
                { crate::init::CHORDED_MAX_CHORD_SIZE },
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
                    chorded: smart_keymap::key::chorded::Config::new(),
                }
            }
        }

        /// Aggregate context.
        #[derive(Debug, Clone, Copy)]
        pub struct Context {
            keymap_context: smart_keymap::keymap::KeymapContext,
            chorded: smart_keymap::key::chorded::Context<
                { crate::init::CHORDED_MAX_CHORDS },
                { crate::init::CHORDED_MAX_CHORD_SIZE },
                CHORDED_MAX_PRESSED_INDICES,
            >,
            keyboard: smart_keymap::key::keyboard::Context,
        }

        impl Context {
            /// Constructs a [Context] from the given [Config].
            pub const fn from_config(config: Config) -> Self {
                let _ = &config;
                Self {
                    keymap_context: smart_keymap::keymap::KeymapContext::new(),
                    chorded: smart_keymap::key::chorded::Context::from_config(config.chorded),
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
                event: key::Event<Self::Event>,
            ) -> key::KeyEvents<Self::Event> {
                let mut pke = key::KeyEvents::no_events();
                if let Ok(e) = event.try_into_key_event() {
                    pke.extend(self.chorded.handle_event(e).into_events());
                }
                pke
            }
        }

        impl keymap::SetKeymapContext for Context {
            fn set_keymap_context(&mut self, context: keymap::KeymapContext) {
                self.keymap_context = context;
                self.chorded.update_keymap_context(&context);
            }
        }

        /// Aggregate event.
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum Event {
            /// [smart_keymap::key::chorded] variant.
            Chorded(smart_keymap::key::chorded::Event),
            /// [smart_keymap::key::keyboard] variant.
            Keyboard(smart_keymap::key::keyboard::Event),
        }

        impl From<smart_keymap::key::chorded::Event> for Event {
            fn from(v: smart_keymap::key::chorded::Event) -> Self {
                Event::Chorded(v)
            }
        }
        impl From<smart_keymap::key::keyboard::Event> for Event {
            fn from(v: smart_keymap::key::keyboard::Event) -> Self {
                Event::Keyboard(v)
            }
        }
        impl TryFrom<Event> for smart_keymap::key::chorded::Event {
            type Error = smart_keymap::key::EventError;
            fn try_from(v: Event) -> Result<Self, Self::Error> {
                match v {
                    Event::Chorded(v) => Ok(v),
                    _ => Err(smart_keymap::key::EventError::UnmappableEvent),
                }
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
            /// [smart_keymap::key::chorded] variant.
            Chorded(
                smart_keymap::key::chorded::PendingKeyState<
                    { crate::init::CHORDED_MAX_CHORDS },
                    { crate::init::CHORDED_MAX_CHORD_SIZE },
                    CHORDED_MAX_PRESSED_INDICES,
                >,
            ),
            /// [smart_keymap::key::keyboard] variant.
            Keyboard(smart_keymap::key::keyboard::PendingKeyState),
        }

        impl
            From<
                smart_keymap::key::chorded::PendingKeyState<
                    { crate::init::CHORDED_MAX_CHORDS },
                    { crate::init::CHORDED_MAX_CHORD_SIZE },
                    CHORDED_MAX_PRESSED_INDICES,
                >,
            > for PendingKeyState
        {
            fn from(
                pks: smart_keymap::key::chorded::PendingKeyState<
                    { crate::init::CHORDED_MAX_CHORDS },
                    { crate::init::CHORDED_MAX_CHORD_SIZE },
                    CHORDED_MAX_PRESSED_INDICES,
                >,
            ) -> Self {
                PendingKeyState::Chorded(pks)
            }
        }
        impl From<smart_keymap::key::keyboard::PendingKeyState> for PendingKeyState {
            fn from(pks: smart_keymap::key::keyboard::PendingKeyState) -> Self {
                PendingKeyState::Keyboard(pks)
            }
        }
        impl<'pks> TryFrom<&'pks mut PendingKeyState>
            for &'pks mut smart_keymap::key::chorded::PendingKeyState<
                { crate::init::CHORDED_MAX_CHORDS },
                { crate::init::CHORDED_MAX_CHORD_SIZE },
                CHORDED_MAX_PRESSED_INDICES,
            >
        {
            type Error = ();
            fn try_from(pks: &'pks mut PendingKeyState) -> Result<Self, Self::Error> {
                match pks {
                    PendingKeyState::Chorded(pks) => Ok(pks),
                    _ => Err(()),
                }
            }
        }

        /// Aggregate key state.
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum KeyState {
            /// No-op key state (e.g. auxiliary chorded keys).
            NoOp,
            /// [smart_keymap::key::chorded] key state.
            Chorded(smart_keymap::key::chorded::KeyState),
            /// [smart_keymap::key::keyboard] key state.
            Keyboard(smart_keymap::key::keyboard::KeyState),
        }

        impl From<key::NoOpKeyState> for KeyState {
            fn from(_: key::NoOpKeyState) -> Self {
                KeyState::NoOp
            }
        }

        impl From<smart_keymap::key::chorded::KeyState> for KeyState {
            fn from(ks: smart_keymap::key::chorded::KeyState) -> Self {
                KeyState::Chorded(ks)
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
            chorded: smart_keymap::key::chorded::System<
                Ref,
                [smart_keymap::key::chorded::Key<
                    Ref,
                    { crate::init::CHORDED_MAX_CHORDS },
                    { crate::init::CHORDED_MAX_CHORD_SIZE },
                    { crate::init::CHORDED_MAX_OVERLAPPING_CHORD_SIZE },
                    CHORDED_MAX_PRESSED_INDICES,
                >; { crate::init::CHORDED }],
                [smart_keymap::key::chorded::AuxiliaryKey<
                    Ref,
                    { crate::init::CHORDED_MAX_CHORDS },
                    { crate::init::CHORDED_MAX_CHORD_SIZE },
                    CHORDED_MAX_PRESSED_INDICES,
                >; { crate::init::CHORDED_AUXILIARY }],
                { crate::init::CHORDED_MAX_CHORDS },
                { crate::init::CHORDED_MAX_CHORD_SIZE },
                { crate::init::CHORDED_MAX_OVERLAPPING_CHORD_SIZE },
                CHORDED_MAX_PRESSED_INDICES,
            >,
            keyboard: smart_keymap::key::keyboard::System<
                Ref,
                [smart_keymap::key::keyboard::Key; { crate::init::KEYBOARD }],
            >,
        }

        impl System {
            /// Constructs the system from data-carrying subsystems.
            #[allow(clippy::too_many_arguments)]
            pub const fn new(
                chorded: smart_keymap::key::chorded::System<
                    Ref,
                    [smart_keymap::key::chorded::Key<
                        Ref,
                        { crate::init::CHORDED_MAX_CHORDS },
                        { crate::init::CHORDED_MAX_CHORD_SIZE },
                        { crate::init::CHORDED_MAX_OVERLAPPING_CHORD_SIZE },
                        CHORDED_MAX_PRESSED_INDICES,
                    >; { crate::init::CHORDED }],
                    [smart_keymap::key::chorded::AuxiliaryKey<
                        Ref,
                        { crate::init::CHORDED_MAX_CHORDS },
                        { crate::init::CHORDED_MAX_CHORD_SIZE },
                        CHORDED_MAX_PRESSED_INDICES,
                    >; { crate::init::CHORDED_AUXILIARY }],
                    { crate::init::CHORDED_MAX_CHORDS },
                    { crate::init::CHORDED_MAX_CHORD_SIZE },
                    { crate::init::CHORDED_MAX_OVERLAPPING_CHORD_SIZE },
                    CHORDED_MAX_PRESSED_INDICES,
                >,
                keyboard: smart_keymap::key::keyboard::System<
                    Ref,
                    [smart_keymap::key::keyboard::Key; { crate::init::KEYBOARD }],
                >,
            ) -> Self {
                Self { chorded, keyboard }
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
                    Ref::Chorded(key_ref) => {
                        let (pkr, pke) =
                            self.chorded
                                .new_pressed_key(keymap_index, &context.chorded, key_ref);
                        (pkr.into_result(), pke.into_events())
                    }
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
                    (Ref::Chorded(key_ref), PendingKeyState::Chorded(pending_state)) => {
                        if let Ok(event) = event.try_into_key_event() {
                            let (maybe_npk, pke) = self.chorded.update_pending_state(
                                pending_state,
                                keymap_index,
                                &context.chorded,
                                key_ref,
                                event,
                            );
                            (maybe_npk, pke.into_events())
                        } else {
                            (None, smart_keymap::key::KeyEvents::no_events())
                        }
                    }
                    (_, _) => panic!("mismatched key_ref and pending_state variants"),
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
    pub const KEY_COUNT: usize = 2;

    /// The key references.
    pub const KEY_REFS: [Ref; KEY_COUNT] = [
        key_system::Ref::Chorded(smart_keymap::key::chorded::Ref::Chorded(0)),
        key_system::Ref::Chorded(smart_keymap::key::chorded::Ref::Auxiliary(0)),
    ];

    /// The keymap config.
    pub const CONFIG: key_system::Config = key_system::Config {
        chorded: smart_keymap::key::chorded::Config {
            chords: smart_keymap::slice::Slice::from_slice(&[
                smart_keymap::key::chorded::ChordIndices::from_slice(&[0, 1]),
            ]),
            ..smart_keymap::key::chorded::Config::new()
        },
    };

    /// Initial [Context] value.
    pub const CONTEXT: Context = key_system::Context::from_config(key_system::Config {
        chorded: smart_keymap::key::chorded::Config {
            chords: smart_keymap::slice::Slice::from_slice(&[
                smart_keymap::key::chorded::ChordIndices::from_slice(&[0, 1]),
            ]),
            ..smart_keymap::key::chorded::Config::new()
        },
    });

    /// The key system.
    pub const SYSTEM: System = key_system::System::new(
        smart_keymap::key::chorded::System::new(
            [smart_keymap::key::chorded::Key::new(
                &[(
                    0,
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(6)),
                )],
                key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(4)),
            )],
            [smart_keymap::key::chorded::AuxiliaryKey::new(
                key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(5)),
            )],
        ),
        smart_keymap::key::keyboard::System::new([]),
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
