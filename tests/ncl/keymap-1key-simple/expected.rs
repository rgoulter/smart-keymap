crate::tuples::define_keys!(1);

type KeyDefinitionsType = Keys1<crate::key::simple::Key>;

const KEY_DEFINITIONS: KeyDefinitionsType = Keys1::new((crate::key::simple::Key(4),));
