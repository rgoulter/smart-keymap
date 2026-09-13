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

    const CALLBACK: usize = 1;
    const CONSUMER: usize = 0;
    const KEYBOARD: usize = 0;
    const LAYERED: usize = 66;
    const LAYER_MODIFIERS: usize = 1;

    /// Per-keymap composite key system (generated; only families used by this keymap).
    pub mod key_system {
        use smart_keymap::key;
        use smart_keymap::keymap;

        /// Aggregate key reference.
        #[derive(serde::Deserialize, Debug, Clone, Copy, PartialEq)]
        pub enum Ref {
            /// [smart_keymap::key::callback] variant.
            Callback(smart_keymap::key::callback::Ref),
            /// [smart_keymap::key::caps_word] variant.
            CapsWord(smart_keymap::key::caps_word::Ref),
            /// [smart_keymap::key::consumer] variant.
            Consumer(smart_keymap::key::consumer::Ref),
            /// [smart_keymap::key::keyboard] variant.
            Keyboard(smart_keymap::key::keyboard::Ref),
            /// [smart_keymap::key::layered] variant.
            Layered(smart_keymap::key::layered::Ref),
        }

        /// Aggregate config for families used by this keymap.
        #[derive(serde::Deserialize, Debug, Clone, Copy, PartialEq)]
        pub struct Config {
            /// Config for [smart_keymap::key::layered].
            pub layered: smart_keymap::key::layered::Config<{ super::CONDITIONAL_LAYER_COUNT }>,
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
                }
            }
        }

        /// Aggregate context.
        #[derive(Debug, Clone, Copy)]
        pub struct Context {
            keymap_context: smart_keymap::keymap::KeymapContext,
            callback: smart_keymap::key::callback::Context,
            caps_word: smart_keymap::key::caps_word::Context,
            consumer: smart_keymap::key::consumer::Context,
            keyboard: smart_keymap::key::keyboard::Context,
            layered: smart_keymap::key::layered::Context<
                { super::LAYERED_LAYER_COUNT },
                { super::CONDITIONAL_LAYER_COUNT },
            >,
        }

        impl Context {
            /// Constructs a [Context] from the given [Config].
            pub const fn from_config(config: Config) -> Self {
                let _ = &config;
                Self {
                    keymap_context: smart_keymap::keymap::KeymapContext::new(),
                    callback: smart_keymap::key::callback::Context,
                    caps_word: smart_keymap::key::caps_word::Context::new(),
                    consumer: smart_keymap::key::consumer::Context,
                    keyboard: smart_keymap::key::keyboard::Context,
                    layered: smart_keymap::key::layered::Context::from_config(config.layered),
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
                    pke.extend(self.layered.handle_event(e).into_events());
                }
                pke
            }

            fn reset(&mut self) {
                self.keymap_context = smart_keymap::keymap::KeymapContext::new();
                self.callback.reset();
                self.caps_word.reset();
                self.consumer.reset();
                self.keyboard.reset();
                self.layered.reset();
            }
        }

        impl keymap::SetKeymapContext for Context {
            fn set_keymap_context(&mut self, context: keymap::KeymapContext) {
                self.keymap_context = context;
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
            /// [smart_keymap::key::callback] variant.
            Callback(smart_keymap::key::callback::Event),
            /// [smart_keymap::key::caps_word] variant.
            CapsWord(smart_keymap::key::caps_word::Event),
            /// [smart_keymap::key::consumer] variant.
            Consumer(smart_keymap::key::consumer::Event),
            /// [smart_keymap::key::keyboard] variant.
            Keyboard(smart_keymap::key::keyboard::Event),
            /// [smart_keymap::key::layered] variant.
            Layered(smart_keymap::key::layered::LayerEvent),
        }

        impl From<smart_keymap::key::callback::Event> for Event {
            fn from(v: smart_keymap::key::callback::Event) -> Self {
                Event::Callback(v)
            }
        }
        impl From<smart_keymap::key::caps_word::Event> for Event {
            fn from(v: smart_keymap::key::caps_word::Event) -> Self {
                Event::CapsWord(v)
            }
        }
        impl From<smart_keymap::key::consumer::Event> for Event {
            fn from(v: smart_keymap::key::consumer::Event) -> Self {
                Event::Consumer(v)
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
        #[allow(unreachable_patterns)]
        impl TryFrom<Event> for smart_keymap::key::callback::Event {
            type Error = smart_keymap::key::EventError;
            fn try_from(v: Event) -> Result<Self, Self::Error> {
                match v {
                    Event::Callback(v) => Ok(v),
                    _ => Err(smart_keymap::key::EventError::UnmappableEvent),
                }
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
        impl TryFrom<Event> for smart_keymap::key::consumer::Event {
            type Error = smart_keymap::key::EventError;
            fn try_from(v: Event) -> Result<Self, Self::Error> {
                match v {
                    Event::Consumer(v) => Ok(v),
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

        /// Aggregate pending key state.
        #[derive(Debug, Clone, PartialEq)]
        #[allow(clippy::large_enum_variant)]
        pub enum PendingKeyState {
            /// [smart_keymap::key::callback] variant.
            Callback(smart_keymap::key::callback::PendingKeyState),
            /// [smart_keymap::key::caps_word] variant.
            CapsWord(smart_keymap::key::caps_word::PendingKeyState),
            /// [smart_keymap::key::consumer] variant.
            Consumer(smart_keymap::key::consumer::PendingKeyState),
            /// [smart_keymap::key::keyboard] variant.
            Keyboard(smart_keymap::key::keyboard::PendingKeyState),
            /// [smart_keymap::key::layered] variant.
            Layered(smart_keymap::key::layered::PendingKeyState),
        }

        impl From<smart_keymap::key::callback::PendingKeyState> for PendingKeyState {
            fn from(pks: smart_keymap::key::callback::PendingKeyState) -> Self {
                PendingKeyState::Callback(pks)
            }
        }
        impl From<smart_keymap::key::caps_word::PendingKeyState> for PendingKeyState {
            fn from(pks: smart_keymap::key::caps_word::PendingKeyState) -> Self {
                PendingKeyState::CapsWord(pks)
            }
        }
        impl From<smart_keymap::key::consumer::PendingKeyState> for PendingKeyState {
            fn from(pks: smart_keymap::key::consumer::PendingKeyState) -> Self {
                PendingKeyState::Consumer(pks)
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

        /// Aggregate key state.
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum KeyState {
            /// No-op key state (e.g. auxiliary chorded keys).
            NoOp,
            /// [smart_keymap::key::callback] key state.
            Callback(smart_keymap::key::callback::KeyState),
            /// [smart_keymap::key::caps_word] key state.
            CapsWord(smart_keymap::key::caps_word::KeyState),
            /// [smart_keymap::key::consumer] key state.
            Consumer(smart_keymap::key::consumer::KeyState),
            /// [smart_keymap::key::keyboard] key state.
            Keyboard(smart_keymap::key::keyboard::KeyState),
            /// [smart_keymap::key::layered] key state.
            LayerModifier(smart_keymap::key::layered::ModifierKeyState),
        }

        impl From<key::NoOpKeyState> for KeyState {
            fn from(_: key::NoOpKeyState) -> Self {
                KeyState::NoOp
            }
        }

        impl From<smart_keymap::key::callback::KeyState> for KeyState {
            fn from(ks: smart_keymap::key::callback::KeyState) -> Self {
                KeyState::Callback(ks)
            }
        }
        impl From<smart_keymap::key::caps_word::KeyState> for KeyState {
            fn from(ks: smart_keymap::key::caps_word::KeyState) -> Self {
                KeyState::CapsWord(ks)
            }
        }
        impl From<smart_keymap::key::consumer::KeyState> for KeyState {
            fn from(ks: smart_keymap::key::consumer::KeyState) -> Self {
                KeyState::Consumer(ks)
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

        /// Aggregate [key::System] for this keymap.
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct System {
            callback: smart_keymap::key::callback::System<
                Ref,
                [smart_keymap::key::callback::Key; super::CALLBACK],
            >,
            caps_word: smart_keymap::key::caps_word::System<Ref>,
            consumer: smart_keymap::key::consumer::System<
                Ref,
                [smart_keymap::key::consumer::Key; super::CONSUMER],
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
        }

        impl System {
            /// Constructs the system from data-carrying subsystems.
            #[allow(clippy::too_many_arguments)]
            pub const fn new(
                callback: smart_keymap::key::callback::System<
                    Ref,
                    [smart_keymap::key::callback::Key; super::CALLBACK],
                >,
                consumer: smart_keymap::key::consumer::System<
                    Ref,
                    [smart_keymap::key::consumer::Key; super::CONSUMER],
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
            ) -> Self {
                Self {
                    callback,
                    consumer,
                    keyboard,
                    layered,
                    caps_word: smart_keymap::key::caps_word::System::new(),
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
                    Ref::Callback(key_ref) => {
                        let (pkr, pke) =
                            self.callback
                                .new_pressed_key(keymap_index, &context.callback, key_ref);
                        (pkr.into_result(), pke.into_events())
                    }
                    Ref::CapsWord(key_ref) => {
                        let (pkr, pke) = self.caps_word.new_pressed_key(
                            keymap_index,
                            &context.caps_word,
                            key_ref,
                        );
                        (pkr.into_result(), pke.into_events())
                    }
                    Ref::Consumer(key_ref) => {
                        let (pkr, pke) =
                            self.consumer
                                .new_pressed_key(keymap_index, &context.consumer, key_ref);
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
                    (Ref::Consumer(key_ref), KeyState::Consumer(key_state)) => {
                        if let Ok(event) = event.try_into_key_event() {
                            self.consumer
                                .update_state(
                                    key_state,
                                    key_ref,
                                    &context.consumer,
                                    keymap_index,
                                    event,
                                )
                                .into_events()
                        } else {
                            smart_keymap::key::KeyEvents::no_events()
                        }
                    }
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
                    (Ref::Consumer(r), KeyState::Consumer(ks)) => self.consumer.key_output(r, ks),
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
    pub const KEY_COUNT: usize = 66;

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
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(34)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(35)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(36)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(37)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(38)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(39)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(40)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(41)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(42)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(43)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(44)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(45)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(46)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(47)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(48)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(49)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(50)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(51)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(52)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(53)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(54)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(55)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(56)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(57)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(58)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(59)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(60)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(61)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(62)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(63)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(64)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(65)),
    ];

    /// The keymap config.
    pub const CONFIG: key_system::Config = key_system::Config {
        layered: smart_keymap::key::layered::Config::new(),
    };

    /// Initial [Context] value.
    pub const CONTEXT: Context = key_system::Context::from_config(key_system::Config {
        layered: smart_keymap::key::layered::Config::new(),
    });

    /// The key system.
    pub const SYSTEM: System = key_system::System::new(
        smart_keymap::key::callback::System::new([smart_keymap::key::callback::Key::new(
            smart_keymap::keymap::KeymapCallback::ResetToBootloader,
        )]),
        smart_keymap::key::consumer::System::new([]),
        smart_keymap::key::keyboard::System::new([]),
        smart_keymap::key::layered::System::new(
            [smart_keymap::key::layered::ModifierKey::hold(1)],
            [
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(53)),
                    [Some(key_system::Ref::Callback(
                        smart_keymap::key::callback::Ref(0),
                    ))],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(30)),
                    [Some(key_system::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(58),
                    ))],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(31)),
                    [Some(key_system::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(59),
                    ))],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(32)),
                    [Some(key_system::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(60),
                    ))],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(33)),
                    [Some(key_system::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(61),
                    ))],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(34)),
                    [Some(key_system::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(62),
                    ))],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(35)),
                    [Some(key_system::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(63),
                    ))],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(36)),
                    [Some(key_system::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(64),
                    ))],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(37)),
                    [Some(key_system::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(65),
                    ))],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(38)),
                    [Some(key_system::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(66),
                    ))],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(39)),
                    [Some(key_system::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(67),
                    ))],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(45)),
                    [Some(key_system::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(68),
                    ))],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(46)),
                    [Some(key_system::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(69),
                    ))],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(42)),
                    [Some(key_system::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(76),
                    ))],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(43)),
                    [None],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(20)),
                    [None],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(26)),
                    [None],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(8)),
                    [Some(key_system::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(41),
                    ))],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(21)),
                    [None],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(23)),
                    [None],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(28)),
                    [None],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(24)),
                    [Some(key_system::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(74),
                    ))],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(12)),
                    [Some(key_system::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(82),
                    ))],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(18)),
                    [Some(key_system::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(77),
                    ))],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(19)),
                    [Some(key_system::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(70),
                    ))],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(47)),
                    [Some(key_system::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(71),
                    ))],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(48)),
                    [Some(key_system::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(72),
                    ))],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(49)),
                    [Some(key_system::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(73),
                    ))],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(57)),
                    [Some(key_system::Ref::CapsWord(
                        smart_keymap::key::caps_word::Ref(
                            smart_keymap::key::caps_word::Key::ToggleCapsWord,
                        ),
                    ))],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(4)),
                    [None],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(22)),
                    [None],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(7)),
                    [None],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(9)),
                    [None],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(10)),
                    [None],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(11)),
                    [Some(key_system::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(75),
                    ))],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(13)),
                    [Some(key_system::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(80),
                    ))],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(14)),
                    [Some(key_system::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(81),
                    ))],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(15)),
                    [Some(key_system::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(79),
                    ))],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(51)),
                    [None],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(52)),
                    [None],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(40)),
                    [None],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::Modifiers(2)),
                    [None],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(29)),
                    [Some(key_system::Ref::Consumer(
                        smart_keymap::key::consumer::Ref::UsageCode(226),
                    ))],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(27)),
                    [Some(key_system::Ref::Consumer(
                        smart_keymap::key::consumer::Ref::UsageCode(234),
                    ))],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(6)),
                    [Some(key_system::Ref::Consumer(
                        smart_keymap::key::consumer::Ref::UsageCode(233),
                    ))],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(25)),
                    [Some(key_system::Ref::Consumer(
                        smart_keymap::key::consumer::Ref::UsageCode(205),
                    ))],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(5)),
                    [None],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(17)),
                    [Some(key_system::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(78),
                    ))],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(16)),
                    [None],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(54)),
                    [None],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(55)),
                    [None],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(56)),
                    [None],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::Modifiers(32)),
                    [None],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::Modifiers(1)),
                    [None],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::Modifiers(8)),
                    [None],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::Modifiers(4)),
                    [None],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(41)),
                    [None],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Layered(smart_keymap::key::layered::Ref::Modifier(0)),
                    [Some(key_system::Ref::Keyboard(
                        smart_keymap::key::keyboard::Ref::KeyCode(0),
                    ))],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(44)),
                    [None],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(44)),
                    [None],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(44)),
                    [None],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(76)),
                    [None],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::Modifiers(64)),
                    [None],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::Modifiers(128)),
                    [None],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(101)),
                    [None],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::Modifiers(16)),
                    [None],
                ),
            ],
        ),
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
