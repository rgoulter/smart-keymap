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

      # Authoring form: named-layer strings in layer_mod (resolved at km→json).
      check_named_layer_mod_strings = {
        actual =
          K.layer_mod.set_active_layers_amongst {
            set_active_layers_to = ["nav"],
            affected_layers = ["fn", "nav", "sym"],
          },
        expected = {
          layer_modifier = {
            set_active_layers_to = ["nav"],
            affected_layers = ["fn", "nav", "sym"],
          },
        },
      },

      check_named_layer_mod_strings_is_key =
        keymap_ncl.layer_modifier.is_key (
          K.layer_mod.set_active_layers_amongst {
            set_active_layers_to = ["nav"],
            affected_layers = ["fn", "nav"],
          }
        ),

      # set_semantic_variant_to: sugar over set_active_layers_amongst for exclusive groups.
      check_set_semantic_variant_to = {
        actual = K.layer_mod.set_semantic_variant_to ["fn", "nav", "sym"] "nav",
        expected = {
          layer_modifier = {
            set_active_layers_to = ["nav"],
            affected_layers = ["fn", "nav", "sym"],
          },
        },
      },

      check_set_semantic_variant_to_is_key =
        keymap_ncl.layer_modifier.is_key (
          K.layer_mod.set_semantic_variant_to ["fn", "nav", "sym"] "nav"
        ),
    },

  # Layer index: numeric (1-based) or named-layer string (resolved before to_json_value).
  layer_index =
    validators.any_of [validators.is_number, validators.is_string],

  unique_layer_index_array = fun arr =>
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
      (validators.array.validator layer_index),
      unique_layer_index_array,
    ],

  # Back-compat alias used by some checks.
  unique_number_array = unique_layer_index_array,

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
          { layer_modifier = { default_ } } => layer_index default_,
          { layer_modifier = { hold }, modifiers } =>
            validators.all_ok [
              layer_index hold,
              keyboard_modifiers.validator modifiers,
            ],
          { layer_modifier = { hold } } => layer_index hold,
          { layer_modifier = { sticky } } => layer_index sticky,
          { layer_modifier = { toggle } } =>
            # Named layers: string ok; numeric toggle still requires > 0.
            if std.is_string toggle then
              'Ok
            else
              validators.all_of [validators.is_number, validators.is_greater_than 0] toggle,
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

      # JSON projection expects numeric layer indices. Named-layer strings must be
      # resolved (see keymap-ncl-to-json resolve_named_layer_refs) before to_json_value.
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

      check_named_layers_is_key =
        let key = K.A & { named_layers = { nav = K.B, fn = K.C } } in
        keymap_ncl.layered.is_key key,

      check_named_layers_validator_ok =
        keymap_ncl.layered.key_validator (K.A & { named_layers = { windows = K.B } }) == 'Ok,

      check_layered_and_named_layers_is_key =
        let key = K.A & { layered = [ null, K.B], named_layers = { fn = K.C } } in
        keymap_ncl.layered.is_key key,

      # K.semantic is thin sugar over named_layers (unit {} base).
      check_semantic_is_named_layered =
        let key = K.semantic { fn = K.A, nav = K.B, sym = K.C } in
        keymap_ncl.layered.is_key key
        && std.record.has_field "named_layers" key,

      check_semantic_authoring_form = {
        actual = K.semantic { fn = K.A, nav = K.B, sym = K.C },
        expected = {
          named_layers = {
            fn = { key_code = 4 },
            nav = { key_code = 5 },
            sym = { key_code = 6 },
          },
        },
      },

      # Pure layered form (unit {} base) serializes with key_code 0 base.
      check_pure_layered_unit_base = {
        actual = { layered = [ null, null, K.A, K.B, K.C] } |> keymap_ncl.key.to_json_value,
        expected = {
          base = { key_code = 0 },
          layered = [
            null,
            null,
            { key_code = 4 },
            { key_code = 5 },
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

      # Element of a layered / named_layers slot: Key or transparent null.
      layer_element_validator =
        validators.any_of [validators.is_null, keymap_ncl.key.key_validator],

      layered_array_validator =
        validators.array.validator layer_element_validator,

      # named_layers: Record String -> (Key|null)
      named_layers_validator =
        validators.all_of [
          validators.is_record,
          fun r => r |> std.record.values |> layered_array_validator,
        ],

      # A layered key has at least one of:
      # - `layered` = Array (Key|null): numbered layers (layer index 1 + i)
      # - `named_layers` = Record { layer_name = Key|null, ... }: named layers
      #   (indices after all numbered layers; lowered before JSON)
      # Both may be present on the same key (numbered + named compose via `&`).
      key_validator = fun k =>
        k
        |> match {
          { layered, named_layers, ..base_key } =>
            validators.all_ok [
              layered_array_validator layered,
              named_layers_validator named_layers,
              keymap_ncl.key.key_validator base_key,
            ],
          { layered, ..base_key } =>
            validators.all_ok [
              layered_array_validator layered,
              keymap_ncl.key.key_validator base_key,
            ],
          { named_layers, ..base_key } =>
            validators.all_ok [
              named_layers_validator named_layers,
              keymap_ncl.key.key_validator base_key,
            ],
          _ =>
            'Error {
              message = "expected { layered = Array Key and/or named_layers = Record String -> Key, ..base_key }",
            },
        },

      is_key = fun k => 'Ok == key_validator k,

      # JSON projection expects dense array form only (`layered` array, no `named_layers`).
      # Named layers must be lowered first (see keymap-ncl-to-json named layer transform).
      to_json_value = fun k =>
        k
        |> match {
          { named_layers = _, .. } =>
            std.fail_with "named_layers must be lowered to array form before to_json_value",
          { layered = layered_keys, ..base_key } =>
            {
              base = keymap_ncl.key.to_json_value base_key,
              layered =
                std.array.map
                  (fun k => if k != null then keymap_ncl.key.to_json_value k else null)
                  layered_keys,
            },
        },

      # Visit children under `layered` (array) and/or `named_layers` (record), then base.
      # Assumes a valid layered key shape (is_key); no defensive has_field checks.
      map_layered_array = fun f acc layered =>
        layered
        |> std.array.fold_left
          (fun { acc, layered } child =>
            let { acc, k } = f acc child in
            # let-bind before record: record fields are recursive by default.
            let layered = layered @ [k] in
            { include acc, include layered }
          )
          { include acc, layered = [] },

      map_named_layers = fun f acc named_layers =>
        std.record.fields named_layers
        |> std.array.fold_left
          (fun { acc, named_layers = parts } layer_name =>
            let child = std.record.get layer_name named_layers in
            let { acc, k = new_child } = f acc child in
            let named_layers = parts & { "%{layer_name}" = new_child } in
            { include acc, include named_layers }
          )
          { include acc, named_layers = {} },

      map_accum = fun f acc k =>
        k
        |> match {
          { layered = numbered, named_layers = named, ..base_key } =>
            let { acc, layered } = map_layered_array f acc numbered in
            let { acc, named_layers } = map_named_layers f acc named in
            let { acc, k = base_key } = f acc base_key in
            # n.b.: `.. base_key` results in format error.
            {
              include acc,
              k = base_key & { include layered, include named_layers },
            },
          { layered = numbered, ..base_key } =>
            let { acc, layered } = map_layered_array f acc numbered in
            let { acc, k = base_key } = f acc base_key in
            { include acc, k = base_key & { include layered } },
          { named_layers = named, ..base_key } =>
            let { acc, named_layers } = map_named_layers f acc named in
            let { acc, k = base_key } = f acc base_key in
            { include acc, k = base_key & { include named_layers } },
        },
    },
}
