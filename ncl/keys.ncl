let hid_codes = import "hid-usage-keyboard.ncl" in
hid_codes
|> std.record.map_values (fun kc => { key_code = kc })

& {
  hold
  | doc "creates a hold key modifier"
  # TapHold's hold is just the key_code, not a nested key.
  = fun { key_code, .. } => { hold = key_code },

  LeftCtrl,
  LeftShift,
  LeftAlt,
  LeftGUI,
  RightCtrl,
  RightShift,
  RightAlt,
  RightGUI,

  H_LCtrl = hold LeftCtrl,
  H_LShift = hold LeftShift,
  H_LAlt = hold LeftAlt,
  H_LGUI = hold LeftGUI,
  H_RCtrl = hold RightCtrl,
  H_RShift = hold RightShift,
  H_RAlt = hold RightAlt,
  H_RGUI = hold RightGUI,
}
