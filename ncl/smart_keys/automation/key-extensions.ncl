{
  key_extensions = {
    automation = {
      string_macro = fun s =>
        {
          automation_instructions =
            let K = import "keys.ncl" in
            s
            |> std.string.characters
            |> std.array.map (fun char =>
              let char_upper = std.string.uppercase char in
              let is_upper = std.string.contains char "ABCDEFGHIJKLMNOPQRSTUVWXYZ" in
              let { key_code = kc, modifiers = km_ncl } = K."%{char_upper}" & { modifiers = {} } in
              let km_ncl = if is_upper then km_ncl & K.LeftShift.modifiers else km_ncl in
              let km =
                km_ncl
                |> std.record.fields
                |> std.array.map (fun f =>
                  {
                    left_ctrl = 1,
                    left_shift = 2,
                    left_alt = 4,
                    left_gui = 8,
                    right_ctrl = 16,
                    right_shift = 32,
                    right_alt = 64,
                    right_gui = 128,
                  }."%{f}"
                )
                |> std.array.fold_left (+) 0
              in
              let ko = { key_code = { Keyboard = kc } } & (if km != 0 then { key_modifiers = km } else {}) in
              [
                { Tap = ko, },
              ]
            )
            |> std.array.flatten,
        },
    },
  },
}
