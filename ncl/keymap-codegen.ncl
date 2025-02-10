let validators = import "validators.ncl" in
{
  serialized_json_keymap
    | doc "The 'JSON serialized' value of the keymap. e.g. imported from keymap.json."
    | { keys | Array key.SerializedJson, .. },

  checks.check_keyboard = {
    # Use a serialized value to check the "is" predicate.
    check_serialized_is =
      let serialized_value = { key_code = 4 } in
      keyboard.is_serialized_json serialized_value,
    check_serialized_with_modifier_is =
      let serialized_value = { key_code = 4, modifiers = { left_ctrl = true } } in
      keyboard.is_serialized_json serialized_value,
    check_serialized_modifiers_is =
      let serialized_value = { modifiers = { left_ctrl = true } } in
      keyboard.is_serialized_json serialized_value,

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
      SerializedJson = std.contract.from_validator serialized_json_validator,

      # c.f. doc_de_keyboard.md.
      serialized_json_validator =
        validators.record.validator {
          fields_validator =
            validators.all_of [
              validators.record.has_any_field_of ["key_code", "modifiers"],
              validators.record.has_only_fields ["key_code", "modifiers"],
            ],
          field_validators = {
            key_code = validators.is_number,
            modifiers =
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
          },
        },

      is_serialized_json = fun k => 'Ok == serialized_json_validator k,

      codegen_values = fun k =>
        let kt = {
          key_type = "crate::key::keyboard::Key",
          as_rust_type_path = key_type,
        }
        in

        let expr =
          let rec modifiers_expr = fun m =>
            m
            |> match {
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
            }
          in
          k
          |> match {
            { key_code } =>
              "crate::key::keyboard::Key::new(%{std.to_string key_code})",
            { modifiers } =>
              "crate::key::keyboard::Key::from_modifiers(%{modifiers_expr modifiers})",
            { key_code, modifiers } =>
              "crate::key::keyboard::Key::new_with_modifiers(%{std.to_string key_code}, %{modifiers_expr modifiers})",
          }
        in

        {
          key_type = kt,
          rust_expr = expr,
          serialized_json = k,
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
      SerializedJson = std.contract.from_validator serialized_json_validator,

      # c.f. doc_de_layered.md.
      # JSON serialization of key::layered::ModifierKey has variants: Hold(layer).
      serialized_json_validator =
        validators.record.validator {
          fields_validator = validators.record.has_exact_fields ["Hold"],
          field_validators = {
            Hold = validators.is_number,
          },
        },

      is_serialized_json = fun k => 'Ok == serialized_json_validator k,

      codegen_values = fun k @ { Hold = layer_index } =>
        let kt = {
          key_type = "crate::key::layered::ModifierKey",
          as_rust_type_path = key_type,
        }
        in

        let expr = "crate::key::layered::ModifierKey::Hold(%{std.to_string layer_index})"
        in

        {
          key_type = kt,
          rust_expr = expr,
          serialized_json = k,
        },
    },

  checks.check_layered = {
    # Use a serialized value to check the "is" predicate.
    check_serialized_base_keyboard_layered_keyboard_is =
      let serialized_value = {
        base = { key_code = 4 },
        layered = [
          null,
          { key_code = 6 },
        ]
      }
      in
      layered.is_serialized_json serialized_value,

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
            actual = cv.key_type,
            expected = {
              as_rust_type_path = "crate::key::layered::LayeredKey<crate::key::keyboard::Key>",
              key_type = "crate::key::layered::LayeredKey",
            },
          },
          check_rust_expr = {
            actual = cv.rust_expr,
            expected = "crate::key::layered::LayeredKey::new(crate::key::keyboard::Key::new(4), [None,Some(crate::key::keyboard::Key::new(6))])",
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
            actual = cv.key_type,
            expected = {
              as_rust_type_path = "crate::key::layered::LayeredKey<crate::key::composite::TapHoldKey<crate::key::composite::BaseKey>>",
              key_type = "crate::key::layered::LayeredKey"
            },
          },
          check_rust_expr = {
            actual = cv.rust_expr,
            expected = "crate::key::layered::LayeredKey::new(crate::key::composite::TapHoldKey::tap_hold(crate::key::tap_hold::Key::new(crate::key::composite::BaseKey::keyboard(crate::key::keyboard::Key::new(4)), crate::key::composite::BaseKey::keyboard(crate::key::keyboard::Key::new(224)))), [None,Some(crate::key::composite::TapHoldKey::Pass(crate::key::composite::BaseKey::keyboard(crate::key::keyboard::Key::new(6))))])",
          },
        },
    },
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
      serialized_json_validator =
        validators.record.validator {
          fields_validator = validators.record.has_exact_fields ["base", "layered"],
          field_validators = {
            base = key.serialized_json_validator,
            layered =
              validators.array.validator (
                validators.any_of [
                  validators.is_null,
                  key.serialized_json_validator,
                ]
              ),
          },
        },

      is_serialized_json = fun k => 'Ok == serialized_json_validator k,

      codegen_values = fun k @ { base, layered } =>
        let base_cv = key.codegen_values base in
        let non_null_layered = std.array.filter ((!=) null) layered in
        let layered_cvs =
          std.array.map (fun k => if k != null then key.codegen_values k else null) layered
        in
        let nn_layered_codegen_values =
          std.array.filter ((!=) null) layered_cvs
        in

        # Unify nested composite values.
        #  In order to construct a key::layered::LayeredKey,
        #  the `tap` and `hold` keys must have the same (Rust) type.
        #  - case where both same (e.g. both be key::keyboard::Key):
        #     can use that as the 'unified' type.
        #  - case where types differ (e.g. one is layered::ModifierKey, other is keyboard::Key),
        #     need to 'lift' each of tap, hold to a unified key type.
        #     i.e. a type which implements composite::LayeredNestable.
        #     (i.e. composite::TapHold<BaseKey>).
        let unify_cv =
          let base_key_type = base_cv.key_type in
          let layered_key_types = std.array.map (fun { key_type, .. } => key_type) nn_layered_codegen_values in
          if std.array.all (fun kt => kt == base_key_type) layered_key_types then
            fun cv => cv
          else
            fun cv =>
              (
                cv
                |> match {
                  cv @ { serialized_json = k, .. } if tap_hold.is_serialized_json k =>
                    composite.tap_hold_key.codegen_values cv,
                  cv @ { serialized_json = k, .. } if composite.base_key.is_serialized_json k =>
                    let bk_cv = composite.base_key.codegen_values cv in
                    composite.tap_hold_key.codegen_values bk_cv,
                  cv => std.fail_with "bad codegen_values for layered nested key: %{cv |> std.serialize 'Json}",
                }
              )
        in

        {
          nested = {
            base = base_cv |> unify_cv,
            layered = layered_cvs
                    |> std.array.map (fun cv => if cv != null then unify_cv cv else null),
          },
          key_type =
            {
              key_type = "crate::key::layered::LayeredKey",
              as_rust_type_path = "%{key_type}<%{nested.base.key_type.as_rust_type_path}>",
            },
          rust_expr =
            let nk_expr = fun cv => cv.rust_expr in
            let base_expr = nk_expr nested.base in
            let layered_exprs =
              nested.layered
              |> std.array.map (fun cv => if cv == null then "None" else "Some(%{nk_expr cv})")
              |> std.string.join ","
            in
            "crate::key::layered::LayeredKey::new(%{base_expr}, [%{layered_exprs}])",
          serialized_json = k,
        },

      map_nested = fun f cv@{ nested, ..rest } =>
        rest & {
          nested = {
            base = f cv.nested.base,
            layered = std.array.map (fun cv => if cv != null then f cv else null) cv.nested.layered,
          },
        },
    },

  checks.check_tap_hold = {
    # Use a serialized value to check the "is" predicate.
    check_serialized_tap_keyboard_hold_keyboard_is =
      let serialized_value = {
        hold = { key_code = 224 },
        tap = { key_code = 4 },
      }
      in
      tap_hold.is_serialized_json serialized_value,

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
            actual = cv.key_type,
            expected = {
              as_rust_type_path = "crate::key::tap_hold::Key<crate::key::composite::BaseKey>",
              key_type = "crate::key::tap_hold::Key",
            }
          },
          check_rust_expr = {
            actual = cv.rust_expr,
            expected = "crate::key::tap_hold::Key::new(crate::key::composite::BaseKey::keyboard(crate::key::keyboard::Key::new(4)), crate::key::composite::BaseKey::keyboard(crate::key::keyboard::Key::new(224)))",
          },
        },
    },
  },

  tap_hold
    | doc "for key::tap_hold::Key."
    = {
      SerializedJson = std.contract.from_validator serialized_json_validator,

      # c.f. doc_de_tap_hold.md.
      # JSON serialization of key::tap_hold::Key is { tap: key, hold: key }
      serialized_json_validator =
        validators.record.validator {
          fields_validator = validators.record.has_exact_fields ["tap", "hold"],
          field_validators = {
            tap = key.serialized_json_validator,
            hold = key.serialized_json_validator,
          },
        },

      is_serialized_json = fun k => 'Ok == serialized_json_validator k,

      codegen_values = fun k @ { hold, tap } =>
        let hold_codegen_values = key.codegen_values hold in
        let tap_codegen_values = key.codegen_values tap in

        let tap_kt = tap_codegen_values.key_type in
        let hold_kt = hold_codegen_values.key_type in

        # Unify nested composite values.
        #  In order to construct a key::tap_hold::Key,
        #  the `tap` and `hold` keys must have the same (Rust) type.
        #  - case where both same (e.g. both be key::keyboard::Key):
        #     can use that as the 'unified' type.
        #  - case where types differ (e.g. one is layered::ModifierKey, other is keyboard::Key),
        #     need to 'lift' each of tap, hold to a unified key type.
        #     i.e. a type which implements composite::TapHoldNestable.
        #     (i.e. composite::BaseKey).
        let unify_cv = fun cv =>
          # let nested_key_type =
          #   if tap_key_type == hold_key_type then
          #     tap_key_type
          #   else
          #     composite.base_key.key_type tap in
          composite.base_key.codegen_values cv
        in

        {
          nested = {
            tap = tap_codegen_values |> unify_cv,
            hold = hold_codegen_values |> unify_cv,
          },
          key_type =
            let nested_key_type = nested.tap.key_type in
            {
              key_type = "crate::key::tap_hold::Key",
              as_rust_type_path = "%{key_type}<%{nested_key_type.as_rust_type_path}>"
            },
          rust_expr =
            let hold_expr = nested.hold.rust_expr in
            let tap_expr = nested.tap.rust_expr in
            "crate::key::tap_hold::Key::new(%{tap_expr}, %{hold_expr})",
          serialized_json = k,
        },

      map_nested = fun f cv@{ nested, ..rest } =>
        rest & {
          nested = {
            tap = f cv.nested.tap,
            hold = f cv.nested.hold,
          },
        },
    },

  checks.check_composite = {
    check_base_key_codegen_values = {
      # A keyboard::Key lifted to composite::BaseKey
      check_base_key_keyboard =
        let k = { key_code = 4 } in
        let kcv = keyboard.codegen_values k in
        let cv = composite.base_key.codegen_values kcv in
        {
          check_key_type = {
            actual = cv.key_type,
            expected = {
              as_rust_type_path = "crate::key::composite::BaseKey",
              key_type = "crate::key::composite::BaseKey",
            },
          },
          check_rust_expr = {
            actual = cv.rust_expr,
            expected = "crate::key::composite::BaseKey::keyboard(crate::key::keyboard::Key::new(4))",
          },
        },
    },

    check_tap_hold_key_codegen_values = {
      # A keyboard::Key lifted to composite::TapHoldKey via composite::BaseKey
      check_tap_hold_key_keyboard =
        let k = { key_code = 4 } in
        let kcv = keyboard.codegen_values k in
        let bk_cv = composite.base_key.codegen_values kcv in
        let cv = composite.tap_hold_key.codegen_values bk_cv in
        {
          check_key_type = {
            actual = cv.key_type,
            expected = {
              as_rust_type_path = "crate::key::composite::TapHoldKey<crate::key::composite::BaseKey>",
              key_type = "crate::key::composite::TapHoldKey",
              nested_key_type = "crate::key::composite::BaseKey",
            },
          },
          check_rust_expr = {
            actual = cv.rust_expr,
            expected = "crate::key::composite::TapHoldKey::Pass(crate::key::composite::BaseKey::keyboard(crate::key::keyboard::Key::new(4)))",
          },
        },
    },
  },

  composite = {
    base_key = {
      SerializedJson = std.contract.from_validator serialized_json_validator,

      serialized_json_validator =
        validators.any_of [
          layer_modifier.serialized_json_validator,
          keyboard.serialized_json_validator,
        ],

      is_serialized_json = fun k => 'Ok == serialized_json_validator k,

      codegen_values = match {
        cv @ { serialized_json = k, .. } if layer_modifier.is_serialized_json k =>
          let { rust_expr = k_expr, .. } = cv in
          {
            key_type = {
              key_type = "crate::key::composite::BaseKey",
              as_rust_type_path = key_type,
            },
            rust_expr = "crate::key::composite::BaseKey::layer_modifier(%{k_expr})",
            serialized_json = k,
          },
        cv @ { serialized_json = k, .. } if keyboard.is_serialized_json k =>
          let { rust_expr = k_expr, .. } = cv in
          {
            key_type = {
              key_type = "crate::key::composite::BaseKey",
              as_rust_type_path = key_type,
            },
            rust_expr = "crate::key::composite::BaseKey::keyboard(%{k_expr})",
            serialized_json = k,
          },
        cv => std.fail_with "bad codegen_values for base_key: %{cv |> std.serialize 'Json}",
      },
    },

    tap_hold_key = {
      SerializedJson = std.contract.from_validator serialized_json_validator,

      serialized_json_validator =
        validators.any_of [
          tap_hold.serialized_json_validator,
          composite.base_key.serialized_json_validator,
        ],

      is_serialized_json = fun k => 'Ok == serialized_json_validator k,

      codegen_values = match {
        cv @ { serialized_json = k, .. } if tap_hold.is_serialized_json k =>
          let { key_type = k_key_type, rust_expr = k_expr, .. } = cv in
          {
            key_type = {
              key_type = "crate::key::composite::TapHoldKey",
              nested_key_type = "crate::key::composite::BaseKey",
              as_rust_type_path = "%{key_type}<%{nested_key_type}>",
            },
            rust_expr = "crate::key::composite::TapHoldKey::tap_hold(%{k_expr})",
            serialized_json = k,
          },
        cv @ { serialized_json = k, .. } if composite.base_key.is_serialized_json k =>
          let { key_type = k_key_type, rust_expr = k_expr, .. } = cv in
          {
            key_type = {
              key_type = "crate::key::composite::TapHoldKey",
              nested_key_type = "crate::key::composite::BaseKey",
              as_rust_type_path = "%{key_type}<%{nested_key_type}>",
            },
            rust_expr = "crate::key::composite::TapHoldKey::Pass(%{k_expr})",
            serialized_json = k,
          },
        cv => std.fail_with "bad codegen_values for tap_hold_key: %{cv |> std.serialize 'Json}",
      },
    },

    tap_hold_ = {
      codegen_values = fun cv =>
        {
          nested = cv,
          key_type = {
            key_type = "crate::key::composite::TapHold",
            as_rust_type_path = "%{key_type}<%{nested.key_type.as_rust_type_path}>",
          },
          rust_expr = "crate::key::composite::TapHold(%{nested.rust_expr})",
        },
    },

    layered_key = {
      SerializedJson = std.contract.from_validator serialized_json_validator,

      serialized_json_validator =
        validators.any_of [
          layered.serialized_json_validator,
          composite.tap_hold_key.serialized_json_validator,
        ],

      is_serialized_json = fun k => 'Ok == serialized_json_validator k,

      codegen_values = match {
        cv @ { serialized_json = k, .. } if layered.is_serialized_json k =>
          let { key_type = k_key_type, rust_expr = k_expr, .. } = cv in
          {
            key_type = {
              key_type = "crate::key::composite::LayeredKey",
              nested_key_type = "crate::key::composite::TapHoldKey",
              as_rust_type_path = "%{key_type}<%{nested_key_type}>",
            },
            rust_expr = "crate::key::composite::LayeredKey::layered(%{k_expr})",
            serialized_json = k,
          },
        cv @ { serialized_json = k, .. } if composite.tap_hold_key.is_serialized_json k =>
          let { key_type = k_key_type, rust_expr = k_expr, .. } = cv in
          {
            key_type = {
              key_type = "crate::key::composite::LayeredKey",
              nested_key_type = "crate::key::composite::TapHoldKey",
              as_rust_type_path = "%{key_type}<%{nested_key_type}>",
            },
            rust_expr = "crate::key::composite::LayeredKey::Pass(%{k_expr})",
            serialized_json = k,
          },
        cv => std.fail_with "bad codegen_values for layered_key: %{cv |> std.serialize 'Json}",
      },
    },

    layered_ = {
      codegen_values = fun cv =>
        {
          nested = cv,
          key_type = {
            key_type = "crate::key::composite::Layered",
            as_rust_type_path = "%{key_type}<%{nested.key_type.as_rust_type_path}>",
          },
          rust_expr = "crate::key::composite::Layered(%{nested.rust_expr})",
        },
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
    #  so it needs to be lifted to `Layered(TapHold(keyboard::Key))`.
    #
    # e.g. assuming that any nested keys have been 'lifted'
    #  to a suitable type:
    # - for key::keyboard::Key, wrap with:
    #    key::composite::Layered::keyboard(key::keyboard::Key).
    # - for key::tap_hold::Key wrap with:
    #    key::composite::Layered::tap_hold(key::tap_hold::Key<NK>).
    #   - n.b.: NK must be an implemented type.
    # - for key::layered::LayeredKey,
    #    key::composite::Layered::layered(key::layered::LayeredKey<NK>).
    #   - n.b.: NK must be an implemented type.
    lift_to_key =
      match {
        cv if cv.key_type.key_type == "crate::key::keyboard::Key" =>
          cv |> tap_hold_.codegen_values |> layered_.codegen_values,
        cv => std.fail_with "bad codegen value for to_keymap_key: %{cv |> std.serialize 'Json}",
      }
  },

  key = {
    SerializedJson = std.contract.from_validator serialized_json_validator,

    serialized_json_validator =
      validators.any_of [
        keyboard.serialized_json_validator,
        layer_modifier.serialized_json_validator,
        layered.serialized_json_validator,
        tap_hold.serialized_json_validator,
      ],

    is_serialized_json = fun k => 'Ok == serialized_json_validator k,

    codegen_values = match {
      k if keyboard.is_serialized_json k => keyboard.codegen_values k,
      k if layer_modifier.is_serialized_json k => layer_modifier.codegen_values k,
      k if layered.is_serialized_json k => layered.codegen_values k,
      k if tap_hold.is_serialized_json k => tap_hold.codegen_values k,
      k => std.fail_with "bad serialized_json for key: %{k |> std.serialize 'Json}",
    },
  },

  keymap_rs
    | doc "Text contents of the keymap.rs generated from the keymap.json"
    =
      let { config, keys } =
        serialized_json_keymap
        & {
          config = {
            tap_hold = {
              timeout | default = 200,
              interrupt_response | default = "Ignore",
            },
          },
        }
      in
      let tap_hold_config_expr = m%"
        crate::key::tap_hold::Config {
            timeout: %{std.to_string config.tap_hold.timeout},
            interrupt_response: crate::key::tap_hold::InterruptResponse::%{config.tap_hold.interrupt_response},
        }
      "%
      in
      let keymap_len = std.array.length keys in
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
      m%"
/// Types and initial data used for constructing [KEYMAP].
pub mod init {
    /// Config used by tap-hold keys.
    pub const CONFIG: crate::key::composite::Config = crate::key::composite::Config {
        tap_hold: %{tap_hold_config_expr},
    };

    /// Number of layers supported by the [crate::key::layered] implementation.
    pub const LAYER_COUNT: usize = %{std.to_string num_layers};

    /// Alias for Context type; i.e. [crate::key::composite::Context] with generics.
    pub type Context = crate::key::composite::Context;

    /// Alias for Event type; i.e. [crate::key::composite::Event].
    pub type Event = crate::key::composite::Event;

    /// Alias for PressedKey type; i.e. [crate::key::composite::PressedKey].
    pub type PressedKey = crate::key::composite::PressedKey;

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
        Event,
        PressedKey,
    >;

    /// A [tuples] KeysN value with keys, as generated by keymap.ncl.
    pub const KEY_DEFINITIONS: KeyDefinitionsType = %{keys_id}::new((
        %{key_exprs}
    ));
}
"%
}
