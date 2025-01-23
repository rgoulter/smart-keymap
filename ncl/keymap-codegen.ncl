let validators = import "validators.ncl" in
# "Nested depth":
#   number of composite::Key<NK> it takes to represent the key in keymap.rs.
# e.g.:
#
# | Key                                            | ND |
# +------------------------------------------------+----+
# | keyboard::Key                                  |  0 |
# | tap_hold::Key<keyboard::Key>                   |  0 |
# | composite::Key<keyboard::Key>                  |  1 |
# | tap_hold::Key<composite::Key<keyboard::Key>>   |  1 |
# | composite::Key<composite::Key<keyboard::Key>>  |  2 |
#
# Discussion:
# - Kinds of keys:
#   - 'base keys': keys which do not contain nested keys.
#      e.g. keyboard::Key.
#   - 'modifier keys': keys which contain nested keys,
#      'modifying' the functionality of the key.
#      e.g. tap_hold::Key<NK>, layered::LayeredKey<NK>.
#   - 'aggregate key': key type which can be formed from other key types,
#      allowing the construction of "Array of aggregate keys".
#      e.g. composite::Key<NK>.
# - The 'modifier keys' in Rust require the same type for NK.
#   - e.g. CANNOT have tap_hold key with keyboard::Key tap and layered::LayerModifier hold;
#   - e.g.    CAN have tap_hold key with composite::Key<keyboard::Key>,
#                  where tap = composite::Key::keyboard(...),
#                   and hold = composite::Key::layer_modifier(...).
# - The nested keys of the 'modifier keys' can be more freely formed in Nickel.
# - If all the nested keys in a modifier key (e.g. both tap, hold of tap_hold::Key)
#    can be represented by the same key type
#    (e.g. both by keyboard::Key, or both by composite::Key<keyboard::Key>),
#   then no further nesting is needed.
# - Differing types can be formed into an 'aggregate' type;
#   - e.g. a keyboard::Key and a layered::LayerModifier key can
#     both be represented by composite::Key<NK> for any NK.
#     (Base keys contain no nested keys)
#   - if all modifier keys have the same depth of nesting,
#      (e.g. tap_hold::Key<NK> and layered::LayeredKey<NK> have same NK),
#     then these can be represented by composite::Key<NK>.
#     - e.g. a tap_hold::Key<NK> can be represented by composite::Key<NK>.
#   - a modifier key can increase the amount of nesting it has
#     by wrapping its nested keys with composite::Key<>.
#     - e.g. a tap_hold::Key<NK> can wrap its NK with composite::Key<>
#       in order to be represented by composite::Key<composite::Key<NK>>.
#   - a modifier key with shallower depth of nesting can
#      increase the amount of nesting it has until it has the same depth
#      of nesting as another modifier key.
#      (Assuming the same base nested key).
let nested_key_type_for
  = fun key_types =>
      let kt =
        std.array.reduce_left
          (fun x y =>
            let kt_max_nested_depth =
              if x.nested_depth >= y.nested_depth then
                x
              else
                y in
            if x.key_type == y.key_type then
              kt_max_nested_depth
            else
              {
                key_type = "crate::key::composite::Key",
                nested_depth = 1 + kt_max_nested_depth.nested_depth,
              })
          key_types in
      let rec nested_key_type_for_depth = fun n nk =>
        if n == 0 then
          nk
        else
          nested_key_type_for_depth
            (n - 1)
            "crate::key::composite::Key<crate::key::composite::CompositeImpl<LayersImpl, %{nk}>>" in
      kt & {
        as_nested_key_type =
          let key_type_generics = "%{nested_key_type_for_depth (kt.nested_depth - 1) "crate::key::keyboard::Key"}" in
          if kt.nested_depth > 0 then
            if kt.key_type == "crate::key::composite::Key" then
              "%{kt.key_type}<crate::key::composite::CompositeImpl<LayersImpl, %{key_type_generics}>>"
            else
              "%{kt.key_type}<%{key_type_generics}>"
          else
            kt.key_type,
      }
  in
{
  serialized_json_keymap
    | doc "The 'JSON serialized' value of the keymap. e.g. imported from keymap.json.",

  checks.nested_key_type_for = {
    nk_depth_for_kbd_kbd =
      let kbd = { key_code = 0x04 } in
      let kbd_kt = key.key_type kbd in
      {
        actual = (nested_key_type_for [kbd_kt, kbd_kt]).nested_depth,
        expected = 0,
      },

    nk_type_for_kbd_kbd =
      let kbd = { key_code = 0x04 } in
      let kbd_kt = key.key_type kbd in
      {
        actual = (nested_key_type_for [kbd_kt, kbd_kt]).key_type,
        expected = "crate::key::keyboard::Key",
      },

    nk_as_nested_key_type_for_kbd_kbd =
      let kbd = { key_code = 0x04 } in
      let kbd_kt = key.key_type kbd in
      {
        actual = (nested_key_type_for [kbd_kt, kbd_kt]).as_nested_key_type,
        expected = "crate::key::keyboard::Key",
      },

    nk_depth_for_lmod_kbd =
      let kbd = { key_code = 0x04 } in
      let kbd_kt = key.key_type kbd in
      let lmod = { Hold = 0 } in
      let lmod_kt = key.key_type lmod in
      {
        actual = (nested_key_type_for [kbd_kt, lmod_kt]).nested_depth,
        expected = 1,
      },

    nk_type_for_lmod_kbd =
      let kbd = { key_code = 0x04 } in
      let kbd_kt = key.key_type kbd in
      let lmod = { Hold = 0 } in
      let lmod_kt = key.key_type lmod in
      {
        actual = (nested_key_type_for [kbd_kt, lmod_kt]).key_type,
        expected = "crate::key::composite::Key",
      },

    nk_as_nested_key_type_for_lmod_kbd =
      let kbd = { key_code = 0x04 } in
      let kbd_kt = key.key_type kbd in
      let lmod = { Hold = 0 } in
      let lmod_kt = key.key_type lmod in
      {
        actual = (nested_key_type_for [lmod_kt, kbd_kt]).as_nested_key_type,
        expected = "crate::key::composite::Key<crate::key::composite::CompositeImpl<LayersImpl, crate::key::keyboard::Key>>",
      },

    nk_depth_for_th_lmod_kbd_and_kbd =
      let kbd = { key_code = 0x04 } in
      let kbd_kt = key.key_type kbd in
      let lmod = { Hold = 0 } in
      let th = { tap = kbd, hold = lmod } in
      let th_kt = key.key_type th in
      {
        actual = (nested_key_type_for [kbd_kt, th_kt]).nested_depth,
        expected = 2,
      },
  },

  checks.keyboard_is
    = {
        # Use a serialized value to check the "is" predicate.
        check_serialized_is =
          let serialized_value = { key_code = 0x04 } in
          keyboard.is_serialized_json serialized_value,
        check_serialized_with_modifier_is =
          let serialized_value = { key_code = 0x04, modifiers = { left_ctrl = true} } in
          keyboard.is_serialized_json serialized_value,
        check_serialized_modifiers_is =
          let serialized_value = { modifiers = { left_ctrl = true} } in
          keyboard.is_serialized_json serialized_value,
      },

  keyboard
    | doc "for key::keyboard::Key."
    = {
        # c.f. doc_de_keyboard.md.
        serialized_json_validator
          = validators.record.validator {
              fields_validator = validators.all_of [
                validators.record.has_any_field_of ["key_code", "modifiers"],
                validators.record.has_only_fields ["key_code", "modifiers"],
              ],
              field_validators = {
                key_code = validators.is_number,
                modifiers = validators.record.validator {
                  fields_validator = validators.record.has_only_fields [
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
              },
            },

        is_serialized_json
          = fun k => 'Ok == serialized_json_validator k,

        key_type
          = {
              key_type = "crate::key::keyboard::Key",
              nested_depth = 0,
              as_rust_type_path = key_type,
            },

        rust_expr
          = fun key =>
              let rec modifiers_expr = fun m =>
                m |> match {
                  {} => "crate::key::KeyboardModifiers::new()",
                  { left_ctrl, ..modifiers } if left_ctrl =>
                    "crate::key::KeyboardModifiers::LEFT_CTRL.union(&%{modifiers_expr modifiers})",
                  { left_alt, ..modifiers } if left_alt =>
                    "crate::key::KeyboardModifiers::LEFT_ALT.union(&%{modifiers_expr modifiers})",
                  { left_shift, ..modifiers } if left_shift =>
                    "crate::key::KeyboardModifiers::LEFT_SHIFT.union(&%{modifiers_expr modifiers})",
                  { left_gui, ..modifiers } if left_gui =>
                    "crate::key::KeyboardModifiers::LEFT_GUI.union(&%{modifiers_expr modifiers})",
                  { right_ctrl, ..modifiers } if right_ctrl =>
                    "crate::key::KeyboardModifiers::RIGHT_CTRL.union(&%{modifiers_expr modifiers})",
                  { right_alt, ..modifiers } if right_alt =>
                    "crate::key::KeyboardModifiers::RIGHT_ALT.union(&%{modifiers_expr modifiers})",
                  { right_shift, ..modifiers } if right_shift =>
                    "crate::key::KeyboardModifiers::RIGHT_SHIFT.union(&%{modifiers_expr modifiers})",
                  { right_gui, ..modifiers } if right_gui =>
                    "crate::key::KeyboardModifiers::RIGHT_GUI.union(&%{modifiers_expr modifiers})",
                } in
              key |> match {
                { key_code } =>
                  "crate::key::keyboard::Key::new(%{std.to_string key_code})",
                { modifiers } =>
                  "crate::key::keyboard::Key::from_modifiers(%{modifiers_expr modifiers})",
                { key_code, modifiers } =>
                  "crate::key::keyboard::Key::new_with_modifiers(%{std.to_string key_code}, %{modifiers_expr modifiers})",
              },
      },

  checks.layer_modifier_is
    = {
        # Use a serialized value to check the "is" predicate.
        check_serialized_is
          = let serialized_value = { Hold = 0 } in
            layer_modifier.is_serialized_json serialized_value
      },

  layer_modifier
    | doc "for key::layered::ModifierKey."
    = {
        # c.f. doc_de_layered.md.
        # JSON serialization of key::layered::ModifierKey has variants: Hold(layer).
        serialized_json_validator
          = validators.record.validator {
              fields_validator = validators.record.has_exact_fields ["Hold"],
              field_validators = {
                Hold = validators.is_number,
              },
            },

        is_serialized_json
          = fun k => 'Ok == serialized_json_validator k,

        key_type
          = {
              key_type = "crate::key::layered::ModifierKey",
              nested_depth = 0,
              as_rust_type_path = key_type,
            },

        rust_expr
          = fun { Hold = layer_index } =>
            "crate::key::layered::ModifierKey::Hold(%{std.to_string layer_index})",
      },

  checks.layered_is
    = {
        # Use a serialized value to check the "is" predicate.
        check_serialized_base_keyboard_layered_keyboard_is
          = let serialized_value = {
              base = { key_code = 0x04 },
              layered = [
                null,
                { key_code = 0x06 },
              ]
            } in
            layered.is_serialized_json serialized_value
      },

  layered
    | doc "for key::layered::LayeredKey."
    = {
        # c.f. doc_de_layered.md.
        # e.g.:
        # ```
        #   {
        #     "base": 4,
        #     "layered": [5, null, 7]
        #   }
        # ```
        # JSON serialization of key::layered::Layered has variants: { base, layered }
        serialized_json_validator
          = validators.record.validator {
              fields_validator = validators.record.has_exact_fields ["base", "layered"],
              field_validators = {
                base = key.serialized_json_validator,
                layered = validators.array.validator
                  (validators.any_of [
                    validators.is_null,
                    key.serialized_json_validator,
                  ]),
              },
            },

        is_serialized_json
          = fun k => 'Ok == serialized_json_validator k,

        # c.f. doc_de_layered.md.
        # ```
        #   type L = layered::ArrayImpl<3>;
        #   type Key = layered::LayeredKey<keyboard::Key, L>;
        # ```
        key_type
          = fun { base, layered } =>
              let num_layers = layered |> std.array.length |> std.to_string in
              let layer_impl = "crate::key::layered::ArrayImpl<%{num_layers}>" in
              let base_key_type = key.key_type base in
              let non_null_layered = std.array.filter ((!=) null) layered in
              let layered_key_types = std.array.map (fun nk => key.key_type nk) non_null_layered in
              let nested_key_type = nested_key_type_for ([base_key_type] @ layered_key_types) in
              let nested_type_path = nested_key_type.as_nested_key_type in
              {
                key_type = "crate::key::layered::LayeredKey",
                nested_depth = nested_key_type.nested_depth,
                as_rust_type_path = "%{key_type}<%{nested_type_path}, %{layer_impl}>",
              },

        rust_expr
          = fun { base, layered } =>
              let base_key_type = key.key_type base in
              let non_null_layered = std.array.filter ((!=) null) layered in
              let layered_key_types = std.array.map (fun nk => key.key_type nk) non_null_layered in
              let nested_kt = nested_key_type_for ([base_key_type] @ layered_key_types) in
              let nk_expr =
                if nested_kt.key_type == "crate::key::composite::Key" then
                  composite.wrap_rust_expr
                else
                  key.rust_expr in
              let base_expr = nk_expr base in
              let layered_exprs = layered
                |> std.array.map (fun lk => if lk == null then "None" else "Some(%{nk_expr lk})")
                |> std.string.join "," in
              "crate::key::layered::LayeredKey::new(%{base_expr}, [%{layered_exprs}])",
      },

  checks.check_tap_hold = {
    # Use a serialized value to check the "is" predicate.
    check_serialized_tap_keyboard_hold_keyboard_is
      = let serialized_value = {
          hold = { key_code = 0xE0 },
          tap = { key_code = 0x04 },
        } in
        tap_hold.is_serialized_json serialized_value,

    check_nested_depth = {
      check_th_keyboard = {
        actual =
          let th_key = {
            hold = { key_code = 0xE0 },
            tap = { key_code = 0x04 },
          } in
          (tap_hold.key_type th_key).nested_depth,
        expected = 0,
      },

      check_th_composite_keyboard = {
        actual =
          # Technically, this is not "well-formed"
          #  serialized-json, since it couldn't
          #  be deserialized into a Rust type.
          let th_key = {
            hold = { Hold = 0 },
            tap = { key_code = 0x04 },
          } in
          (tap_hold.key_type th_key).nested_depth,
        expected = 1,
      },
    },
  },

  tap_hold
    | doc "for key::tap_hold::Key."
    = {
        # c.f. doc_de_tap_hold.md.
        # JSON serialization of key::tap_hold::Key is { tap: key, hold: key }
        serialized_json_validator
          = validators.record.validator {
            fields_validator = validators.record.has_exact_fields ["tap", "hold"],
            field_validators = {
              tap = key.serialized_json_validator,
              hold = key.serialized_json_validator,
            },
          },

        is_serialized_json
          = fun k => 'Ok == serialized_json_validator k,

        key_type
          = fun { hold, tap } =>
              let hold_key_type = key.key_type hold in
              let tap_key_type = key.key_type tap in
              let nested_key_type = nested_key_type_for [hold_key_type, tap_key_type] in
              let nested_type_path = nested_key_type.as_nested_key_type in
              {
                key_type = "crate::key::tap_hold::Key",
                nested_depth = nested_key_type.nested_depth,
                as_rust_type_path = "%{key_type}<%{nested_type_path}>"
              },

        rust_expr
          = fun { hold, tap, } =>
              let tap_kt = key.key_type tap in
              let hold_kt = key.key_type hold in
              let nested_kt = nested_key_type_for [tap_kt, hold_kt] in
              let nk_expr =
                if nested_kt.key_type == "crate::key::composite::Key" then
                  composite.wrap_rust_expr
                else
                  key.rust_expr in
              let hold_expr = nk_expr hold in
              let tap_expr = nk_expr tap in
              "crate::key::tap_hold::Key::new(%{tap_expr}, %{hold_expr})",
      },

  composite
    = {
        wrap_rust_expr
          = match {
              k if keyboard.is_serialized_json k =>
                let k_expr = keyboard.rust_expr k in
                "crate::key::composite::Key::keyboard(%{k_expr})",
              k if layer_modifier.is_serialized_json k =>
                let k_expr = layer_modifier.rust_expr k in
                "crate::key::composite::Key::layer_modifier(%{k_expr})",
              k if layered.is_serialized_json k =>
                let k_expr = layered.key_expr k in
                "crate::key::composite::Key::layered(%{k_expr})",
              k if tap_hold.is_serialized_json k =>
                let k_expr = tap_hold.rust_expr k in
                "crate::key::composite::Key::tap_hold(%{k_expr})",
              _ => 'Error { message = "unknown variant for key::composite::Key" },
            },
      },

  key
    = {
        serialized_json_validator
          = validators.any_of
              [
                keyboard.serialized_json_validator,
                layer_modifier.serialized_json_validator,
                layered.serialized_json_validator,
                tap_hold.serialized_json_validator,
              ],

        is_serialized_json
          = fun k => 'Ok == serialized_json_validator k,

        key_type
          = match {
              key if keyboard.is_serialized_json key => keyboard.key_type,
              key if layer_modifier.is_serialized_json key => layer_modifier.key_type,
              key if layered.is_serialized_json key => layered.key_type key,
              key if tap_hold.is_serialized_json key => tap_hold.key_type key,
              _ => 'unknown_key_type
            },

        rust_expr
          = match {
              key if keyboard.is_serialized_json key => keyboard.rust_expr key,
              key if layer_modifier.is_serialized_json key => layer_modifier.rust_expr key,
              key if layered.is_serialized_json key => layered.rust_expr key,
              key if tap_hold.is_serialized_json key => tap_hold.rust_expr key,
              _ => 'unknown_key_type
            },
      },

  keymap_rs
    | doc "Text contents of the keymap.rs generated from the keymap.json"
    = let keymap_len = std.array.length serialized_json_keymap in
      let num_layers =
        serialized_json_keymap |>
        std.array.fold_left
          (fun max_num_layers key =>
            let num_layers = key |> match {
              { layered, .. } => std.array.length layered,
              _ => 0,
            } in
            if num_layers > max_num_layers then
              num_layers
            else
              max_num_layers)
          0 in
      let keys_id = "Keys%{std.to_string keymap_len}" in
      let key_types = serialized_json_keymap
        |> std.array.map key.key_type
        |> std.array.map (fun key_type => key_type.as_rust_type_path)
        |> std.string.join "," in
      let key_exprs = serialized_json_keymap
        |> std.array.map (fun k => "%{key.rust_expr k},")
        |> std.string.join "" in
      m%"
/// Types and initial data used for constructing [KEYMAP].
pub mod init {
    const NUM_LAYERS: usize = %{std.to_string num_layers};

    /// Alias for layers impl.
    pub type LayersImpl = crate::key::layered::ArrayImpl<NUM_LAYERS>;

    /// Alias for the NestedKey used for the [Context].
    pub type NestedKey = crate::key::composite::DefaultNestableKey;

    /// Types used in Composite keys.
    pub type CompositeImpl = crate::key::composite::CompositeImpl<LayersImpl, NestedKey>;

    /// Alias for Context type; i.e. [crate::key::context::Context] with generics.
    pub type Context = crate::key::composite::Context<LayersImpl>;

    /// Alias for Event type; i.e. [crate::key::context::Event].
    pub type Event = crate::key::composite::Event;

    /// Initial [Context] value.
    pub const CONTEXT: Context = crate::key::composite::Context {
        layer_context: crate::key::layered::Context {
            active_layers: [false; NUM_LAYERS],
        },
    };

    crate::tuples::define_keys!(%{std.to_string keymap_len});

    /// Alias for a [tuples] KeysN type, as generated by keymap.ncl.
    pub type KeyDefinitionsType = %{keys_id}<
    %{key_types},
    Context,
    Event
    >;

    /// A [tuples] KeysN value with keys, as generated by keymap.ncl.
    pub const KEY_DEFINITIONS: KeyDefinitionsType = %{keys_id}::new((
    %{key_exprs}
    ));
}
"%
}
