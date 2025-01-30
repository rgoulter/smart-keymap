{
  modifiers | default = {},

  key_codes | default = [],

  as_bytes
    =
      let modifier =
        modifiers |>
        std.record.map (fun mod is_pressed =>
          mod |> match {
            "left_ctrl"   if is_pressed =>   1, # 0x01
            "left_shift"  if is_pressed =>   2, # 0x02
            "left_alt"    if is_pressed =>   4, # 0x04
            "left_gui"    if is_pressed =>   8, # 0x08
            "right_ctrl"  if is_pressed =>  16, # 0x10
            "right_shift" if is_pressed =>  32, # 0x20
            "right_alt"   if is_pressed =>  64, # 0x40
            "right_gui"   if is_pressed => 128, # 0x80
            _ => 0,
          })
        |> std.record.values
        |> std.array.fold_left (+) 0 in
      let key_codes_6 = std.array.generate (fun idx => std.array.at_or idx 0 key_codes) 6 in
      std.array.concat [modifier, 0] key_codes_6
}
