let validators = import "validators.ncl" in
{
  ChordIndicesJson = Array Number,

  ChordedConfigJson = {
    chords | optional | Array ChordIndicesJson,
    timeout | optional | Number,
  },

  StickyActivationJson =
    std.contract.from_validator (
      validators.is_elem_of [
        "OnStickyKeyRelease"
      ]
    ),

  StickyReleaseJson =
    std.contract.from_validator (
      validators.is_elem_of [
        "OnModifiedKeyRelease",
        "OnNextKeyPress",
      ]
    ),

  StickyConfigJson = {
    activation
      | optional
      | StickyActivationJson,
    release
      | optional
      | StickyReleaseJson,
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

  # Given an array of codegen_values,
  #  return the 'wrapper type' that unifies all the values.
  #
  # e.g.:
  # - unified_ [kbd::Key, kbd::Key] => { base = kbd::Key },
  # - unified_ [kbd::Key, layered::ModKey] => { base = comp::BaseKey },
  unified_key_impl_for_codegen_values = fun codegen_values =>
    let key_impls = std.array.map (fun { key_impl, .. } => key_impl) codegen_values in
    let impls_for = fun field =>
      key_impls
      |> std.array.map (fun ki => if std.record.has_field field ki then ki."%{field}" else false)
      |> std.array.filter ((!=) false)
    in
    let base_impls = impls_for "base" in
    let tap_hold_impls = impls_for "tap_hold" in
    let layered_impls = impls_for "layered" in
    let chorded_impls = impls_for "chorded" in
    let impl_for = fun field aggr impls =>
      (
        impls
        |> match {
          [] => {},
          [x, ..xs] =>
            {
              "%{field}" =
                if std.array.all ((==) x) xs then
                  # If all the impls are the same (e.g. all are keyboard::Key),
                  #  then can use that as the impl.
                  x
                else
                  # Otherwise, use the aggregating type. (e.g. composite::BaseKey)
                  aggr,
            },
        }
      )
    in
    (impl_for "base" "crate::key::composite::BaseKey" base_impls)
    & (impl_for "tap_hold" "crate::key::composite::TapHoldKey" tap_hold_impls)
    & (impl_for "layered" "crate::key::composite::LayeredKey" layered_impls)
    & (impl_for "chorded" "crate::key::composite::ChordedKey" chorded_impls),

  keyboard_modifiers = {
    JsonValue = std.contract.from_validator json_value_validator,

    json_value_validator = validators.is_number,

    is_json_value = fun k => 'Ok == json_value_validator k,

    expr = fun b => "crate::key::KeyboardModifiers::from_byte(%{std.to_string b})",
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
              as_rust_type_path = "crate::key::keyboard::Key",
              key_type = "crate::key::keyboard::Key",
            },
          },
          check_rust_expr = {
            actual = cv.rust_expr,
            expected = "crate::key::keyboard::Key::new(4)",
          },
        },
    },
  },

  keyboard
    | doc "for key::keyboard::Key."
    = {
      JsonValue = std.contract.from_validator json_value_validator,

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
            key_type = "crate::key::keyboard::Key",
            as_rust_type_path = key_type,
          },
          key_impl = {
            base = key_type.key_type,
          },
          rust_expr =
            k
            |> match {
              { key_code } =>
                "crate::key::keyboard::Key::new(%{std.to_string key_code})",
              { modifiers } =>
                "crate::key::keyboard::Key::from_modifiers(%{keyboard_modifiers.expr modifiers})",
              { key_code, modifiers } =>
                "crate::key::keyboard::Key::new_with_modifiers(%{std.to_string key_code}, %{keyboard_modifiers.expr modifiers})",
            },
          json_value = k,
        },

      lift_to = fun key_impl cv =>
        if key_impl.base == "crate::key::composite::BaseKey" then
          cv |> composite.base_key.keyboard
        else
          cv,

      traverse = fun f acc cv => f acc cv,
    },

  caps_word
    | doc "for key::caps_word::Key."
    = {
      JsonValue = std.contract.from_validator json_value_validator,

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
            key_type = "crate::key::caps_word::Key",
            as_rust_type_path = key_type,
          },
          key_impl = {
            base = key_type.key_type,
          },
          rust_expr = "crate::key::caps_word::Key::new()",
          json_value = k,
        },

      lift_to = fun key_impl cv =>
        if key_impl.base == "crate::key::composite::BaseKey" then
          cv |> composite.base_key.caps_word
        else
          cv,

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

      # c.f. doc_de_layered.md.
      # JSON serialization of key::layered::ModifierKey has variants: Default(layer), Hold(layer).
      json_value_validator =
        validators.record.validator {
          fields_validator =
            validators.all_of [
              validators.record.has_any_field_of ["Default", "Hold"],
              validators.record.has_only_fields ["Default", "Hold"],
            ],
          field_validators = {
            Hold = validators.is_number,
          },
        },

      is_json_value = fun k => 'Ok == json_value_validator k,

      codegen_values = fun k =>
        k
        |> match {
          { Default = layer_index } =>
            {
              key_type = {
                key_type = "crate::key::layered::ModifierKey",
                as_rust_type_path = key_type,
              },
              key_impl = {
                base = key_type.key_type,
              },
              rust_expr = "crate::key::layered::ModifierKey::default(%{std.to_string layer_index})",
              json_value = k,
            },
          { Hold = layer_index } =>
            {
              key_type = {
                key_type = "crate::key::layered::ModifierKey",
                as_rust_type_path = key_type,
              },
              key_impl = {
                base = key_type.key_type,
              },
              rust_expr = "crate::key::layered::ModifierKey::hold(%{std.to_string layer_index})",
              json_value = k,
            },
        },

      lift_to = fun key_impl cv =>
        if key_impl.base == "crate::key::composite::BaseKey" then
          cv |> composite.base_key.layer_modifier
        else
          cv,

      traverse = fun f acc cv => f acc cv,
    },

  keymap_callback
    | doc "for key::callback::Key."
    = {
      JsonValue = std.contract.from_validator json_value_validator,

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
            key_type = "crate::key::callback::Key",
            as_rust_type_path = key_type,
          },
          key_impl = {
            base = key_type.key_type,
          },
          rust_expr =
            let cb_expr =
              keymap_callback
              |> match {
                { Custom = [i, j] } =>
                  "crate::keymap::KeymapCallback::Custom(%{i |> std.to_string}, %{j |> std.to_string})",
                _ => "crate::keymap::KeymapCallback::%{keymap_callback}"
              }
            in "crate::key::callback::Key::new(%{cb_expr})",
          json_value = k,
        },

      lift_to = fun key_impl cv =>
        if key_impl.base == "crate::key::composite::BaseKey" then
          cv |> composite.base_key.keymap_callback
        else
          cv,

      traverse = fun f acc cv => f acc cv,
    },

  sticky
    | doc "for key::sticky::Key."
    = {
      JsonValue = std.contract.from_validator json_value_validator,

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
            key_type = "crate::key::sticky::Key",
            as_rust_type_path = key_type,
          },
          key_impl = {
            base = key_type.key_type,
          },
          rust_expr =
            k
            |> match {
              { sticky_modifiers } =>
                "crate::key::sticky::Key::new(%{keyboard_modifiers.expr sticky_modifiers})",
            },
          json_value = k,
        },

      lift_to = fun key_impl cv =>
        if key_impl.base == "crate::key::composite::BaseKey" then
          cv |> composite.base_key.sticky
        else
          cv,

      traverse = fun f acc cv => f acc cv,
    },

  custom
    | doc "for key::custom::Key."
    = {
      JsonValue = std.contract.from_validator json_value_validator,

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
            key_type = "crate::key::custom::Key",
            as_rust_type_path = key_type,
          },
          key_impl = {
            base = key_type.key_type,
          },
          rust_expr = "crate::key::custom::Key::new(%{std.to_string custom})",
          json_value = k,
        },

      lift_to = fun key_impl cv =>
        if key_impl.base == "crate::key::composite::BaseKey" then
          cv |> composite.base_key.custom
        else
          cv,

      traverse = fun f acc cv => f acc cv,
    },

  # Related to key::composite::TapHoldNestable trait.
  tap_hold_nestable = {
    # Wraps the codegen values for
    #   - key::keyboard::Key
    #   - key::layered::ModifierKey
    #   - key::caps_word::Key
    #   - key::callback::Key
    #   - key::sticky::Key
    #   - key::custom::Key
    #
    # In particular, the returned value is a codegen values record
    #  for an item which implements key::Key with associated
    #  PressedKey = key::composite::BasePressedKey.
    wrap = fun cv @ { key_type = { key_type, .. }, .. } =>
      key_type
      |> match {
        "crate::key::keyboard::Key" => cv,
        "crate::key::layered::ModifierKey" => cv,
        "crate::key::caps_word::Key" => cv,
        "crate::key::callback::Key" => cv,
        "crate::key::sticky::Key" => cv,
        "crate::key::custom::Key" => cv,
        cv => std.fail_with "bad codegen_values for tap_hold_nestable.wrap: %{cv |> std.serialize 'Json}",
      },
  },

  tap_dance
    | doc "for key::tap_dance::Key."
    = {
      JsonValue = std.contract.from_validator json_value_validator,

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
        let definitions_cv = definitions |> std.array.map (fun k => k |> key.codegen_values |> tap_hold_nestable.wrap) in

        let target_type = unified_key_impl_for_codegen_values definitions_cv in

        {
          nested = {
            definitions = definitions_cv |> std.array.map (fun cv => cv |> key.lift_to target_type),
          },
          key_type = {
            key_type = "crate::key::tap_dance::Key",
            nested_key_type = (std.array.first nested.definitions).key_type,
            as_rust_type_path = "%{key_type}<%{nested_key_type.as_rust_type_path}>"
          },
          key_impl = {} & (std.array.first nested.definitions).key_impl,
          rust_expr =
            let definitions_expr = nested.definitions |> std.array.map (fun cv => cv.rust_expr) in
            let definitions_slice_expr = "&[%{definitions_expr |> std.string.join ", "}]" in
            "crate::key::tap_dance::Key::from_definitions(%{definitions_slice_expr})",
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
          check_key_type = {
            actual = cv.key_type.as_rust_type_path,
            expected = "crate::key::tap_hold::Key<crate::key::keyboard::Key>",
          },
          check_rust_expr = {
            actual = cv.rust_expr,
            expected = "crate::key::tap_hold::Key::new(crate::key::keyboard::Key::new(4), crate::key::keyboard::Key::new(224))",
          },
        },
    },
  },

  tap_hold
    | doc "for key::tap_hold::Key."
    = {
      JsonValue = std.contract.from_validator json_value_validator,

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
        let hold_cv = hold |> key.codegen_values |> tap_hold_nestable.wrap in
        let tap_cv = tap |> key.codegen_values |> tap_hold_nestable.wrap in

        let target_type = unified_key_impl_for_codegen_values [hold_cv, tap_cv] in

        {
          nested = {
            tap = tap_cv |> key.lift_to target_type,
            hold = hold_cv |> key.lift_to target_type,
          },
          key_type = {
            key_type = "crate::key::tap_hold::Key",
            nested_key_type = nested.tap.key_type,
            as_rust_type_path = "%{key_type}<%{nested_key_type.as_rust_type_path}>"
          },
          key_impl = {} & nested.tap.key_impl,
          rust_expr =
            let hold_expr = nested.hold.rust_expr in
            let tap_expr = nested.tap.rust_expr in
            "crate::key::tap_hold::Key::new(%{tap_expr}, %{hold_expr})",
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
    },

  # Related to key::composite::LayeredNestable trait.
  layered_nestable = {
    # Wraps the codegen values for
    #   - key::tap_dance::Key.
    #   - key::tap_hold::Key.
    #   - TapHoldNestable impls.
    #     - e.g. key::keyboard::Key, key::layered::ModifierKey, etc.
    #
    # In particular, the returned value is a codegen values record
    #  for an item which implements key::Key with associated
    #  PressedKey = key::composite::TapHoldPressedKey.
    wrap = fun cv @ { key_type = { key_type, .. }, .. } =>
      key_type
      |> match {
        "crate::key::tap_dance::Key" =>
          cv |> composite.tap_hold_key.tap_dance_,
        "crate::key::tap_hold::Key" =>
          cv |> composite.tap_hold_key.tap_hold_,
        _ =>
          cv |> tap_hold_nestable.wrap |> composite.tap_hold_.wrap,
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
          check_key_type = {
            actual = cv.key_type.as_rust_type_path,
            expected = "crate::key::layered::LayeredKey<crate::key::composite::TapHold<crate::key::keyboard::Key>>",
          },
          check_rust_expr = {
            actual = cv.rust_expr,
            expected = "crate::key::layered::LayeredKey::new(crate::key::composite::TapHold(crate::key::keyboard::Key::new(4)), [None,Some(crate::key::composite::TapHold(crate::key::keyboard::Key::new(6)))])",
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
          check_key_type = {
            actual = cv.key_type.as_rust_type_path,
            expected = "crate::key::layered::LayeredKey<crate::key::composite::TapHoldKey<crate::key::keyboard::Key>>",
          },
          check_rust_expr = {
            actual = cv.rust_expr,
            expected = "crate::key::layered::LayeredKey::new(crate::key::composite::TapHoldKey::TapHold(crate::key::tap_hold::Key::new(crate::key::keyboard::Key::new(4), crate::key::keyboard::Key::new(224))), [None,Some(crate::key::composite::TapHoldKey::Pass(crate::key::keyboard::Key::new(6)))])",
          },
        },
    },
  },

  layered
    | doc "for key::layered::LayeredKey."
    = {
      JsonValue = std.contract.from_validator json_value_validator,

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
        let base_cv = base |> key.codegen_values |> layered_nestable.wrap in
        let non_null_layered = std.array.filter ((!=) null) layered in
        let layered_cvs =
          std.array.map (fun k => if k != null then k |> key.codegen_values |> layered_nestable.wrap else null) layered
        in
        let nn_layered_cvs =
          std.array.filter ((!=) null) layered_cvs
        in

        let target_type = unified_key_impl_for_codegen_values ([base_cv] @ nn_layered_cvs) in

        {
          nested = {
            base = base_cv |> key.lift_to target_type,
            layered =
              layered_cvs
              |> std.array.map (fun cv => if cv != null then cv |> key.lift_to target_type else null),
          },
          key_type = {
            key_type = "crate::key::layered::LayeredKey",
            nested_key_type = nested.base.key_type,
            as_rust_type_path = "%{key_type}<%{nested_key_type.as_rust_type_path}>",
          },
          key_impl = {} & nested.base.key_impl,
          rust_expr =
            let base_expr = nested.base.rust_expr in
            let layered_exprs =
              nested.layered
              |> std.array.map (fun cv => if cv == null then "None" else "Some(%{cv.rust_expr})")
              |> std.string.join ","
            in
            "crate::key::layered::LayeredKey::new(%{base_expr}, [%{layered_exprs}])",
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
    },

  # Related to key::composite::ChordedNestable trait.
  chorded_nestable = {
    # Wraps the codegen values for
    #   - key::layered::LayeredKey.
    #   - LayeredNestable impls.
    #     - e.g. key::tap_hold::Key,
    #     - TapHoldNestable impls.
    #       - e.g. key::keyboard::Key, key::layered::ModifierKey, etc.
    #
    # In particular, the returned value is a codegen values record
    #  for an item which implements key::Key with associated
    #  PressedKey = key::composite::ChordedPressedKey.
    wrap = fun cv @ { key_type = { key_type, .. }, .. } =>
      key_type
      |> match {
        "crate::key::layered::LayeredKey" =>
          cv |> composite.layered_key.layered_,
        _ =>
          cv |> layered_nestable.wrap |> composite.layered_.wrap,
      },
  },

  chorded
    | doc "for key::chorded::Key."
    = {
      JsonValue = std.contract.from_validator json_value_validator,

      # JSON serialization of key::chorded::Key is { chord: key, passthrough: key }
      json_value_validator =
        validators.record.validator {
          fields_validator = validators.record.has_exact_fields ["chord", "passthrough"],
          field_validators = {
            chord = key.json_value_validator,
            passthrough = key.json_value_validator,
          },
        },

      is_json_value = fun k => 'Ok == json_value_validator k,

      codegen_values = fun k @ { chord, passthrough } =>
        let chord_cv = chord |> key.codegen_values |> chorded_nestable.wrap in
        let passthrough_cv = passthrough |> key.codegen_values |> chorded_nestable.wrap in

        let target_type = unified_key_impl_for_codegen_values [chord_cv, passthrough_cv] in

        {
          nested = {
            chord = chord_cv |> key.lift_to target_type,
            passthrough = passthrough_cv |> key.lift_to target_type,
          },
          key_type = {
            key_type = "crate::key::chorded::Key",
            nested_key_type = nested.chord.key_type,
            as_rust_type_path = "%{key_type}<%{nested_key_type.as_rust_type_path}>"
          },
          key_impl =
            nested.passthrough.key_impl
            & {
              chorded = key_type.key_type,
            },
          rust_expr =
            let chord_expr = nested.chord.rust_expr in
            let passthrough_expr = nested.passthrough.rust_expr in
            "crate::key::chorded::Key::new(%{chord_expr}, %{passthrough_expr})",
          json_value = k,
        },

      map_nested = fun f cv @ { nested, ..rest } =>
        rest
        & {
          nested = {
            chord = f cv.nested.chord,
            passthrough = f cv.nested.passthrough,
          },
        },

      # Since key::chorded::Key is a key::Key impl. for keymap::Keymap,
      #  we want a lift_to fn
      lift_to = fun key_impl cv =>
        chorded.map_nested (fun cv_ => key.lift_to key_impl cv_) cv,

      # Traverse by visiting the chorded key,
      #  then traversing the chord, passthrough keys.
      traverse = fun f acc cv =>
        let acc = f acc cv in
        let acc = key.traverse f acc cv.nested.chord in
        key.traverse f acc cv.nested.passthrough,
    },

  chorded_aux
    | doc "for key::chorded::AuxiliaryKey."
    = {
      JsonValue = std.contract.from_validator json_value_validator,

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
        let passthrough_cv = passthrough |> key.codegen_values |> chorded_nestable.wrap in

        {
          nested = {
            passthrough = passthrough_cv,
          },
          key_type = {
            key_type = "crate::key::chorded::AuxiliaryKey",
            nested_key_type = nested.passthrough.key_type,
            as_rust_type_path = "%{key_type}<%{nested_key_type.as_rust_type_path}>"
          },
          key_impl =
            nested.passthrough.key_impl
            & {
              chorded = key_type.key_type,
            },
          rust_expr =
            let passthrough_expr = nested.passthrough.rust_expr in
            "crate::key::chorded::AuxiliaryKey::new(%{passthrough_expr})",
          json_value = k,
        },

      map_nested = fun f cv @ { nested, ..rest } =>
        rest
        & {
          nested = {
            passthrough = f cv.nested.passthrough,
          },
        },

      # Since key::chorded::AuxiliaryKey is a key::Key impl. for keymap::Keymap,
      #  we want a lift_to fn
      lift_to = fun key_impl cv =>
        chorded_aux.map_nested (fun cv_ => key.lift_to key_impl cv_) cv,

      # Traverse by visiting the chorded aux key,
      #  then traversing the passthrough key.
      traverse = fun f acc cv =>
        let acc = f acc cv in
        key.traverse f acc cv.nested.passthrough,
    },

  checks.check_composite = {
    check_base_key_codegen_values = {
      # A keyboard::Key lifted to composite::BaseKey
      check_base_key_keyboard =
        let k = { key_code = 4 } in
        let kcv = keyboard.codegen_values k in
        let cv = composite.base_key.keyboard kcv in
        {
          check_key_type = {
            actual = cv.key_type.as_rust_type_path,
            expected = "crate::key::composite::BaseKey",
          },
          check_rust_expr = {
            actual = cv.rust_expr,
            expected = "crate::key::composite::BaseKey::Keyboard(crate::key::keyboard::Key::new(4))",
          },
        },
    },

    check_tap_hold_key_codegen_values = {
      # A keyboard::Key lifted to composite::TapHoldKey via composite::BaseKey
      check_tap_hold_key_keyboard =
        let k = { key_code = 4 } in
        let kcv = keyboard.codegen_values k in
        let bk_cv = composite.base_key.keyboard kcv in
        let cv = composite.tap_hold_key.passthrough bk_cv in
        {
          check_key_type = {
            actual = cv.key_type.as_rust_type_path,
            expected = "crate::key::composite::TapHoldKey<crate::key::composite::BaseKey>",
          },
          check_rust_expr = {
            actual = cv.rust_expr,
            expected = "crate::key::composite::TapHoldKey::Pass(crate::key::composite::BaseKey::Keyboard(crate::key::keyboard::Key::new(4)))",
          },
        },
    },
  },

  composite = {
    base_key = {
      layer_modifier = fun cv =>
        {
          nested = cv,
          key_type = {
            key_type = "crate::key::composite::BaseKey",
            as_rust_type_path = key_type,
            variant = "LayerModifier",
          },
          key_impl = {
            base = key_type.key_type,
          },
          rust_expr = "crate::key::composite::BaseKey::LayerModifier(%{nested.rust_expr})",
        },

      keymap_callback = fun cv =>
        {
          nested = cv,
          key_type = {
            key_type = "crate::key::composite::BaseKey",
            as_rust_type_path = key_type,
            variant = "Callback",
          },
          key_impl = {
            base = key_type.key_type,
          },
          rust_expr = "crate::key::composite::BaseKey::Callback(%{nested.rust_expr})",
        },

      keyboard = fun cv =>
        {
          nested = cv,
          key_type = {
            key_type = "crate::key::composite::BaseKey",
            as_rust_type_path = key_type,
            variant = "Keyboard",
          },
          key_impl = {
            base = key_type.key_type,
          },
          rust_expr = "crate::key::composite::BaseKey::Keyboard(%{nested.rust_expr})",
        },

      caps_word = fun cv =>
        {
          nested = cv,
          key_type = {
            as_rust_type_path = "crate::key::composite::BaseKey",
            key_type = "crate::key::composite::BaseKey",
          },
          rust_expr = "crate::key::composite::BaseKey::CapsWord(%{cv.rust_expr})",
        },

      sticky = fun cv =>
        {
          nested = cv,
          key_type = {
            as_rust_type_path = "crate::key::composite::BaseKey",
            key_type = "crate::key::composite::BaseKey",
          },
          rust_expr = "crate::key::composite::BaseKey::Sticky(%{cv.rust_expr})",
        },

      custom = fun cv =>
        {
          nested = cv,
          key_type = {
            key_type = "crate::key::composite::BaseKey",
            as_rust_type_path = key_type,
            variant = "Custom",
          },
          key_impl = {
            base = key_type.key_type,
          },
          rust_expr = "crate::key::composite::BaseKey::Custom(%{nested.rust_expr})",
        },

      lift_to = fun key_impl cv =>
        cv,

      # Traverse by visiting the wrapper key,
      #  then by visiting the nested key.
      traverse = fun f acc cv =>
        let acc = f acc cv in
        key.traverse f acc cv.nested,
    },

    tap_hold_key = {
      tap_dance_ = fun cv =>
        {
          nested = cv,
          key_type = {
            key_type = "crate::key::composite::TapHoldKey",
            nested_key_type = nested.key_type.nested_key_type,
            as_rust_type_path = "%{key_type}<%{nested_key_type.as_rust_type_path}>",
            variant = "TapDance",
          },
          key_impl =
            nested.key_impl
            & {
              tap_hold = key_type.key_type,
            },
          rust_expr = "crate::key::composite::TapHoldKey::TapDance(%{nested.rust_expr})",
        },

      tap_hold_ = fun cv =>
        {
          nested = cv,
          key_type = {
            key_type = "crate::key::composite::TapHoldKey",
            nested_key_type = nested.key_type.nested_key_type,
            as_rust_type_path = "%{key_type}<%{nested_key_type.as_rust_type_path}>",
            variant = "TapHold",
          },
          key_impl =
            nested.key_impl
            & {
              tap_hold = key_type.key_type,
            },
          rust_expr = "crate::key::composite::TapHoldKey::TapHold(%{nested.rust_expr})",
        },

      passthrough = fun cv =>
        {
          nested = cv,
          key_type = {
            key_type = "crate::key::composite::TapHoldKey",
            nested_key_type = nested.key_type,
            as_rust_type_path = "%{key_type}<%{nested_key_type.as_rust_type_path}>",
            variant = "Pass",
          },
          key_impl =
            nested.key_impl
            & {
              tap_hold = key_type.key_type,
            },
          rust_expr = "crate::key::composite::TapHoldKey::Pass(%{nested.rust_expr})",
        },

      lift_to = fun key_impl { nested = nested_, ..cv } =>
        cv.key_type.variant
        |> match {
          "TapDance" => cv & { nested = tap_dance.map_nested (fun cv_ => key.lift_to key_impl cv_) nested_ },
          "TapHold" => cv & { nested = tap_hold.map_nested (fun cv_ => key.lift_to key_impl cv_) nested_ },
          "Pass" => cv & { nested = key.lift_to key_impl nested_ },
        },

      # Traverse by visiting the wrapper key,
      #  then by visiting the nested key.
      traverse = fun f acc cv =>
        let acc = f acc cv in
        key.traverse f acc cv.nested,
    },

    tap_hold_ = {
      wrap = fun cv =>
        {
          nested = cv,
          key_type = {
            key_type = "crate::key::composite::TapHold",
            nested_key_type = nested.key_type,
            as_rust_type_path = "%{key_type}<%{nested_key_type.as_rust_type_path}>",
          },
          key_impl =
            nested.key_impl
            & {
              tap_hold = key_type.key_type,
            },
          rust_expr = "crate::key::composite::TapHold(%{nested.rust_expr})",
        },

      lift_to = fun key_impl { nested = nested_, ..cv } =>
        if key_impl.tap_hold == "crate::key::composite::TapHoldKey" then
          # Lift the key impl. of the nested key,
          #  and construct as a TapHoldKey::Pass variant.
          nested_ |> key.lift_to key_impl |> tap_hold_key.passthrough
        else
          cv & { nested = key.lift_to key_impl nested_ },

      # Traverse by visiting the wrapper key,
      #  then by visiting the nested key.
      traverse = fun f acc cv =>
        let acc = f acc cv in
        key.traverse f acc cv.nested,
    },

    layered_key = {
      layered_ = fun cv =>
        {
          nested = cv,
          key_type = {
            key_type = "crate::key::composite::LayeredKey",
            nested_key_type = nested.key_type.nested_key_type,
            as_rust_type_path = "%{key_type}<%{nested_key_type.as_rust_type_path}>",
            variant = "Layered",
          },
          key_impl =
            nested.key_impl
            & {
              layered = key_type.key_type,
            },
          rust_expr = "crate::key::composite::LayeredKey::Layered(%{nested.rust_expr})",
        },

      passthrough = fun cv =>
        {
          nested = cv,
          key_type = {
            key_type = "crate::key::composite::LayeredKey",
            nested_key_type = nested.key_type,
            as_rust_type_path = "%{key_type}<%{nested_key_type.as_rust_type_path}>",
            variant = "Pass",
          },
          key_impl =
            nested.key_impl
            & {
              layered = key_type.key_type,
            },
          rust_expr = "crate::key::composite::LayeredKey::Pass(%{nested.rust_expr})",
        },

      lift_to = fun key_impl { nested = nested_, ..cv } =>
        cv.key_type.variant
        |> match {
          "Layered" => cv & { nested = layered.map_nested (fun cv_ => key.lift_to key_impl cv_) nested_ },
          "Pass" => cv & { nested = key.lift_to key_impl nested_ },
        },

      # Traverse by visiting the wrapper key,
      #  then by visiting the nested key.
      traverse = fun f acc cv =>
        let acc = f acc cv in
        key.traverse f acc cv.nested,
    },

    layered_ = {
      wrap = fun cv =>
        {
          nested = cv,
          key_type = {
            key_type = "crate::key::composite::Layered",
            nested_key_type = nested.key_type,
            as_rust_type_path = "%{key_type}<%{nested.key_type.as_rust_type_path}>",
          },
          key_impl =
            nested.key_impl
            & {
              layered = key_type.key_type,
            },
          rust_expr = "crate::key::composite::Layered(%{nested.rust_expr})",
        },

      lift_to = fun key_impl { nested = nested_, ..cv } =>
        if key_impl.layered == "crate::key::composite::LayeredKey" then
          # Lift the key impl. of the nested key,
          #  and construct as a LayeredKey::Pass variant.
          nested_ |> key.lift_to key_impl |> layered_key.passthrough
        else
          cv & { nested = key.lift_to key_impl nested_ },

      # Traverse by visiting the wrapper key,
      #  then by visiting the nested key.
      traverse = fun f acc cv =>
        let acc = f acc cv in
        key.traverse f acc cv.nested,
    },

    chorded_ = {
      wrap = fun cv =>
        {
          nested = cv,
          key_type = {
            key_type = "crate::key::composite::Chorded",
            nested_key_type = nested.key_type,
            as_rust_type_path = "%{key_type}<%{nested.key_type.as_rust_type_path}>",
          },
          key_impl =
            nested.key_impl
            & {
              chorded = key_type.key_type,
            },
          rust_expr = "crate::key::composite::Chorded(%{nested.rust_expr})",
        },

      lift_to = fun key_impl { nested = nested_, ..cv } =>
        if key_impl.chorded == "crate::key::composite::Chorded" then
          std.fail_with "not implemented"
        else
          cv & { nested = key.lift_to key_impl nested_ },

      # Traverse by visiting the wrapper key,
      #  then by visiting the nested key.
      traverse = fun f acc cv =>
        let acc = f acc cv in
        key.traverse f acc cv.nested,
    },

    # Construct codegen values to used with keymap::Keymap.
    #
    # keymap::Keymap takes in a `keys` which is an indexable collection
    #  of `key::Key` implementations which have the associated types
    #  Context, Event, and PressedKey.
    #
    # Since `keymap::Keymap` uses `key::Key` with `PressedKey = key::composite::PressedKey`,
    #  each of the `K0`, `K1`, etc. passed to `KeysN`
    #  must impl. `key::Key` with those associated types.
    # e.g. `key::keyboard::Key` implements `key::Key` with `PressedKey=BasePressedKey`;
    #  so it needs to be lifted to `Chorded(Layered(TapHold(keyboard::Key)))`.
    lift_to_key = fun cv @ { key_type = { key_type, .. }, .. } =>
      key_type
      |> match {
        "crate::key::chorded::Key" =>
          cv,
        "crate::key::chorded::AuxiliaryKey" =>
          cv,
        _ =>
          cv |> chorded_nestable.wrap |> chorded_.wrap,
      }
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

    codegen_module_for_codegen_value = fun cv @ { key_type = { key_type, .. }, .. } =>
      key_type
      |> match {
        "crate::key::keyboard::Key" => keyboard,
        "crate::key::caps_word::Key" => caps_word,
        "crate::key::layered::ModifierKey" => layer_modifier,
        "crate::key::callback::Key" => keymap_callback,
        "crate::key::sticky::Key" => sticky,
        "crate::key::custom::Key" => custom,
        "crate::key::tap_dance::Key" => tap_dance,
        "crate::key::tap_hold::Key" => tap_hold,
        "crate::key::layered::LayeredKey" => layered,
        "crate::key::chorded::Key" => chorded,
        "crate::key::chorded::AuxiliaryKey" => chorded_aux,
        "crate::key::composite::BaseKey" => composite.base_key,
        "crate::key::composite::TapHoldKey" => composite.tap_hold_key,
        "crate::key::composite::TapHold" => composite.tap_hold_,
        "crate::key::composite::LayeredKey" => composite.layered_key,
        "crate::key::composite::Layered" => composite.layered_,
        cv => std.fail_with "bad codegen_values for key.codegen_module_for_codegen_value: %{cv |> std.serialize 'Json}",
      },

    lift_to = fun key_impl cv @ { key_type = { key_type, .. }, .. } =>
      let impl =
        key_type
        |> match {
          "crate::key::keyboard::Key" => keyboard,
          "crate::key::caps_word::Key" => caps_word,
          "crate::key::layered::ModifierKey" => layer_modifier,
          "crate::key::callback::Key" => keymap_callback,
          "crate::key::sticky::Key" => sticky,
          "crate::key::custom::Key" => custom,
          "crate::key::chorded::Key" => chorded,
          "crate::key::chorded::AuxiliaryKey" => chorded_aux,
          "crate::key::composite::BaseKey" => composite.base_key,
          "crate::key::composite::TapHoldKey" => composite.tap_hold_key,
          "crate::key::composite::TapHold" => composite.tap_hold_,
          "crate::key::composite::LayeredKey" => composite.layered_key,
          "crate::key::composite::Layered" => composite.layered_,
          cv => std.fail_with "bad codegen_values for key.lift_to: %{cv |> std.serialize 'Json}",
        }
      in
      impl.lift_to key_impl cv,

    traverse = fun f acc cv =>
      let codegen_module = codegen_module_for_codegen_value cv in
      codegen_module.traverse f acc cv,
  },

  keymap_rs
    | doc "Text contents of the keymap.rs generated from the keymap.json"
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
                |> std.array.map (match {
                  [i, j] => "Some(crate::key::chorded::ChordIndices::from_slice(&[%{std.to_string i}, %{std.to_string j}]))"
                }
                )
                |> std.string.join ","
              in
              "chords: [%{chords_fragment}],"
            else
              ""
          in
          m%"
          crate::key::chorded::Config {
              %{timeout_field_fragment}
              %{chords_field_fragment}
              ..crate::key::chorded::DEFAULT_CONFIG
          }
        "%
        else
          "crate::key::chorded::DEFAULT_CONFIG"
      in
      let sticky_config_expr =
        if std.record.has_field "sticky" config then
          let activation_field_fragment =
            if std.record.has_field "activation" config.sticky then
              "activation: crate::key::sticky::StickyKeyActivation::%{config.sticky.activation},"
            else
              ""
          in
          let release_field_fragment =
            if std.record.has_field "release" config.sticky then
              "release: crate::key::sticky::StickyKeyRelease::%{config.sticky.release},"
            else
              ""
          in
          let config_fragment =
            [
              activation_field_fragment,
              release_field_fragment,
              "..crate::key::sticky::DEFAULT_CONFIG"
            ]
            |> std.array.filter ((!=) "")
            |> std.string.join "\n"
          in
          m%"
            crate::key::sticky::Config {
                %{config_fragment}
            }
          "%
        else
          "crate::key::sticky::DEFAULT_CONFIG"
      in
      let tap_dance_config_expr =
        if std.record.has_field "tap_dance" config then
          let timeout_field_fragment =
            if std.record.has_field "timeout" config.dance then
              "timeout: %{std.to_string config.tap_dance.timeout},"
            else
              ""
          in
          m%"
            crate::key::tap_dance::Config {
                %{timeout_field_fragment}
                ..crate::key::tap_dance::DEFAULT_CONFIG
            }
          "%
        else
          "crate::key::tap_dance::DEFAULT_CONFIG"
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
              "interrupt_response: crate::key::tap_hold::InterruptResponse::%{config.tap_hold.interrupt_response},"
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
            crate::key::tap_hold::Config {
                %{timeout_field_fragment}
                %{interrupt_response_field_fragment}
                %{required_idle_time_field_fragment}
                ..crate::key::tap_hold::DEFAULT_CONFIG
            }
          "%
        else
          "crate::key::tap_hold::DEFAULT_CONFIG"
      in
      let keymap_len = std.array.length keys in
      let max_tap_dance_definitions =
        # For each of the keys,
        #  compute max length of any tap dance definitions.
        let max_td_defs =
          std.array.map
            (fun k =>
              let cv = key.codegen_values k in
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
            keys
        in
        # Compute the max over all
        std.array.reduce_left (fun x y => if x > y then x else y) max_td_defs
      in
      let num_layers =
        keys
        |> std.array.fold_left
          (fun max_num_layers key =>
            let num_layers =
              key
              |> match {
                { layered, .. } => std.array.length layered,
                _ => 0,
              }
            in
            if num_layers > max_num_layers then
              num_layers
            else
              max_num_layers
          )
          0
      in
      let keys_id = "Keys%{std.to_string keymap_len}" in
      let codegen_values =
        keys
        |> std.array.map key.codegen_values
        |> std.array.map composite.lift_to_key
      in
      let key_types =
        codegen_values
        |> std.array.map (fun { key_type, .. } => key_type)
        |> std.array.map (fun { as_rust_type_path, .. } => as_rust_type_path)
        |> std.string.join ","
      in
      let key_exprs =
        codegen_values
        |> std.array.map (fun k => "%{k.rust_expr},")
        |> std.string.join ""
      in
      let chord_count =
        if std.record.has_field "chorded" config && std.record.has_field "chords" config.chorded then
          config.chorded.chords |> std.array.length
        else
          0
      in
      m%"
/// Types and initial data used for constructing [KEYMAP].
pub mod init {
    /// Config used by tap-hold keys.
    pub const CONFIG: crate::key::composite::Config = crate::key::composite::Config {
        chorded: %{chorded_config_expr},
        sticky: %{sticky_config_expr},
        tap_dance: %{tap_dance_config_expr},
        tap_hold: %{tap_hold_config_expr},
        ..crate::key::composite::DEFAULT_CONFIG
    };

    /// Number of layers supported by the [crate::key::layered] implementation.
    pub const LAYER_COUNT: usize = %{std.to_string num_layers};

    /// The maximum number of keys in a chord.
    pub const MAX_CHORD_SIZE: usize = 2;

    /// The maximum number of chords.
    pub const MAX_CHORDS: usize = %{chord_count |> std.to_string};

    /// The tap-dance definitions.
    pub const MAX_TAP_DANCE_DEFINITIONS: usize = %{max_tap_dance_definitions |> std.to_string};

    pub use crate::key::composite::Context;

    pub use crate::key::composite::Event;

    pub use crate::key::composite::PendingKeyState;

    pub use crate::key::composite::KeyState;

    pub use crate::key::composite::Key;

    /// Initial [Context] value.
    pub const CONTEXT: Context = crate::key::composite::Context::from_config(CONFIG);

    crate::tuples::define_keys!(%{std.to_string keymap_len});

    /// Alias for a [tuples] KeysN type, as generated by keymap.ncl.
    pub type KeyDefinitionsType = %{keys_id}<
        %{key_types},
        Context,
        Event,
        PendingKeyState,
        KeyState,
    >;

    /// Alias for the [keymap::Keymap] type.
    pub type Keymap = crate::keymap::Keymap<Context, Event, PendingKeyState, KeyState, KeyDefinitionsType>;

    /// A [tuples] KeysN value with keys, as generated by keymap.ncl.
    pub const KEY_DEFINITIONS: KeyDefinitionsType = %{keys_id}::new((
        %{key_exprs}
    ));
}
"%
}
