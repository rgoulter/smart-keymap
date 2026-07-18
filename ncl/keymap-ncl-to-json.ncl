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
#   - `layers_of_keys`     layers normalized to Array Key
#   - `layered_keys`       multi-layer authoring folded into base & { layered }
#                           (per-key `named_layers` still allowed)
#   - `chorded_keys`       global `chords` attached as chorded / passthrough
#   - `automation_transform`
#       { automation_instructions, keys }
#       macros flattened: keys hold { start, length } refs into the instruction list
#
# - Late key rewrites (inside `json_keymap`, after automation)
#   - lower named layers
#       named_layers -> dense `layered` array layout: [numbered slots…] ++ [named slots…]
#       global name  -> index is alphabetical (see named_layer_*)
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
& {
  validators | default = import "validators.ncl",

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

  checks.check_layered_key_element =
    let K = import "keys.ncl" in
    {
      check_plain_key_ok =
        layered_key_element_validator K.A == 'Ok,

      check_null_ok =
        layered_key_element_validator null == 'Ok,

      check_string_token_ok =
        layered_key_element_validator K.caps_word.toggle == 'Ok,

      check_layered_composite_ok =
        layered_key_element_validator (K.A & { layered = [ null, K.B] }) == 'Ok,

      check_invalid_is_error =
        layered_key_element_validator { not_a_key = true } |> validators.check_is_error,
    },

  checks.check_chorded_key_element =
    let K = import "keys.ncl" in
    {
      check_plain_key_ok =
        chorded_key_element_validator K.A == 'Ok,

      check_null_ok =
        chorded_key_element_validator null == 'Ok,

      check_string_token_ok =
        chorded_key_element_validator K.caps_word.toggle == 'Ok,

      check_passthrough_ok =
        chorded_key_element_validator { passthrough = K.A } == 'Ok,

      check_full_chord_ok =
        chorded_key_element_validator {
          chords = [[0, K.C]],
          passthrough = K.A,
        } == 'Ok,

      check_passthrough_invalid_inner_is_error =
        chorded_key_element_validator { passthrough = { not_a_key = true } }
        |> validators.check_is_error,
    },

  checks.check_automation_key_element =
    let K = import "keys.ncl" in
    {
      check_plain_keyboard_ok =
        automation_key_element_validator K.A == 'Ok,

      check_string_macro_pre_transform_is_keymap_key =
        keymap_ncl.key.key_validator (K.string_macro "a") == 'Ok,

      check_string_macro_post_transform_ok =
        let exec = { start = 0, length = 1 } in
        automation_key_element_validator {
          automation_instructions = {
            on_press = exec,
            while_pressed = exec,
            on_release = exec,
          },
        } == 'Ok,
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

  checks.check_layer_as_array_of_keys = {
    check_layer_as_array_of_keys = {
      actual = layer_as_array_of_keys " A ",
      expected = [{ key_code = 4 }],
    },

    check_layer_as_array_of_keys_tttt = {
      actual = layer_as_array_of_keys " TTTT ",
      expected = [ null ],
    },
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

  layers_of_keys = layers |> std.array.map layer_as_array_of_keys,

  layered_key_element_validator = fun k =>
    k
    |> match {
      { layered, .. } => keymap_ncl.layered.key_validator k,
      { named_layers, .. } => keymap_ncl.layered.key_validator k,
      _ => keymap_ncl.nullable_key.key_validator k,
    },

  LayeredKeyElement = std.contract.from_validator layered_key_element_validator,

  chorded_key_element_validator = fun k =>
    k
    |> match {
      { chords, .. } => keymap_ncl.chorded.key_validator k,
      { passthrough, .. } => keymap_ncl.chorded_aux.key_validator k,
      _ => keymap_ncl.nullable_key.key_validator k,
    },

  ChordedKeyElement = std.contract.from_validator chorded_key_element_validator,

  layered_keys
    | Array LayeredKeyElement
    | doc "Constructs array of key::layered::LayeredKey values from layers."
    =
      if std.array.length layers == 0 then
        std.fail_with "keys or layers must be provided"
      else if std.array.length layers <= 1 then
        std.array.at 0 layers |> layer_as_array_of_keys
      else
        let base_keys = std.array.first layers_of_keys in
        let layered_keys = std.array.drop_first layers_of_keys in
        std.array.generate
          (fun idx =>
            let base_key = std.array.at idx base_keys in
            let layered_ = std.array.map (std.array.at idx) layered_keys in
            let no_layered = std.array.all (fun x => x == null) layered_ in
            if no_layered then
              base_key
            else
              base_key & { layered = layered_ }
          )
          (std.array.length base_keys),

  # Pre-lowering: only `layered` (array) contributes to N (numbered slots).
  #
  # Keys with only `named_layers` return 0 here. Their M slots are counted via
  # named_layer_names_* and appended in lower_named_layers_on_key as:
  #   [null * N] ++ [named slots in global order].
  numbered_layered_length_of = fun k =>
    k
    |> match {
      { layered, .. } if keymap_ncl.layered.is_key k => std.array.length layered,
      _ => 0,
    },

  # Longest numbered layered array in a key tree (via map_accum for nesting).
  max_numbered_layered_length_accum = fun acc k =>
    let len = numbered_layered_length_of k in
    let acc = if len > acc then len else acc in
    let { acc, k = _ } =
      keymap_ncl.nullable_key.map_accum
        (fun acc k =>
          let len = numbered_layered_length_of k in
          let acc = if len > acc then len else acc in
          { include acc, include k }
        )
        acc
        k
    in
    acc,

  # Alias used by padding / checks for dense array length after named-layer lowering
  # (everything is array-form by then; pre-lowering this is intentionally numbered-only).
  max_layered_length_accum = max_numbered_layered_length_accum,

  # Collect named-layer field names into a record set { layer_name = true, ... }.
  # std.record.fields of the result is alphabetically sorted (Nickel field order).
  named_layer_names_from_key = fun k =>
    k
    |> match {
      { named_layers, .. } if keymap_ncl.layered.is_key k =>
        named_layers
        |> std.record.fields
        |> std.array.fold_left
          (fun names layer_name => names & { "%{layer_name}" = true })
          {},
      _ => {},
    },

  named_layer_names_accum = fun acc_names k =>
    let acc_names = acc_names & named_layer_names_from_key k in
    let { acc = acc_names, k = _ } =
      keymap_ncl.nullable_key.map_accum
        (fun acc_names k =>
          { acc = acc_names & named_layer_names_from_key k, include k }
        )
        acc_names
        k
    in
    acc_names,

  named_layer_names_from_keys = fun keys =>
    keys
    |> std.array.fold_left named_layer_names_accum {}
    |> std.record.fields,

  numbered_layer_count_from_keys = fun keys =>
    keys |> std.array.fold_left max_numbered_layered_length_accum 0,

  # Total dense layered array length = numbered slots + named slots (named after numbered).
  total_layered_length_from_keys = fun keys =>
    let numbered = numbered_layer_count_from_keys keys in
    let named = named_layer_names_from_keys keys in
    numbered + std.array.length named,

  named_layer_names
    | doc "Alphabetically sorted named layer names used in the keymap."
    = named_layer_names_from_keys layered_keys,

  numbered_layer_count
    | doc "Max numbered (`layered` array) length across the keymap."
    = numbered_layer_count_from_keys layered_keys,

  max_layered_length
    | doc "Dense layered array length after named layers are appended (numbered + named)."
    = numbered_layer_count + std.array.length named_layer_names,

  # 1-based layer indices for layer_mod.* (layered[i] <-> layer i+1).
  # Named layers start after numbered slots: first named layer -> numbered_layer_count + 1.
  named_layer_indices
    | doc "Record of named layer name -> 1-based layer index (for layer_mod)."
    =
      named_layer_names
      |> std.array.map_with_index (fun i layer_name => { "%{layer_name}" = numbered_layer_count + i + 1 })
      |> std.array.fold_left (&) {},

  # Pad or truncate a numbered `layered` array to exactly `target_len`.
  pad_or_slice_layered = fun target_len layered =>
    let len = std.array.length layered in
    if len >= target_len then
      std.array.slice 0 target_len layered
    else
      layered @ std.array.generate (std.function.const null) (target_len - len),

  # Look up each global named-layer name in `named_rec`; missing -> null (transparency).
  named_slots_from = fun named_names named_rec =>
    let filled =
      named_names
      |> std.array.fold_left
        (fun acc layer_name => acc & { "%{layer_name}" | default = null })
        named_rec
    in
    named_names |> std.array.map (fun layer_name => std.record.get layer_name filled),

  # Scope: one key record (no nested walk).
  # Merge layered (numbered) + named_layers into a dense layered array.
  # Named layers occupy indices [numbered_len, numbered_len + |names|).
  # Strips named_layers. Non-layered keys (and null) pass through unchanged.
  lower_named_layers_on_key = fun numbered_len named_names k =>
    k
    |> match {
      k if !(std.is_record k && keymap_ncl.layered.is_key k) => k,
      { layered = numbered, named_layers = named, ..base_key } =>
        base_key
        & {
          layered =
            pad_or_slice_layered numbered_len numbered
            @ named_slots_from named_names named,
        },
      { layered = numbered, ..base_key } =>
        base_key
        & {
          layered =
            pad_or_slice_layered numbered_len numbered
            @ named_slots_from named_names {},
        },
      { named_layers = named, ..base_key } =>
        base_key
        & {
          layered =
            std.array.generate (std.function.const null) numbered_len
            @ named_slots_from named_names named,
        },
    },

  # Scope: one key tree (self, then nested via map_accum).
  lower_named_layers_in_key_tree = fun numbered_len named_names k =>
    let lower_f = fun acc key =>
      let lowered = lower_named_layers_on_key numbered_len named_names key in
      { include acc, k = lowered }
    in
    let k = lower_named_layers_on_key numbered_len named_names k in
    let { acc = _, k } = keymap_ncl.nullable_key.map_accum lower_f null k in
    k,

  # Scope: array of key trees (pipeline pass). No-op when no named layers used.
  lower_named_layers_on_keys = fun keys =>
    let named_names = named_layer_names_from_keys keys in
    named_names
    |> match {
      [] => keys,
      _ =>
        let numbered_len = numbered_layer_count_from_keys keys in
        keys |> std.array.map (lower_named_layers_in_key_tree numbered_len named_names),
    },

  # Pad a layered key's `layered` array with null (transparency) up to `target_len`.
  # Expects post-lowering form (array `layered` only; no `named_layers`).
  pad_layered_key = fun target_len k =>
    k
    |> match {
      { named_layers = _, .. } =>
        std.fail_with "pad_layered_key expects array-form layered (lower named layers first)",
      { layered = numbered, ..base_key } =>
        base_key & { layered = pad_or_slice_layered target_len numbered },
      k => k,
    },

  # Pad every layered key in a key tree to `target_len` (self, then nested via map_accum).
  pad_key_layered_arrays = fun target_len k =>
    let pad_f = fun acc key =>
      let padded = pad_layered_key target_len key in
      { include acc, k = padded }
    in
    let k = pad_layered_key target_len k in
    let { acc = _, k } = keymap_ncl.nullable_key.map_accum pad_f null k in
    k,

  checks.check_max_layered_length =
    let K = import "keys.ncl" in
    {
      check_unequal_lengths_uses_longest = {
        actual =
          [
            K.A & { layered = [K.B] },
            K.C & { layered = [ null, K.D] },
          ]
          |> std.array.fold_left max_layered_length_accum 0,
        expected = 2,
      },

      check_first_key_not_layered = {
        actual =
          [
            K.layer_mod.hold 1,
            K.A & { layered = [ null, K.B] },
          ]
          |> std.array.fold_left max_layered_length_accum 0,
        expected = 2,
      },

      check_no_layered_keys = {
        actual = [K.A, K.B] |> std.array.fold_left max_layered_length_accum 0,
        expected = 0,
      },

      check_pad_shorter_layered_array = {
        actual =
          pad_layered_key 3 (K.A & { layered = [K.B] }),
        expected = K.A & { layered = [K.B, null, null ] },
      },

      check_json_pads_unequal_layered_lengths =
        let keys = [
          K.A & { layered = [K.B] },
          K.C & { layered = [ null, K.D] },
        ]
        in
        let layer_count = keys |> std.array.fold_left max_layered_length_accum 0 in
        {
          actual =
            keys
            |> std.array.map (pad_key_layered_arrays layer_count)
            |> std.array.map keymap_ncl.key.to_json_value,
          expected = [
            {
              base = { key_code = 4 },
              layered = [
                { key_code = 5 },
                null,
              ],
            },
            {
              base = { key_code = 6 },
              layered = [
                null,
                { key_code = 7 },
              ],
            },
          ],
        },
    },

  checks.check_named_layers =
    let K = import "keys.ncl" in
    {
      check_named_layer_names_sorted = {
        actual =
          named_layer_names_from_keys [
            K.A & { named_layers = { windows = K.B, linux = K.C } },
            K.D & { named_layers = { macos = K.E } },
          ],
        expected = ["linux", "macos", "windows"],
      },

      check_named_layer_indices_after_numbered =
        let keys = [
          K.A & { layered = [ null, K.B] },
          K.C & { named_layers = { windows = K.D, linux = K.E } },
        ]
        in
        let numbered = numbered_layer_count_from_keys keys in
        let names = named_layer_names_from_keys keys in
        {
          actual =
            names
            |> std.array.map_with_index (fun i layer_name => { "%{layer_name}" = numbered + i + 1 })
            |> std.array.fold_left (&) {},
          # numbered length 2 -> linux=3, windows=4
          expected = { linux = 3, windows = 4 },
        },

      check_total_length_numbered_plus_named = {
        actual =
          total_layered_length_from_keys [
            K.A & { layered = [K.B] },
            K.C & { named_layers = { nav = K.D } },
          ],
        expected = 2,
      },

      check_lower_named_only =
        let keys = [K.A & { named_layers = { fn = K.B, nav = K.C } }] in
        {
          actual = lower_named_layers_on_keys keys |> std.array.map keymap_ncl.key.to_json_value,
          expected = [
            {
              base = { key_code = 4 },
              # named sorted: fn, nav -> indices 0, 1
              layered = [
                { key_code = 5 },
                { key_code = 6 },
              ],
            },
          ],
        },

      check_lower_numbered_then_named =
        let keys = [
          K.A & { layered = [ null, K.B] },
          K.C & { named_layers = { windows = K.D } },
        ]
        in
        {
          actual = lower_named_layers_on_keys keys |> std.array.map keymap_ncl.key.to_json_value,
          expected = [
            {
              base = { key_code = 4 },
              # numbered [null, B] + named slot windows
              layered = [
                null,
                { key_code = 5 },
                null,
              ],
            },
            {
              base = { key_code = 6 },
              # numbered padding + windows = D
              layered = [
                null,
                null,
                { key_code = 7 },
              ],
            },
          ],
        },

      check_lower_partial_named_union =
        let keys = [
          K.A & { named_layers = { linux = K.B, windows = K.C } },
          K.D & { named_layers = { macos = K.E } },
        ]
        in
        {
          actual = lower_named_layers_on_keys keys |> std.array.map keymap_ncl.key.to_json_value,
          expected = [
            {
              base = { key_code = 4 },
              # sorted: linux, macos, windows
              layered = [
                { key_code = 5 },
                null,
                { key_code = 6 },
              ],
            },
            {
              base = { key_code = 7 },
              layered = [
                null,
                { key_code = 8 },
                null,
              ],
            },
          ],
        },

      # Numbered + named on the same key (composable via `&`).
      check_lower_both_on_same_key =
        let keys = [
          K.A
          & { layered = [ null, K.B] }
          & { named_layers = { fn = K.C } },
        ]
        in
        {
          actual = lower_named_layers_on_keys keys |> std.array.map keymap_ncl.key.to_json_value,
          expected = [
            {
              base = { key_code = 4 },
              # numbered [null, B] + named fn = C
              layered = [
                null,
                { key_code = 5 },
                { key_code = 6 },
              ],
            },
          ],
        },
    },

  chorded_keys
    | Array ChordedKeyElement
    =
      std.array.map_with_index
        (fun key_index k =>
          chords
          |> std.array.map_with_index (fun idx ch => ch & { chord_index = idx })
          |> std.array.filter (fun { indices = [chord_primary_key_index, ..], .. } => chord_primary_key_index == key_index)
          |> match {
            [] =>
              if std.array.any (fun { indices, .. } => std.array.elem key_index indices) chords then
                { passthrough = k }
              else
                k,
            chords_ =>
              let chords_ = chords_ |> std.array.map (fun { chord_index, key, .. } => [chord_index, key]) in
              { chords = chords_, passthrough = k },
          }
        )
        layered_keys,

  # One slot in `automation_transform.keys` after the instruction fold.
  #
  # Usually an ordinary keymap key (keyboard, null, layered, chorded, …). Automation
  # keys — e.g. `K.string_macro "a"` (`{ automation_instructions.on_press = […] }`
  # before this stage) — are rewritten to `automation::Key`: `automation_instructions`
  # holds `{ start, length }` execution refs into the accumulated instruction list.
  automation_key_element_validator = fun k =>
    k
    |> match {
      { automation_instructions, .. } =>
        keymap_ncl.automation.key_validator k,
      _ =>
        keymap_ncl.nullable_key.key_validator k,
    },

  AutomationKeyElement = std.contract.from_validator automation_key_element_validator,

  AutomationTransform = {
    automation_instructions | Array keymap_ncl.automation.Instruction,
    keys | Array AutomationKeyElement,
  },

  automation_transform
    | AutomationTransform
    | doc "Fold chorded keys: flat `{ automation_instructions, keys }` — config instruction list plus post-transform key slots."
    =
      chorded_keys
      |> std.array.fold_left
        (fun { automation_instructions = prior_instructions, keys = prior_keys } k =>
          let { acc, k } =
            keymap_ncl.key.map_accum
              keymap_ncl.automation.transform_keys_to_instructions
              { automation_instructions = prior_instructions }
              k
          in
          {
            automation_instructions = acc.automation_instructions,
            keys = prior_keys @ [k],
          }
        )
        { automation_instructions = [], keys = [] },

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
      # Named order is alphabetical (Nickel record field order). Skipped if unused.
      let keys = lower_named_layers_on_keys keys in
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
