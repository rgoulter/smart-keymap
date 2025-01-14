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
    pub type Context = crate::key::composite::Context<CompositeImpl>;

    /// Alias for Event type; i.e. [crate::key::context::Event].
    pub type Event = crate::key::composite::Event;

    /// Initial [Context] value.
    pub const CONTEXT: Context = crate::key::composite::Context {
        layer_context: crate::key::layered::Context {
            active_layers: [false; NUM_LAYERS],
        },
    };

    crate::tuples::define_keys!(1);

    /// Alias for a [tuples] KeysN type, as generated by keymap.ncl.
    pub type KeyDefinitionsType = Keys1<crate::key::tap_hold::Key, Context, Event>;

    /// A [tuples] KeysN value with keys, as generated by keymap.ncl.
    pub const KEY_DEFINITIONS: KeyDefinitionsType =
        Keys1::new((crate::key::tap_hold::Key { hold: 224, tap: 4 },));
}
