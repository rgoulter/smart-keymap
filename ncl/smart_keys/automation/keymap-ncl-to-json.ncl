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
        keymap_ncl.automation.key_validator { automation_instructions.on_press = ins },
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
        { automation_instructions.on_press = ins },
    },
  },

  checks.check_automation_transformed_key = {
    check_key_validation = {
      actual =
        let exec = { start = 0, length = 6 } in
        keymap_ncl.automation.key_validator { automation_instructions.on_press = exec },
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

      key_instructions = {
        ncl_validator =
          validators.record.validator {
            fields_validator =
              validators.record.has_only_fields ["on_press", "while_pressed", "on_release"],
            field_validators =
              let instructions_or_execution =
                validators.any_of [
                  smart_keymap.automation.execution.json_validator,
                  validators.array.validator smart_keymap.automation.instruction.json_validator
                ]
              in
              {
                on_press = instructions_or_execution,
                while_pressed = instructions_or_execution,
                on_release = instructions_or_execution,
              },
          },
      },

      key_validator =
        validators.record.validator {
          fields_validator =
            validators.record.has_only_fields ["automation_instructions"],
          field_validators = {
            automation_instructions = key_instructions.ncl_validator,
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
          { automation_instructions = key_instructions } =>
            let { automation_instructions = acc_instructions, ..other_acc } = acc in
            let transform = fun acc_instructions instructions =>
              let start = std.array.length acc_instructions in
              let length = std.array.length instructions in
              let acc_instructions = acc_instructions @ instructions in
              {
                execution = { include start, include length, },
                include acc_instructions,
              }
            in
            let key_instructions =
              key_instructions
              & {
                on_press | default = [],
                while_pressed | default = [],
                on_release | default = []
              }
            in
            let { acc_instructions, execution = on_press } = transform acc_instructions key_instructions.on_press in
            let { acc_instructions, execution = while_pressed } = transform acc_instructions key_instructions.while_pressed in
            let { acc_instructions, execution = on_release } = transform acc_instructions key_instructions.on_release in
            let k = {
              automation_instructions = {
                include on_press,
                include while_pressed,
                include on_release,
              },
            }
            in
            let automation_instructions = acc_instructions in
            let acc = other_acc & { include automation_instructions, } in
            { include acc, include k },
          _ => { include acc, include k },
        },

      map_accum = fun f acc k =>
        f acc k,
    },
}
