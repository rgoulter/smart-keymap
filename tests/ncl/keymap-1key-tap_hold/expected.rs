crate::tuples::define_keys!(1);

type KeyDefinitionsType = Keys1<crate::key::tap_hold::Key>;

const KEY_DEFINITIONS: KeyDefinitionsType =
    Keys1::new((crate::key::tap_hold::Key { hold: 224, tap: 4 },));
