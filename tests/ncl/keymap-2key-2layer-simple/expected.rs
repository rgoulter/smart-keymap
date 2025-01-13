/// Types and initial data used for constructing [KEYMAP].
pub mod init {
    /// Alias for layers impl.
    pub type LayersImpl = crate::key::layered::ArrayImpl<1>;

    /// Alias for the NestedKey used for the [Context].
    pub type NestedKey = crate::key::composite::DefaultNestableKey;

    /// Alias for Context type; i.e. [crate::key::context::Context] with generics.
    pub type Context = crate::key::composite::Context<LayersImpl>;

    /// Initial [Context] value.
    pub const CONTEXT: Context = crate::key::composite::Context::new();

    crate::tuples::define_keys!(2);

    /// Alias for a [tuples] KeysN type, as generated by keymap.ncl.
    pub type KeyDefinitionsType = Keys2<
        crate::key::layered::ModifierKey,
        crate::key::layered::LayeredKey<crate::key::simple::Key, crate::key::layered::ArrayImpl<1>>,
        Context,
    >;

    /// A [tuples] KeysN value with keys, as generated by keymap.ncl.
    pub const KEY_DEFINITIONS: KeyDefinitionsType = Keys2::new((
        crate::key::layered::ModifierKey::Hold(0),
        crate::key::layered::LayeredKey {
            base: crate::key::simple::Key(4),
            layered: [Some(crate::key::simple::Key(5))],
        },
    ));
}
