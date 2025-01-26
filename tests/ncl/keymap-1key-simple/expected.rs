/// Types and initial data used for constructing [KEYMAP].
pub mod init {
    /// Number of layers supported by the [crate::key::layered] implementation.
    pub const LAYER_COUNT: usize = 0;

    /// Alias for Context type; i.e. [crate::key::context::Context] with generics.
    pub type Context = crate::key::composite::Context;

    /// Alias for Event type; i.e. [crate::key::context::Event].
    pub type Event = crate::key::composite::Event;

    /// Initial [Context] value.
    pub const CONTEXT: Context = crate::key::composite::Context {
        layer_context: crate::key::layered::Context {
            active_layers: [false; crate::key::layered::LAYER_COUNT],
        },
    };

    crate::tuples::define_keys!(1);

    /// Alias for a [tuples] KeysN type, as generated by keymap.ncl.
    pub type KeyDefinitionsType = Keys1<crate::key::keyboard::Key, Context, Event>;

    /// A [tuples] KeysN value with keys, as generated by keymap.ncl.
    pub const KEY_DEFINITIONS: KeyDefinitionsType =
        Keys1::new((crate::key::keyboard::Key::new(4),));
}
