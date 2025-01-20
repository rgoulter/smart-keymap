/// Types and initial data used for constructing [KEYMAP].
pub mod init {
    const NUM_LAYERS: usize = 0;

    /// Alias for layers impl.
    pub type LayersImpl = crate::key::layered::ArrayImpl<NUM_LAYERS>;

    /// Alias for the NestedKey used for the [Context].
    pub type NestedKey = crate::key::composite::DefaultNestableKey;

    /// Types used in Composite keys.
    pub type CompositeImpl = crate::key::composite::CompositeImpl<NestedKey, LayersImpl>;

    /// Alias for Context type; i.e. [crate::key::context::Context] with generics.
    pub type Context = crate::key::composite::Context<LayersImpl>;

    /// Alias for Event type; i.e. [crate::key::context::Event].
    pub type Event = crate::key::composite::Event;

    /// Initial [Context] value.
    pub const CONTEXT: Context = crate::key::composite::Context {
        layer_context: crate::key::layered::Context {
            active_layers: [false; NUM_LAYERS],
        },
    };

    crate::tuples::define_keys!(60);

    /// Alias for a [tuples] KeysN type, as generated by keymap.ncl.
    pub type KeyDefinitionsType = Keys60<
        crate::key::keyboard::Key,
        crate::key::keyboard::Key,
        crate::key::keyboard::Key,
        crate::key::keyboard::Key,
        crate::key::keyboard::Key,
        crate::key::keyboard::Key,
        crate::key::keyboard::Key,
        crate::key::keyboard::Key,
        crate::key::keyboard::Key,
        crate::key::keyboard::Key,
        crate::key::keyboard::Key,
        crate::key::keyboard::Key,
        crate::key::keyboard::Key,
        crate::key::keyboard::Key,
        crate::key::keyboard::Key,
        crate::key::keyboard::Key,
        crate::key::keyboard::Key,
        crate::key::keyboard::Key,
        crate::key::keyboard::Key,
        crate::key::keyboard::Key,
        crate::key::keyboard::Key,
        crate::key::keyboard::Key,
        crate::key::keyboard::Key,
        crate::key::keyboard::Key,
        crate::key::keyboard::Key,
        crate::key::tap_hold::Key<crate::key::keyboard::Key>,
        crate::key::tap_hold::Key<crate::key::keyboard::Key>,
        crate::key::tap_hold::Key<crate::key::keyboard::Key>,
        crate::key::tap_hold::Key<crate::key::keyboard::Key>,
        crate::key::keyboard::Key,
        crate::key::keyboard::Key,
        crate::key::tap_hold::Key<crate::key::keyboard::Key>,
        crate::key::tap_hold::Key<crate::key::keyboard::Key>,
        crate::key::tap_hold::Key<crate::key::keyboard::Key>,
        crate::key::tap_hold::Key<crate::key::keyboard::Key>,
        crate::key::keyboard::Key,
        crate::key::keyboard::Key,
        crate::key::keyboard::Key,
        crate::key::keyboard::Key,
        crate::key::keyboard::Key,
        crate::key::keyboard::Key,
        crate::key::keyboard::Key,
        crate::key::keyboard::Key,
        crate::key::keyboard::Key,
        crate::key::keyboard::Key,
        crate::key::keyboard::Key,
        crate::key::keyboard::Key,
        crate::key::keyboard::Key,
        crate::key::keyboard::Key,
        crate::key::keyboard::Key,
        crate::key::keyboard::Key,
        crate::key::keyboard::Key,
        crate::key::keyboard::Key,
        crate::key::keyboard::Key,
        crate::key::keyboard::Key,
        crate::key::keyboard::Key,
        crate::key::keyboard::Key,
        crate::key::keyboard::Key,
        crate::key::keyboard::Key,
        crate::key::keyboard::Key,
        Context,
        Event,
    >;

    /// A [tuples] KeysN value with keys, as generated by keymap.ncl.
    pub const KEY_DEFINITIONS: KeyDefinitionsType = Keys60::new((
        crate::key::keyboard::Key::new(53),
        crate::key::keyboard::Key::new(30),
        crate::key::keyboard::Key::new(31),
        crate::key::keyboard::Key::new(32),
        crate::key::keyboard::Key::new(33),
        crate::key::keyboard::Key::new(34),
        crate::key::keyboard::Key::new(35),
        crate::key::keyboard::Key::new(36),
        crate::key::keyboard::Key::new(37),
        crate::key::keyboard::Key::new(38),
        crate::key::keyboard::Key::new(39),
        crate::key::keyboard::Key::new(76),
        crate::key::keyboard::Key::new(43),
        crate::key::keyboard::Key::new(52),
        crate::key::keyboard::Key::new(54),
        crate::key::keyboard::Key::new(55),
        crate::key::keyboard::Key::new(19),
        crate::key::keyboard::Key::new(28),
        crate::key::keyboard::Key::new(9),
        crate::key::keyboard::Key::new(10),
        crate::key::keyboard::Key::new(6),
        crate::key::keyboard::Key::new(21),
        crate::key::keyboard::Key::new(15),
        crate::key::keyboard::Key::new(42),
        crate::key::keyboard::Key::new(41),
        crate::key::tap_hold::Key::new(
            crate::key::keyboard::Key::new(4),
            crate::key::keyboard::Key::from_modifiers(
                crate::key::KeyboardModifiers::LEFT_ALT
                    .union(&crate::key::KeyboardModifiers::new()),
            ),
        ),
        crate::key::tap_hold::Key::new(
            crate::key::keyboard::Key::new(18),
            crate::key::keyboard::Key::from_modifiers(
                crate::key::KeyboardModifiers::LEFT_GUI
                    .union(&crate::key::KeyboardModifiers::new()),
            ),
        ),
        crate::key::tap_hold::Key::new(
            crate::key::keyboard::Key::new(8),
            crate::key::keyboard::Key::from_modifiers(
                crate::key::KeyboardModifiers::LEFT_CTRL
                    .union(&crate::key::KeyboardModifiers::new()),
            ),
        ),
        crate::key::tap_hold::Key::new(
            crate::key::keyboard::Key::new(24),
            crate::key::keyboard::Key::from_modifiers(
                crate::key::KeyboardModifiers::LEFT_SHIFT
                    .union(&crate::key::KeyboardModifiers::new()),
            ),
        ),
        crate::key::keyboard::Key::new(12),
        crate::key::keyboard::Key::new(7),
        crate::key::tap_hold::Key::new(
            crate::key::keyboard::Key::new(11),
            crate::key::keyboard::Key::from_modifiers(
                crate::key::KeyboardModifiers::RIGHT_SHIFT
                    .union(&crate::key::KeyboardModifiers::new()),
            ),
        ),
        crate::key::tap_hold::Key::new(
            crate::key::keyboard::Key::new(23),
            crate::key::keyboard::Key::from_modifiers(
                crate::key::KeyboardModifiers::RIGHT_CTRL
                    .union(&crate::key::KeyboardModifiers::new()),
            ),
        ),
        crate::key::tap_hold::Key::new(
            crate::key::keyboard::Key::new(17),
            crate::key::keyboard::Key::from_modifiers(
                crate::key::KeyboardModifiers::RIGHT_GUI
                    .union(&crate::key::KeyboardModifiers::new()),
            ),
        ),
        crate::key::tap_hold::Key::new(
            crate::key::keyboard::Key::new(22),
            crate::key::keyboard::Key::from_modifiers(
                crate::key::KeyboardModifiers::RIGHT_ALT
                    .union(&crate::key::KeyboardModifiers::new()),
            ),
        ),
        crate::key::keyboard::Key::new(40),
        crate::key::keyboard::Key::from_modifiers(
            crate::key::KeyboardModifiers::LEFT_SHIFT.union(&crate::key::KeyboardModifiers::new()),
        ),
        crate::key::keyboard::Key::new(51),
        crate::key::keyboard::Key::new(20),
        crate::key::keyboard::Key::new(13),
        crate::key::keyboard::Key::new(14),
        crate::key::keyboard::Key::new(27),
        crate::key::keyboard::Key::new(5),
        crate::key::keyboard::Key::new(16),
        crate::key::keyboard::Key::new(26),
        crate::key::keyboard::Key::new(25),
        crate::key::keyboard::Key::new(29),
        crate::key::keyboard::Key::from_modifiers(
            crate::key::KeyboardModifiers::RIGHT_SHIFT.union(&crate::key::KeyboardModifiers::new()),
        ),
        crate::key::keyboard::Key::from_modifiers(
            crate::key::KeyboardModifiers::LEFT_CTRL.union(&crate::key::KeyboardModifiers::new()),
        ),
        crate::key::keyboard::Key::from_modifiers(
            crate::key::KeyboardModifiers::LEFT_GUI.union(&crate::key::KeyboardModifiers::new()),
        ),
        crate::key::keyboard::Key::from_modifiers(
            crate::key::KeyboardModifiers::LEFT_ALT.union(&crate::key::KeyboardModifiers::new()),
        ),
        crate::key::keyboard::Key::new(43),
        crate::key::keyboard::Key::new(41),
        crate::key::keyboard::Key::new(44),
        crate::key::keyboard::Key::new(42),
        crate::key::keyboard::Key::new(40),
        crate::key::keyboard::Key::new(76),
        crate::key::keyboard::Key::from_modifiers(
            crate::key::KeyboardModifiers::RIGHT_ALT.union(&crate::key::KeyboardModifiers::new()),
        ),
        crate::key::keyboard::Key::from_modifiers(
            crate::key::KeyboardModifiers::RIGHT_GUI.union(&crate::key::KeyboardModifiers::new()),
        ),
        crate::key::keyboard::Key::from_modifiers(
            crate::key::KeyboardModifiers::RIGHT_CTRL.union(&crate::key::KeyboardModifiers::new()),
        ),
    ));
}
