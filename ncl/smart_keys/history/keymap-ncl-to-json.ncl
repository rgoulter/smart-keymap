{
  validators,

  keyboard_modifiers,

  keymap_ncl.history
    | doc "for key::history::Key."
    = {
      # Author-facing alt-repeat rule: keyboard-style keys (K.Left, …).
      # Converted to KeyOutput JSON in json_keymap / to_json_config.
      AltRepeatRuleNcl = {
        prev | keymap_ncl.keyboard.Key,
        emit | keymap_ncl.keyboard.Key,
      },

      Config = {
        alt_repeat
          | optional
          | Array AltRepeatRuleNcl,
      },

      Key = std.contract.from_validator key_validator,

      key_validator = fun k =>
        k
        |> match {
          "Repeat" => 'Ok,
          "AltRepeat" => 'Ok,
          _ => 'Error { message = "Expected \"Repeat\" or \"AltRepeat\"" },
        },

      is_key = fun k => 'Ok == key_validator k,

      to_json_value = fun key => key,

      # Leaves have no nested keys; map_accum maps children only.
      map_accum = fun f acc k => { include acc, include k },

      # Keyboard key (author form) → KeyOutput JSON used by codegen / serde.
      keyboard_key_to_key_output = fun k =>
        k
        |> match {
          { key_code = kc, modifiers = km } =>
            {
              key_code = { Keyboard = kc },
              key_modifiers = keyboard_modifiers.to_json_value km,
            },
          { key_code = kc } => { key_code = { Keyboard = kc } },
          { modifiers = km } =>
            {
              key_modifiers = keyboard_modifiers.to_json_value km,
            },
          {} => {},
        },

      alt_repeat_rule_to_json = fun rule =>
        {
          prev = keyboard_key_to_key_output rule.prev,
          emit = keyboard_key_to_key_output rule.emit,
        },

      config_to_json = fun c =>
        (
          if std.record.has_field "alt_repeat" c then
            {
              alt_repeat =
                c.alt_repeat |> std.array.map alt_repeat_rule_to_json,
            }
          else
            {}
        ),
    },

  checks.check_history =
    let K = import "keys.ncl" in
    {
      check_repeat_ok =
        keymap_ncl.history.key_validator "Repeat" == 'Ok,

      check_alt_repeat_ok =
        keymap_ncl.history.key_validator "AltRepeat" == 'Ok,

      check_keyboard_to_key_output_left = {
        actual = keymap_ncl.history.keyboard_key_to_key_output K.Left,
        expected = { key_code = { Keyboard = 80 } },
      },

      check_alt_repeat_rule_json = {
        actual =
          keymap_ncl.history.alt_repeat_rule_to_json {
            prev = K.Left,
            emit = K.Right,
          },
        expected = {
          prev = { key_code = { Keyboard = 80 } },
          emit = { key_code = { Keyboard = 79 } },
        },
      },
    },
}
