{
  validators,

  lib,

  key_data_and_refs,

  smart_keymap.callback
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
                    fields_validator = validators.record.has_exact_fields ["Bluetooth"],
                    field_validators = {
                      Bluetooth =
                        validators.any_of [
                          validators.equals_string "Clear",
                          validators.equals_string "ClearAll",
                          validators.equals_string "Disconnect",
                          validators.equals_string "Next",
                          validators.equals_string "Previous",
                          validators.record.validator {
                            fields_validator = validators.record.has_exact_fields ["Select"],
                            field_validators = {
                              Select = validators.is_number,
                            },
                          },
                        ],
                    },
                  },
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
                  { Bluetooth = { Select } } =>
                    m%"
                      smart_keymap::keymap::KeymapCallback::Bluetooth(
                        BluetoothProfileCommand::Select(%{Select |> std.to_string})
                      )
                    "%,
                  { Bluetooth } =>
                    m%"
                      smart_keymap::keymap::KeymapCallback::Bluetooth(
                        BluetoothProfileCommand::%{Bluetooth}
                      )
                    "%,
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
          "%{module}::System::new(%{callback_data |> lib.array_rust_expr})",
      },
    },
}
