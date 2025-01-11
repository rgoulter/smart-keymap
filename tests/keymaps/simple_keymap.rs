/// Types and initial data used for constructing [KEYMAP].
pub mod init {
    /// Alias for layers impl.
    pub type LayersImpl = crate::key::layered::ArrayImpl<0>;

    /// Alias for the NestedKey used for the [Context].
    pub type NestedKey = crate::key::composite::DefaultNestableKey;

    /// Alias for keys.
    pub type Key = crate::key::composite::Key<NestedKey, LayersImpl>;

    /// Alias for Context type; i.e. [crate::key::context::Context] with generics.
    pub type Context = crate::key::composite::Context<NestedKey, LayersImpl>;

    /// Initial [Context] value.
    pub const CONTEXT: Context = crate::key::composite::Context::new();

    /// Alias for a [tuples] KeysN type.
    pub type KeyDefinitionsType = crate::tuples::Keys4<
        Key,
        Key,
        Key,
        Key,
    >;

    /// Alias for a [tuples] KeysN value.
    pub const KEY_DEFINITIONS: KeyDefinitionsType = crate::tuples::Keys4::new((
        Key::tap_hold(crate::key::tap_hold::Key {
            tap: 0x06,
            hold: 0xE0,
        }), // Tap C, Hold LCtrl
        Key::tap_hold(crate::key::tap_hold::Key {
            tap: 0x07,
            hold: 0xE1,
        }), // Tap D, Hold LShift
        Key::simple(crate::key::simple::Key(0x04)), // A
        Key::simple(crate::key::simple::Key(0x05)), // B
    ));
}
