{
  validators,
  lib,
  key_data_and_refs,
  json_keymap,

  key_output,

  checks.check_automation_instruction = {
    check_json_validation = {
      actual =
        let ko = { key_code = { Keyboard = 4 } } in
        let ins = { Press = ko } in
        smart_keymap.automation.instruction.json_validator ins,
      expected = 'Ok,
    },
  },

  smart_keymap.automation
    | doc "for key::automation::Key."
    = {
      module = "smart_keymap::key::automation",

      instruction = {
        Json = std.contract.from_validator json_validator,

        json_validator =
          validators.any_of [
            validators.is_string, # NoOp
            validators.record.validator {
              fields_validator = validators.record.has_any_field_of ["Press", "Release", "Wait"],
              field_validators = {
                Press = key_output.json_validator,
                Release = key_output.json_validator,
                Wait = validators.is_number,
              },
            },
          ],

        is_json = fun json => 'Ok == json_validator json,

        rust_expr = fun json =>
          json
          |> match {
            "NoOp" => "%{module}::Instruction::NoOp",
            { Press } => "%{module}::Instruction::Press(%{key_output.rust_expr Press})",
            { Release } => "%{module}::Instruction::Release(%{key_output.rust_expr Release})",
            { Wait } => "%{module}::Instruction::Wait(%{std.to_string Wait})",
          },
      },

      execution = {
        Json = std.contract.from_validator json_validator,

        json_validator =
          validators.record.validator {
            fields_validator = validators.record.has_exact_fields ["start", "length"],
            field_validators = {
              start = validators.is_number,
              length = validators.is_number,
            },
          },

        is_json = fun json => 'Ok == json_validator json,

        rust_expr = fun { start, length } =>
          m%"%{module}::Execution {
            start: %{std.to_string start},
            length: %{std.to_string length},
          }"%,
      },

      key = {
        Json = std.contract.from_validator json_validator,

        key_type = "%{module}::Key",

        json_validator =
          validators.record.validator {
            fields_validator =
              validators.record.has_exact_fields ["automation_instructions"],
            field_validators = {
              automation_instructions = execution.json_validator,
            },
          },

        is_json = fun json => 'Ok == json_validator json,

        codegen_values = fun json =>
          {
            include json,
            include module,
            include key_type,
          },

        traverse = fun f acc cv => f acc cv,

        data_and_ref = fun key_data cv @ { json, .. } =>
          let { automation = automation_, ..other_data } = key_data & { automation | default = [] } in
          let new_index = std.array.length automation_ in
          let new_key = {
            include json,
            rust_expr = m%"
            %{module}::Key {
              automation_instructions: %{execution.rust_expr json.automation_instructions},
            }
          "%,
          }
          in
          {
            key_data = other_data & { automation = std.array.append new_key automation_ },
            ref = {
              include module,
              json = new_index,
              rust_expr = "%{module}::Ref(%{std.to_string new_index})",
            },
          },
      },

      config = {
        Json = {
          instructions | optional | Array instruction.Json,
          instruction_duration | optional | Number,
        },

        rust_expr =
          if std.record.has_field "automation" json_keymap.config then
            let automation_config = json_keymap.config.automation in
            let instructions_field_fragment =
              if std.record.has_field "instructions" automation_config then
                let instructions = automation_config.instructions in
                m%"instructions: %{module}::instructions([
                   %{instructions |> std.array.map instruction.rust_expr |> std.string.join ", "}
                ]),"%
              else
                ""
            in
            let instruction_duration_field_fragment =
              if std.record.has_field "instruction_duration" automation_config then
                "instruction_duration: %{std.to_string automation_config.instruction_duration},"
              else
                ""
            in
            m%"
            %{module}::Config {
                %{instructions_field_fragment}
                %{instruction_duration_field_fragment}
                ..%{module}::Config::new()
            }
          "%
          else
            "%{module}::Config::new()",
      },

      system = {
        rust_expr =
          let automation_data = (key_data_and_refs.key_data & { automation | default = [] }).automation in
          "%{module}::System::new(%{automation_data |> lib.array_rust_expr})",
      },
    },
}
