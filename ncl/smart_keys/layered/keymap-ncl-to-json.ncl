{
  validators,

  keyboard_modifiers,

  checks.layer_modifier =
    let K = import "keys.ncl" in
    {
      keymap_example_hold = {
        actual = K.layer_mod.hold 0,
        expected = { layer_modifier.hold = 0 },
      },

      check_json_value_hold = {
        actual = K.layer_mod.hold 3 |> keymap_ncl.key.to_json_value,
        expected = { Hold = [3, 0] },
      },

      keymap_example_toggle = {
        actual = K.layer_mod.toggle 1,
        expected = { layer_modifier.toggle = 1 },
      },

      check_json_value_toggle = {
        actual = K.layer_mod.toggle 1 |> keymap_ncl.key.to_json_value,
        expected = { Toggle = 1 },
      },
      check_json_value_set_active_layers_amongst = {
        actual =
          K.layer_mod.set_active_layers_amongst {
            set_active_layers_to = [3],
            affected_layers = [3, 4],
          }
          |> keymap_ncl.key.to_json_value,
        expected = {
          SetActiveLayers = {
            layers = std.number.pow 2 3,
            mask = (std.number.pow 2 3) + (std.number.pow 2 4),
          },
        },
      },

      check_layer_indices_to_bitset_empty = {
        actual = layer_indices_to_bitset [],
        expected = 0,
      },

      check_layer_indices_to_bitset_rejects_duplicates = {
        actual = layer_indices_array [3, 3],
        expected = 'Error { message = "layer indices must be unique" },
      },
    },

  unique_number_array = fun arr =>
    if !std.is_array arr then
      'Error { message = "Expected array" }
    else
      let unique =
        arr
        |> std.array.fold_left
          (fun acc x => if std.array.elem x acc then acc else acc @ [x])
          []
      in
      if (std.array.length unique) == (std.array.length arr) then
        'Ok
      else
        'Error { message = "layer indices must be unique" },

  layer_indices_array =
    validators.all_of [
      (validators.array.validator validators.is_number),
      unique_number_array,
    ],

  layer_indices_to_bitset = fun layer_indices =>
    layer_indices
    |> std.array.map (std.number.pow 2)
    |> std.array.fold_left (+) 0,

  keymap_ncl.layer_modifier
    | doc "for key::layered::ModifierKey."
    = {
      Key = std.contract.from_validator key_validator,

      key_validator = fun k =>
        k
        |> match {
          { layer_modifier = { default_ } } => validators.is_number default_,
          { layer_modifier = { hold }, modifiers } =>
            validators.all_ok [
              validators.is_number hold,
              keyboard_modifiers.validator modifiers,
            ],
          { layer_modifier = { hold } } => validators.is_number hold,
          { layer_modifier = { sticky } } => validators.is_number sticky,
          { layer_modifier = { toggle } } => validators.all_of [validators.is_number, validators.is_greater_than 0] toggle,
          { layer_modifier = { set_active_layers_to } } => layer_indices_array set_active_layers_to,
          { layer_modifier = { set_active_layers_to, affected_layers } } =>
            validators.all_ok [
              layer_indices_array set_active_layers_to,
              layer_indices_array affected_layers,
            ],
          _ =>
            'Error {
              message = "expected { layer_modifier = { default_ } }, { hold }, { sticky }, { toggle }, { set_active_layers_to }, or { set_active_layers_to, affected_layers }",
            },
        },

      is_key = fun k => 'Ok == key_validator k,

      to_json_value = fun layer_modifier_key =>
        layer_modifier_key
        |> match {
          { layer_modifier = { default_ = default_layer } } =>
            { Default = default_layer },
          { layer_modifier = { hold = hold_layer }, modifiers = km } =>
            let km = keyboard_modifiers.to_json_value km in
            { Hold = [hold_layer, km] },
          { layer_modifier = { hold = hold_layer } } =>
            { Hold = [hold_layer, 0] },
          { layer_modifier = { toggle = toggle_layer } } =>
            { Toggle = toggle_layer },
          { layer_modifier = { sticky = sticky_layer } } =>
            { Sticky = sticky_layer },
          { layer_modifier = { set_active_layers_to = active_layers } } =>
            let modifier_bitset = { layers = layer_indices_to_bitset active_layers } in
            { SetActiveLayers = modifier_bitset },
          { layer_modifier = { set_active_layers_to = active_layers, affected_layers = mask_layers } } =>
            let modifier_bitset = {
              layers = layer_indices_to_bitset active_layers,
              mask = layer_indices_to_bitset mask_layers,
            }
            in
            { SetActiveLayers = modifier_bitset },
          _ =>
            'Error {
              message = "expected { layer_modifier = { default_ } }, { hold }, { sticky }, { toggle }, { set_active_layers_to }, or { set_active_layers_to, affected_layers }",
            },
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
      Config = {
        sticky_timeout | optional | Number,
      },

      Key = std.contract.from_validator key_validator,

      key_validator = fun k =>
        k
        |> match {
          { layered, ..base_key } =>
            validators.all_ok [
              layered
              |> validators.array.validator (
                validators.any_of [validators.is_null, keymap_ncl.key.key_validator]
              ),
              keymap_ncl.key.key_validator base_key,
            ],
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
