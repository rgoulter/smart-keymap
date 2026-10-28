{
  key_extensions.mod_conditioned = fun keys =>
    {
      # Classic Grave Escape: Escape alone; Shift/GUI → Grave, trigger mods suppressed.
      gresc = {
        base = keys.Escape,
        morphed = keys.Grave,
        mods = {
          left_shift = true,
          right_shift = true,
          left_gui = true,
          right_gui = true,
        },
      },

      # Backspace alone; either Shift → Delete. Keep Right Shift in the report.
      bspc_del = {
        base = keys.Backspace,
        morphed = keys.Delete,
        mods = {
          left_shift = true,
          right_shift = true,
        },
        keep_mods = {
          right_shift = true,
        },
      },
    },
}
