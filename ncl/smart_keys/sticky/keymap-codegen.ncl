{
  validators,

  lib,

  json_keymap,

  key_data_and_refs,

  keyboard_modifiers,

  smart_keymap.sticky
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
                  "%{module}::Key::new(%{keyboard_modifiers.rust_expr sticky_modifiers})",
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

        expr =
          if std.record.has_field "sticky" json_keymap.config then
            let c = json_keymap.config.sticky in
            (
              if std.record.has_field "activation" c then
                {
                  activation = "%{module}::StickyKeyActivation::%{c.activation}",
                }
              else
                {}
            )
            & (
              if std.record.has_field "release" c then
                {
                  release = "%{module}::StickyKeyRelease::%{c.release}",
                }
              else
                {}
            )
          else
            {},

        rust_expr = lib.config_rust_expr module expr,
      },

      system = {
        rust_expr =
          let sticky_data = (key_data_and_refs.key_data & { sticky | default = [] }).sticky in
          "%{module}::System::new(%{sticky_data |> lib.array_rust_expr})"
      },
    },
}
