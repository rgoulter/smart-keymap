# keymap.ncl -> keymap.json
#
# This module is a multi-pass transform:
#  author-facing fields are gradually rewritten
#  into the dense JSON shape consumed by codegen / firmware.
#
# Pipeline (authored -> intermediate -> output)
# -------------------------------------------
#
# - Authored inputs (keymap.ncl surface)
#   - `keys` / `layers`   KeymapLayer: string tokens or Array Key
#   - `named_layers`      Record name → KeymapLayer (full rows by name; optional)
#   - `named_layer_order` Array String — optional index order for named layers
#                         (listed used names first; rest alphabetical; default alpha)
#   - `chords`            [{ indices, key }, …]
#   - `config`            optional per-feature config
#   - `custom_keys`       extends token vocabulary for string layers
#   Per-key sugar still present here:
#     { layered = [Key|null, ...] }            # numbered layers (index i -> layer i+1)
#     { named_layers = { name = Key|null } }   # named layers (lowered later)
#     automation keys with instruction lists (e.g. string macros)
#     ordinary composite keys (tap_hold, ...)
#
# - Intermediate key arrays (same length; each pass rewrites slots)
#   - `layers_of_keys`     layers normalized to Array Key     (passes/fold-layers.ncl)
#   - `layered_keys`       multi-layer authoring folded       (passes/fold-layers.ncl)
#                           KM `named_layers` rows zipped into per-key `named_layers`
#                           (per-key `named_layers` still allowed; per-key wins on conflict)
#   - `chorded_keys`       global `chords` attached           (passes/attach-chords.ncl)
#   - `automation_transform`                                  (passes/automation.ncl)
#       { automation_instructions, keys }
#       macros flattened: keys hold { start, length } refs into the instruction list
#
# - Late key rewrites (inside `json_keymap`, after automation)
#   - prepare_keys_named_layers                               (passes/named-layers.ncl)
#       named_layers -> dense `layered` array layout: [numbered slots…] ++ [named slots…]
#       named slots ordered by `named_layer_order` (else alphabetical)
#       resolve layer_mod name strings -> 1-based indices
#   - pad layered arrays   every layered key shares the same array length
#   - to_json_value        per-key module projection (array-form layered only)
#
# - Output (`json_keymap`)
#   {
#     config = author config
#              & { automation.instructions? }
#              & { chorded.chords = indices },
#     keys   = [ json key, … ],
#   }
#
# Exported layer-index helpers (for layer_mod / docs; 1-based like layered[i]):
#    numbered_layer_count, named_layer_names, named_layer_indices, max_layered_length
#
# Nesting: several passes walk composite keys via keymap_ncl.*.map_accum so
# layered / named_layers / automation under tap_hold etc. are rewritten too.
#
# Pass implementations live under ncl/passes/; this file is the driver:
# contracts, key dispatch, authored fields, and json_keymap.
(import "smart_keys/automation/keymap-ncl-to-json.ncl")
& (import "smart_keys/callback/keymap-ncl-to-json.ncl")
& (import "smart_keys/caps_word/keymap-ncl-to-json.ncl")
& (import "smart_keys/consumer/keymap-ncl-to-json.ncl")
& (import "smart_keys/chorded/keymap-ncl-to-json.ncl")
& (import "smart_keys/custom/keymap-ncl-to-json.ncl")
& (import "smart_keys/keyboard/keymap-ncl-to-json.ncl")
& (import "smart_keys/layered/keymap-ncl-to-json.ncl")
& (import "smart_keys/mouse/keymap-ncl-to-json.ncl")
& (import "smart_keys/sticky/keymap-ncl-to-json.ncl")
& (import "smart_keys/tap_dance/keymap-ncl-to-json.ncl")
& (import "smart_keys/tap_hold/keymap-ncl-to-json.ncl")
& (import "passes/fold-layers.ncl")
& (import "passes/attach-chords.ncl")
& (import "passes/automation.ncl")
& (import "passes/named-layers.ncl")
& {
  validators | default = import "validators.ncl",

  # Free fields filled by pass modules (ncl/passes/*); needed so the
  # driver record can refer to them (Nickel free ids don't cross `&`).
  automation_transform,
  prepare_keys_named_layers,
  max_layered_length_accum,
  pad_key_layered_arrays,

  Chord = { indices | Array Number, key | KeymapKey },

  Config = {
    automation | optional | keymap_ncl.automation.Config,
    chorded | optional | keymap_ncl.chorded.Config,
    layered | optional | keymap_ncl.layered.Config,
    sticky | optional | keymap_ncl.sticky.Config,
    tap_dance | optional | keymap_ncl.tap_dance.Config,
    tap_hold | optional | keymap_ncl.tap_hold.Config,
  },

  KeymapKey = keymap_ncl.nullable_key.Key,

  keymap_layer_string_tokens = fun extended_keys v =>
    v
    |> validators.is_string
    |> match {
      'Ok =>
        let words = words_from_whitespace_delimited_string v in
        words
        |> std.array.filter (fun w => !(std.record.has_field w extended_keys))
        |> match {
          [] =>
            let rec validate_tokens = fun tokens =>
              tokens
              |> match {
                [] => 'Ok,
                [word, ..rest] =>
                  std.record.get word extended_keys
                  |> keymap_ncl.nullable_key.key_validator
                  |> match {
                    'Ok => validate_tokens rest,
                    'Error { message = err_msg, .. } =>
                      'Error {
                        message = "Invalid keymap layer string: key %{word}: %{err_msg}",
                      },
                  },
              }
            in
            validate_tokens words,
          words =>
            'Error {
              message = "Invalid keymap layer string: keys not defined: %{words |> std.string.join ", "}",
            },
        },
      e => e,
    },

  keymap_layer_string = fun v => keymap_layer_string_tokens extended_keys v,

  KeymapLayerString = std.contract.from_validator keymap_layer_string,

  KeymapLayer =
    std.contract.custom (fun label value =>
      if std.is_string value then
        std.contract.check KeymapLayerString label value
      else if std.is_array value then
        std.contract.check (Array KeymapKey) label value
      else
        'Error {
          message = "Expected keymap layer to be a whitespace-delimited string or an array of keys",
        }
    ),

  config | Config = {},

  keys
    | optional,

  layers
    | default
    | Array KeymapLayer
    | doc "The layers of the keymap that gets rendered to JSON"
    = [keys],

  # Full named layer rows at keymap scope (name → layer of same width as base).
  # Zipped into per-key `named_layers` in fold-layers, then lowered with the
  # usual named-layer pass. Coexists with numbered `layers` (named after numbered).
  named_layers
    | default
    | { _ | KeymapLayer }
    | doc "Named layers: whole rows keyed by name (same width as the base layer)."
    = {},

  # Optional override for dense named-layer slot / layer_mod index order.
  # Without this field (or with []), used names are ordered alphabetically.
  # Listed names that appear in the keymap come first; remaining used names
  # append alphabetically. Names listed but never used are ignored.
  # Changing order renumbers all named layers (breaking for fixed numeric indices).
  named_layer_order
    | default
    | Array String
    | doc "Override named-layer index order (used names listed first; rest alphabetical)."
    = [],

  chords
    | Array Chord
    | default
    = [],

  custom_keys
    | default
    | doc "Key extension which gets applied when using whitespace-delimited string keymap layers"
    = fun K => {},

  keyboard_modifiers = {
    validator =
      validators.record.validator {
        fields_validator =
          validators.record.has_only_fields [
            "left_ctrl",
            "left_shift",
            "left_alt",
            "left_gui",
            "right_ctrl",
            "right_shift",
            "right_alt",
            "right_gui",
          ],
        field_validators = {
          left_ctrl = validators.is_bool,
          left_shift = validators.is_bool,
          left_alt = validators.is_bool,
          left_gui = validators.is_bool,
          right_ctrl = validators.is_bool,
          right_shift = validators.is_bool,
          right_alt = validators.is_bool,
          right_gui = validators.is_bool,
        },
      },

    # Must agree with consts used in key::KeyboardModifiers
    to_json_value = fun km =>
      let km =
        km
        & {
          left_ctrl | default = false,
          left_shift | default = false,
          left_alt | default = false,
          left_gui | default = false,
          right_ctrl | default = false,
          right_shift | default = false,
          right_alt | default = false,
          right_gui | default = false,
        }
      in
      0
      + (if km.left_ctrl then 1 else 0)
      + (if km.left_shift then 2 else 0)
      + (if km.left_alt then 4 else 0)
      + (if km.left_gui then 8 else 0)
      + (if km.right_ctrl then 16 else 0)
      + (if km.right_shift then 32 else 0)
      + (if km.right_alt then 64 else 0)
      + (if km.right_gui then 128 else 0),
  },

  keymap_ncl.null_ = {
    Key = std.contract.from_validator key_validator,

    key_validator = fun k =>
      k
      |> match {
        null => 'Ok,
        _ => 'Err "not null",
      },

    is_key = fun k => 'Ok == key_validator k,

    to_json_value = fun k => null,

    map_accum = fun f acc k => f acc k,
  },

  keymap_ncl.key = {
    # Dispatch order = outermost wrapper wins when several modules' is_key match
    # the same record (Nickel record "abuse"). Used by key_validator and
    # key_module_for_value. Chorded → layered → tap_hold → leaf keys.
    key_modules = [
      keymap_ncl.chorded,
      keymap_ncl.chorded_aux,
      keymap_ncl.automation,
      keymap_ncl.callback,
      keymap_ncl.caps_word,
      keymap_ncl.consumer,
      keymap_ncl.custom,
      keymap_ncl.mouse,
      keymap_ncl.sticky,
      keymap_ncl.tap_dance,
      keymap_ncl.layered,
      keymap_ncl.tap_hold,
      keymap_ncl.layer_modifier,
      keymap_ncl.keyboard,
    ],

    Key = std.contract.from_validator key_validator,

    key_validator = fun k =>
      key_modules
      |> std.array.filter (fun km => km.is_key k)
      |> match {
        [km, ..] => km.key_validator k,
        [] =>
          'Error {
            message = "Value is not a valid keymap key",
          },
      },

    is_key = fun k => 'Ok == key_validator k,

    key_module_for_value = fun k =>
      key_modules
      |> std.array.filter (fun km => km.is_key k)
      |> match {
        [km, ..] => km,
        [] => std.fail_with "unknown key type",
      },

    to_json_value = fun k =>
      let km = key_module_for_value k in
      km.to_json_value k,

    map_accum = fun f acc k =>
      let km = key_module_for_value k in
      km.map_accum f acc k,
  },

  keymap_ncl.nullable_key = {
    Key = std.contract.from_validator key_validator,

    key_validator = fun k =>
      k
      |> match {
        null => keymap_ncl.null_.key_validator k,
        _ => keymap_ncl.key.key_validator k,
      },

    is_key = fun k => 'Ok == key_validator k,

    to_json_value = match {
      k if keymap_ncl.key.is_key k => keymap_ncl.key.to_json_value k,
      k if keymap_ncl.null_.is_key k => keymap_ncl.null_.to_json_value k,
      _ => std.fail_with "unsupported nullable_key item in keymap.ncl",
    },

    map_accum = fun f acc k =>
      k
      |> match {
        k if keymap_ncl.key.is_key k => keymap_ncl.key.map_accum f acc k,
        k if keymap_ncl.null_.is_key k => keymap_ncl.null_.map_accum f acc k,
        _ => std.fail_with "unsupported nullable_key item in keymap.ncl",
      },
  },

  checks.check_words_from_whitespace_delimited_string = {
    check_words_from_whitespace_delimited_string = {
      actual = words_from_whitespace_delimited_string "   a b  c ",
      expected = ["a", "b", "c"],
    },
  },

  words_from_whitespace_delimited_string
    | doc "Return array of fields from whitespace-delimited string."
    = fun s =>
      s
      |> std.string.replace_regex "\\s+" " "
      |> std.string.split " "
      |> std.array.filter ((!=) ""),

  checks.check_map_fields_with_record = {
    check_map_fields_with_record = {
      actual = map_fields_with_record { a = 1, b = 2, c = 3 } ["a", "c"],
      expected = [1, 3],
    },
  },

  map_fields_with_record
    | doc "Maps each field in an array to the value of the given record."
    = fun record fields =>
      fields |> std.array.map (std.function.flip std.record.get record),

  checks.check_key_validator = {
    check_unknown_record_is_error =
      keymap_ncl.key.key_validator { not_a_key = true } |> validators.check_is_error,

    check_keyboard_is_ok =
      keymap_ncl.key.key_validator { key_code = 4 } == 'Ok,

    check_nullable_null_is_ok =
      keymap_ncl.nullable_key.key_validator null == 'Ok,
  },

  checks.check_key_modules_dispatch =
    let K = import "keys.ncl" in
    {
      check_layered_composite_uses_layered_module =
        let k = K.Tab & K.hold (K.layer_mod.hold 1) & { layered = [ null, K.A] } in
        let json = keymap_ncl.key.to_json_value k in
        keymap_ncl.layered.is_key k
        && keymap_ncl.tap_hold.is_key k
        && std.record.has_field "base" json
        && std.record.has_field "layered" json,

      check_chorded_aux_passthrough_uses_chorded_aux_module =
        let k = { passthrough = K.A } in
        let json = keymap_ncl.key.to_json_value k in
        keymap_ncl.chorded_aux.is_key k
        && std.record.has_field "passthrough" json
        && !(std.record.has_field "chords" json),
    },

  checks.check_keymap_layer_string = {
    check_unknown_key_is_error =
      keymap_layer_string " A BadKeyName " |> validators.check_is_error,

    check_unknown_key_message =
      keymap_layer_string " A BadKeyName "
      |> match {
        'Error { message } =>
          std.string.contains "keys not defined: BadKeyName" message,
        _ => false,
      },

    check_valid_string_is_ok =
      keymap_layer_string " A " == 'Ok,
  },

  checks.check_extended_keys_layer_string =
    let K = import "keys.ncl" in
    let { extend_keys, .. } = import "key-extensions.ncl" in
    let extended_with_bad_custom =
      extend_keys K (fun K => { BadCustom = { not_a_key = true } })
    in
    let extended_with_hold_tab =
      extend_keys K (fun K => { HoldTab = K.Tab & K.hold K.LeftCtrl })
    in
    {
      check_invalid_custom_key_value_is_error =
        keymap_layer_string_tokens extended_with_bad_custom " BadCustom "
        |> validators.check_is_error,

      check_invalid_custom_key_value_message =
        keymap_layer_string_tokens extended_with_bad_custom " BadCustom "
        |> match {
          'Error { message } =>
            std.string.contains "key BadCustom" message,
          _ => false,
        },

      check_valid_custom_key_value_is_ok =
        keymap_layer_string_tokens extended_with_hold_tab " HoldTab " == 'Ok,
    },

  extended_keys =
    let { extend_keys, .. } = import "key-extensions.ncl" in
    extend_keys (import "keys.ncl") custom_keys,

  # Return array-of-keys representation of layer.
  #
  # A KeymapLayer may be either:
  #   - Array Key
  #   - String
  layer_as_array_of_keys = fun l =>
    if std.is_string l then
      l
      |> words_from_whitespace_delimited_string
      |> map_fields_with_record extended_keys
    else
      l,

  json_keymap
    | default
    | doc "The keymap.json output value."
    =
      let { automation_instructions, keys } = automation_transform in
      let automation_config =
        if std.array.length automation_instructions > 0 then
          { automation.instructions = automation_instructions }
        else
          {}
      in
      # named_layers -> dense array after numbered `layered` slots:
      #   layered = [ numbered slots… ] ++ [ named slots… ]
      # Named order: `named_layer_order` if set, else alphabetical. Skipped if unused.
      # Layer_mod fields may use those names as strings; resolve to 1-based indices.
      let keys = prepare_keys_named_layers keys in
      # Equalise layered array lengths (pad shorter with null / transparency).
      let layer_count =
        keys |> std.array.fold_left max_layered_length_accum 0
      in
      let keys_json =
        keys
        |> std.array.map (pad_key_layered_arrays layer_count)
        |> std.array.map keymap_ncl.key.to_json_value
      in
      let { chorded = km_config_chorded, ..km_config } = config & { chorded = {} } in
      let chord_indices = chords |> std.array.map (fun { key, indices, .. } => indices) in
      let config_json_value =
        km_config
        & automation_config
        & {
          chorded =
            km_config_chorded
            & {
              chords = chord_indices,
            },
        }
      in
      {
        config = config_json_value,
        keys = keys_json,
      },
}
