/// Types and initial data used for constructing [KEYMAP].
pub mod init {
    const NUM_LAYERS: usize = 1;

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

    crate::tuples::define_keys!(2);

    /// Alias for a [tuples] KeysN type, as generated by keymap.ncl.
    pub type KeyDefinitionsType = Keys2<
        crate::key::layered::ModifierKey,
        crate::key::layered::LayeredKey<
            crate::key::keyboard::Key,
            crate::key::layered::ArrayImpl<1>,
        >,
        Context,
        Event,
    >;

    /// A [tuples] KeysN value with keys, as generated by keymap.ncl.
    pub const KEY_DEFINITIONS: KeyDefinitionsType = Keys2::new((
        crate::key::layered::ModifierKey::Hold(0),
        crate::key::layered::LayeredKey {
            base: crate::key::keyboard::Key::new(4),
            layered: [Some(crate::key::keyboard::Key::new(5))],
        },
    ));
}
