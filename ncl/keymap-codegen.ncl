let validators = import "validators.ncl" in
{
  KeymapJson
    | doc "Contract for the keymap.json value"
    = {
      keys | Array smart_key.Json,
      config | composite.config.Json | default = {},
      ..
    },

  lib = {
    array_rust_expr = fun data =>
      "[%{data |> std.array.map (fun { rust_expr, .. } => rust_expr) |> std.string.join ", "}]",
  },

  json_keymap
    | doc "The 'JSON' value of the keymap. e.g. imported from keymap.json."
    | KeymapJson,

  key_codegen_values =
    json_keymap.keys
    |> std.array.map smart_key.codegen_values,

  key_data_and_refs =
    key_codegen_values
    |> std.array.fold_left
      (fun { key_data, key_refs = refs } cv =>
        let { key_data = kd, ref = r } = smart_key.data_and_ref key_data cv in
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
    Json = std.contract.from_validator json_validator,

    json_validator = validators.is_number,

    is_json = fun json => 'Ok == json_validator json,

    expr = fun b => "smart_keymap::key::KeyboardModifiers::from_byte(%{std.to_string b})",
  },

  checks.check_keyboard = {
    # Use a JSON value to check the "is" predicate.
    check_json_is =
      let json = { key_code = 4 } in
      keyboard.key.is_json json,
    check_json_with_modifier_is =
      let json = { key_code = 4, modifiers = 1 } in
      keyboard.key.is_json json,
    check_json_modifiers_is =
      let json = { modifiers = 1 } in
      keyboard.key.is_json json,

    check_keyboard_codegen_values = {
      check_basic =
        let json = { key_code = 4 } in
        let cv = keyboard.key.codegen_values json in
        {
          check_module_type = {
            actual = cv.key_type,
            expected = "smart_keymap::key::keyboard::Key",
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
      module = "smart_keymap::key::keyboard",

      key = {
        Json = std.contract.from_validator json_validator,

        key_type = "%{module}::Key",

        # c.f. doc_de_keyboard.md.
        json_validator =
          validators.record.validator {
            fields_validator =
              validators.all_of [
                validators.record.has_any_field_of ["key_code", "modifiers"],
                validators.record.has_only_fields ["key_code", "modifiers"],
              ],
            field_validators = {
              key_code = validators.is_number,
              modifiers = keyboard_modifiers.json_validator,
            },
          },

        is_json = fun json => 'Ok == json_validator json,

        codegen_values = fun json =>
          {
            include json,
            include module,
            include key_type,
            rust_expr =
              json
              |> match {
                { key_code } =>
                  "smart_keymap::key::keyboard::Key::new(%{std.to_string key_code})",
                { modifiers } =>
                  "smart_keymap::key::keyboard::Key::from_modifiers(%{keyboard_modifiers.expr modifiers})",
                { key_code, modifiers } =>
                  "smart_keymap::key::keyboard::Key::new_with_modifiers(%{std.to_string key_code}, %{keyboard_modifiers.expr modifiers})",
              },
          },

        traverse = fun f acc cv => f acc cv,

        data_and_ref = fun key_data cv @ { json, .. } =>
          json
          |> match {
            { key_code } =>
              {
                include key_data,
                ref = {
                  include module,
                  variant = "KeyCode",
                  json = { "%{variant}" = key_code },
                  rust_expr = "%{module}::Ref::%{variant}(%{std.to_string key_code})",
                },
              },
            { modifiers } =>
              {
                include key_data,
                ref = {
                  include module,
                  variant = "Modifiers",
                  json = { "%{variant}" = modifiers },
                  rust_expr = "%{module}::Ref::%{variant}(%{std.to_string modifiers})",
                },
              },
            { key_code, modifiers } =>
              let { keyboard = keyboard_, ..other_data } = key_data & { keyboard | default = [] } in
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

      system = {
        rust_expr =
          let keyboard_data = (key_data_and_refs.key_data & { keyboard | default = [] }).keyboard in
          "smart_keymap::key::keyboard::System::new(%{keyboard_data |> lib.array_rust_expr})",
      },
    },

  caps_word
    | doc "for key::caps_word::Key."
    = {
      module = "smart_keymap::key::caps_word",

      key = {
        Json = std.contract.from_validator json_validator,

        key_type = "%{module}::Key",

        json_validator = fun json =>
          json
          |> match {
            "ToggleCapsWord" => 'Ok,
            _ => 'Error { message = "Expected \"ToggleCapsWord\"" },
          },

        is_json = fun json => 'Ok == json_validator json,

        codegen_values = fun json =>
          {
            include json,
            include module,
            include key_type,
            rust_expr = "smart_keymap::key::caps_word::Key::new()",
          },

        traverse = fun f acc cv => f acc cv,

        data_and_ref = fun key_data cv @ { json, .. } =>
          {
            include key_data,
            ref = {
              include module,
              include json,
              rust_expr = "%{module}::Ref(%{module}::Key::%{json})",
            },
          },
      },
    },

  callback
    | doc "for key::callback::Key."
    = {
      module = "smart_keymap::key::callback",

      key = {
        Json = std.contract.from_validator json_validator,

        key_type = "%{module}::Key",

        json_validator =
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

        is_json = fun json => 'Ok == json_validator json,

        codegen_values = fun json @ { keymap_callback } =>
          {
            include json,
            include module,
            include key_type,
            rust_expr =
              let cb_expr =
                keymap_callback
                |> match {
                  { Custom = [i, j] } =>
                    "smart_keymap::keymap::KeymapCallback::Custom(%{i |> std.to_string}, %{j |> std.to_string})",
                  _ => "smart_keymap::keymap::KeymapCallback::%{keymap_callback}"
                }
              in "%{module}::Key::new(%{cb_expr})",
          },

        traverse = fun f acc cv => f acc cv,

        data_and_ref = fun key_data cv =>
          let { callback = callback_, ..other_data } = key_data & { callback | default = [] } in
          let new_index = std.array.length callback_ in
          let new_key = {
            json = cv.json,
            rust_expr = cv.rust_expr,
          }
          in
          {
            key_data = other_data & { callback = std.array.append new_key callback_ },
            ref = {
              include module,
              json = new_index,
              rust_expr = "%{module}::Ref(%{std.to_string new_index})",
            },
          },
      },

      system = {
        rust_expr =
          let callback_data = (key_data_and_refs.key_data & { callback | default = [] }).callback in
          "smart_keymap::key::callback::System::new(%{callback_data |> lib.array_rust_expr})",
      },
    },

  sticky
    | doc "for key::sticky::Key."
    = {
      module = "smart_keymap::key::sticky",

      key = {
        Json = std.contract.from_validator json_validator,

        key_type = "%{module}::Key",

        json_validator =
          validators.record.validator {
            fields_validator =
              validators.all_of [
                validators.record.has_exact_fields ["sticky_modifiers"],
              ],
            field_validators = {
              sticky_modifiers = keyboard_modifiers.json_validator,
            },
          },

        is_json = fun json => 'Ok == json_validator json,

        codegen_values = fun json =>
          {
            include json,
            include module,
            include key_type,
            rust_expr =
              json
              |> match {
                { sticky_modifiers } =>
                  "%{module}::Key::new(%{keyboard_modifiers.expr sticky_modifiers})",
              },
          },

        traverse = fun f acc cv => f acc cv,

        data_and_ref = fun key_data cv =>
          let { sticky = sticky_, ..other_data } = key_data & { sticky | default = [] } in
          let new_index = std.array.length sticky_ in
          let new_key = {
            json = cv.json,
            rust_expr = cv.rust_expr,
          }
          in
          {
            key_data = other_data & { sticky = std.array.append new_key sticky_ },
            ref = {
              include module,
              json = new_index,
              rust_expr = "%{module}::Ref(%{std.to_string new_index})",
            },
          },
      },

      config = {
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

        Json = {
          activation
            | optional
            | StickyActivationJson_,
          release
            | optional
            | StickyReleaseJson_,
        },

        rust_expr =
          if std.record.has_field "sticky" json_keymap.config then
            let sticky_config = json_keymap.config.sticky in
            let activation_field_fragment =
              if std.record.has_field "activation" sticky_config then
                "activation: smart_keymap::key::sticky::StickyKeyActivation::%{sticky_config.activation},"
              else
                ""
            in
            let release_field_fragment =
              if std.record.has_field "release" sticky_config then
                "release: smart_keymap::key::sticky::StickyKeyRelease::%{sticky_config.release},"
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
            "smart_keymap::key::sticky::DEFAULT_CONFIG",
      },

      system = {
        rust_expr =
          let sticky_data = (key_data_and_refs.key_data & { sticky | default = [] }).sticky in
          "smart_keymap::key::sticky::System::new(%{sticky_data |> lib.array_rust_expr})"
      },
    },

  custom
    | doc "for key::custom::Key."
    = {
      module = "smart_keymap::key::custom",

      key = {
        Json = std.contract.from_validator json_validator,

        key_type = "%{module}::Key",

        json_validator =
          validators.record.validator {
            fields_validator = validators.record.has_exact_fields ["custom"],
            field_validators = {
              custom = validators.is_number,
            },
          },

        is_json = fun json => 'Ok == json_validator json,

        codegen_values = fun json @ { custom } =>
          {
            include json,
            include module,
            include key_type,
            rust_expr = "%{module}::Key::new(%{std.to_string custom})",
          },

        traverse = fun f acc cv => f acc cv,

        data_and_ref = fun key_data cv @ { json, .. } =>
          {
            include key_data,
            ref = {
              include json,
              include module,
              rust_expr = "%{module}::Ref(%{json.custom |> std.to_string})",
            },
          },
      },
    },

  tap_dance
    | doc "for key::tap_dance::Key."
    = {
      module = "smart_keymap::key::tap_dance",

      key = {
        Json = std.contract.from_validator json_validator,

        key_type = "%{module}::Key",

        # JSON serialization of key::tap_dance::Key is { definitions: [Option<json>] }
        #  and the codegen uses the from_definitions(&[json]) constructor.
        json_validator =
          validators.record.validator {
            fields_validator = validators.record.has_exact_fields ["definitions"],
            field_validators = {
              definitions = validators.array.validator smart_key.json_validator,
            },
          },

        is_json = fun json => 'Ok == json_validator json,

        codegen_values = fun json @ { definitions } =>
          let definitions_cv = definitions |> std.array.map (fun json => json |> smart_key.codegen_values) in

          {
            nested = {
              definitions = definitions_cv,
            },
            include json,
            include module,
            include key_type,
            rust_expr =
              let definitions_expr = nested.definitions |> std.array.map (fun cv => cv.rust_expr) in
              let definitions_slice_expr = "&[%{definitions_expr |> std.string.join ", "}]" in
              "%{module}::Key::from_definitions(%{definitions_slice_expr})",
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
            (fun acc nested_cv => smart_key.traverse f acc nested_cv)
            acc
            cv.nested.definitions,

        data_and_ref = fun key_data cv @ { nested = { definitions = definitions_cvs }, .. } =>
          let { key_data, definitions_refs } =
            std.array.fold_left
              (fun { key_data, definitions_refs = parts } cv =>
                let { key_data, ref = definition_ref } = smart_key.data_and_ref key_data cv in
                let definition_ref = definition_ref |> composite.ref.wrap in
                { include key_data, definitions_refs = std.array.append definition_ref parts }
              )
              { include key_data, definitions_refs = [] }
              definitions_cvs
          in
          let { tap_dance = tap_dance_, ..other_data } = key_data & { tap_dance | default = [] } in
          let new_index = std.array.length tap_dance_ in
          let new_key = {
            json = {
              definitions = definitions_refs |> std.array.map (fun { json, .. } => json),
            },
            rust_expr = m%"
            %{module}::Key::from_definitions(
              &[%{
                definitions_refs
                |> std.array.map (fun definition_ref => definition_ref.rust_expr)
                |> std.string.join ", "
              }],
            )
          "%,
          }
          in
          {
            key_data = other_data & { tap_dance = std.array.append new_key tap_dance_ },
            ref = {
              include module,
              variant = "TapDance",
              json = new_index,
              rust_expr = "%{module}::Ref(%{std.to_string new_index})",
            },
          },
      },

      config = {
        Json = {
          timeout | optional | Number,
        },

        rust_expr =
          if std.record.has_field "tap_dance" json_keymap.config then
            let tap_dance_config = json_keymap.config.tap_dance in
            let timeout_field_fragment =
              if std.record.has_field "timeout" tap_dance_config then
                "timeout: %{std.to_string tap_dance_config.timeout},"
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
            "smart_keymap::key::tap_dance::DEFAULT_CONFIG",
      },

      system = {
        rust_expr =
          let tap_dance_data = (key_data_and_refs.key_data & { tap_dance | default = [] }).tap_dance in
          "smart_keymap::key::tap_dance::System::new(%{tap_dance_data |> lib.array_rust_expr})",
      },
    },

  checks.check_tap_hold = {
    # Use a json value to check the "is" predicate.
    check_json_tap_keyboard_hold_keyboard_is =
      let json = {
        hold = { key_code = 224 },
        tap = { key_code = 4 },
      }
      in
      tap_hold.key.is_json json,

    check_taphold_codegen_values = {
      # Current impl: tap_hold::Key<keyboard::Key> is still th::Key<composite::Base>
      check_taphold_keyboard =
        let json = {
          hold = { key_code = 224 },
          tap = { key_code = 4 },
        }
        in
        let cv = tap_hold.key.codegen_values json in
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
      TapHoldInterruptResponseJson =
        std.contract.from_validator (
          validators.is_elem_of [
            "HoldOnKeyPress",
            "HoldOnKeyTap",
            "Ignore",
          ]
        ),

      module = "smart_keymap::key::tap_hold",

      key = {
        Json = std.contract.from_validator json_validator,

        key_type = "%{module}::Key",

        # c.f. doc_de_tap_hold.md.
        # JSON serialization of key::tap_hold::Key is { tap: key, hold: key }
        json_validator =
          validators.record.validator {
            fields_validator = validators.record.has_exact_fields ["tap", "hold"],
            field_validators = {
              tap = smart_key.json_validator,
              hold = smart_key.json_validator,
            },
          },

        is_json = fun json => 'Ok == json_validator json,

        codegen_values = fun json @ { hold, tap } =>
          let hold_cv = hold |> smart_key.codegen_values in
          let tap_cv = tap |> smart_key.codegen_values in

          {
            nested = {
              tap = tap_cv,
              hold = hold_cv,
            },
            include json,
            include module,
            include key_type,
            rust_expr =
              let hold_expr = nested.hold.rust_expr in
              let tap_expr = nested.tap.rust_expr in
              "%{module}::Key::new(%{tap_expr}, %{hold_expr})",
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
          let acc = smart_key.traverse f acc cv.nested.tap in
          smart_key.traverse f acc cv.nested.hold,

        data_and_ref = fun key_data cv @ { nested = { tap = tap_cv, hold = hold_cv }, .. } =>
          let { key_data, ref = tap_ref } = smart_key.data_and_ref key_data tap_cv in
          let tap_ref = tap_ref |> composite.ref.wrap in
          let { key_data, ref = hold_ref } = smart_key.data_and_ref key_data hold_cv in
          let hold_ref = hold_ref |> composite.ref.wrap in
          let { tap_hold = tap_hold_, ..other_data } = key_data & { tap_hold | default = [] } in
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

      config = {
        Json = {
          timeout | optional | Number,
          interrupt_response | optional | TapHoldInterruptResponseJson,
          required_idle_time | optional | Number,
        },

        rust_expr =
          if std.record.has_field "tap_hold" json_keymap.config then
            let tap_hold_config = json_keymap.config.tap_hold in
            let timeout_field_fragment =
              if std.record.has_field "timeout" tap_hold_config then
                "timeout: %{std.to_string tap_hold_config.timeout},"
              else
                ""
            in
            let interrupt_response_field_fragment =
              if std.record.has_field "interrupt_response" tap_hold_config then
                "interrupt_response: smart_keymap::key::tap_hold::InterruptResponse::%{tap_hold_config.interrupt_response},"
              else
                ""
            in
            let required_idle_time_field_fragment =
              if std.record.has_field "required_idle_time" tap_hold_config then
                "required_idle_time: Some(%{std.to_string tap_hold_config.required_idle_time}),"
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
            "smart_keymap::key::tap_hold::DEFAULT_CONFIG",
      },

      system = {
        rust_expr =
          let tap_hold_data = (key_data_and_refs.key_data & { tap_hold | default = [] }).tap_hold in
          "smart_keymap::key::tap_hold::System::new(%{tap_hold_data |> lib.array_rust_expr})",
      },
    },

  checks.check_layered = {
    # Use a json value to check the "is" predicate.
    check_json_base_keyboard_layered_keyboard_is =
      let json = {
        base = { key_code = 4 },
        layered = [
          null,
          { key_code = 6 },
        ]
      }
      in
      layered.layered_key.is_json json,

    check_layered_codegen_values = {
      # A layered::LayeredKey of keyboard::Key
      #  codegens to layered::LayeredKey<keyboard::Key>
      check_layered_keyboard =
        let json = {
          base = { key_code = 4 },
          layered = [
            null,
            { key_code = 6 },
          ]
        }
        in
        let cv = layered.layered_key.codegen_values json in
        {
          check_rust_expr = {
            actual = cv.rust_expr,
            expected = "smart_keymap::key::layered::LayeredKey::new(smart_keymap::key::keyboard::Key::new(4), [None,Some(smart_keymap::key::keyboard::Key::new(6))])",
          },
        },

      check_layered_composite_taphold_keyboard =
        let json = {
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
        let cv = layered.layered_key.codegen_values json in
        {
          check_rust_expr = {
            actual = cv.rust_expr,
            expected = "smart_keymap::key::layered::LayeredKey::new(smart_keymap::key::tap_hold::Key::new(smart_keymap::key::keyboard::Key::new(4), smart_keymap::key::keyboard::Key::new(224)), [None,Some(smart_keymap::key::keyboard::Key::new(6))])",
          },
        },
    },
  },

  checks.layer_modifier_is = {
    # Use a json value to check the "is" predicate.
    check_json_is =
      let json = { Hold = 0 } in
      layered.modifier_key.is_json json
  },

  layered
    | doc "for key::layered::LayeredKey."
    = {
      module = "smart_keymap::key::layered",

      modifier_key = {
        Json = std.contract.from_validator json_validator,

        key_type = "%{module}::ModifierKey",

        # c.f. doc_de_layered.md.
        # JSON serialization of key::layered::ModifierKey has variants: Default(layer), Hold(layer), SetActiveLayers([layer, ...]).
        json_validator =
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

        is_json = fun json => 'Ok == json_validator json,

        codegen_values = fun json =>
          json
          |> match {
            { Default = layer_index } =>
              {
                include json,
                include module,
                include key_type,
                variant = "Default",
                rust_expr = "%{module}::ModifierKey::default(%{std.to_string layer_index})",
              },
            { Hold = layer_index } =>
              {
                include json,
                include module,
                include key_type,
                variant = "Hold",
                rust_expr = "%{module}::ModifierKey::hold(%{std.to_string layer_index})",
              },
            { SetActiveLayers = active_layers_bitset } =>
              {
                include json,
                include module,
                include key_type,
                variant = "SetActiveLayers",
                rust_expr = "%{module}::ModifierKey::set_active_layers_from_bitset(%{active_layers_bitset |> std.to_string})",
              },
          },

        traverse = fun f acc cv => f acc cv,

        data_and_ref = fun key_data cv @ { json, .. } =>
          let { layer_modifiers = layer_modifiers_, ..other_data } = key_data & { layer_modifiers | default = [] } in
          let new_index = std.array.length layer_modifiers_ in
          let new_key = {
            include json,
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

      layered_key = {
        Json = std.contract.from_validator json_validator,

        key_type = "%{module}::LayeredKey",

        # c.f. doc_de_layered.md.
        # e.g.:
        # ```
        #   {
        #     "base": 4,
        #     "layered": [5, null, 7]
        #   }
        # ```
        # JSON serialization of key::layered::Layered has fields: { base, layered }
        json_validator =
          validators.record.validator {
            fields_validator = validators.record.has_exact_fields ["base", "layered"],
            field_validators = {
              base = smart_key.json_validator,
              layered =
                validators.array.validator (
                  validators.any_of [
                    validators.is_null,
                    smart_key.json_validator,
                  ]
                ),
            },
          },

        is_json = fun json => 'Ok == json_validator json,

        codegen_values = fun json @ { base, layered } =>
          let base_cv = base |> smart_key.codegen_values in
          let non_null_layered = std.array.filter ((!=) null) layered in
          let layered_cvs =
            std.array.map (fun json => if json != null then json |> smart_key.codegen_values else null) layered
          in
          let nn_layered_cvs =
            std.array.filter ((!=) null) layered_cvs
          in

          {
            nested = {
              base = base_cv,
              layered = layered_cvs,
            },
            include json,
            include module,
            include key_type,
            rust_expr =
              let base_expr = nested.base.rust_expr in
              let layered_exprs =
                nested.layered
                |> std.array.map (fun cv => if cv == null then "None" else "Some(%{cv.rust_expr})")
                |> std.string.join ","
              in
              "smart_keymap::key::layered::LayeredKey::new(%{base_expr}, [%{layered_exprs}])",
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
          let acc = smart_key.traverse f acc cv.nested.base in
          let nn_layered_cvs =
            std.array.filter ((!=) null) cv.nested.layered
          in
          std.array.fold_left
            (fun acc nested_cv => smart_key.traverse f acc nested_cv)
            acc
            nn_layered_cvs,

        data_and_ref = fun key_data cv @ { nested = { base = base_cv, layered = layered_cvs }, .. } =>
          let { key_data, ref = base_ref } = smart_key.data_and_ref key_data base_cv in
          let base_ref = base_ref |> composite.ref.wrap in
          let { key_data, layered_refs } =
            std.array.fold_left
              (fun { key_data, layered_refs = parts } cv =>
                if cv == null then
                  { include key_data, layered_refs = std.array.append null parts }
                else
                  let { key_data, ref = layer_ref } = smart_key.data_and_ref key_data cv in
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

      system = {
        rust_expr =
          let layer_modifiers_data = (key_data_and_refs.key_data & { layer_modifiers | default = [] }).layer_modifiers in
          let layered_data = (key_data_and_refs.key_data & { layered | default = [] }).layered in
          m%"
          smart_keymap::key::layered::System::new(
            %{layer_modifiers_data |> lib.array_rust_expr},
            %{layered_data |> lib.array_rust_expr},
          )
        "%,
      },
    },

  chorded
    | doc "for key::chorded::Key."
    = {
      module = "smart_keymap::key::chorded",

      key = {
        Json = std.contract.from_validator json_validator,

        key_type = "%{module}::Key",

        # JSON serialization of key::chorded::Key is { chords: [[id, key]], passthrough: key }
        json_validator =
          validators.record.validator {
            fields_validator = validators.record.has_exact_fields ["chords", "passthrough"],
            field_validators = {
              chords =
                validators.array.validator (validators.tuple.validator [validators.is_number, smart_key.json_validator]),
              passthrough = smart_key.json_validator,
            },
          },

        is_json = fun json => 'Ok == json_validator json,

        map_chords = fun f chords => chords |> std.array.map (fun [id, ch_json] => [id, ch_json |> f]),

        codegen_values = fun json @ { chords, passthrough } =>
          let chords_cv = chords |> map_chords (fun ch_json => ch_json |> smart_key.codegen_values) in
          let passthrough_cv = passthrough |> smart_key.codegen_values in

          {
            nested = {
              chords = chords_cv,
              passthrough = passthrough_cv,
            },
            include json,
            include module,
            include key_type,
            rust_expr =
              let chords_expr = m%"&[%{
              nested.chords
              |> std.array.map (fun [i, ch_cv] => "(%{i |> std.to_string}, %{ch_cv.rust_expr})")
              |> std.string.join ", "
            }]"%
              in
              let passthrough_expr = nested.passthrough.rust_expr in
              "smart_keymap::key::chorded::Key::new(%{chords_expr}, %{passthrough_expr})",
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
          let acc = std.array.fold_left (fun acc ch_cv => smart_key.traverse f acc ch_cv) acc chords_cvs in
          smart_key.traverse f acc cv.nested.passthrough,

        data_and_ref = fun key_data cv @ { nested = { chords = chords_cvs, passthrough = passthrough_cv }, .. } =>
          let { key_data, chord_indices_and_refs } =
            std.array.fold_left
              (fun { key_data, chord_indices_and_refs = parts } [chord_index, cv] =>
                let { key_data, ref = chord_ref } = smart_key.data_and_ref key_data cv in
                let chord_ref = chord_ref |> composite.ref.wrap in
                { include key_data, chord_indices_and_refs = std.array.append [chord_index, chord_ref] parts }
              )
              { include key_data, chord_indices_and_refs = [] }
              chords_cvs
          in
          let { key_data, ref = passthrough_ref } = smart_key.data_and_ref key_data passthrough_cv in
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

      auxiliary_key = {
        Json = std.contract.from_validator json_validator,

        key_type = "%{module}::AuxiliaryKey",

        # JSON serialization of key::chorded::AuxiliaryKey is { passthrough: key }
        json_validator =
          validators.record.validator {
            fields_validator = validators.record.has_exact_fields ["passthrough"],
            field_validators = {
              passthrough = smart_key.json_validator,
            },
          },

        is_json = fun json => 'Ok == json_validator json,

        codegen_values = fun json @ { passthrough } =>
          let passthrough_cv = passthrough |> smart_key.codegen_values in

          {
            nested = {
              passthrough = passthrough_cv,
            },
            include json,
            include module,
            include key_type,
            rust_expr =
              let passthrough_expr = nested.passthrough.rust_expr in
              "smart_keymap::key::chorded::AuxiliaryKey::new(%{passthrough_expr})",
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
          smart_key.traverse f acc cv.nested.passthrough,

        data_and_ref = fun key_data cv @ { nested = { passthrough = passthrough_cv }, .. } =>
          let { key_data, ref = passthrough_ref } = smart_key.data_and_ref key_data passthrough_cv in
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

      config = {
        ChordIndicesJson = Array Number,

        Json = {
          chords | optional | Array ChordIndicesJson,
          timeout | optional | Number,
        },

        rust_expr =
          if std.record.has_field "chorded" json_keymap.config then
            let chorded_config = json_keymap.config.chorded in
            let timeout_field_fragment =
              if std.record.has_field "timeout" chorded_config then
                "timeout: %{std.to_string chorded_config.timeout},"
              else
                ""
            in
            let chords_field_fragment =
              if std.record.has_field "chords" chorded_config then
                let chords_fragment =
                  chorded_config.chords
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
            "smart_keymap::key::chorded::DEFAULT_CONFIG",
      },

      system = {
        rust_expr =
          let chorded_key_data = (key_data_and_refs.key_data & { chorded | default = [] }).chorded in
          let auxiliary_key_data = (key_data_and_refs.key_data & { chorded_auxiliary | default = [] }).chorded_auxiliary in
          m%"
        smart_keymap::key::chorded::System::new(
            %{chorded_key_data |> lib.array_rust_expr},
            %{auxiliary_key_data |> lib.array_rust_expr},
        )
      "%,
      },
    },

  composite = {
    ref = {
      wrap = fun ref @ { module, json, rust_expr, .. } =>
        let variant =
          module
          |> match {
            "smart_keymap::key::keyboard" => "Keyboard",
            "smart_keymap::key::callback" => "Callback",
            "smart_keymap::key::caps_word" => "CapsWord",
            "smart_keymap::key::sticky" => "Sticky",
            "smart_keymap::key::custom" => "Custom",
            "smart_keymap::key::tap_dance" => "TapDance",
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

    config = {
      Json =
        let Chorded = chorded.config.Json in
        let Sticky = sticky.config.Json in
        let TapDance = tap_dance.config.Json in
        let TapHold = tap_hold.config.Json in
        {
          chorded | optional | Chorded,
          sticky | optional | Sticky,
          tap_dance | optional | TapDance,
          tap_hold | optional | TapHold,
        },

      rust_expr = m%"smart_keymap::key::composite::Config {
            chorded: %{chorded.config.rust_expr},
            sticky: %{sticky.config.rust_expr},
            tap_dance: %{tap_dance.config.rust_expr},
            tap_hold: %{tap_hold.config.rust_expr},
            ..smart_keymap::key::composite::DEFAULT_CONFIG
        }"%,
    },

    system = {
      rust_expr = m%"
        smart_keymap::key::composite::System::array_based(
          %{keyboard.system.rust_expr},
          %{callback.system.rust_expr},
          %{sticky.system.rust_expr},
          %{tap_dance.system.rust_expr},
          %{tap_hold.system.rust_expr},
          %{layered.system.rust_expr},
          %{chorded.system.rust_expr},
        )
      "%,
    },
  },

  smart_key = {
    Json = std.contract.from_validator json_validator,

    codegen_modules = [
      callback.key,
      caps_word.key,
      chorded.auxiliary_key,
      chorded.key,
      custom.key,
      keyboard.key,
      layered.layered_key,
      layered.modifier_key,
      sticky.key,
      tap_dance.key,
      tap_hold.key,
    ],

    json_validator =
      let json_validators = codegen_modules |> std.array.map (fun m => m.json_validator) in
      validators.any_of json_validators,

    is_json = fun json => 'Ok == json_validator json,

    codegen_values = fun json =>
      codegen_modules
      |> std.array.filter (fun m => m.is_json json)
      |> match {
        [m] => m.codegen_values json,
        _ => std.fail_with "bad json for key: %{json |> std.serialize 'Json}",
      },

    codegen_module_for_key_type = fun key_type =>
      codegen_modules
      |> std.array.filter (fun m => m.key_type == key_type)
      |> match {
        [m] => m,
        _ => std.fail_with "bad key_type for smart_key.codegen_module_for_key_type: %{key_type}",
      },

    codegen_module_for_codegen_value = fun cv @ { key_type, .. } =>
      codegen_module_for_key_type key_type,

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
      let keymap_len = std.array.length json_keymap.keys in
      let max_tap_dance_definitions =
        # For each of the keys,
        #  compute max length of any tap dance definitions.
        let max_td_defs =
          std.array.map
            (fun cv =>
              smart_key.traverse
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
              smart_key.traverse
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
      let chords = ((json_keymap.config & { chorded | default = {} }).chorded & { chords | default = [] }).chords in
      let chord_count = chords |> std.array.length in
      let max_chord_size = chords |> std.array.map std.array.length |> std.array.fold_left std.number.max 0 in
      let max_overlapping_chord_size =
        chords
        |> std.array.map (fun [i, ..] => std.to_string i)
        |> std.array.fold_left
          (fun r i =>
            std.record.update i (1 + (std.record.get_or i 0 r)) r
          )
          {}
        |> std.record.values
        |> std.array.fold_left std.number.max 0
      in
      let { key_data, key_refs } = key_data_and_refs in
      let data_len_rust_expr = fun field_name =>
        let data = (key_data & { "%{field_name}" | default = [] })."%{field_name}" in
        data |> std.array.length |> std.to_string
      in
      let key_refs_expr = "[%{key_refs |> std.array.map (fun { rust_expr, .. } => rust_expr) |> std.string.join ", "}]" in
      {
        config = composite.config.rust_expr,
        context = "smart_keymap::key::composite::Context::from_config(%{config})",
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

    smart_keymap::keymap::Keymap::new(KEY_REFS, CONTEXT, %{composite.system.rust_expr})
}
"%,
        keymap_module = m%"
/// Types and initial data used for constructing [KEYMAP].
pub mod init {
    use crate as smart_keymap;

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

    pub use smart_keymap::key::composite::Ref;

    pub use smart_keymap::key::composite::Context;

    pub use smart_keymap::key::composite::Event;

    pub use smart_keymap::key::composite::PendingKeyState;

    pub use smart_keymap::key::composite::KeyState;

    const KEYBOARD_DATA_LEN: usize = %{data_len_rust_expr "keyboard"};
    const CALLBACK_DATA_LEN: usize = %{data_len_rust_expr "callback"};
    const STICKY_DATA_LEN: usize = %{data_len_rust_expr "sticky"};
    const TAP_DANCE_DATA_LEN: usize = %{data_len_rust_expr "tap_dance"};
    const TAP_HOLD_DATA_LEN: usize = %{data_len_rust_expr "tap_hold"};
    const LAYER_MODIFIERS_DATA_LEN: usize = %{data_len_rust_expr "layer_modifiers"};
    const LAYERED_DATA_LEN: usize = %{data_len_rust_expr "layered"};
    const CHORDED_DATA_LEN: usize = %{data_len_rust_expr "chorded"};
    const CHORDED_AUXILIARY_DATA_LEN: usize = %{data_len_rust_expr "chorded_auxiliary"};

    /// The System type
    pub type System = smart_keymap::key::composite::System<
            smart_keymap::key::composite::KeyArrays<
                KEYBOARD_DATA_LEN,
                CALLBACK_DATA_LEN,
                STICKY_DATA_LEN,
                TAP_DANCE_DATA_LEN,
                TAP_HOLD_DATA_LEN,
                LAYER_MODIFIERS_DATA_LEN,
                LAYERED_DATA_LEN,
                CHORDED_DATA_LEN,
                CHORDED_AUXILIARY_DATA_LEN,
            >,
        >;

    /// The number of keys in the keymap.
    pub const KEY_COUNT: usize = %{std.to_string keymap_len};

    /// The key references.
    pub const KEY_REFS: [Ref; KEY_COUNT] = %{key_refs};

    /// The keymap config.
    pub const CONFIG: smart_keymap::key::composite::Config = %{config};

    /// Initial [Context] value.
    pub const CONTEXT: Context = %{context};

    /// The key system.
    pub const SYSTEM: System = %{composite.system.rust_expr};

    /// Alias for the [keymap::Keymap] type.
    pub type Keymap = smart_keymap::keymap::Keymap<
        [Ref; KEY_COUNT],
        Ref,
        Context,
        Event,
        PendingKeyState,
        KeyState,
        System,
    >;
}
"%,
      },

  keymap_rs
    | doc "Text contents of the keymap.rs generated from the keymap.json"
    = rust_expressions.keymap_module,
}
