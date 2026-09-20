{
  validators,

  keymap_ncl.key_lock
    | doc "for key::key_lock::Key."
    = {
      Key = std.contract.from_validator key_validator,

      key_validator = fun k =>
        k
        |> match {
          { key_lock = "KeyLock" } => 'Ok,
          _ => 'Error { message = "Expected { key_lock = \"KeyLock\" }" },
        },

      is_key = fun k => 'Ok == key_validator k,

      to_json_value = fun key =>
        key
        |> match {
          { key_lock = json } => json,
          _ => std.fail_with "Expected { key_lock = \"KeyLock\" }",
        },

      # Leaves have no nested keys; map_accum maps children only.
      map_accum = fun f acc k => { include acc, include k },
    },

  checks.check_key_lock =
    let K = import "keys.ncl" in
    {
      check_key_lock_ok =
        keymap_ncl.key_lock.key_validator K.key_lock == 'Ok,

      check_key_lock_json = {
        actual = keymap_ncl.key_lock.to_json_value K.key_lock,
        expected = "KeyLock",
      },

      # Record form merges with layered authoring as a layered base.
      check_key_lock_layered_base_ok =
        keymap_ncl.key.key_validator (K.key_lock & { layered = [K.A] }) == 'Ok,
    },
}
