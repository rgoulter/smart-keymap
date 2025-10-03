let { string_to_instructions, .. } = import "smart_keys/automation/lib.ncl" in
{
  key_extensions = {
    automation = {
      string_macro = fun s =>
        {
          automation_instructions.on_press = string_to_instructions s,
        },
    },
  },
}
