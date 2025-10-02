{
  validators,

  checks.layer_modifier =
    let K = import "keys.ncl" in
    {
      keymap_example_hold = {
        actual = K.layer_mod.hold 0,
        expected = { layer_modifier.hold = 0 },
      },

      check_json_value_hold = {
        actual = K.layer_mod.hold 0 |> keymap_ncl.key.to_json_value,
        expected = { Hold = 0 },
      },

      keymap_example_toggle = {
        actual = K.layer_mod.toggle 1,
        expected = { layer_modifier.toggle = 1 },
      },

      check_json_value_toggle = {
        actual = K.layer_mod.toggle 1 |> keymap_ncl.key.to_json_value,
        expected = { Toggle = 1 },
      },
    },

  keymap_ncl.layer_modifier
    | doc "for key::layered::ModifierKey."
    = {
      Key = std.contract.from_validator key_validator,

      key_validator = fun k =>
        k
        |> match {
          { layer_modifier = { default_ } } => validators.is_number default_,
          { layer_modifier = { hold } } => validators.is_number hold,
          { layer_modifier = { sticky } } => validators.is_number sticky,
          { layer_modifier = { toggle } } => validators.all_of [validators.is_number, validators.is_greater_than 0] toggle,
          { layer_modifier = { set_active_layers_to } } => (validators.array.validator validators.is_number) set_active_layers_to,
          _ => 'Error { message = "expected { layer_modifier = { default_ } } or { layer_modifier = { hold } } or { layer_modifier = { toggle } }" },
        },

      is_key = fun k => 'Ok == key_validator k,

      to_json_value = fun layer_modifier_key =>
        layer_modifier_key
        |> match {
          { layer_modifier = { default_ = default_layer } } =>
            { Default = default_layer },
          { layer_modifier = { hold = hold_layer } } =>
            { Hold = hold_layer },
          { layer_modifier = { toggle = toggle_layer } } =>
            { Toggle = toggle_layer },
          { layer_modifier = { sticky = sticky_layer } } =>
            { Sticky = sticky_layer },
          { layer_modifier = { set_active_layers_to = active_layers } } =>
            let active_layers_bitset =
              active_layers
              |> std.array.map (std.number.pow 2)
              |> std.array.reduce_left (+)
            in
            { SetActiveLayers = active_layers_bitset },
          _ => 'Error { message = "expected { layer_modifier = { default_ } } or { layer_modifier = { hold } } or { layer_modifier = { toggle } }" },
        },

      map_accum = fun f acc k => f acc k,
    },

  checks.layered =
    let K = import "keys.ncl" in
    {
      keymap_example_layered_keyboard = {
        actual = K.A & { layered = [ null, K.C] },
        expected = {
          key_code = 4,
          layered = [
            null,
            { key_code = 6 },
          ],
        },
      },

      check_layered_base_keyboard_base_is_key =
        let k = K.A & { layered = [ null, K.C] } in
        let { layered, ..base_key } = k in
        keymap_ncl.key.is_key base_key,

      check_layered_base_keyboard_is_key =
        let key = K.A & { layered = [ null, K.C] } in
        keymap_ncl.layered.is_key key,

      check_json_value_layered_keyboard = {
        actual = K.A & { layered = [ null, K.C] } |> keymap_ncl.key.to_json_value,
        expected = {
          base = { key_code = 4 },
          layered = [
            null,
            { key_code = 6 },
          ],
        },
      },
    },

  keymap_ncl.layered
    | doc "for key::layered::LayeredKey."
    = {
      Key = std.contract.from_validator key_validator,

      key_validator = fun k =>
        k
        |> match {
          { layered, ..base_key } =>
            let valid_layered =
              layered
              |> validators.array.validator (
                validators.any_of [validators.is_null, keymap_ncl.key.key_validator]
              )
            in
            let valid_base_key = keymap_ncl.key.key_validator base_key in
            [valid_layered, valid_base_key]
            |> match {
              ['Ok, 'Ok] => 'Ok,
              [err, _] => err,
              [_, err] => err,
            },
          _ => 'Error { message = "expected { layered = Array Key, ..base_key }" },
        },

      is_key = fun k => 'Ok == key_validator k,

      to_json_value = fun { layered = layered_keys, ..base_key } =>
        {
          base = keymap_ncl.key.to_json_value base_key,
          layered =
            std.array.map
              (fun k => if k != null then keymap_ncl.key.to_json_value k else null)
              layered_keys,
        },

      map_accum = fun f acc k @ { layered, ..base_key } =>
        let { acc, layered } =
          layered
          |> std.array.fold_left
            (fun { acc, layered } k =>
              let { acc, k } = f acc k in
              let layered = layered @ [k] in
              { include acc, include layered }
            )
            { include acc, layered = [] }
        in
        let { acc, k = base_key } = f acc base_key in
        # n.b.: `.. base_key` results in format error.
        { include acc, k = { include layered, } & base_key, },
    },
}
