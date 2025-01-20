let validators = import "validators.ncl" in
{
  serialized_json_keymap
    | doc "The 'JSON serialized' value of the keymap. e.g. imported from keymap.json.",

  nestable
    | doc "for key::composite::NestableKey implementors."
    = {
      serialized_json_validator
        = keyboard.serialized_json_validator,

      is_serialized_json
        = keyboard.is_serialized_json,

      key_type
        = keyboard.key_type,

      rust_expr
        = keyboard.rust_expr,
    },


  checks.keyboard_is = {
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

      key_type = "crate::key::keyboard::Key",

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

  checks.layer_modifier_is = {
    # Use a serialized value to check the "is" predicate.
    check_serialized_is =
      let serialized_value = { Hold = 0 } in
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

      key_type = "crate::key::layered::ModifierKey",

      rust_expr = fun { Hold = layer_index } => "crate::key::layered::ModifierKey::Hold(%{std.to_string layer_index})",
    },

  checks.layered_is = {
    # Use a serialized value to check the "is" predicate.
    check_serialized_base_keyboard_layered_keyboard_is =
      let serialized_value = {
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
            base = keyboard.serialized_json_validator,
            layered = validators.array.validator
              (validators.any_of [
                validators.is_null,
                nestable.serialized_json_validator,
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
      key_type = fun { layered, .. } =>
        let nested_key_type = "crate::key::keyboard::Key" in
        let num_layers = layered |> std.array.length |> std.to_string in
        let layer_impl = "crate::key::layered::ArrayImpl<%{num_layers}>" in
        "crate::key::layered::LayeredKey<%{nested_key_type}, %{layer_impl}>",

      rust_expr = fun { base, layered } =>
        let nested_key_expr = keyboard.rust_expr in
        let base_expr = nested_key_expr base in
        let layered_exprs = layered
          |> std.array.map (fun lk => if lk == null then "None" else "Some(%{nested_key_expr lk})")
          |> std.string.join "," in
        m%"
          crate::key::layered::LayeredKey { base: %{base_expr}, layered: [%{layered_exprs}] }
        "%,
    },

  checks.tap_hold_is = {
    # Use a serialized value to check the "is" predicate.
    check_serialized_tap_keyboard_hold_keyboard_is =
      let serialized_value = {
        hold = { key_code = 0xE0 },
        tap = { key_code = 0x04 },
      } in
      tap_hold.is_serialized_json serialized_value
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
            tap = nestable.serialized_json_validator,
            hold = nestable.serialized_json_validator,
          },
        },

      is_serialized_json
        = fun k => 'Ok == serialized_json_validator k,

      # kludge: support tap_hold::Key<K> in Rust first, then impl. NCL support later.
      key_type = "crate::key::tap_hold::Key<crate::key::keyboard::Key>",
      rust_expr = fun { hold, tap, } =>
        let hold_expr = rust_expr_of_key hold in
        let tap_expr = rust_expr_of_key tap in
        m%"
        crate::key::tap_hold::Key {
          hold: %{hold_expr},
          tap: %{tap_expr},
        }
        "%,
    },

  key_type_of_key
    | doc "The type declaration for the key."
    = match {
      key if keyboard.is_serialized_json key => keyboard.key_type,
      key if layer_modifier.is_serialized_json key => layer_modifier.key_type,
      key if layered.is_serialized_json key => layered.key_type key,
      key if tap_hold.is_serialized_json key => tap_hold.key_type,
      _ => 'unknown_key_type
    },

  rust_expr_of_key
    | doc "The Rust expression for the key."
    = match {
      key if keyboard.is_serialized_json key => keyboard.rust_expr key,
      key if layer_modifier.is_serialized_json key => layer_modifier.rust_expr key,
      key if layered.is_serialized_json key => layered.rust_expr key,
      key if tap_hold.is_serialized_json key => tap_hold.rust_expr key,
      _ => 'unknown_key_type
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
        |> std.array.map key_type_of_key
        |> std.string.join "," in
      let key_exprs = serialized_json_keymap
        |> std.array.map (fun key => "%{rust_expr_of_key key},")
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
    pub type CompositeImpl = crate::key::composite::CompositeImpl<NestedKey, LayersImpl>;

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
