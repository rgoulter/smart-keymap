let validators = import "validators.ncl" in
{
  serialized_json_keymap
    | doc "The 'JSON serialized' value of the keymap. e.g. imported from keymap.json."
    | { keys | Array key.SerializedJson, .. },

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
        SerializedJson = std.contract.from_validator serialized_json_validator,

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
        SerializedJson = std.contract.from_validator serialized_json_validator,

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
        SerializedJson = std.contract.from_validator serialized_json_validator,

        # c.f. doc_de_layered.md.
        # e.g.:
        # ```
        #   {
        #     "base": 4,
        #     "layered": [5, null, 7]
        #   }
        # ```
        # JSON serialization of key::layered::Layered has fields: { base, layered }
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
        #   type Key = layered::LayeredKey<keyboard::Key>;
        # ```
        key_type
          = fun { base, layered } =>
              let base_key_type = key.key_type base in
              let non_null_layered = std.array.filter ((!=) null) layered in
              let layered_key_types = std.array.map (fun nk => key.key_type nk) non_null_layered in
              let nested_key_type =
                if std.array.all (fun kt => kt == base_key_type) layered_key_types then
                  base_key_type
                else
                  composite.tap_hold_key.key_type base in
              {
                key_type = "crate::key::layered::LayeredKey",
                as_rust_type_path = "%{key_type}<%{nested_key_type.as_rust_type_path}>",
              },

        rust_expr
          = fun { base, layered } =>
              let base_key_type = key.key_type base in
              let non_null_layered = std.array.filter ((!=) null) layered in
              let layered_key_types = std.array.map (fun nk => key.key_type nk) non_null_layered in
              let nk_expr =
                if std.array.all (fun kt => kt == base_key_type) layered_key_types then
                  key.rust_expr
                else
                  composite.tap_hold_key.rust_expr in
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
  },

  tap_hold
    | doc "for key::tap_hold::Key."
    = {
        SerializedJson = std.contract.from_validator serialized_json_validator,

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
              # let nested_key_type =
              #   if tap_key_type == hold_key_type then
              #     tap_key_type
              #   else
              #     composite.base_key.key_type tap in
              let nested_key_type = composite.base_key.key_type tap in
              {
                key_type = "crate::key::tap_hold::Key",
                as_rust_type_path = "%{key_type}<%{nested_key_type.as_rust_type_path}>"
              },

        rust_expr
          = fun { hold, tap, } =>
              let tap_kt = key.key_type tap in
              let hold_kt = key.key_type hold in
              # let nk_expr =
              #   if tap_kt == hold_kt then
              #     key.rust_expr
              #   else
              #     composite.base_key.rust_expr in
              let nk_expr = composite.base_key.rust_expr in
              let hold_expr = nk_expr hold in
              let tap_expr = nk_expr tap in
              "crate::key::tap_hold::Key::new(%{tap_expr}, %{hold_expr})",
      },

  composite
    = {
        base_key
          = {
              SerializedJson = std.contract.from_validator serialized_json_validator,

              serialized_json_validator
                = validators.any_of
                    [
                      layer_modifier.serialized_json_validator,
                      keyboard.serialized_json_validator,
                    ],

              is_serialized_json
                = fun k => 'Ok == serialized_json_validator k,

              key_type
                = match {
                    key if layer_modifier.is_serialized_json key =>
                      {
                        key_type = "crate::key::composite::BaseKey",
                        as_rust_type_path = key_type,
                      },
                    key if keyboard.is_serialized_json key =>
                      {
                        key_type = "crate::key::composite::BaseKey",
                        as_rust_type_path = key_type,
                      },
                    _ => 'unknown_key_type
                  },

              rust_expr
                = match {
                    key if layer_modifier.is_serialized_json key =>
                      let expr = layer_modifier.rust_expr key in
                      "crate::key::composite::BaseKey::layer_modifier(%{expr})",
                    key if keyboard.is_serialized_json key =>
                      let expr = keyboard.rust_expr key in
                      "crate::key::composite::BaseKey::keyboard(%{expr})",
                    _ => 'unknown_key_type
                  },
            },

        tap_hold_key
          = {
              SerializedJson = std.contract.from_validator serialized_json_validator,

              serialized_json_validator
                = validators.any_of
                    [
                      tap_hold.serialized_json_validator,
                      composite.base_key.serialized_json_validator,
                    ],

              is_serialized_json
                = fun k => 'Ok == serialized_json_validator k,

              key_type
                = match {
                    key if tap_hold.is_serialized_json key =>
                      {
                        key_type = "crate::key::composite::TapHoldKey",
                        nested_key_type = "crate::key::composite::BaseKey",
                        as_rust_type_path = "%{key_type}<%{nested_key_type}>",
                      },
                    key if composite.base_key.is_serialized_json key =>
                      {
                        key_type = "crate::key::composite::TapHoldKey",
                        nested_key_type = "crate::key::composite::BaseKey",
                        as_rust_type_path = "%{key_type}<%{nested_key_type}>",
                      },
                    _ => 'unknown_key_type
                  },

              rust_expr
                = match {
                    key if tap_hold.is_serialized_json key =>
                      let expr = tap_hold.rust_expr key in
                      "crate::key::composite::TapHoldKey::tap_hold(%{expr})",
                    key if composite.base_key.is_serialized_json key =>
                      let expr = composite.base_key.rust_expr key in
                      "crate::key::composite::TapHoldKey::Pass(%{expr})",
                    _ => 'unknown_key_type
                  },
            },

        layered_key
          = {
              SerializedJson = std.contract.from_validator serialized_json_validator,

              serialized_json_validator
                = validators.any_of
                    [
                      layered.serialized_json_validator,
                      composite.tap_hold_key.serialized_json_validator,
                    ],

              is_serialized_json
                = fun k => 'Ok == serialized_json_validator k,

              key_type
                = match {
                    key if layered.is_serialized_json key =>
                      {
                        key_type = "crate::key::composite::LayeredKey",
                        nested_key_type = "crate::key::composite::TapHoldKey",
                        as_rust_type_path = "%{key_type}<%{nested_key_type}>",
                      },
                    key if composite.tap_hold_key.is_serialized_json key =>
                      {
                        key_type = "crate::key::composite::LayeredKey",
                        nested_key_type = "crate::key::composite::TapHoldKey",
                        as_rust_type_path = "%{key_type}<%{nested_key_type}>",
                      },
                    _ => 'unknown_key_type
                  },

              rust_expr
                = match {
                    key if layered.is_serialized_json key =>
                      let expr = layered.rust_expr key in
                      "crate::key::composite::LayeredKey::layered(%{expr})",
                    key if composite.tap_hold_key.is_serialized_json key =>
                      let expr = composite.tap_hold_key.rust_expr key in
                      "crate::key::composite::LayeredKey::Pass(%{expr})",
                    _ => 'unknown_key_type
                  },
            },

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
        SerializedJson = std.contract.from_validator serialized_json_validator,

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
    = let { config, keys } = serialized_json_keymap &
        {
          config = {
            tap_hold = {
              timeout | default = 200,
              interrupt_response | default = "Ignore",
            },
          },
        } in
      let tap_hold_config_expr = m%"
        crate::key::tap_hold::Config {
            timeout: %{std.to_string config.tap_hold.timeout},
            interrupt_response: crate::key::tap_hold::InterruptResponse::%{config.tap_hold.interrupt_response},
        }
      "% in
      let keymap_len = std.array.length keys in
      let num_layers =
        keys |>
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
      let key_types = keys
        |> std.array.map key.key_type
        |> std.array.map (fun key_type => key_type.as_rust_type_path)
        |> std.string.join "," in
      let key_exprs = keys
        |> std.array.map (fun k => "%{key.rust_expr k},")
        |> std.string.join "" in
      m%"
/// Types and initial data used for constructing [KEYMAP].
pub mod init {
    /// Config used by tap-hold keys.
    pub const CONFIG: crate::key::composite::Config = crate::key::composite::Config {
        tap_hold: %{tap_hold_config_expr},
    };

    /// Number of layers supported by the [crate::key::layered] implementation.
    pub const LAYER_COUNT: usize = %{std.to_string num_layers};

    /// Alias for Context type; i.e. [crate::key::context::Context] with generics.
    pub type Context = crate::key::composite::Context;

    /// Alias for Event type; i.e. [crate::key::context::Event].
    pub type Event = crate::key::composite::Event;

    /// Initial [Context] value.
    pub const CONTEXT: Context = crate::key::composite::Context {
        layer_context: crate::key::layered::Context {
            active_layers: [false; crate::key::layered::LAYER_COUNT],
        },
        tap_hold_context: crate::key::tap_hold::Context::from_config(CONFIG.tap_hold),
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
