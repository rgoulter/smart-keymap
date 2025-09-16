let validators = import "validators.ncl" in
{
  ChordIndicesJson = Array Number,

  ChordedConfigJson = {
    chords | optional | Array ChordIndicesJson,
    timeout | optional | Number,
  },

  StickyActivationJson_ =
    std.contract.from_validator (
      validators.is_elem_of [
        "OnStickyKeyRelease"
      ]
    ),

  StickyReleaseJson_ =
    std.contract.from_validator (
      validators.is_elem_of [
        "OnModifiedKeyRelease",
        "OnNextKeyPress",
      ]
    ),

  StickyConfigJson = {
    activation
      | optional
      | StickyActivationJson_,
    release
      | optional
      | StickyReleaseJson_,
  },

  TapDanceConfigJson = {
    timeout | optional | Number,
  },

  TapHoldInterruptResponseJson =
    std.contract.from_validator (
      validators.is_elem_of [
        "HoldOnKeyPress",
        "HoldOnKeyTap",
        "Ignore",
      ]
    ),

  TapHoldConfigJson = {
    timeout | optional | Number,
    interrupt_response | optional | TapHoldInterruptResponseJson,
    required_idle_time | optional | Number,
  },

  ConfigJson = {
    chorded | optional | ChordedConfigJson,
    sticky | optional | StickyConfigJson,
    tap_dance | optional | TapDanceConfigJson,
    tap_hold | optional | TapHoldConfigJson,
  },

  KeymapJson
    | doc "Contract for the keymap.json value"
    = {
      keys | Array key.JsonValue,
      config | optional | ConfigJson,
      ..
    },

  json_keymap
    | doc "The 'JSON' value of the keymap. e.g. imported from keymap.json."
    | KeymapJson,

  key_codegen_values =
    json_keymap.keys
    |> std.array.map key.codegen_values,

  key_data_and_refs =
    key_codegen_values
    |> std.array.fold_left
      (fun { key_data, key_refs = refs } cv =>
        let { key_data = kd, ref = r } = key.data_and_ref key_data cv in
        {
          key_data = kd,
          key_refs = refs @ [r],
        }
      )
      { key_data = {}, key_refs = [] }
    |> (fun { key_data, key_refs = refs } =>
      {
        include key_data,
        key_refs = refs |> std.array.map composite.ref.wrap,
      }
    ),

  # DocstringKeymap in cucumber keymap tests.
  json_deserializable_keymap = {
    config = json_keymap.config,
    key_refs =
      key_data_and_refs.key_refs |> std.array.map (fun { json, .. } => json),
    key_data = key_data_and_refs.key_data |> std.record.map_values (std.array.map (fun { json, .. } => json)),
  },

  keyboard_modifiers = {
    JsonValue = std.contract.from_validator json_value_validator,

    json_value_validator = validators.is_number,

    is_json_value = fun k => 'Ok == json_value_validator k,

    expr = fun b => "smart_keymap::key::KeyboardModifiers::from_byte(%{std.to_string b})",
  },

  checks.check_keyboard = {
    # Use a JSON value to check the "is" predicate.
    check_json_is =
      let json_value = { key_code = 4 } in
      keyboard.is_json_value json_value,
    check_json_with_modifier_is =
      let json_value = { key_code = 4, modifiers = 1 } in
      keyboard.is_json_value json_value,
    check_json_modifiers_is =
      let json_value = { modifiers = 1 } in
      keyboard.is_json_value json_value,

    check_keyboard_codegen_values = {
      check_basic =
        let k = { key_code = 4 } in
        let cv = keyboard.codegen_values k in
        {
          check_key_type = {
            actual = cv.key_type,
            expected = {
              key_type = "smart_keymap::key::keyboard::Key",
            },
          },
          check_rust_expr = {
            actual = cv.rust_expr,
            expected = "smart_keymap::key::keyboard::Key::new(4)",
          },
        },
    },
  },

  keyboard
    | doc "for key::keyboard::Key."
    = {
      JsonValue = std.contract.from_validator json_value_validator,

      module = "smart_keymap::key::keyboard",

      # c.f. doc_de_keyboard.md.
      json_value_validator =
        validators.record.validator {
          fields_validator =
            validators.all_of [
              validators.record.has_any_field_of ["key_code", "modifiers"],
              validators.record.has_only_fields ["key_code", "modifiers"],
            ],
          field_validators = {
            key_code = validators.is_number,
            modifiers = keyboard_modifiers.json_value_validator,
          },
        },

      is_json_value = fun k => 'Ok == json_value_validator k,

      codegen_values = fun k =>
        {
          key_type = {
            key_type = "%{module}::Key",
          },
          rust_expr =
            k
            |> match {
              { key_code } =>
                "smart_keymap::key::keyboard::Key::new(%{std.to_string key_code})",
              { modifiers } =>
                "smart_keymap::key::keyboard::Key::from_modifiers(%{keyboard_modifiers.expr modifiers})",
              { key_code, modifiers } =>
                "smart_keymap::key::keyboard::Key::new_with_modifiers(%{std.to_string key_code}, %{keyboard_modifiers.expr modifiers})",
            },
          json_value = k,
        },

      traverse = fun f acc cv => f acc cv,

      data_and_ref = fun key_data cv @ { json_value = k, .. } =>
        let data_ = key_data in
        k
        |> match {
          { key_code } =>
            {
              key_data = data_,
              ref = {
                include module,
                variant = "KeyCode",
                json = { "%{variant}" = key_code },
                rust_expr = "%{module}::Ref::%{variant}(%{std.to_string key_code})",
              },
            },
          { modifiers } =>
            {
              key_data = data_,
              ref = {
                include module,
                variant = "Modifiers",
                json = { "%{variant}" = modifiers },
                rust_expr = "%{module}::Ref::%{variant}(%{std.to_string modifiers})",
              },
            },
          { key_code, modifiers } =>
            let { keyboard = keyboard_, ..other_data } = data_ & { keyboard | default = [] } in
            let new_index = std.array.length keyboard_ in
            let new_key = {
              json = {
                include key_code,
                include modifiers,
              },
              rust_expr = m%"
                %{module}::Key {
                  key_code: %{std.to_string key_code},
                  modifiers: %{keyboard_modifiers.expr modifiers},
                }
              "%,
            }
            in
            {
              key_data = other_data & { keyboard = std.array.append new_key keyboard_ },
              ref = {
                include module,
                variant = "KeyCodeAndModifier",
                json = { "%{variant}" = new_index },
                rust_expr = "%{module}::Ref::%{variant}(%{std.to_string new_index})",
              },
            },
        },
    },

  caps_word
    | doc "for key::caps_word::Key."
    = {
      JsonValue = std.contract.from_validator json_value_validator,

      module = "smart_keymap::key::caps_word",

      json_value_validator = fun k =>
        k
        |> match {
          "ToggleCapsWord" => 'Ok,
          _ => 'Error { message = "Expected \"ToggleCapsWord\"" },
        },

      is_json_value = fun k => 'Ok == json_value_validator k,

      codegen_values = fun k =>
        {
          key_type = {
            key_type = "%{module}::Key",
          },
          rust_expr = "smart_keymap::key::caps_word::Key::new()",
          json_value = k,
        },

      traverse = fun f acc cv => f acc cv,
    },

  checks.layer_modifier_is = {
    # Use a json value to check the "is" predicate.
    check_json_is =
      let json_value = { Hold = 0 } in
      layer_modifier.is_json_value json_value
  },

  layer_modifier
    | doc "for key::layered::ModifierKey."
    = {
      JsonValue = std.contract.from_validator json_value_validator,

      module = "smart_keymap::key::layered",

      # c.f. doc_de_layered.md.
      # JSON serialization of key::layered::ModifierKey has variants: Default(layer), Hold(layer), SetActiveLayers([layer, ...]).
      json_value_validator =
        validators.record.validator {
          fields_validator =
            validators.all_of [
              validators.record.has_any_field_of ["Default", "Hold", "SetActiveLayers"],
              validators.record.has_only_fields ["Default", "Hold", "SetActiveLayers"],
            ],
          field_validators = {
            Hold = validators.is_number,
            SetActiveLayers = validators.is_number,
          },
        },

      is_json_value = fun k => 'Ok == json_value_validator k,

      codegen_values = fun k =>
        k
        |> match {
          { Default = layer_index } =>
            {
              key_type = {
                key_type = "%{module}::ModifierKey",
              },
              variant = "Default",
              rust_expr = "%{module}::ModifierKey::default(%{std.to_string layer_index})",
              json_value = k,
            },
          { Hold = layer_index } =>
            {
              key_type = {
                key_type = "%{module}::ModifierKey",
              },
              variant = "Hold",
              rust_expr = "%{module}::ModifierKey::hold(%{std.to_string layer_index})",
              json_value = k,
            },
          { SetActiveLayers = active_layers_bitset } =>
            {
              key_type = {
                key_type = "%{module}::ModifierKey",
              },
              variant = "SetActiveLayers",
              rust_expr = "%{module}::ModifierKey::set_active_layers_from_bitset(%{active_layers_bitset |> std.to_string})",
              json_value = k,
            },
        },

      traverse = fun f acc cv => f acc cv,

      data_and_ref = fun key_data cv @ { json_value = k, .. } =>
        let { layer_modifiers = layer_modifiers_, ..other_data } = key_data & { layer_modifiers | default = [] } in
        let new_index = std.array.length layer_modifiers_ in
        let new_key = {
          json = k,
          rust_expr = cv.rust_expr,
        }
        in
        {
          key_data = other_data & { layer_modifiers = std.array.append new_key layer_modifiers_ },
          ref = {
            include module,
            variant = "Modifier",
            json = { "%{variant}" = new_index },
            rust_expr = "%{module}::Ref::%{variant}(%{std.to_string new_index})",
          },
        },
    },

  keymap_callback
    | doc "for key::callback::Key."
    = {
      JsonValue = std.contract.from_validator json_value_validator,

      module = "key::callback::Key",

      json_value_validator =
        validators.record.validator {
          fields_validator = validators.record.has_exact_fields ["keymap_callback"],
          field_validators = {
            keymap_callback =
              validators.any_of [
                validators.is_string,
                validators.record.validator {
                  fields_validator = validators.record.has_exact_fields ["Custom"],
                  field_validators = {
                    Custom = validators.is_array,
                  },
                }
              ],
          },
        },

      is_json_value = fun k => 'Ok == json_value_validator k,

      codegen_values = fun k @ { keymap_callback } =>
        {
          key_type = {
            key_type = "%{module}::Key",
          },
          rust_expr =
            let cb_expr =
              keymap_callback
              |> match {
                { Custom = [i, j] } =>
                  "smart_keymap::keymap::KeymapCallback::Custom(%{i |> std.to_string}, %{j |> std.to_string})",
                _ => "smart_keymap::keymap::KeymapCallback::%{keymap_callback}"
              }
            in "%{module}::Key::new(%{cb_expr})",
          json_value = k,
        },

      traverse = fun f acc cv => f acc cv,
    },

  sticky
    | doc "for key::sticky::Key."
    = {
      JsonValue = std.contract.from_validator json_value_validator,

      module = "smart_keymap::key::sticky",

      json_value_validator =
        validators.record.validator {
          fields_validator =
            validators.all_of [
              validators.record.has_exact_fields ["sticky_modifiers"],
            ],
          field_validators = {
            sticky_modifiers = keyboard_modifiers.json_value_validator,
          },
        },

      is_json_value = fun k => 'Ok == json_value_validator k,

      codegen_values = fun k =>
        {
          key_type = {
            key_type = "%{module}::Key",
          },
          rust_expr =
            k
            |> match {
              { sticky_modifiers } =>
                "%{module}::Key::new(%{keyboard_modifiers.expr sticky_modifiers})",
            },
          json_value = k,
        },

      traverse = fun f acc cv => f acc cv,
    },

  custom
    | doc "for key::custom::Key."
    = {
      JsonValue = std.contract.from_validator json_value_validator,

      module = "smart_keymap::key::custom",

      json_value_validator =
        validators.record.validator {
          fields_validator = validators.record.has_exact_fields ["custom"],
          field_validators = {
            custom = validators.is_number,
          },
        },

      is_json_value = fun k => 'Ok == json_value_validator k,

      codegen_values = fun k @ { custom } =>
        {
          key_type = {
            key_type = "%{module}::Key",
          },
          rust_expr = "%{module}::Key::new(%{std.to_string custom})",
          json_value = k,
        },

      traverse = fun f acc cv => f acc cv,
    },

  tap_dance
    | doc "for key::tap_dance::Key."
    = {
      JsonValue = std.contract.from_validator json_value_validator,

      module = "smart_keymap::key::tap_dance",

      # JSON serialization of key::tap_dance::Key is { definitions: [Option<K>] }
      #  and the codegen uses the from_definitions(&[K]) constructor.
      json_value_validator =
        validators.record.validator {
          fields_validator = validators.record.has_exact_fields ["definitions"],
          field_validators = {
            definitions = validators.array.validator key.json_value_validator,
          },
        },

      is_json_value = fun k => 'Ok == json_value_validator k,

      codegen_values = fun k @ { definitions } =>
        let definitions_cv = definitions |> std.array.map (fun k => k |> key.codegen_values) in

        {
          nested = {
            definitions = definitions_cv,
          },
          key_type = {
            key_type = "%{module}::Key",
          },
          rust_expr =
            let definitions_expr = nested.definitions |> std.array.map (fun cv => cv.rust_expr) in
            let definitions_slice_expr = "&[%{definitions_expr |> std.string.join ", "}]" in
            "%{module}::Key::from_definitions(%{definitions_slice_expr})",
          json_value = k,
        },

      map_nested = fun f cv @ { nested, ..rest } =>
        rest
        & {
          nested = {
            definitions = std.array.map f cv.nested.definitions,
          },
        },

      # Traverse by visiting the tap_dance key,
      #  then traversing the definitions.
      traverse = fun f acc cv =>
        let acc = f acc cv in
        std.array.fold_left
          (fun acc nested_cv => key.traverse f acc nested_cv)
          acc
          cv.nested.definitions,
    },

  checks.check_tap_hold = {
    # Use a json value to check the "is" predicate.
    check_json_tap_keyboard_hold_keyboard_is =
      let json_value = {
        hold = { key_code = 224 },
        tap = { key_code = 4 },
      }
      in
      tap_hold.is_json_value json_value,

    check_taphold_codegen_values = {
      # Current impl: tap_hold::Key<keyboard::Key> is still th::Key<composite::Base>
      check_taphold_keyboard =
        let k = {
          hold = { key_code = 224 },
          tap = { key_code = 4 },
        }
        in
        let cv = tap_hold.codegen_values k in
        {
          check_rust_expr = {
            actual = cv.rust_expr,
            expected = "smart_keymap::key::tap_hold::Key::new(smart_keymap::key::keyboard::Key::new(4), smart_keymap::key::keyboard::Key::new(224))",
          },
        },
    },
  },

  tap_hold
    | doc "for key::tap_hold::Key."
    = {
      JsonValue = std.contract.from_validator json_value_validator,

      module = "smart_keymap::key::tap_hold",

      # c.f. doc_de_tap_hold.md.
      # JSON serialization of key::tap_hold::Key is { tap: key, hold: key }
      json_value_validator =
        validators.record.validator {
          fields_validator = validators.record.has_exact_fields ["tap", "hold"],
          field_validators = {
            tap = key.json_value_validator,
            hold = key.json_value_validator,
          },
        },

      is_json_value = fun k => 'Ok == json_value_validator k,

      codegen_values = fun k @ { hold, tap } =>
        let hold_cv = hold |> key.codegen_values in
        let tap_cv = tap |> key.codegen_values in

        {
          nested = {
            tap = tap_cv,
            hold = hold_cv,
          },
          key_type = {
            key_type = "%{module}::Key",
          },
          rust_expr =
            let hold_expr = nested.hold.rust_expr in
            let tap_expr = nested.tap.rust_expr in
            "%{module}::Key::new(%{tap_expr}, %{hold_expr})",
          json_value = k,
        },

      map_nested = fun f cv @ { nested, ..rest } =>
        rest
        & {
          nested = {
            tap = f cv.nested.tap,
            hold = f cv.nested.hold,
          },
        },

      # Traverse by visiting the tap_hold key,
      #  then traversing the tap, hold keys.
      traverse = fun f acc cv =>
        let acc = f acc cv in
        let acc = key.traverse f acc cv.nested.tap in
        key.traverse f acc cv.nested.hold,

      data_and_ref = fun key_data cv @ { nested = { tap = tap_cv, hold = hold_cv }, .. } =>
        let { key_data, ref = tap_ref } = key.data_and_ref key_data tap_cv in
        let tap_ref = tap_ref |> composite.ref.wrap in
        let { key_data, ref = hold_ref } = key.data_and_ref key_data hold_cv in
        let hold_ref = hold_ref |> composite.ref.wrap in
        let data_ = key_data in
        let { tap_hold = tap_hold_, ..other_data } = data_ & { tap_hold | default = [] } in
        let new_index = std.array.length tap_hold_ in
        let new_key = {
          json = {
            tap = tap_ref.json,
            hold = hold_ref.json,
          },
          rust_expr = m%"
            %{module}::Key {
              tap: %{tap_ref.rust_expr},
              hold: %{hold_ref.rust_expr},
            }
          "%,
        }
        in
        {
          key_data = other_data & { tap_hold = std.array.append new_key tap_hold_ },
          ref = {
            include module,
            json = new_index,
            rust_expr = "%{module}::Ref(%{std.to_string new_index})",
          },
        },
    },

  checks.check_layered = {
    # Use a json value to check the "is" predicate.
    check_json_base_keyboard_layered_keyboard_is =
      let json_value = {
        base = { key_code = 4 },
        layered = [
          null,
          { key_code = 6 },
        ]
      }
      in
      layered.is_json_value json_value,

    check_layered_codegen_values = {
      # A layered::LayeredKey of keyboard::Key
      #  codegens to layered::LayeredKey<keyboard::Key>
      check_layered_keyboard =
        let k = {
          base = { key_code = 4 },
          layered = [
            null,
            { key_code = 6 },
          ]
        }
        in
        let cv = layered.codegen_values k in
        {
          check_rust_expr = {
            actual = cv.rust_expr,
            expected = "smart_keymap::key::layered::LayeredKey::new(smart_keymap::key::composite::TapHold(smart_keymap::key::keyboard::Key::new(4)), [None,Some(smart_keymap::key::composite::TapHold(smart_keymap::key::keyboard::Key::new(6)))])",
          },
        },

      # A layered::LayeredKey of mix of tap_hold::Key<'keyboard::Key'> and keyboard::Key
      #  codegens to layered::LayeredKey<composite::TapHoldKey<composite::BaseKey>>
      check_layered_composite_taphold_keyboard =
        let k = {
          base = {
            hold = { key_code = 224 },
            tap = { key_code = 4 },
          },
          layered = [
            null,
            { key_code = 6 },
          ]
        }
        in
        let cv = layered.codegen_values k in
        {
          check_rust_expr = {
            actual = cv.rust_expr,
            expected = "smart_keymap::key::layered::LayeredKey::new(smart_keymap::key::composite::TapHoldKey::TapHold(smart_keymap::key::tap_hold::Key::new(smart_keymap::key::keyboard::Key::new(4), smart_keymap::key::keyboard::Key::new(224))), [None,Some(smart_keymap::key::composite::TapHoldKey::Pass(smart_keymap::key::keyboard::Key::new(6)))])",
          },
        },
    },
  },

  layered
    | doc "for key::layered::LayeredKey."
    = {
      JsonValue = std.contract.from_validator json_value_validator,

      module = "smart_keymap::key::layered",

      # c.f. doc_de_layered.md.
      # e.g.:
      # ```
      #   {
      #     "base": 4,
      #     "layered": [5, null, 7]
      #   }
      # ```
      # JSON serialization of key::layered::Layered has fields: { base, layered }
      json_value_validator =
        validators.record.validator {
          fields_validator = validators.record.has_exact_fields ["base", "layered"],
          field_validators = {
            base = key.json_value_validator,
            layered =
              validators.array.validator (
                validators.any_of [
                  validators.is_null,
                  key.json_value_validator,
                ]
              ),
          },
        },

      is_json_value = fun k => 'Ok == json_value_validator k,

      codegen_values = fun k @ { base, layered } =>
        let base_cv = base |> key.codegen_values in
        let non_null_layered = std.array.filter ((!=) null) layered in
        let layered_cvs =
          std.array.map (fun k => if k != null then k |> key.codegen_values else null) layered
        in
        let nn_layered_cvs =
          std.array.filter ((!=) null) layered_cvs
        in

        {
          nested = {
            base = base_cv,
            layered = layered_cvs,
          },
          key_type = {
            key_type = "%{module}::LayeredKey",
          },
          rust_expr =
            let base_expr = nested.base.rust_expr in
            let layered_exprs =
              nested.layered
              |> std.array.map (fun cv => if cv == null then "None" else "Some(%{cv.rust_expr})")
              |> std.string.join ","
            in
            "smart_keymap::key::layered::LayeredKey::new(%{base_expr}, [%{layered_exprs}])",
          json_value = k,
        },

      map_nested = fun f cv @ { nested, ..rest } =>
        rest
        & {
          nested = {
            base = f cv.nested.base,
            layered = std.array.map (fun cv => if cv != null then f cv else null) cv.nested.layered,
          },
        },

      # Traverse by visiting the layered key itself,
      #  then traversing the base, non-null nested layered keys.
      traverse = fun f acc cv =>
        let acc = f acc cv in
        let acc = key.traverse f acc cv.nested.base in
        let nn_layered_cvs =
          std.array.filter ((!=) null) cv.nested.layered
        in
        std.array.fold_left
          (fun acc nested_cv => key.traverse f acc nested_cv)
          acc
          nn_layered_cvs,

      data_and_ref = fun key_data cv @ { nested = { base = base_cv, layered = layered_cvs }, .. } =>
        let { key_data, ref = base_ref } = key.data_and_ref key_data base_cv in
        let base_ref = base_ref |> composite.ref.wrap in
        let { key_data, layered_refs } =
          std.array.fold_left
            (fun { key_data, layered_refs = parts } cv =>
              if cv == null then
                { include key_data, layered_refs = std.array.append null parts }
              else
                let { key_data, ref = layer_ref } = key.data_and_ref key_data cv in
                let layer_ref = layer_ref |> composite.ref.wrap in
                { include key_data, layered_refs = std.array.append layer_ref parts }
            )
            { include key_data, layered_refs = [] }
            layered_cvs
        in
        let { layered = layered_, ..other_data } = key_data & { layered | default = [] } in
        let new_index = std.array.length layered_ in
        let new_key = {
          json = {
            base = base_ref.json,
            layered = layered_refs |> std.array.map (fun layer_ref => if layer_ref == null then null else layer_ref.json),
          },
          rust_expr = m%"
            %{module}::LayeredKey::new(
              %{base_ref.rust_expr},
              [%{
                layered_refs
                |> std.array.map (fun layer_ref => if layer_ref == null then "None" else "Some(%{layer_ref.rust_expr})")
                |> std.string.join ", "
              }],
            )
          "%,
        }
        in
        {
          key_data = other_data & { layered = std.array.append new_key layered_ },
          ref = {
            include module,
            variant = "Layered",
            json = { "%{variant}" = new_index },
            rust_expr = "%{module}::Ref::%{variant}(%{std.to_string new_index})",
          },
        },
    },

  chorded
    | doc "for key::chorded::Key."
    = {
      JsonValue = std.contract.from_validator json_value_validator,

      module = "smart_keymap::key::chorded",

      # JSON serialization of key::chorded::Key is { chords: [[id, key]], passthrough: key }
      json_value_validator =
        validators.record.validator {
          fields_validator = validators.record.has_exact_fields ["chords", "passthrough"],
          field_validators = {
            chords =
              validators.array.validator (validators.tuple.validator [validators.is_number, key.json_value_validator]),
            passthrough = key.json_value_validator,
          },
        },

      is_json_value = fun k => 'Ok == json_value_validator k,

      map_chords = fun f chords => chords |> std.array.map (fun [id, ch_k] => [id, ch_k |> f]),

      codegen_values = fun k @ { chords, passthrough } =>
        let chords_cv = chords |> map_chords (fun ch_k => ch_k |> key.codegen_values) in
        let passthrough_cv = passthrough |> key.codegen_values in

        {
          nested = {
            chords = chords_cv,
            passthrough = passthrough_cv,
          },
          key_type = {
            key_type = "%{module}::Key",
          },
          rust_expr =
            let chords_expr = m%"&[%{
              nested.chords
              |> std.array.map (fun [i, ch_cv] => "(%{i |> std.to_string}, %{ch_cv.rust_expr})")
              |> std.string.join ", "
            }]"%
            in
            let passthrough_expr = nested.passthrough.rust_expr in
            "smart_keymap::key::chorded::Key::new(%{chords_expr}, %{passthrough_expr})",
          json_value = k,
        },

      map_nested = fun f cv @ { nested, ..rest } =>
        rest
        & {
          nested = {
            chords = cv.nested.chords |> map_chords f,
            passthrough = f cv.nested.passthrough,
          },
        },

      # Traverse by visiting the chorded key,
      #  then traversing the chord keys, passthrough keys.
      traverse = fun f acc cv =>
        let acc = f acc cv in
        let chords_cvs = cv.nested.chords |> std.array.map (fun [_, ch_cv] => ch_cv) in
        let acc = std.array.fold_left (fun acc ch_cv => key.traverse f acc ch_cv) acc chords_cvs in
        key.traverse f acc cv.nested.passthrough,

      data_and_ref = fun key_data cv @ { nested = { chords = chords_cvs, passthrough = passthrough_cv }, .. } =>
        let { key_data, chord_indices_and_refs } =
          std.array.fold_left
            (fun { key_data, chord_indices_and_refs = parts } [chord_index, cv] =>
              let { key_data, ref = chord_ref } = key.data_and_ref key_data cv in
              let chord_ref = chord_ref |> composite.ref.wrap in
              { include key_data, chord_indices_and_refs = std.array.append [chord_index, chord_ref] parts }
            )
            { include key_data, chord_indices_and_refs = [] }
            chords_cvs
        in
        let { key_data, ref = passthrough_ref } = key.data_and_ref key_data passthrough_cv in
        let passthrough_ref = passthrough_ref |> composite.ref.wrap in
        let { chorded = chorded_, ..other_data } = key_data & { chorded | default = [] } in
        let new_index = std.array.length chorded_ in
        let new_key = {
          json = {
            chords = chord_indices_and_refs |> map_chords (fun chord_ref => chord_ref.json),
            passthrough = passthrough_ref.json,
          },
          rust_expr = m%"
            %{module}::Key::new(
              &[%{
                chord_indices_and_refs
                |> std.array.map (fun [i, chord_ref] => "(%{std.to_string i}, %{chord_ref.rust_expr})")
                |> std.string.join ", "
              }],
              %{passthrough_ref.rust_expr},
            )
          "%,
        }
        in
        {
          key_data = other_data & { chorded = std.array.append new_key chorded_ },
          ref = {
            include module,
            variant = "Chorded",
            json = { "%{variant}" = new_index },
            rust_expr = "%{module}::Ref::%{variant}(%{std.to_string new_index})",
          },
        },
    },

  chorded_aux
    | doc "for key::chorded::AuxiliaryKey."
    = {
      JsonValue = std.contract.from_validator json_value_validator,

      module = "smart_keymap::key::chorded",

      # JSON serialization of key::chorded::AuxiliaryKey is { passthrough: key }
      json_value_validator =
        validators.record.validator {
          fields_validator = validators.record.has_exact_fields ["passthrough"],
          field_validators = {
            passthrough = key.json_value_validator,
          },
        },

      is_json_value = fun k => 'Ok == json_value_validator k,

      codegen_values = fun k @ { passthrough } =>
        let passthrough_cv = passthrough |> key.codegen_values in

        {
          nested = {
            passthrough = passthrough_cv,
          },
          key_type = {
            key_type = "%{module}::AuxiliaryKey",
          },
          rust_expr =
            let passthrough_expr = nested.passthrough.rust_expr in
            "smart_keymap::key::chorded::AuxiliaryKey::new(%{passthrough_expr})",
          json_value = k,
        },

      map_nested = fun f cv @ { nested, ..rest } =>
        rest
        & {
          nested = {
            passthrough = f cv.nested.passthrough,
          },
        },

      # Traverse by visiting the chorded aux key,
      #  then traversing the passthrough key.
      traverse = fun f acc cv =>
        let acc = f acc cv in
        key.traverse f acc cv.nested.passthrough,

      data_and_ref = fun key_data cv @ { nested = { passthrough = passthrough_cv }, .. } =>
        let { key_data, ref = passthrough_ref } = key.data_and_ref key_data passthrough_cv in
        let passthrough_ref = passthrough_ref |> composite.ref.wrap in
        let { chorded_auxiliary = chorded_auxiliary_, ..other_data } = key_data & { chorded_auxiliary | default = [] } in
        let new_index = std.array.length chorded_auxiliary_ in
        let new_key = {
          json = {
            passthrough = passthrough_ref.json,
          },
          rust_expr = m%"
            %{module}::AuxiliaryKey::new(
              %{passthrough_ref.rust_expr},
            )
          "%,
        }
        in
        {
          key_data = other_data & { chorded_auxiliary = std.array.append new_key chorded_auxiliary_ },
          ref = {
            include module,
            variant = "Auxiliary",
            json = { "%{variant}" = new_index },
            rust_expr = "%{module}::Ref::%{variant}(%{std.to_string new_index})",
          },
        },
    },

  checks.check_composite = {},

  composite = {
    ref = {
      wrap = fun ref @ { module, json, rust_expr, .. } =>
        let variant =
          module
          |> match {
            "smart_keymap::key::keyboard" => "Keyboard",
            "smart_keymap::key::tap_hold" => "TapHold",
            "smart_keymap::key::layered" => "Layered",
            "smart_keymap::key::chorded" => "Chorded",
            _ => std.fail_with "bad ref for composite.ref.wrap: %{std.serialize ref 'Json}",
          }
        in
        {
          include variant,
          module = "smart_keymap::key::composite",
          json = { "%{variant}" = ref.json },
          rust_expr = "%{module}::Ref::%{variant}(%{ref.rust_expr})",
        },
    },
  },

  key = {
    JsonValue = std.contract.from_validator json_value_validator,

    json_value_validator =
      validators.any_of [
        keyboard.json_value_validator,
        caps_word.json_value_validator,
        layer_modifier.json_value_validator,
        keymap_callback.json_value_validator,
        sticky.json_value_validator,
        custom.json_value_validator,
        layered.json_value_validator,
        tap_dance.json_value_validator,
        tap_hold.json_value_validator,
        chorded.json_value_validator,
        chorded_aux.json_value_validator,
      ],

    is_json_value = fun k => 'Ok == json_value_validator k,

    codegen_values = match {
      k if keyboard.is_json_value k => keyboard.codegen_values k,
      k if caps_word.is_json_value k => caps_word.codegen_values k,
      k if layer_modifier.is_json_value k => layer_modifier.codegen_values k,
      k if keymap_callback.is_json_value k => keymap_callback.codegen_values k,
      k if sticky.is_json_value k => sticky.codegen_values k,
      k if custom.is_json_value k => custom.codegen_values k,
      k if layered.is_json_value k => layered.codegen_values k,
      k if tap_dance.is_json_value k => tap_dance.codegen_values k,
      k if tap_hold.is_json_value k => tap_hold.codegen_values k,
      k if chorded.is_json_value k => chorded.codegen_values k,
      k if chorded_aux.is_json_value k => chorded_aux.codegen_values k,
      k => std.fail_with "bad json_value for key: %{k |> std.serialize 'Json}",
    },

    codegen_module_for_key_type = match {
      "smart_keymap::key::keyboard::Key" => keyboard,
      "smart_keymap::key::caps_word::Key" => caps_word,
      "smart_keymap::key::layered::ModifierKey" => layer_modifier,
      "smart_keymap::key::callback::Key" => keymap_callback,
      "smart_keymap::key::sticky::Key" => sticky,
      "smart_keymap::key::custom::Key" => custom,
      "smart_keymap::key::tap_dance::Key" => tap_dance,
      "smart_keymap::key::tap_hold::Key" => tap_hold,
      "smart_keymap::key::layered::LayeredKey" => layered,
      "smart_keymap::key::chorded::Key" => chorded,
      "smart_keymap::key::chorded::AuxiliaryKey" => chorded_aux,
      cv => std.fail_with "bad codegen_values for key.codegen_module_for_codegen_value: %{cv |> std.serialize 'Json}",
    },

    # codegen_module_for_codegen_value = fun cv @ { key_type = { key_type, .. }, .. } =>
      # codegen_module_for_key_type key_type,
    codegen_module_for_codegen_value = fun cv => cv |>
      match {
        { key_type = { key_type, .. }, .. } => codegen_module_for_key_type key_type,
        _ => std.fail_with "bad codegen_values for key.codegen_module_for_codegen_value: %{cv |> std.serialize 'Json}",
      },


    traverse = fun f acc cv =>
      let codegen_module = codegen_module_for_codegen_value cv in
      codegen_module.traverse f acc cv,

    data_and_ref = fun key_data cv =>
      let codegen_module = codegen_module_for_codegen_value cv in
      codegen_module.data_and_ref key_data cv,
  },

  rust_expressions
    | doc "Record of rust expressions generated from keymap.json"
    =
      let { config, keys } =
        json_keymap
        & {
          config | default = {},
        }
      in
      let chorded_config_expr =
        if std.record.has_field "chorded" config then
          let timeout_field_fragment =
            if std.record.has_field "timeout" config.chorded then
              "timeout: %{std.to_string config.chorded.timeout},"
            else
              ""
          in
          let chords_field_fragment =
            if std.record.has_field "chords" config.chorded then
              let chords_fragment =
                config.chorded.chords
                |> std.array.map (fun indices =>
                  "smart_keymap::key::chorded::ChordIndices::from_slice(&[%{indices |> std.array.map std.to_string |> std.string.join ", "}])"
                )
                |> std.string.join ","
              in
              "chords: smart_keymap::slice::Slice::from_slice(&[%{chords_fragment}]),"
            else
              ""
          in
          m%"
          smart_keymap::key::chorded::Config {
              %{timeout_field_fragment}
              %{chords_field_fragment}
              ..smart_keymap::key::chorded::DEFAULT_CONFIG
          }
        "%
        else
          "smart_keymap::key::chorded::DEFAULT_CONFIG"
      in
      let sticky_config_expr =
        if std.record.has_field "sticky" config then
          let activation_field_fragment =
            if std.record.has_field "activation" config.sticky then
              "activation: smart_keymap::key::sticky::StickyKeyActivation::%{config.sticky.activation},"
            else
              ""
          in
          let release_field_fragment =
            if std.record.has_field "release" config.sticky then
              "release: smart_keymap::key::sticky::StickyKeyRelease::%{config.sticky.release},"
            else
              ""
          in
          let config_fragment =
            [
              activation_field_fragment,
              release_field_fragment,
              "..smart_keymap::key::sticky::DEFAULT_CONFIG"
            ]
            |> std.array.filter ((!=) "")
            |> std.string.join "\n"
          in
          m%"
            smart_keymap::key::sticky::Config {
                %{config_fragment}
            }
          "%
        else
          "smart_keymap::key::sticky::DEFAULT_CONFIG"
      in
      let tap_dance_config_expr =
        if std.record.has_field "tap_dance" config then
          let timeout_field_fragment =
            if std.record.has_field "timeout" config.tap_dance then
              "timeout: %{std.to_string config.tap_dance.timeout},"
            else
              ""
          in
          m%"
            smart_keymap::key::tap_dance::Config {
                %{timeout_field_fragment}
                ..smart_keymap::key::tap_dance::DEFAULT_CONFIG
            }
          "%
        else
          "smart_keymap::key::tap_dance::DEFAULT_CONFIG"
      in
      let tap_hold_config_expr =
        if std.record.has_field "tap_hold" config then
          let timeout_field_fragment =
            if std.record.has_field "timeout" config.tap_hold then
              "timeout: %{std.to_string config.tap_hold.timeout},"
            else
              ""
          in
          let interrupt_response_field_fragment =
            if std.record.has_field "interrupt_response" config.tap_hold then
              "interrupt_response: smart_keymap::key::tap_hold::InterruptResponse::%{config.tap_hold.interrupt_response},"
            else
              ""
          in
          let required_idle_time_field_fragment =
            if std.record.has_field "required_idle_time" config.tap_hold then
              "required_idle_time: Some(%{std.to_string config.tap_hold.required_idle_time}),"
            else
              ""
          in
          m%"
            smart_keymap::key::tap_hold::Config {
                %{timeout_field_fragment}
                %{interrupt_response_field_fragment}
                %{required_idle_time_field_fragment}
                ..smart_keymap::key::tap_hold::DEFAULT_CONFIG
            }
          "%
        else
          "smart_keymap::key::tap_hold::DEFAULT_CONFIG"
      in
      let keymap_len = std.array.length keys in
      let max_tap_dance_definitions =
        # For each of the keys,
        #  compute max length of any tap dance definitions.
        let max_td_defs =
          std.array.map
            (fun cv =>
              key.traverse
                (fun acc cv =>
                  cv
                  |> match {
                    { nested = { definitions }, .. } =>
                      let len = std.array.length definitions in
                      if len > acc then len else acc,
                    _ => acc,
                  }
                )
                0
                cv
            )
            key_codegen_values
        in
        # Compute the max over all
        std.array.reduce_left (fun x y => if x > y then x else y) max_td_defs
      in
      let num_layers =
        let max_layered_lengths =
          std.array.map
            (fun cv =>
              key.traverse
                (fun acc cv =>
                  cv
                  |> match {
                    { nested = { layered, .. }, .. } =>
                      let len = std.array.length layered in
                      if len > acc then len else acc,
                    _ => acc,
                  }
                )
                0
                cv
            )
            key_codegen_values
        in
        # Compute the max over all
        std.array.reduce_left (fun x y => if x > y then x else y) max_layered_lengths
      in
      let keys_id = "Keys%{std.to_string keymap_len}" in
      let composite_codegen_values = key_codegen_values in
      let key_types = std.fail_with "TBI: replace old implementation detail" in
      let key_exprs =
        composite_codegen_values
        |> std.array.map (fun k => "%{k.rust_expr},")
        |> std.string.join ""
      in
      let chord_count =
        if std.record.has_field "chorded" config && std.record.has_field "chords" config.chorded then
          config.chorded.chords |> std.array.length
        else
          0
      in
      let max_chord_size =
        if std.record.has_field "chorded" config && std.record.has_field "chords" config.chorded then
          config.chorded.chords |> std.array.map std.array.length |> std.array.fold_left std.number.max 0
        else
          0
      in
      let max_overlapping_chord_size =
        if std.record.has_field "chorded" config && std.record.has_field "chords" config.chorded then
          config.chorded.chords
          |> std.array.map (fun [i, ..] => std.to_string i)
          |> std.array.fold_left
            (fun r i =>
              std.record.update i (1 + (std.record.get_or i 0 r)) r
            )
            {}
          |> std.record.values
          |> std.array.fold_left std.number.max 0
        else
          0
      in
      let { key_data, key_refs } = key_data_and_refs in
      let keyboard_system_expr =
        let keyboard_data = (key_data & { keyboard | default = [] }).keyboard in
        m%"
        smart_keymap::key::keyboard::System::new([
          %{keyboard_data |> std.array.map (fun { rust_expr, .. } => rust_expr) |> std.string.join ", "}
        ])
      "%
      in
      let tap_hold_system_expr =
        let tap_hold_data = (key_data & { tap_hold | default = [] }).tap_hold in
        m%"
        smart_keymap::key::tap_hold::System::new([
          %{tap_hold_data |> std.array.map (fun { rust_expr, .. } => rust_expr) |> std.string.join ", "}
        ])
      "%
      in
      let layered_system_expr =
        let layer_modifiers_data = (key_data & { layer_modifiers | default = [] }).layer_modifiers in
        let layered_data = (key_data & { layered | default = [] }).layered in
        m%"
        smart_keymap::key::layered::System::new(
          [
            %{layer_modifiers_data |> std.array.map (fun { rust_expr, .. } => rust_expr) |> std.string.join ", "}
          ],
          [
            %{layered_data |> std.array.map (fun { rust_expr, .. } => rust_expr) |> std.string.join ", "}
          ],
        )
      "%
      in
      let chorded_system_expr =
        let chorded_key_data = (key_data & { chorded | default = [] }).chorded in
        let auxiliary_key_data = (key_data & { chorded_auxiliary | default = [] }).chorded_auxiliary in
        m%"
        smart_keymap::key::chorded::System::new(
          [
            %{chorded_key_data |> std.array.map (fun { rust_expr, .. } => rust_expr) |> std.string.join ", "}
          ],
          [
            %{auxiliary_key_data |> std.array.map (fun { rust_expr, .. } => rust_expr) |> std.string.join ", "}
          ],
        )
      "%
      in
      let key_system_expr = m%"
        smart_keymap::key::composite::System::array_based(
          %{keyboard_system_expr},
          %{tap_hold_system_expr},
          %{layered_system_expr},
          %{chorded_system_expr},
        )
      "%
      in
      let key_refs_expr = "[%{key_refs |> std.array.map (fun { rust_expr, .. } => rust_expr) |> std.string.join ", "}]" in
      {
        # config = m%"smart_keymap::key::composite::Config {
        #     chorded: %{chorded_config_expr},
        #     sticky: %{sticky_config_expr},
        #     tap_dance: %{tap_dance_config_expr},
        #     tap_hold: %{tap_hold_config_expr},
        #     ..smart_keymap::key::composite::DEFAULT_CONFIG
        # }"%,
        config = m%"smart_keymap::key::composite::Config {
            tap_hold: %{tap_hold_config_expr},
            chorded: %{chorded_config_expr},
            ..smart_keymap::key::composite::DEFAULT_CONFIG
        }"%,
        context = "smart_keymap::key::composite::Context::from_config(%{config})",
        key_definitions = "%{keys_id}::new((%{key_exprs}))",
        key_refs = key_refs_expr,
        keymap = m%"
{
    use smart_keymap::key::composite as key_system;
    use key_system::Ref;
    use key_system::Context;
    use key_system::Event;
    use key_system::KeyState;
    use key_system::PendingKeyState;
    use key_system::System;
    const KEY_COUNT: usize = %{keymap_len};
    const KEY_REFS: [Ref; KEY_COUNT] = %{key_refs};
    const CONTEXT: Context = %{context};

    smart_keymap::keymap::Keymap::new(KEY_REFS, CONTEXT, %{key_system_expr})
}
"%,
        keymap_module = m%"
/// Types and initial data used for constructing [KEYMAP].
pub mod init {
    use crate as smart_keymap;

    /// Config used by tap-hold keys.
    pub const CONFIG: smart_keymap::key::composite::Config = %{config};

    /// Number of layers supported by the [smart_keymap::key::layered] implementation.
    pub const LAYER_COUNT: usize = %{std.to_string num_layers};

    /// The maximum number of keys in a chord.
    pub const MAX_CHORD_SIZE: usize = %{max_chord_size |> std.to_string};

    /// The maximum number of chords.
    pub const MAX_CHORDS: usize = %{chord_count |> std.to_string};

    /// The maximum number of overlapping chords for a chorded key.
    pub const MAX_OVERLAPPING_CHORD_SIZE: usize = %{max_overlapping_chord_size |> std.to_string};

    /// The tap-dance definitions.
    pub const MAX_TAP_DANCE_DEFINITIONS: usize = %{max_tap_dance_definitions |> std.to_string};

    pub use smart_keymap::key::composite::Context;

    pub use smart_keymap::key::composite::Event;

    pub use smart_keymap::key::composite::PendingKeyState;

    pub use smart_keymap::key::composite::KeyState;

    pub use smart_keymap::key::composite::Key;

    /// Initial [Context] value.
    pub const CONTEXT: Context = %{context};

    smart_keymap::tuples::define_keys!(%{std.to_string keymap_len});

    /// Alias for a [tuples] KeysN type, as generated by keymap.ncl.
    pub type KeyDefinitionsType = %{keys_id}<
        %{key_types},
        Context,
        Event,
        PendingKeyState,
        KeyState,
    >;

    /// Alias for the [keymap::Keymap] type.
    pub type Keymap = smart_keymap::keymap::Keymap<Context, Event, PendingKeyState, KeyState, KeyDefinitionsType>;

    /// A [tuples] KeysN value with keys, as generated by keymap.ncl.
    pub const KEY_DEFINITIONS: KeyDefinitionsType = %{key_definitions};
}
"%,
      },

  keymap_rs
    | doc "Text contents of the keymap.rs generated from the keymap.json"
    = rust_expressions.keymap_module,
}
