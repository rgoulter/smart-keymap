/// Types and initial data used for constructing [KEYMAP].
pub mod init {
    use crate as smart_keymap;

    /// Number of instructions used by the [crate::key::automation] implementation.
    pub const AUTOMATION_INSTRUCTION_COUNT: usize = 0;

    /// Number of layers supported by the [smart_keymap::key::layered] implementation.
    pub const LAYERED_LAYER_COUNT: usize = 9;

    /// The maximum number of keys in a chord.
    pub const CHORDED_MAX_CHORD_SIZE: usize = 2;

    /// The maximum number of chords.
    pub const CHORDED_MAX_CHORDS: usize = 4;

    /// The maximum number of overlapping chords for a chorded key.
    pub const CHORDED_MAX_OVERLAPPING_CHORD_SIZE: usize = 1;

    /// The tap-dance definitions.
    pub const TAP_DANCE_MAX_DEFINITIONS: usize = 2;

    const CALLBACK: usize = 7;
    const CHORDED: usize = 4;
    const CHORDED_AUXILIARY: usize = 4;
    const KEYBOARD: usize = 20;
    const LAYERED: usize = 48;
    const LAYER_MODIFIERS: usize = 50;
    const STICKY: usize = 24;
    const TAP_DANCE: usize = 30;
    const TAP_HOLD: usize = 42;

    /// Per-keymap composite key system (generated; only families used by this keymap).
    pub mod key_system {
        use crate as smart_keymap;
        use smart_keymap::key;
        use smart_keymap::keymap;

        const CHORDED_MAX_PRESSED_INDICES: usize = crate::init::CHORDED_MAX_CHORD_SIZE * 2;

        /// Aggregate key reference.
        #[derive(serde::Deserialize, Debug, Clone, Copy, PartialEq)]
        pub enum Ref {
            /// [smart_keymap::key::callback] variant.
            Callback(smart_keymap::key::callback::Ref),
            /// [smart_keymap::key::chorded] variant.
            Chorded(smart_keymap::key::chorded::Ref),
            /// [smart_keymap::key::consumer] variant.
            Consumer(smart_keymap::key::consumer::Ref),
            /// [smart_keymap::key::keyboard] variant.
            Keyboard(smart_keymap::key::keyboard::Ref),
            /// [smart_keymap::key::layered] variant.
            Layered(smart_keymap::key::layered::Ref),
            /// [smart_keymap::key::mouse] variant.
            Mouse(smart_keymap::key::mouse::Ref),
            /// [smart_keymap::key::sticky] variant.
            Sticky(smart_keymap::key::sticky::Ref),
            /// [smart_keymap::key::tap_dance] variant.
            TapDance(smart_keymap::key::tap_dance::Ref),
            /// [smart_keymap::key::tap_hold] variant.
            TapHold(smart_keymap::key::tap_hold::Ref),
        }

        /// Aggregate config for families used by this keymap.
        #[derive(serde::Deserialize, Debug, Clone, Copy, PartialEq)]
        pub struct Config {
            /// Config for [smart_keymap::key::chorded].
            pub chorded: smart_keymap::key::chorded::Config<
                { crate::init::CHORDED_MAX_CHORDS },
                { crate::init::CHORDED_MAX_CHORD_SIZE },
            >,
            /// Config for [smart_keymap::key::layered].
            pub layered: smart_keymap::key::layered::Config,
            /// Config for [smart_keymap::key::sticky].
            pub sticky: smart_keymap::key::sticky::Config,
            /// Config for [smart_keymap::key::tap_dance].
            pub tap_dance: smart_keymap::key::tap_dance::Config,
            /// Config for [smart_keymap::key::tap_hold].
            pub tap_hold: smart_keymap::key::tap_hold::Config,
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
                    layered: smart_keymap::key::layered::Config::new(),
                    sticky: smart_keymap::key::sticky::Config::new(),
                    tap_dance: smart_keymap::key::tap_dance::Config::new(),
                    tap_hold: smart_keymap::key::tap_hold::Config::new(),
                }
            }
        }

        /// Aggregate context.
        #[derive(Debug, Clone, Copy)]
        pub struct Context {
            keymap_context: smart_keymap::keymap::KeymapContext,
            callback: smart_keymap::key::callback::Context,
            chorded: smart_keymap::key::chorded::Context<
                { crate::init::CHORDED_MAX_CHORDS },
                { crate::init::CHORDED_MAX_CHORD_SIZE },
                CHORDED_MAX_PRESSED_INDICES,
            >,
            consumer: smart_keymap::key::consumer::Context,
            keyboard: smart_keymap::key::keyboard::Context,
            layered: smart_keymap::key::layered::Context<{ crate::init::LAYERED_LAYER_COUNT }>,
            mouse: smart_keymap::key::mouse::Context,
            sticky: smart_keymap::key::sticky::Context,
            tap_dance: smart_keymap::key::tap_dance::Context,
            tap_hold: smart_keymap::key::tap_hold::Context,
        }

        impl Context {
            /// Constructs a [Context] from the given [Config].
            pub const fn from_config(config: Config) -> Self {
                let _ = &config;
                Self {
                    keymap_context: smart_keymap::keymap::KeymapContext::new(),
                    callback: smart_keymap::key::callback::Context,
                    chorded: smart_keymap::key::chorded::Context::from_config(config.chorded),
                    consumer: smart_keymap::key::consumer::Context,
                    keyboard: smart_keymap::key::keyboard::Context,
                    layered: smart_keymap::key::layered::Context::from_config(config.layered),
                    mouse: smart_keymap::key::mouse::Context,
                    sticky: smart_keymap::key::sticky::Context::from_config(config.sticky),
                    tap_dance: smart_keymap::key::tap_dance::Context::from_config(config.tap_dance),
                    tap_hold: smart_keymap::key::tap_hold::Context::from_config(config.tap_hold),
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
                self.chorded.update_keymap_context(&context);
                self.tap_hold.update_keymap_context(&context);
            }
        }

        /// Aggregate event.
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum Event {
            /// [smart_keymap::key::callback] variant.
            Callback(smart_keymap::key::callback::Event),
            /// [smart_keymap::key::chorded] variant.
            Chorded(smart_keymap::key::chorded::Event),
            /// [smart_keymap::key::consumer] variant.
            Consumer(smart_keymap::key::consumer::Event),
            /// [smart_keymap::key::keyboard] variant.
            Keyboard(smart_keymap::key::keyboard::Event),
            /// [smart_keymap::key::layered] variant.
            Layered(smart_keymap::key::layered::LayerEvent),
            /// [smart_keymap::key::mouse] variant.
            Mouse(smart_keymap::key::mouse::Event),
            /// [smart_keymap::key::sticky] variant.
            Sticky(smart_keymap::key::sticky::Event),
            /// [smart_keymap::key::tap_dance] variant.
            TapDance(smart_keymap::key::tap_dance::Event),
            /// [smart_keymap::key::tap_hold] variant.
            TapHold(smart_keymap::key::tap_hold::Event),
        }

        impl From<smart_keymap::key::callback::Event> for Event {
            fn from(v: smart_keymap::key::callback::Event) -> Self {
                Event::Callback(v)
            }
        }
        impl From<smart_keymap::key::chorded::Event> for Event {
            fn from(v: smart_keymap::key::chorded::Event) -> Self {
                Event::Chorded(v)
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
        impl From<smart_keymap::key::mouse::Event> for Event {
            fn from(v: smart_keymap::key::mouse::Event) -> Self {
                Event::Mouse(v)
            }
        }
        impl From<smart_keymap::key::sticky::Event> for Event {
            fn from(v: smart_keymap::key::sticky::Event) -> Self {
                Event::Sticky(v)
            }
        }
        impl From<smart_keymap::key::tap_dance::Event> for Event {
            fn from(v: smart_keymap::key::tap_dance::Event) -> Self {
                Event::TapDance(v)
            }
        }
        impl From<smart_keymap::key::tap_hold::Event> for Event {
            fn from(v: smart_keymap::key::tap_hold::Event) -> Self {
                Event::TapHold(v)
            }
        }
        impl TryFrom<Event> for smart_keymap::key::callback::Event {
            type Error = smart_keymap::key::EventError;
            fn try_from(v: Event) -> Result<Self, Self::Error> {
                match v {
                    Event::Callback(v) => Ok(v),
                    _ => Err(smart_keymap::key::EventError::UnmappableEvent),
                }
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
        impl TryFrom<Event> for smart_keymap::key::consumer::Event {
            type Error = smart_keymap::key::EventError;
            fn try_from(v: Event) -> Result<Self, Self::Error> {
                match v {
                    Event::Consumer(v) => Ok(v),
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
        impl TryFrom<Event> for smart_keymap::key::layered::LayerEvent {
            type Error = smart_keymap::key::EventError;
            fn try_from(v: Event) -> Result<Self, Self::Error> {
                match v {
                    Event::Layered(v) => Ok(v),
                    _ => Err(smart_keymap::key::EventError::UnmappableEvent),
                }
            }
        }
        impl TryFrom<Event> for smart_keymap::key::mouse::Event {
            type Error = smart_keymap::key::EventError;
            fn try_from(v: Event) -> Result<Self, Self::Error> {
                match v {
                    Event::Mouse(v) => Ok(v),
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
        impl TryFrom<Event> for smart_keymap::key::tap_dance::Event {
            type Error = smart_keymap::key::EventError;
            fn try_from(v: Event) -> Result<Self, Self::Error> {
                match v {
                    Event::TapDance(v) => Ok(v),
                    _ => Err(smart_keymap::key::EventError::UnmappableEvent),
                }
            }
        }
        impl TryFrom<Event> for smart_keymap::key::tap_hold::Event {
            type Error = smart_keymap::key::EventError;
            fn try_from(v: Event) -> Result<Self, Self::Error> {
                match v {
                    Event::TapHold(v) => Ok(v),
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
            /// [smart_keymap::key::chorded] variant.
            Chorded(
                smart_keymap::key::chorded::PendingKeyState<
                    { crate::init::CHORDED_MAX_CHORDS },
                    { crate::init::CHORDED_MAX_CHORD_SIZE },
                    CHORDED_MAX_PRESSED_INDICES,
                >,
            ),
            /// [smart_keymap::key::consumer] variant.
            Consumer(smart_keymap::key::consumer::PendingKeyState),
            /// [smart_keymap::key::keyboard] variant.
            Keyboard(smart_keymap::key::keyboard::PendingKeyState),
            /// [smart_keymap::key::layered] variant.
            Layered(smart_keymap::key::layered::PendingKeyState),
            /// [smart_keymap::key::mouse] variant.
            Mouse(smart_keymap::key::mouse::PendingKeyState),
            /// [smart_keymap::key::sticky] variant.
            Sticky(smart_keymap::key::sticky::PendingKeyState),
            /// [smart_keymap::key::tap_dance] variant.
            TapDance(smart_keymap::key::tap_dance::PendingKeyState),
            /// [smart_keymap::key::tap_hold] variant.
            TapHold(smart_keymap::key::tap_hold::PendingKeyState),
        }

        impl From<smart_keymap::key::callback::PendingKeyState> for PendingKeyState {
            fn from(pks: smart_keymap::key::callback::PendingKeyState) -> Self {
                PendingKeyState::Callback(pks)
            }
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
        impl From<smart_keymap::key::mouse::PendingKeyState> for PendingKeyState {
            fn from(pks: smart_keymap::key::mouse::PendingKeyState) -> Self {
                PendingKeyState::Mouse(pks)
            }
        }
        impl From<smart_keymap::key::sticky::PendingKeyState> for PendingKeyState {
            fn from(pks: smart_keymap::key::sticky::PendingKeyState) -> Self {
                PendingKeyState::Sticky(pks)
            }
        }
        impl From<smart_keymap::key::tap_dance::PendingKeyState> for PendingKeyState {
            fn from(pks: smart_keymap::key::tap_dance::PendingKeyState) -> Self {
                PendingKeyState::TapDance(pks)
            }
        }
        impl From<smart_keymap::key::tap_hold::PendingKeyState> for PendingKeyState {
            fn from(pks: smart_keymap::key::tap_hold::PendingKeyState) -> Self {
                PendingKeyState::TapHold(pks)
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
        impl<'pks> TryFrom<&'pks mut PendingKeyState>
            for &'pks mut smart_keymap::key::tap_dance::PendingKeyState
        {
            type Error = ();
            fn try_from(pks: &'pks mut PendingKeyState) -> Result<Self, Self::Error> {
                match pks {
                    PendingKeyState::TapDance(pks) => Ok(pks),
                    _ => Err(()),
                }
            }
        }
        impl<'pks> TryFrom<&'pks mut PendingKeyState>
            for &'pks mut smart_keymap::key::tap_hold::PendingKeyState
        {
            type Error = ();
            fn try_from(pks: &'pks mut PendingKeyState) -> Result<Self, Self::Error> {
                match pks {
                    PendingKeyState::TapHold(pks) => Ok(pks),
                    _ => Err(()),
                }
            }
        }

        /// Aggregate key state.
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum KeyState {
            /// No-op key state (e.g. auxiliary chorded keys).
            NoOp,
            /// [smart_keymap::key::callback] key state.
            Callback(smart_keymap::key::callback::KeyState),
            /// [smart_keymap::key::chorded] key state.
            Chorded(smart_keymap::key::chorded::KeyState),
            /// [smart_keymap::key::consumer] key state.
            Consumer(smart_keymap::key::consumer::KeyState),
            /// [smart_keymap::key::keyboard] key state.
            Keyboard(smart_keymap::key::keyboard::KeyState),
            /// [smart_keymap::key::layered] key state.
            LayerModifier(smart_keymap::key::layered::ModifierKeyState),
            /// [smart_keymap::key::mouse] key state.
            Mouse(smart_keymap::key::mouse::KeyState),
            /// [smart_keymap::key::sticky] key state.
            Sticky(smart_keymap::key::sticky::KeyState),
            /// [smart_keymap::key::tap_dance] key state.
            TapDance(smart_keymap::key::tap_dance::KeyState),
            /// [smart_keymap::key::tap_hold] key state.
            TapHold(smart_keymap::key::tap_hold::KeyState),
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
        impl From<smart_keymap::key::chorded::KeyState> for KeyState {
            fn from(ks: smart_keymap::key::chorded::KeyState) -> Self {
                KeyState::Chorded(ks)
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
        impl From<smart_keymap::key::mouse::KeyState> for KeyState {
            fn from(ks: smart_keymap::key::mouse::KeyState) -> Self {
                KeyState::Mouse(ks)
            }
        }
        impl From<smart_keymap::key::sticky::KeyState> for KeyState {
            fn from(ks: smart_keymap::key::sticky::KeyState) -> Self {
                KeyState::Sticky(ks)
            }
        }
        impl From<smart_keymap::key::tap_dance::KeyState> for KeyState {
            fn from(ks: smart_keymap::key::tap_dance::KeyState) -> Self {
                KeyState::TapDance(ks)
            }
        }
        impl From<smart_keymap::key::tap_hold::KeyState> for KeyState {
            fn from(ks: smart_keymap::key::tap_hold::KeyState) -> Self {
                KeyState::TapHold(ks)
            }
        }

        /// Aggregate [key::System] for this keymap.
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct System {
            callback: smart_keymap::key::callback::System<
                Ref,
                [smart_keymap::key::callback::Key; { crate::init::CALLBACK }],
            >,
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
            consumer: smart_keymap::key::consumer::System<Ref>,
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
            mouse: smart_keymap::key::mouse::System<Ref>,
            sticky: smart_keymap::key::sticky::System<
                Ref,
                [smart_keymap::key::sticky::Key; { crate::init::STICKY }],
            >,
            tap_dance: smart_keymap::key::tap_dance::System<
                Ref,
                [smart_keymap::key::tap_dance::Key<Ref, { crate::init::TAP_DANCE_MAX_DEFINITIONS }>;
                    { crate::init::TAP_DANCE }],
                { crate::init::TAP_DANCE_MAX_DEFINITIONS },
            >,
            tap_hold: smart_keymap::key::tap_hold::System<
                Ref,
                [smart_keymap::key::tap_hold::Key<Ref>; { crate::init::TAP_HOLD }],
            >,
        }

        impl System {
            /// Constructs the system from data-carrying subsystems.
            #[allow(clippy::too_many_arguments)]
            pub const fn new(
                callback: smart_keymap::key::callback::System<
                    Ref,
                    [smart_keymap::key::callback::Key; { crate::init::CALLBACK }],
                >,
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
                tap_dance: smart_keymap::key::tap_dance::System<
                    Ref,
                    [smart_keymap::key::tap_dance::Key<
                        Ref,
                        { crate::init::TAP_DANCE_MAX_DEFINITIONS },
                    >; { crate::init::TAP_DANCE }],
                    { crate::init::TAP_DANCE_MAX_DEFINITIONS },
                >,
                tap_hold: smart_keymap::key::tap_hold::System<
                    Ref,
                    [smart_keymap::key::tap_hold::Key<Ref>; { crate::init::TAP_HOLD }],
                >,
            ) -> Self {
                Self {
                    callback,
                    chorded,
                    keyboard,
                    layered,
                    sticky,
                    tap_dance,
                    tap_hold,
                    consumer: smart_keymap::key::consumer::System::new(),
                    mouse: smart_keymap::key::mouse::System::new(),
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
                    Ref::Chorded(key_ref) => {
                        let (pkr, pke) =
                            self.chorded
                                .new_pressed_key(keymap_index, &context.chorded, key_ref);
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
                    Ref::Mouse(key_ref) => {
                        let (pkr, pke) =
                            self.mouse
                                .new_pressed_key(keymap_index, &context.mouse, key_ref);
                        (pkr.into_result(), pke.into_events())
                    }
                    Ref::Sticky(key_ref) => {
                        let (pkr, pke) =
                            self.sticky
                                .new_pressed_key(keymap_index, &context.sticky, key_ref);
                        (pkr.into_result(), pke.into_events())
                    }
                    Ref::TapDance(key_ref) => {
                        let (pkr, pke) = self.tap_dance.new_pressed_key(
                            keymap_index,
                            &context.tap_dance,
                            key_ref,
                        );
                        (pkr.into_result(), pke.into_events())
                    }
                    Ref::TapHold(key_ref) => {
                        let (pkr, pke) =
                            self.tap_hold
                                .new_pressed_key(keymap_index, &context.tap_hold, key_ref);
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
                    (Ref::TapDance(key_ref), PendingKeyState::TapDance(pending_state)) => {
                        if let Ok(event) = event.try_into_key_event() {
                            let (maybe_npk, pke) = self.tap_dance.update_pending_state(
                                pending_state,
                                keymap_index,
                                &context.tap_dance,
                                key_ref,
                                event,
                            );
                            (maybe_npk, pke.into_events())
                        } else {
                            (None, smart_keymap::key::KeyEvents::no_events())
                        }
                    }
                    (Ref::TapHold(key_ref), PendingKeyState::TapHold(pending_state)) => {
                        if let Ok(event) = event.try_into_key_event() {
                            let (maybe_npk, pke) = self.tap_hold.update_pending_state(
                                pending_state,
                                keymap_index,
                                &context.tap_hold,
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
                    (Ref::Consumer(r), KeyState::Consumer(ks)) => self.consumer.key_output(r, ks),
                    (Ref::Keyboard(r), KeyState::Keyboard(ks)) => self.keyboard.key_output(r, ks),
                    (Ref::Layered(r), KeyState::LayerModifier(ks)) => {
                        self.layered.key_output(r, ks)
                    }
                    (Ref::Mouse(r), KeyState::Mouse(ks)) => self.mouse.key_output(r, ks),
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
    pub const KEY_COUNT: usize = 48;

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
        key_system::Ref::Chorded(smart_keymap::key::chorded::Ref::Chorded(0)),
        key_system::Ref::Chorded(smart_keymap::key::chorded::Ref::Auxiliary(0)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(28)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(29)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(30)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(31)),
        key_system::Ref::Chorded(smart_keymap::key::chorded::Ref::Chorded(1)),
        key_system::Ref::Chorded(smart_keymap::key::chorded::Ref::Auxiliary(1)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(34)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(35)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(36)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(37)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(38)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(39)),
        key_system::Ref::Chorded(smart_keymap::key::chorded::Ref::Chorded(2)),
        key_system::Ref::Chorded(smart_keymap::key::chorded::Ref::Auxiliary(2)),
        key_system::Ref::Chorded(smart_keymap::key::chorded::Ref::Chorded(3)),
        key_system::Ref::Chorded(smart_keymap::key::chorded::Ref::Auxiliary(3)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(44)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(45)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(46)),
        key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(47)),
    ];

    /// The keymap config.
    pub const CONFIG: key_system::Config = key_system::Config {
        chorded: smart_keymap::key::chorded::Config {
            chords: smart_keymap::slice::Slice::from_slice(&[
                smart_keymap::key::chorded::ChordIndices::from_slice(&[26, 27]),
                smart_keymap::key::chorded::ChordIndices::from_slice(&[32, 33]),
                smart_keymap::key::chorded::ChordIndices::from_slice(&[40, 41]),
                smart_keymap::key::chorded::ChordIndices::from_slice(&[42, 43]),
            ]),
            ..smart_keymap::key::chorded::Config::new()
        },
        layered: smart_keymap::key::layered::Config::new(),
        sticky: smart_keymap::key::sticky::Config::new(),
        tap_dance: smart_keymap::key::tap_dance::Config::new(),
        tap_hold: smart_keymap::key::tap_hold::Config {
            interrupt_response: smart_keymap::key::tap_hold::InterruptResponse::HoldOnKeyTap,
            ..smart_keymap::key::tap_hold::Config::new()
        },
    };

    /// Initial [Context] value.
    pub const CONTEXT: Context = key_system::Context::from_config(key_system::Config {
        chorded: smart_keymap::key::chorded::Config {
            chords: smart_keymap::slice::Slice::from_slice(&[
                smart_keymap::key::chorded::ChordIndices::from_slice(&[26, 27]),
                smart_keymap::key::chorded::ChordIndices::from_slice(&[32, 33]),
                smart_keymap::key::chorded::ChordIndices::from_slice(&[40, 41]),
                smart_keymap::key::chorded::ChordIndices::from_slice(&[42, 43]),
            ]),
            ..smart_keymap::key::chorded::Config::new()
        },
        layered: smart_keymap::key::layered::Config::new(),
        sticky: smart_keymap::key::sticky::Config::new(),
        tap_dance: smart_keymap::key::tap_dance::Config::new(),
        tap_hold: smart_keymap::key::tap_hold::Config {
            interrupt_response: smart_keymap::key::tap_hold::InterruptResponse::HoldOnKeyTap,
            ..smart_keymap::key::tap_hold::Config::new()
        },
    });

    /// The key system.
    pub const SYSTEM: System = key_system::System::new(
        smart_keymap::key::callback::System::new([
            smart_keymap::key::callback::Key::new(
                smart_keymap::keymap::KeymapCallback::ResetToBootloader,
            ),
            smart_keymap::key::callback::Key::new(
                smart_keymap::keymap::KeymapCallback::ResetToBootloader,
            ),
            smart_keymap::key::callback::Key::new(
                smart_keymap::keymap::KeymapCallback::ResetToBootloader,
            ),
            smart_keymap::key::callback::Key::new(
                smart_keymap::keymap::KeymapCallback::ResetToBootloader,
            ),
            smart_keymap::key::callback::Key::new(
                smart_keymap::keymap::KeymapCallback::ResetToBootloader,
            ),
            smart_keymap::key::callback::Key::new(
                smart_keymap::keymap::KeymapCallback::ResetToBootloader,
            ),
            smart_keymap::key::callback::Key::new(
                smart_keymap::keymap::KeymapCallback::ResetToBootloader,
            ),
        ]),
        smart_keymap::key::chorded::System::new(
            [
                smart_keymap::key::chorded::Key::new(
                    &[(
                        0,
                        key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(12),
                        ),
                    )],
                    key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(26)),
                ),
                smart_keymap::key::chorded::Key::new(
                    &[(
                        1,
                        key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(16),
                        ),
                    )],
                    key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(32)),
                ),
                smart_keymap::key::chorded::Key::new(
                    &[(
                        2,
                        key_system::Ref::TapHold(smart_keymap::key::tap_hold::Ref(20)),
                    )],
                    key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(40)),
                ),
                smart_keymap::key::chorded::Key::new(
                    &[(
                        3,
                        key_system::Ref::TapHold(smart_keymap::key::tap_hold::Ref(29)),
                    )],
                    key_system::Ref::Layered(smart_keymap::key::layered::Ref::Layered(42)),
                ),
            ],
            [
                smart_keymap::key::chorded::AuxiliaryKey::new(key_system::Ref::Layered(
                    smart_keymap::key::layered::Ref::Layered(27),
                )),
                smart_keymap::key::chorded::AuxiliaryKey::new(key_system::Ref::Layered(
                    smart_keymap::key::layered::Ref::Layered(33),
                )),
                smart_keymap::key::chorded::AuxiliaryKey::new(key_system::Ref::Layered(
                    smart_keymap::key::layered::Ref::Layered(41),
                )),
                smart_keymap::key::chorded::AuxiliaryKey::new(key_system::Ref::Layered(
                    smart_keymap::key::layered::Ref::Layered(43),
                )),
            ],
        ),
        smart_keymap::key::keyboard::System::new([
            smart_keymap::key::keyboard::Key {
                key_code: 47,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(2),
            },
            smart_keymap::key::keyboard::Key {
                key_code: 36,
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
                key_code: 48,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(2),
            },
            smart_keymap::key::keyboard::Key {
                key_code: 53,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(2),
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
                key_code: 35,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(2),
            },
            smart_keymap::key::keyboard::Key {
                key_code: 46,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(2),
            },
            smart_keymap::key::keyboard::Key {
                key_code: 56,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(2),
            },
            smart_keymap::key::keyboard::Key {
                key_code: 30,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(2),
            },
            smart_keymap::key::keyboard::Key {
                key_code: 75,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(8),
            },
            smart_keymap::key::keyboard::Key {
                key_code: 31,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(2),
            },
            smart_keymap::key::keyboard::Key {
                key_code: 32,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(2),
            },
            smart_keymap::key::keyboard::Key {
                key_code: 49,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(2),
            },
            smart_keymap::key::keyboard::Key {
                key_code: 78,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(8),
            },
            smart_keymap::key::keyboard::Key {
                key_code: 55,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(2),
            },
            smart_keymap::key::keyboard::Key {
                key_code: 39,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(2),
            },
            smart_keymap::key::keyboard::Key {
                key_code: 45,
                modifiers: smart_keymap::key::KeyboardModifiers::from_byte(2),
            },
        ]),
        smart_keymap::key::layered::System::new(
            [
                smart_keymap::key::layered::ModifierKey::default(3),
                smart_keymap::key::layered::ModifierKey::default(3),
                smart_keymap::key::layered::ModifierKey::default(3),
                smart_keymap::key::layered::ModifierKey::default(2),
                smart_keymap::key::layered::ModifierKey::default(2),
                smart_keymap::key::layered::ModifierKey::default(2),
                smart_keymap::key::layered::ModifierKey::default(0),
                smart_keymap::key::layered::ModifierKey::default(0),
                smart_keymap::key::layered::ModifierKey::default(0),
                smart_keymap::key::layered::ModifierKey::default(1),
                smart_keymap::key::layered::ModifierKey::default(1),
                smart_keymap::key::layered::ModifierKey::default(1),
                smart_keymap::key::layered::ModifierKey::default(1),
                smart_keymap::key::layered::ModifierKey::default(1),
                smart_keymap::key::layered::ModifierKey::default(1),
                smart_keymap::key::layered::ModifierKey::default(0),
                smart_keymap::key::layered::ModifierKey::default(0),
                smart_keymap::key::layered::ModifierKey::default(0),
                smart_keymap::key::layered::ModifierKey::default(2),
                smart_keymap::key::layered::ModifierKey::default(2),
                smart_keymap::key::layered::ModifierKey::default(2),
                smart_keymap::key::layered::ModifierKey::default(3),
                smart_keymap::key::layered::ModifierKey::default(3),
                smart_keymap::key::layered::ModifierKey::default(3),
                smart_keymap::key::layered::ModifierKey::hold(8),
                smart_keymap::key::layered::ModifierKey::hold(8),
                smart_keymap::key::layered::ModifierKey::hold(8),
                smart_keymap::key::layered::ModifierKey::hold(8),
                smart_keymap::key::layered::ModifierKey::hold(8),
                smart_keymap::key::layered::ModifierKey::hold(9),
                smart_keymap::key::layered::ModifierKey::hold(9),
                smart_keymap::key::layered::ModifierKey::hold(9),
                smart_keymap::key::layered::ModifierKey::hold(9),
                smart_keymap::key::layered::ModifierKey::hold(7),
                smart_keymap::key::layered::ModifierKey::hold(7),
                smart_keymap::key::layered::ModifierKey::hold(7),
                smart_keymap::key::layered::ModifierKey::hold(7),
                smart_keymap::key::layered::ModifierKey::hold(6),
                smart_keymap::key::layered::ModifierKey::hold(5),
                smart_keymap::key::layered::ModifierKey::hold(5),
                smart_keymap::key::layered::ModifierKey::hold(5),
                smart_keymap::key::layered::ModifierKey::hold(5),
                smart_keymap::key::layered::ModifierKey::hold(4),
                smart_keymap::key::layered::ModifierKey::hold(4),
                smart_keymap::key::layered::ModifierKey::hold(4),
                smart_keymap::key::layered::ModifierKey::hold(4),
                smart_keymap::key::layered::ModifierKey::hold(6),
                smart_keymap::key::layered::ModifierKey::hold(6),
                smart_keymap::key::layered::ModifierKey::hold(6),
                smart_keymap::key::layered::ModifierKey::hold(6),
            ],
            [
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(52)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(52),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(20),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(20),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(47),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(0),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(69),
                        )),
                        Some(key_system::Ref::TapDance(
                            smart_keymap::key::tap_dance::Ref(0),
                        )),
                        Some(key_system::Ref::TapDance(
                            smart_keymap::key::tap_dance::Ref(1),
                        )),
                        Some(key_system::Ref::TapDance(
                            smart_keymap::key::tap_dance::Ref(2),
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
                            smart_keymap::key::keyboard::Ref::KeyCode(26),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(26),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(36),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(1),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(64),
                        )),
                        Some(key_system::Ref::TapDance(
                            smart_keymap::key::tap_dance::Ref(3),
                        )),
                        Some(key_system::Ref::TapDance(
                            smart_keymap::key::tap_dance::Ref(4),
                        )),
                        Some(key_system::Ref::TapDance(
                            smart_keymap::key::tap_dance::Ref(5),
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
                            smart_keymap::key::keyboard::Ref::KeyCode(8),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(8),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(37),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(2),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(65),
                        )),
                        Some(key_system::Ref::TapDance(
                            smart_keymap::key::tap_dance::Ref(6),
                        )),
                        Some(key_system::Ref::TapDance(
                            smart_keymap::key::tap_dance::Ref(7),
                        )),
                        Some(key_system::Ref::TapDance(
                            smart_keymap::key::tap_dance::Ref(8),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(19)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(19),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(21),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(21),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(38),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(3),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(66),
                        )),
                        Some(key_system::Ref::TapDance(
                            smart_keymap::key::tap_dance::Ref(9),
                        )),
                        Some(key_system::Ref::TapDance(
                            smart_keymap::key::tap_dance::Ref(10),
                        )),
                        Some(key_system::Ref::TapDance(
                            smart_keymap::key::tap_dance::Ref(11),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(28)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(28),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(23),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(23),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(48),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(4),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(70),
                        )),
                        Some(key_system::Ref::TapDance(
                            smart_keymap::key::tap_dance::Ref(12),
                        )),
                        Some(key_system::Ref::TapDance(
                            smart_keymap::key::tap_dance::Ref(13),
                        )),
                        Some(key_system::Ref::TapDance(
                            smart_keymap::key::tap_dance::Ref(14),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(0)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
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
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
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
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(0)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
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
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
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
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(9)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(9),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(28),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(28),
                        )),
                        Some(key_system::Ref::TapDance(
                            smart_keymap::key::tap_dance::Ref(15),
                        )),
                        Some(key_system::Ref::TapDance(
                            smart_keymap::key::tap_dance::Ref(16),
                        )),
                        Some(key_system::Ref::TapDance(
                            smart_keymap::key::tap_dance::Ref(17),
                        )),
                        None,
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(10)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(10),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(24),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(24),
                        )),
                        Some(key_system::Ref::TapDance(
                            smart_keymap::key::tap_dance::Ref(18),
                        )),
                        Some(key_system::Ref::TapDance(
                            smart_keymap::key::tap_dance::Ref(19),
                        )),
                        Some(key_system::Ref::TapDance(
                            smart_keymap::key::tap_dance::Ref(20),
                        )),
                        None,
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
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
                            smart_keymap::key::keyboard::Ref::KeyCode(12),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(12),
                        )),
                        Some(key_system::Ref::TapDance(
                            smart_keymap::key::tap_dance::Ref(21),
                        )),
                        Some(key_system::Ref::TapDance(
                            smart_keymap::key::tap_dance::Ref(22),
                        )),
                        Some(key_system::Ref::TapDance(
                            smart_keymap::key::tap_dance::Ref(23),
                        )),
                        None,
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(21)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(21),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(18),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(18),
                        )),
                        Some(key_system::Ref::TapDance(
                            smart_keymap::key::tap_dance::Ref(24),
                        )),
                        Some(key_system::Ref::TapDance(
                            smart_keymap::key::tap_dance::Ref(25),
                        )),
                        Some(key_system::Ref::TapDance(
                            smart_keymap::key::tap_dance::Ref(26),
                        )),
                        None,
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(15)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(15),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(19),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(19),
                        )),
                        Some(key_system::Ref::TapDance(
                            smart_keymap::key::tap_dance::Ref(27),
                        )),
                        Some(key_system::Ref::TapDance(
                            smart_keymap::key::tap_dance::Ref(28),
                        )),
                        Some(key_system::Ref::TapDance(
                            smart_keymap::key::tap_dance::Ref(29),
                        )),
                        None,
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::TapHold(smart_keymap::key::tap_hold::Ref(0)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(4),
                        )),
                        Some(key_system::Ref::TapHold(smart_keymap::key::tap_hold::Ref(
                            1,
                        ))),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(4),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(53),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(5),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(68),
                        )),
                        Some(key_system::Ref::Sticky(smart_keymap::key::sticky::Ref(0))),
                        Some(key_system::Ref::Sticky(smart_keymap::key::sticky::Ref(1))),
                        Some(key_system::Ref::Sticky(smart_keymap::key::sticky::Ref(2))),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::TapHold(smart_keymap::key::tap_hold::Ref(2)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(18),
                        )),
                        Some(key_system::Ref::TapHold(smart_keymap::key::tap_hold::Ref(
                            3,
                        ))),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(22),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(33),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(6),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(61),
                        )),
                        Some(key_system::Ref::Sticky(smart_keymap::key::sticky::Ref(3))),
                        Some(key_system::Ref::Sticky(smart_keymap::key::sticky::Ref(4))),
                        Some(key_system::Ref::Sticky(smart_keymap::key::sticky::Ref(5))),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::TapHold(smart_keymap::key::tap_hold::Ref(4)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(8),
                        )),
                        Some(key_system::Ref::TapHold(smart_keymap::key::tap_hold::Ref(
                            5,
                        ))),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(7),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(34),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(7),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(62),
                        )),
                        Some(key_system::Ref::Sticky(smart_keymap::key::sticky::Ref(6))),
                        Some(key_system::Ref::Sticky(smart_keymap::key::sticky::Ref(7))),
                        Some(key_system::Ref::Sticky(smart_keymap::key::sticky::Ref(8))),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::TapHold(smart_keymap::key::tap_hold::Ref(6)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(24),
                        )),
                        Some(key_system::Ref::TapHold(smart_keymap::key::tap_hold::Ref(
                            7,
                        ))),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(9),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(35),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(8),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(63),
                        )),
                        Some(key_system::Ref::Sticky(smart_keymap::key::sticky::Ref(9))),
                        Some(key_system::Ref::Sticky(smart_keymap::key::sticky::Ref(10))),
                        Some(key_system::Ref::Sticky(smart_keymap::key::sticky::Ref(11))),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(12)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(12),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(10),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(10),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(46),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(9),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(71),
                        )),
                        None,
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(0)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
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
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
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
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(0)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
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
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
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
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(7)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(7),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(11),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(11),
                        )),
                        None,
                        None,
                        None,
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(80),
                        )),
                        Some(key_system::Ref::Mouse(
                            smart_keymap::key::mouse::Ref::CursorLeft,
                        )),
                        Some(key_system::Ref::Consumer(
                            smart_keymap::key::consumer::Ref::UsageCode(182),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::TapHold(smart_keymap::key::tap_hold::Ref(8)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(11),
                        )),
                        Some(key_system::Ref::TapHold(smart_keymap::key::tap_hold::Ref(
                            9,
                        ))),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(13),
                        )),
                        Some(key_system::Ref::Sticky(smart_keymap::key::sticky::Ref(12))),
                        Some(key_system::Ref::Sticky(smart_keymap::key::sticky::Ref(13))),
                        Some(key_system::Ref::Sticky(smart_keymap::key::sticky::Ref(14))),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(81),
                        )),
                        Some(key_system::Ref::Mouse(
                            smart_keymap::key::mouse::Ref::CursorDown,
                        )),
                        Some(key_system::Ref::Consumer(
                            smart_keymap::key::consumer::Ref::UsageCode(234),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::TapHold(smart_keymap::key::tap_hold::Ref(10)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(23),
                        )),
                        Some(key_system::Ref::TapHold(smart_keymap::key::tap_hold::Ref(
                            11,
                        ))),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(14),
                        )),
                        Some(key_system::Ref::Sticky(smart_keymap::key::sticky::Ref(15))),
                        Some(key_system::Ref::Sticky(smart_keymap::key::sticky::Ref(16))),
                        Some(key_system::Ref::Sticky(smart_keymap::key::sticky::Ref(17))),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(82),
                        )),
                        Some(key_system::Ref::Mouse(
                            smart_keymap::key::mouse::Ref::CursorUp,
                        )),
                        Some(key_system::Ref::Consumer(
                            smart_keymap::key::consumer::Ref::UsageCode(233),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::TapHold(smart_keymap::key::tap_hold::Ref(12)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(17),
                        )),
                        Some(key_system::Ref::TapHold(smart_keymap::key::tap_hold::Ref(
                            13,
                        ))),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(15),
                        )),
                        Some(key_system::Ref::Sticky(smart_keymap::key::sticky::Ref(18))),
                        Some(key_system::Ref::Sticky(smart_keymap::key::sticky::Ref(19))),
                        Some(key_system::Ref::Sticky(smart_keymap::key::sticky::Ref(20))),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(79),
                        )),
                        Some(key_system::Ref::Mouse(
                            smart_keymap::key::mouse::Ref::CursorRight,
                        )),
                        Some(key_system::Ref::Consumer(
                            smart_keymap::key::consumer::Ref::UsageCode(181),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::TapHold(smart_keymap::key::tap_hold::Ref(14)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(22),
                        )),
                        Some(key_system::Ref::TapHold(smart_keymap::key::tap_hold::Ref(
                            15,
                        ))),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(51),
                        )),
                        Some(key_system::Ref::Sticky(smart_keymap::key::sticky::Ref(21))),
                        Some(key_system::Ref::Sticky(smart_keymap::key::sticky::Ref(22))),
                        Some(key_system::Ref::Sticky(smart_keymap::key::sticky::Ref(23))),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(57),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(51)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(51),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(29),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(29),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(56),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(10),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(67),
                        )),
                        None,
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(20)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(20),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(27),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(27),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(30),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(11),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(58),
                        )),
                        None,
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(13)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(13),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(6),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(6),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(31),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(13),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(59),
                        )),
                        None,
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(14)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(14),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(25),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(25),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(32),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(14),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(60),
                        )),
                        None,
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(27)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(27),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(5),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(5),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(49),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(15),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(72),
                        )),
                        None,
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(0)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
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
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
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
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(0)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
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
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
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
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(5)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(5),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(17),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(17),
                        )),
                        None,
                        None,
                        None,
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(74),
                        )),
                        Some(key_system::Ref::Mouse(
                            smart_keymap::key::mouse::Ref::WheelLeft,
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(16)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(16),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(16),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(16),
                        )),
                        None,
                        None,
                        None,
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(78),
                        )),
                        Some(key_system::Ref::Mouse(
                            smart_keymap::key::mouse::Ref::WheelDown,
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(26)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(26),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(54),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(54),
                        )),
                        None,
                        None,
                        None,
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(75),
                        )),
                        Some(key_system::Ref::Mouse(
                            smart_keymap::key::mouse::Ref::WheelUp,
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(25)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(25),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(55),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(55),
                        )),
                        None,
                        None,
                        None,
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(77),
                        )),
                        Some(key_system::Ref::Mouse(
                            smart_keymap::key::mouse::Ref::WheelRight,
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
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
                            smart_keymap::key::keyboard::Ref::KeyCode(52),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(52),
                        )),
                        None,
                        None,
                        None,
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(73),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        Some(key_system::Ref::Callback(smart_keymap::key::callback::Ref(
                            6,
                        ))),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(0)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
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
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
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
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(0)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
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
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
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
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(0)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
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
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
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
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::TapHold(smart_keymap::key::tap_hold::Ref(16)),
                    [
                        Some(key_system::Ref::TapHold(smart_keymap::key::tap_hold::Ref(
                            17,
                        ))),
                        Some(key_system::Ref::TapHold(smart_keymap::key::tap_hold::Ref(
                            18,
                        ))),
                        Some(key_system::Ref::TapHold(smart_keymap::key::tap_hold::Ref(
                            19,
                        ))),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(55),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(17),
                        )),
                        None,
                        None,
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        None,
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::TapHold(smart_keymap::key::tap_hold::Ref(21)),
                    [
                        Some(key_system::Ref::TapHold(smart_keymap::key::tap_hold::Ref(
                            22,
                        ))),
                        Some(key_system::Ref::TapHold(smart_keymap::key::tap_hold::Ref(
                            23,
                        ))),
                        Some(key_system::Ref::TapHold(smart_keymap::key::tap_hold::Ref(
                            24,
                        ))),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(39),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(18),
                        )),
                        None,
                        None,
                        None,
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::TapHold(smart_keymap::key::tap_hold::Ref(25)),
                    [
                        Some(key_system::Ref::TapHold(smart_keymap::key::tap_hold::Ref(
                            26,
                        ))),
                        Some(key_system::Ref::TapHold(smart_keymap::key::tap_hold::Ref(
                            27,
                        ))),
                        Some(key_system::Ref::TapHold(smart_keymap::key::tap_hold::Ref(
                            28,
                        ))),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(45),
                        )),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCodeAndModifier(19),
                        )),
                        None,
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        None,
                        None,
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::TapHold(smart_keymap::key::tap_hold::Ref(30)),
                    [
                        Some(key_system::Ref::TapHold(smart_keymap::key::tap_hold::Ref(
                            31,
                        ))),
                        Some(key_system::Ref::TapHold(smart_keymap::key::tap_hold::Ref(
                            32,
                        ))),
                        Some(key_system::Ref::TapHold(smart_keymap::key::tap_hold::Ref(
                            33,
                        ))),
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        None,
                        None,
                        None,
                        Some(key_system::Ref::Mouse(
                            smart_keymap::key::mouse::Ref::Button(1),
                        )),
                        Some(key_system::Ref::Consumer(
                            smart_keymap::key::consumer::Ref::UsageCode(205),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::TapHold(smart_keymap::key::tap_hold::Ref(34)),
                    [
                        Some(key_system::Ref::TapHold(smart_keymap::key::tap_hold::Ref(
                            35,
                        ))),
                        Some(key_system::Ref::TapHold(smart_keymap::key::tap_hold::Ref(
                            36,
                        ))),
                        Some(key_system::Ref::TapHold(smart_keymap::key::tap_hold::Ref(
                            37,
                        ))),
                        None,
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        None,
                        None,
                        Some(key_system::Ref::Mouse(
                            smart_keymap::key::mouse::Ref::Button(2),
                        )),
                        Some(key_system::Ref::Consumer(
                            smart_keymap::key::consumer::Ref::UsageCode(183),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::TapHold(smart_keymap::key::tap_hold::Ref(38)),
                    [
                        Some(key_system::Ref::TapHold(smart_keymap::key::tap_hold::Ref(
                            39,
                        ))),
                        Some(key_system::Ref::TapHold(smart_keymap::key::tap_hold::Ref(
                            40,
                        ))),
                        Some(key_system::Ref::TapHold(smart_keymap::key::tap_hold::Ref(
                            41,
                        ))),
                        None,
                        None,
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                        None,
                        Some(key_system::Ref::Mouse(
                            smart_keymap::key::mouse::Ref::Button(3),
                        )),
                        Some(key_system::Ref::Consumer(
                            smart_keymap::key::consumer::Ref::UsageCode(226),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(0)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
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
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
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
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(0)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
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
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
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
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
                        )),
                    ],
                ),
                smart_keymap::key::layered::LayeredKey::new(
                    key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(0)),
                    [
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
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
                        Some(key_system::Ref::Keyboard(
                            smart_keymap::key::keyboard::Ref::KeyCode(0),
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
            smart_keymap::key::sticky::Key::new(smart_keymap::key::KeyboardModifiers::from_byte(8)),
            smart_keymap::key::sticky::Key::new(smart_keymap::key::KeyboardModifiers::from_byte(8)),
            smart_keymap::key::sticky::Key::new(smart_keymap::key::KeyboardModifiers::from_byte(8)),
            smart_keymap::key::sticky::Key::new(smart_keymap::key::KeyboardModifiers::from_byte(1)),
            smart_keymap::key::sticky::Key::new(smart_keymap::key::KeyboardModifiers::from_byte(1)),
            smart_keymap::key::sticky::Key::new(smart_keymap::key::KeyboardModifiers::from_byte(1)),
            smart_keymap::key::sticky::Key::new(smart_keymap::key::KeyboardModifiers::from_byte(2)),
            smart_keymap::key::sticky::Key::new(smart_keymap::key::KeyboardModifiers::from_byte(2)),
            smart_keymap::key::sticky::Key::new(smart_keymap::key::KeyboardModifiers::from_byte(2)),
            smart_keymap::key::sticky::Key::new(smart_keymap::key::KeyboardModifiers::from_byte(2)),
            smart_keymap::key::sticky::Key::new(smart_keymap::key::KeyboardModifiers::from_byte(2)),
            smart_keymap::key::sticky::Key::new(smart_keymap::key::KeyboardModifiers::from_byte(2)),
            smart_keymap::key::sticky::Key::new(smart_keymap::key::KeyboardModifiers::from_byte(1)),
            smart_keymap::key::sticky::Key::new(smart_keymap::key::KeyboardModifiers::from_byte(1)),
            smart_keymap::key::sticky::Key::new(smart_keymap::key::KeyboardModifiers::from_byte(1)),
            smart_keymap::key::sticky::Key::new(smart_keymap::key::KeyboardModifiers::from_byte(8)),
            smart_keymap::key::sticky::Key::new(smart_keymap::key::KeyboardModifiers::from_byte(8)),
            smart_keymap::key::sticky::Key::new(smart_keymap::key::KeyboardModifiers::from_byte(8)),
            smart_keymap::key::sticky::Key::new(smart_keymap::key::KeyboardModifiers::from_byte(4)),
            smart_keymap::key::sticky::Key::new(smart_keymap::key::KeyboardModifiers::from_byte(4)),
            smart_keymap::key::sticky::Key::new(smart_keymap::key::KeyboardModifiers::from_byte(4)),
        ]),
        smart_keymap::key::tap_dance::System::new([
            smart_keymap::key::tap_dance::Key::from_definitions(&[
                key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(0)),
                key_system::Ref::Callback(smart_keymap::key::callback::Ref(0)),
            ]),
            smart_keymap::key::tap_dance::Key::from_definitions(&[
                key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(0)),
                key_system::Ref::Callback(smart_keymap::key::callback::Ref(1)),
            ]),
            smart_keymap::key::tap_dance::Key::from_definitions(&[
                key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(0)),
                key_system::Ref::Callback(smart_keymap::key::callback::Ref(2)),
            ]),
            smart_keymap::key::tap_dance::Key::from_definitions(&[
                key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(0)),
                key_system::Ref::Layered(smart_keymap::key::layered::Ref::Modifier(0)),
            ]),
            smart_keymap::key::tap_dance::Key::from_definitions(&[
                key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(0)),
                key_system::Ref::Layered(smart_keymap::key::layered::Ref::Modifier(1)),
            ]),
            smart_keymap::key::tap_dance::Key::from_definitions(&[
                key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(0)),
                key_system::Ref::Layered(smart_keymap::key::layered::Ref::Modifier(2)),
            ]),
            smart_keymap::key::tap_dance::Key::from_definitions(&[
                key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(0)),
                key_system::Ref::Layered(smart_keymap::key::layered::Ref::Modifier(3)),
            ]),
            smart_keymap::key::tap_dance::Key::from_definitions(&[
                key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(0)),
                key_system::Ref::Layered(smart_keymap::key::layered::Ref::Modifier(4)),
            ]),
            smart_keymap::key::tap_dance::Key::from_definitions(&[
                key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(0)),
                key_system::Ref::Layered(smart_keymap::key::layered::Ref::Modifier(5)),
            ]),
            smart_keymap::key::tap_dance::Key::from_definitions(&[
                key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(0)),
                key_system::Ref::Layered(smart_keymap::key::layered::Ref::Modifier(6)),
            ]),
            smart_keymap::key::tap_dance::Key::from_definitions(&[
                key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(0)),
                key_system::Ref::Layered(smart_keymap::key::layered::Ref::Modifier(7)),
            ]),
            smart_keymap::key::tap_dance::Key::from_definitions(&[
                key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(0)),
                key_system::Ref::Layered(smart_keymap::key::layered::Ref::Modifier(8)),
            ]),
            smart_keymap::key::tap_dance::Key::from_definitions(&[
                key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(0)),
                key_system::Ref::Layered(smart_keymap::key::layered::Ref::Modifier(9)),
            ]),
            smart_keymap::key::tap_dance::Key::from_definitions(&[
                key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(0)),
                key_system::Ref::Layered(smart_keymap::key::layered::Ref::Modifier(10)),
            ]),
            smart_keymap::key::tap_dance::Key::from_definitions(&[
                key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(0)),
                key_system::Ref::Layered(smart_keymap::key::layered::Ref::Modifier(11)),
            ]),
            smart_keymap::key::tap_dance::Key::from_definitions(&[
                key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(0)),
                key_system::Ref::Layered(smart_keymap::key::layered::Ref::Modifier(12)),
            ]),
            smart_keymap::key::tap_dance::Key::from_definitions(&[
                key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(0)),
                key_system::Ref::Layered(smart_keymap::key::layered::Ref::Modifier(13)),
            ]),
            smart_keymap::key::tap_dance::Key::from_definitions(&[
                key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(0)),
                key_system::Ref::Layered(smart_keymap::key::layered::Ref::Modifier(14)),
            ]),
            smart_keymap::key::tap_dance::Key::from_definitions(&[
                key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(0)),
                key_system::Ref::Layered(smart_keymap::key::layered::Ref::Modifier(15)),
            ]),
            smart_keymap::key::tap_dance::Key::from_definitions(&[
                key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(0)),
                key_system::Ref::Layered(smart_keymap::key::layered::Ref::Modifier(16)),
            ]),
            smart_keymap::key::tap_dance::Key::from_definitions(&[
                key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(0)),
                key_system::Ref::Layered(smart_keymap::key::layered::Ref::Modifier(17)),
            ]),
            smart_keymap::key::tap_dance::Key::from_definitions(&[
                key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(0)),
                key_system::Ref::Layered(smart_keymap::key::layered::Ref::Modifier(18)),
            ]),
            smart_keymap::key::tap_dance::Key::from_definitions(&[
                key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(0)),
                key_system::Ref::Layered(smart_keymap::key::layered::Ref::Modifier(19)),
            ]),
            smart_keymap::key::tap_dance::Key::from_definitions(&[
                key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(0)),
                key_system::Ref::Layered(smart_keymap::key::layered::Ref::Modifier(20)),
            ]),
            smart_keymap::key::tap_dance::Key::from_definitions(&[
                key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(0)),
                key_system::Ref::Layered(smart_keymap::key::layered::Ref::Modifier(21)),
            ]),
            smart_keymap::key::tap_dance::Key::from_definitions(&[
                key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(0)),
                key_system::Ref::Layered(smart_keymap::key::layered::Ref::Modifier(22)),
            ]),
            smart_keymap::key::tap_dance::Key::from_definitions(&[
                key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(0)),
                key_system::Ref::Layered(smart_keymap::key::layered::Ref::Modifier(23)),
            ]),
            smart_keymap::key::tap_dance::Key::from_definitions(&[
                key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(0)),
                key_system::Ref::Callback(smart_keymap::key::callback::Ref(3)),
            ]),
            smart_keymap::key::tap_dance::Key::from_definitions(&[
                key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(0)),
                key_system::Ref::Callback(smart_keymap::key::callback::Ref(4)),
            ]),
            smart_keymap::key::tap_dance::Key::from_definitions(&[
                key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(0)),
                key_system::Ref::Callback(smart_keymap::key::callback::Ref(5)),
            ]),
        ]),
        smart_keymap::key::tap_hold::System::new([
            smart_keymap::key::tap_hold::Key {
                tap: key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(4)),
                hold: key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::Modifiers(4)),
            },
            smart_keymap::key::tap_hold::Key {
                tap: key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(4)),
                hold: key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::Modifiers(4)),
            },
            smart_keymap::key::tap_hold::Key {
                tap: key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(18)),
                hold: key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::Modifiers(8)),
            },
            smart_keymap::key::tap_hold::Key {
                tap: key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(22)),
                hold: key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::Modifiers(8)),
            },
            smart_keymap::key::tap_hold::Key {
                tap: key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(8)),
                hold: key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::Modifiers(1)),
            },
            smart_keymap::key::tap_hold::Key {
                tap: key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(7)),
                hold: key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::Modifiers(1)),
            },
            smart_keymap::key::tap_hold::Key {
                tap: key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(24)),
                hold: key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::Modifiers(2)),
            },
            smart_keymap::key::tap_hold::Key {
                tap: key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(9)),
                hold: key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::Modifiers(2)),
            },
            smart_keymap::key::tap_hold::Key {
                tap: key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(11)),
                hold: key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::Modifiers(32)),
            },
            smart_keymap::key::tap_hold::Key {
                tap: key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(13)),
                hold: key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::Modifiers(32)),
            },
            smart_keymap::key::tap_hold::Key {
                tap: key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(23)),
                hold: key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::Modifiers(1)),
            },
            smart_keymap::key::tap_hold::Key {
                tap: key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(14)),
                hold: key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::Modifiers(1)),
            },
            smart_keymap::key::tap_hold::Key {
                tap: key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(17)),
                hold: key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::Modifiers(128)),
            },
            smart_keymap::key::tap_hold::Key {
                tap: key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(15)),
                hold: key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::Modifiers(128)),
            },
            smart_keymap::key::tap_hold::Key {
                tap: key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(22)),
                hold: key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::Modifiers(4)),
            },
            smart_keymap::key::tap_hold::Key {
                tap: key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(51)),
                hold: key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::Modifiers(4)),
            },
            smart_keymap::key::tap_hold::Key {
                tap: key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(43)),
                hold: key_system::Ref::Layered(smart_keymap::key::layered::Ref::Modifier(24)),
            },
            smart_keymap::key::tap_hold::Key {
                tap: key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(43)),
                hold: key_system::Ref::Layered(smart_keymap::key::layered::Ref::Modifier(25)),
            },
            smart_keymap::key::tap_hold::Key {
                tap: key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(43)),
                hold: key_system::Ref::Layered(smart_keymap::key::layered::Ref::Modifier(26)),
            },
            smart_keymap::key::tap_hold::Key {
                tap: key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(43)),
                hold: key_system::Ref::Layered(smart_keymap::key::layered::Ref::Modifier(27)),
            },
            smart_keymap::key::tap_hold::Key {
                tap: key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(43)),
                hold: key_system::Ref::Layered(smart_keymap::key::layered::Ref::Modifier(28)),
            },
            smart_keymap::key::tap_hold::Key {
                tap: key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(41)),
                hold: key_system::Ref::Layered(smart_keymap::key::layered::Ref::Modifier(29)),
            },
            smart_keymap::key::tap_hold::Key {
                tap: key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(41)),
                hold: key_system::Ref::Layered(smart_keymap::key::layered::Ref::Modifier(30)),
            },
            smart_keymap::key::tap_hold::Key {
                tap: key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(41)),
                hold: key_system::Ref::Layered(smart_keymap::key::layered::Ref::Modifier(31)),
            },
            smart_keymap::key::tap_hold::Key {
                tap: key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(41)),
                hold: key_system::Ref::Layered(smart_keymap::key::layered::Ref::Modifier(32)),
            },
            smart_keymap::key::tap_hold::Key {
                tap: key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(44)),
                hold: key_system::Ref::Layered(smart_keymap::key::layered::Ref::Modifier(33)),
            },
            smart_keymap::key::tap_hold::Key {
                tap: key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(44)),
                hold: key_system::Ref::Layered(smart_keymap::key::layered::Ref::Modifier(34)),
            },
            smart_keymap::key::tap_hold::Key {
                tap: key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(44)),
                hold: key_system::Ref::Layered(smart_keymap::key::layered::Ref::Modifier(35)),
            },
            smart_keymap::key::tap_hold::Key {
                tap: key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(44)),
                hold: key_system::Ref::Layered(smart_keymap::key::layered::Ref::Modifier(36)),
            },
            smart_keymap::key::tap_hold::Key {
                tap: key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(76)),
                hold: key_system::Ref::Layered(smart_keymap::key::layered::Ref::Modifier(37)),
            },
            smart_keymap::key::tap_hold::Key {
                tap: key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(40)),
                hold: key_system::Ref::Layered(smart_keymap::key::layered::Ref::Modifier(38)),
            },
            smart_keymap::key::tap_hold::Key {
                tap: key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(40)),
                hold: key_system::Ref::Layered(smart_keymap::key::layered::Ref::Modifier(39)),
            },
            smart_keymap::key::tap_hold::Key {
                tap: key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(40)),
                hold: key_system::Ref::Layered(smart_keymap::key::layered::Ref::Modifier(40)),
            },
            smart_keymap::key::tap_hold::Key {
                tap: key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(40)),
                hold: key_system::Ref::Layered(smart_keymap::key::layered::Ref::Modifier(41)),
            },
            smart_keymap::key::tap_hold::Key {
                tap: key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(42)),
                hold: key_system::Ref::Layered(smart_keymap::key::layered::Ref::Modifier(42)),
            },
            smart_keymap::key::tap_hold::Key {
                tap: key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(42)),
                hold: key_system::Ref::Layered(smart_keymap::key::layered::Ref::Modifier(43)),
            },
            smart_keymap::key::tap_hold::Key {
                tap: key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(42)),
                hold: key_system::Ref::Layered(smart_keymap::key::layered::Ref::Modifier(44)),
            },
            smart_keymap::key::tap_hold::Key {
                tap: key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(42)),
                hold: key_system::Ref::Layered(smart_keymap::key::layered::Ref::Modifier(45)),
            },
            smart_keymap::key::tap_hold::Key {
                tap: key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(76)),
                hold: key_system::Ref::Layered(smart_keymap::key::layered::Ref::Modifier(46)),
            },
            smart_keymap::key::tap_hold::Key {
                tap: key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(76)),
                hold: key_system::Ref::Layered(smart_keymap::key::layered::Ref::Modifier(47)),
            },
            smart_keymap::key::tap_hold::Key {
                tap: key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(76)),
                hold: key_system::Ref::Layered(smart_keymap::key::layered::Ref::Modifier(48)),
            },
            smart_keymap::key::tap_hold::Key {
                tap: key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(76)),
                hold: key_system::Ref::Layered(smart_keymap::key::layered::Ref::Modifier(49)),
            },
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
