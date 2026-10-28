/// Types and initial data used for constructing [KEYMAP].
pub mod init {

    /// Number of instructions used by the [crate::key::automation] implementation.
    pub const AUTOMATION_INSTRUCTION_COUNT: usize = 0;

    /// The maximum number of keys in a chord.
    pub const CHORDED_MAX_CHORD_SIZE: usize = 0;

    /// The maximum number of chords.
    pub const CHORDED_MAX_CHORDS: usize = 0;

    /// The maximum number of overlapping chords for a chorded key.
    pub const CHORDED_MAX_OVERLAPPING_CHORD_SIZE: usize = 0;

    /// Number of alternate-repeat rules for the [crate::key::history] implementation.
    pub const HISTORY_ALT_REPEAT_RULE_COUNT: usize = 0;

    /// Number of layers supported by the [smart_keymap::key::layered] implementation.
    pub const LAYERED_LAYER_COUNT: usize = 1;

    /// Number of conditional layer rules for the [smart_keymap::key::layered] implementation.
    pub const CONDITIONAL_LAYER_COUNT: usize = 0;

    /// The maximum number of steps in a sequence.
    pub const SEQUENCE_MAX_SEQUENCE_LEN: usize = 0;

    /// The maximum number of sequences.
    pub const SEQUENCE_MAX_SEQUENCES: usize = 0;

    /// The maximum number of sequences sharing a primary key.
    pub const SEQUENCE_MAX_OVERLAPPING: usize = 0;

    /// The tap-dance definitions.
    pub const TAP_DANCE_MAX_DEFINITIONS: usize = 0;

    const HISTORY: usize = 0;
    const KEYBOARD: usize = 0;
    const LAYERED: usize = 4;
    const LAYER_MODIFIERS: usize = 1;
    const SEQUENCE: usize = 0;
    const SEQUENCE_AUXILIARY: usize = 0;

    /// Per-keymap composite key system (generated; only families used by this keymap).
    pub mod key_system {
        use smart_keymap::key;
        use smart_keymap::keymap;

        /// Aggregate key reference.
        #[derive(serde::Deserialize, Debug, Clone, Copy, PartialEq)]
        pub enum Ref {
            /// [smart_keymap::key::caps_word] variant.
            CapsWord(smart_keymap::key::caps_word::Ref),
            /// [smart_keymap::key::history] variant.
            History(smart_keymap::key::history::Ref),
            /// [smart_keymap::key::key_lock] variant.
            KeyLock(smart_keymap::key::key_lock::Ref),
            /// [smart_keymap::key::keyboard] variant.
            Keyboard(smart_keymap::key::keyboard::Ref),
            /// [smart_keymap::key::layered] variant.
            Layered(smart_keymap::key::layered::Ref),
            /// [smart_keymap::key::sequence] variant.
            Sequence(smart_keymap::key::sequence::Ref),
        }

        /// Aggregate config for families used by this keymap.
        #[derive(serde::Deserialize, Debug, Clone, Copy, PartialEq)]
        pub struct Config {
            /// Config for [smart_keymap::key::history].
            pub history:
                smart_keymap::key::history::Config<{ super::HISTORY_ALT_REPEAT_RULE_COUNT }>,
            /// Config for [smart_keymap::key::layered].
            pub layered: smart_keymap::key::layered::Config<{ super::CONDITIONAL_LAYER_COUNT }>,
            /// Config for [smart_keymap::key::sequence].
            pub sequence: smart_keymap::key::sequence::Config<
                { super::SEQUENCE_MAX_SEQUENCES },
                { super::SEQUENCE_MAX_SEQUENCE_LEN },
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
                    history: smart_keymap::key::history::Config::new(),
                    layered: smart_keymap::key::layered::Config::new(),
                    sequence: smart_keymap::key::sequence::Config::new(),
                }
            }
        }

        /// Aggregate context.
        #[derive(Debug, Clone, Copy)]
        pub struct Context {
            keymap_context: smart_keymap::keymap::KeymapContext,
            caps_word: smart_keymap::key::caps_word::Context,
            history: smart_keymap::key::history::Context<{ super::HISTORY_ALT_REPEAT_RULE_COUNT }>,
            key_lock: smart_keymap::key::key_lock::Context,
            keyboard: smart_keymap::key::keyboard::Context,
            layered: smart_keymap::key::layered::Context<
                { super::LAYERED_LAYER_COUNT },
                { super::CONDITIONAL_LAYER_COUNT },
            >,
            sequence: smart_keymap::key::sequence::Context<
                { super::SEQUENCE_MAX_SEQUENCES },
                { super::SEQUENCE_MAX_SEQUENCE_LEN },
            >,
        }

        impl Context {
            /// Constructs a [Context] from the given [Config].
            pub const fn from_config(config: Config) -> Self {
                let _ = &config;
                Self {
                    keymap_context: smart_keymap::keymap::KeymapContext::new(),
                    caps_word: smart_keymap::key::caps_word::Context::new(),
                    history: smart_keymap::key::history::Context::from_config(config.history),
                    key_lock: smart_keymap::key::key_lock::Context::new(),
                    keyboard: smart_keymap::key::keyboard::Context,
                    layered: smart_keymap::key::layered::Context::from_config(config.layered),
                    sequence: smart_keymap::key::sequence::Context::from_config(config.sequence),
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

            #[allow(unused_mut, unused_variables)]
            fn handle_event(
                &mut self,
                event: key::Event<Self::Event>,
            ) -> key::KeyEvents<Self::Event> {
                let mut pke = key::KeyEvents::no_events();
                if let Ok(e) = event.try_into_key_event() {
                    pke.extend(self.caps_word.handle_event(e).into_events());
                }
                if let Ok(e) = event.try_into_key_event() {
                    pke.extend(self.history.handle_event(e).into_events());
                }
                if let Ok(e) = event.try_into_key_event() {
                    pke.extend(self.key_lock.handle_event(e).into_events());
                }
                if let Ok(e) = event.try_into_key_event() {
                    pke.extend(self.layered.handle_event(e).into_events());
                }
                if let Ok(e) = event.try_into_key_event() {
                    pke.extend(self.sequence.handle_event(e).into_events());
                }
                pke
            }

            fn reset(&mut self) {
                self.keymap_context = smart_keymap::keymap::KeymapContext::new();
                self.caps_word.reset();
                self.history.reset();
                self.key_lock.reset();
                self.keyboard.reset();
                self.layered.reset();
                self.sequence.reset();
            }
        }

        impl keymap::SetKeymapContext for Context {
            fn set_keymap_context(&mut self, context: keymap::KeymapContext) {
                self.keymap_context = context;
                self.sequence.update_keymap_context(&context);
            }
        }

        impl keymap::ReportHints for Context {
            fn suppressed_modifiers(&self) -> smart_keymap::key::KeyboardModifiers {
                smart_keymap::key::KeyboardModifiers::NONE
            }
        }

        /// Aggregate event.
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum Event {
            /// [smart_keymap::key::caps_word] variant.
            CapsWord(smart_keymap::key::caps_word::Event),
            /// [smart_keymap::key::history] variant.
            History(smart_keymap::key::history::Event),
            /// [smart_keymap::key::key_lock] variant.
            KeyLock(smart_keymap::key::key_lock::Event),
            /// [smart_keymap::key::keyboard] variant.
            Keyboard(smart_keymap::key::keyboard::Event),
            /// [smart_keymap::key::layered] variant.
            Layered(smart_keymap::key::layered::LayerEvent),
            /// [smart_keymap::key::sequence] variant.
            Sequence(smart_keymap::key::sequence::Event),
        }

        impl From<smart_keymap::key::caps_word::Event> for Event {
            fn from(v: smart_keymap::key::caps_word::Event) -> Self {
                Event::CapsWord(v)
            }
        }
        impl From<smart_keymap::key::history::Event> for Event {
            fn from(v: smart_keymap::key::history::Event) -> Self {
                Event::History(v)
            }
        }
        impl From<smart_keymap::key::key_lock::Event> for Event {
            fn from(v: smart_keymap::key::key_lock::Event) -> Self {
                Event::KeyLock(v)
            }
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
        impl From<smart_keymap::key::sequence::Event> for Event {
            fn from(v: smart_keymap::key::sequence::Event) -> Self {
                Event::Sequence(v)
            }
        }
        #[allow(unreachable_patterns)]
        impl TryFrom<Event> for smart_keymap::key::caps_word::Event {
            type Error = smart_keymap::key::EventError;
            fn try_from(v: Event) -> Result<Self, Self::Error> {
                match v {
                    Event::CapsWord(v) => Ok(v),
                    _ => Err(smart_keymap::key::EventError::UnmappableEvent),
                }
            }
        }
        #[allow(unreachable_patterns)]
        impl TryFrom<Event> for smart_keymap::key::history::Event {
            type Error = smart_keymap::key::EventError;
            fn try_from(v: Event) -> Result<Self, Self::Error> {
                match v {
                    Event::History(v) => Ok(v),
                    _ => Err(smart_keymap::key::EventError::UnmappableEvent),
                }
            }
        }
        #[allow(unreachable_patterns)]
        impl TryFrom<Event> for smart_keymap::key::key_lock::Event {
            type Error = smart_keymap::key::EventError;
            fn try_from(v: Event) -> Result<Self, Self::Error> {
                match v {
                    Event::KeyLock(v) => Ok(v),
                    _ => Err(smart_keymap::key::EventError::UnmappableEvent),
                }
            }
        }
        #[allow(unreachable_patterns)]
        impl TryFrom<Event> for smart_keymap::key::keyboard::Event {
            type Error = smart_keymap::key::EventError;
            fn try_from(v: Event) -> Result<Self, Self::Error> {
                match v {
                    Event::Keyboard(v) => Ok(v),
                    _ => Err(smart_keymap::key::EventError::UnmappableEvent),
                }
            }
        }
        #[allow(unreachable_patterns)]
        impl TryFrom<Event> for smart_keymap::key::layered::LayerEvent {
            type Error = smart_keymap::key::EventError;
            fn try_from(v: Event) -> Result<Self, Self::Error> {
                match v {
                    Event::Layered(v) => Ok(v),
                    _ => Err(smart_keymap::key::EventError::UnmappableEvent),
                }
            }
        }
        #[allow(unreachable_patterns)]
        impl TryFrom<Event> for smart_keymap::key::sequence::Event {
            type Error = smart_keymap::key::EventError;
            fn try_from(v: Event) -> Result<Self, Self::Error> {
                match v {
                    Event::Sequence(v) => Ok(v),
                    _ => Err(smart_keymap::key::EventError::UnmappableEvent),
                }
            }
        }

        /// Aggregate pending key state.
        #[derive(Debug, Clone, PartialEq)]
        #[allow(clippy::large_enum_variant)]
        pub enum PendingKeyState {
            /// [smart_keymap::key::caps_word] variant.
            CapsWord(smart_keymap::key::caps_word::PendingKeyState),
            /// [smart_keymap::key::history] variant.
            History(smart_keymap::key::history::PendingKeyState),
            /// [smart_keymap::key::key_lock] variant.
            KeyLock(smart_keymap::key::key_lock::PendingKeyState),
            /// [smart_keymap::key::keyboard] variant.
            Keyboard(smart_keymap::key::keyboard::PendingKeyState),
            /// [smart_keymap::key::layered] variant.
            Layered(smart_keymap::key::layered::PendingKeyState),
            /// [smart_keymap::key::sequence] variant.
            Sequence(smart_keymap::key::sequence::PendingKeyState),
        }

        impl From<smart_keymap::key::caps_word::PendingKeyState> for PendingKeyState {
            fn from(pks: smart_keymap::key::caps_word::PendingKeyState) -> Self {
                PendingKeyState::CapsWord(pks)
            }
        }
        impl From<smart_keymap::key::history::PendingKeyState> for PendingKeyState {
            fn from(pks: smart_keymap::key::history::PendingKeyState) -> Self {
                PendingKeyState::History(pks)
            }
        }
        impl From<smart_keymap::key::key_lock::PendingKeyState> for PendingKeyState {
            fn from(pks: smart_keymap::key::key_lock::PendingKeyState) -> Self {
                PendingKeyState::KeyLock(pks)
            }
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
        impl From<smart_keymap::key::sequence::PendingKeyState> for PendingKeyState {
            fn from(pks: smart_keymap::key::sequence::PendingKeyState) -> Self {
                PendingKeyState::Sequence(pks)
            }
        }

        /// Aggregate key state.
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum KeyState {
            /// No-op key state (e.g. auxiliary chorded keys).
            NoOp,
            /// [smart_keymap::key::caps_word] key state.
            CapsWord(smart_keymap::key::caps_word::KeyState),
            /// [smart_keymap::key::history] key state.
            History(smart_keymap::key::history::KeyState),
            /// [smart_keymap::key::key_lock] key state.
            KeyLock(smart_keymap::key::key_lock::KeyState),
            /// [smart_keymap::key::keyboard] key state.
            Keyboard(smart_keymap::key::keyboard::KeyState),
            /// [smart_keymap::key::layered] key state.
            LayerModifier(smart_keymap::key::layered::ModifierKeyState),
            /// [smart_keymap::key::sequence] key state.
            Sequence(smart_keymap::key::sequence::KeyState),
        }

        impl From<key::NoOpKeyState> for KeyState {
            fn from(_: key::NoOpKeyState) -> Self {
                KeyState::NoOp
            }
        }

        impl From<smart_keymap::key::caps_word::KeyState> for KeyState {
            fn from(ks: smart_keymap::key::caps_word::KeyState) -> Self {
                KeyState::CapsWord(ks)
            }
        }
        impl From<smart_keymap::key::history::KeyState> for KeyState {
            fn from(ks: smart_keymap::key::history::KeyState) -> Self {
                KeyState::History(ks)
            }
        }
        impl From<smart_keymap::key::key_lock::KeyState> for KeyState {
            fn from(ks: smart_keymap::key::key_lock::KeyState) -> Self {
                KeyState::KeyLock(ks)
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
        impl From<smart_keymap::key::sequence::KeyState> for KeyState {
            fn from(ks: smart_keymap::key::sequence::KeyState) -> Self {
                KeyState::Sequence(ks)
            }
        }

        /// Aggregate [key::System] for this keymap.
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct System {
            caps_word: smart_keymap::key::caps_word::System<Ref>,
            history: smart_keymap::key::history::System<
                Ref,
                [smart_keymap::key::history::AdaptiveKey; super::HISTORY],
                { super::HISTORY_ALT_REPEAT_RULE_COUNT },
            >,
            key_lock: smart_keymap::key::key_lock::System<Ref>,
            keyboard: smart_keymap::key::keyboard::System<
                Ref,
                [smart_keymap::key::keyboard::Key; super::KEYBOARD],
            >,
            layered: smart_keymap::key::layered::System<
                Ref,
                [smart_keymap::key::layered::ModifierKey; super::LAYER_MODIFIERS],
                [smart_keymap::key::layered::LayeredKey<Ref, { super::LAYERED_LAYER_COUNT }>;
                    super::LAYERED],
                { super::LAYERED_LAYER_COUNT },
                { super::CONDITIONAL_LAYER_COUNT },
            >,
            sequence: smart_keymap::key::sequence::System<
                Ref,
                [smart_keymap::key::sequence::Key<Ref, { super::SEQUENCE_MAX_OVERLAPPING }>;
                    super::SEQUENCE],
                [smart_keymap::key::sequence::AuxiliaryKey<Ref>; super::SEQUENCE_AUXILIARY],
                { super::SEQUENCE_MAX_SEQUENCES },
                { super::SEQUENCE_MAX_SEQUENCE_LEN },
                { super::SEQUENCE_MAX_OVERLAPPING },
            >,
        }

        impl System {
            /// Constructs the system from data-carrying subsystems.
            #[allow(clippy::too_many_arguments)]
            pub const fn new(
                history: smart_keymap::key::history::System<
                    Ref,
                    [smart_keymap::key::history::AdaptiveKey; super::HISTORY],
                    { super::HISTORY_ALT_REPEAT_RULE_COUNT },
                >,
                keyboard: smart_keymap::key::keyboard::System<
                    Ref,
                    [smart_keymap::key::keyboard::Key; super::KEYBOARD],
                >,
                layered: smart_keymap::key::layered::System<
                    Ref,
                    [smart_keymap::key::layered::ModifierKey; super::LAYER_MODIFIERS],
                    [smart_keymap::key::layered::LayeredKey<Ref, { super::LAYERED_LAYER_COUNT }>;
                        super::LAYERED],
                    { super::LAYERED_LAYER_COUNT },
                    { super::CONDITIONAL_LAYER_COUNT },
                >,
                sequence: smart_keymap::key::sequence::System<
                    Ref,
                    [smart_keymap::key::sequence::Key<Ref, { super::SEQUENCE_MAX_OVERLAPPING }>;
                        super::SEQUENCE],
                    [smart_keymap::key::sequence::AuxiliaryKey<Ref>; super::SEQUENCE_AUXILIARY],
                    { super::SEQUENCE_MAX_SEQUENCES },
                    { super::SEQUENCE_MAX_SEQUENCE_LEN },
                    { super::SEQUENCE_MAX_OVERLAPPING },
                >,
            ) -> Self {
                Self {
                    history,
                    keyboard,
                    layered,
                    sequence,
                    caps_word: smart_keymap::key::caps_word::System::new(),
                    key_lock: smart_keymap::key::key_lock::System::new(),
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
                    Ref::CapsWord(key_ref) => {
                        let (pkr, pke) = self.caps_word.new_pressed_key(
                            keymap_index,
                            &context.caps_word,
                            key_ref,
                        );
                        (pkr.into_result(), pke.into_events())
                    }
                    Ref::History(key_ref) => {
                        let (pkr, pke) =
                            self.history
                                .new_pressed_key(keymap_index, &context.history, key_ref);
                        (pkr.into_result(), pke.into_events())
                    }
                    Ref::KeyLock(key_ref) => {
                        let (pkr, pke) =
                            self.key_lock
                                .new_pressed_key(keymap_index, &context.key_lock, key_ref);
                        (pkr.into_result(), pke.into_events())
                    }
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
                    Ref::Sequence(key_ref) => {
                        let (pkr, pke) =
                            self.sequence
                                .new_pressed_key(keymap_index, &context.sequence, key_ref);
                        (pkr.into_result(), pke.into_events())
                    }
                }
            }

            #[allow(unused_variables)]
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
                    (_, _) => smart_keymap::key::KeyEvents::no_events(),
                }
            }

            fn key_output(
                &self,
                key_ref: &Self::Ref,
                key_state: &Self::KeyState,
            ) -> Option<key::KeyOutput> {
                match (key_ref, key_state) {
                    (Ref::History(r), KeyState::History(ks)) => self.history.key_output(r, ks),
                    (Ref::Keyboard(r), KeyState::Keyboard(ks)) => self.keyboard.key_output(r, ks),
                    (Ref::Layered(r), KeyState::LayerModifier(ks)) => {
                        self.layered.key_output(r, ks)
                    }
                    (_, _) => None,
                }
            }

            fn pending_output(
                &self,
                pending_key_state: &Self::PendingKeyState,
            ) -> Option<key::KeyOutput> {
                match pending_key_state {
                    _ => None,
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
    pub const KEY_COUNT: usize = 5;

    /// The key references.
    pub const KEY_REFS: [Ref; KEY_COUNT] = [
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Modifier(0)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(0)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(1)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(2)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(3)),
    ];

    /// The keymap config.
    pub const CONFIG: key_system::Config = key_system::Config {
        history: smart_keymap::key::history::Config::new(),
        layered: smart_keymap::key::layered::Config::new(),
        sequence: smart_keymap::key::sequence::Config {
            sequences: smart_keymap::slice::Slice::from_slice(&[]),
            ..smart_keymap::key::sequence::Config::new()
        },
    };

    /// Initial [Context] value.
    pub const CONTEXT: Context = key_system::Context::from_config(key_system::Config {
        history: smart_keymap::key::history::Config::new(),
        layered: smart_keymap::key::layered::Config::new(),
        sequence: smart_keymap::key::sequence::Config {
            sequences: smart_keymap::slice::Slice::from_slice(&[]),
            ..smart_keymap::key::sequence::Config::new()
        },
    });

    /// The key system.
    pub const SYSTEM: System = key_system::System::new(
        smart_keymap::key::history::System::new([]),
        smart_keymap::key::keyboard::System::new([]),
        smart_keymap::key::layered::System::new(
            [smart_keymap::key::layered::ModifierKey::hold(1)],
            [
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::CapsWord(smart_keymap::key::caps_word::Ref(
                        smart_keymap::key::caps_word::Key::ToggleCapsWord,
                    )),
                    [Some(key_system::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(4),
                    ))],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::KeyLock(smart_keymap::key::key_lock::Ref(
                        smart_keymap::key::key_lock::Key::KeyLock,
                    )),
                    [Some(key_system::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(5),
                    ))],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::History(smart_keymap::key::history::Ref(
                        smart_keymap::key::history::Key::Repeat,
                    )),
                    [Some(key_system::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(6),
                    ))],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Sequence(smart_keymap::key::sequence::Ref::SequenceStart),
                    [Some(key_system::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(7),
                    ))],
                ),
            ],
        ),
        smart_keymap::key::sequence::System::new([], []),
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
