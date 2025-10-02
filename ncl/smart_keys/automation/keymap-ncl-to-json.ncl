{
  validators,

  key_output,

  smart_keymap.automation,

  checks.check_automation_key = {
    check_key_validation = {
      actual =
        let ko = { key_code = { Keyboard = 4 } } in
        let press_ins = { Press = ko } in
        let release_ins = { Release = ko } in
        let ins = [press_ins, release_ins] in
        keymap_ncl.automation.key_validator { automation_instructions = ins },
      expected = 'Ok,
    },

    check_simple_macro_key_value = {
      actual =
        let K = import "keys.ncl" in
        K.string_macro "a",
      expected =
        let ko = { key_code = { Keyboard = 4 } } in
        let tap_ins = { Tap = ko } in
        let ins = [tap_ins] in
        { automation_instructions = ins },
    },
  },

  checks.check_automation_transformed_key = {
    check_key_validation = {
      actual =
        let exec = { start = 0, length = 6 } in
        keymap_ncl.automation.key_validator { automation_instructions = exec },
      expected = 'Ok,
    },
  },

  keymap_ncl.automation
    | doc "for key::automation::Key."
    = {
      Config = {
        instruction_duration | optional | Number,
        ..
      },

      Key = std.contract.from_validator key_validator,

      key_validator =
        validators.record.validator {
          fields_validator =
            validators.record.has_only_fields ["automation_instructions"],
          field_validators = {
            automation_instructions =
              validators.any_of [
                smart_keymap.automation.execution.json_validator,
                validators.array.validator smart_keymap.automation.instruction.json_validator
              ],
          },
        },

      is_key = fun k => 'Ok == key_validator k,

      # The automation key needs to be transformed in keymap-ncl-to-json.ncl
      # (concatenating the instructions to config.automation.instructions,
      #  & replacing the automation_instructions field with an Execution value).
      to_json_value = fun k @ { automation_instructions } =>
        k,

      transform_keys_to_instructions = fun acc k =>
        k
        |> match {
          { automation_instructions } if std.is_array automation_instructions =>
            let k =
              let start = std.array.length acc.automation_instructions in
              let length = std.array.length automation_instructions in
              {
                automation_instructions = {
                  include start,
                  include length,
                },
              }
            in
            let { automation_instructions = acc_instructions, ..other_acc } = acc in
            let automation_instructions = acc_instructions @ automation_instructions in
            let acc = other_acc & { include automation_instructions, } in
            { include acc, include k },
          { automation_instructions } =>
            std.fail_with "Expected automation_instructions to be an array",
          _ => { include acc, include k },
        },

      map_accum = fun f acc k =>
        f acc k,
    },
}
