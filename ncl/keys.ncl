let hid_codes = import "hid-usage-keyboard.ncl" in
hid_codes
|> std.record.map_values (fun kc => { key_code = kc })

& {
  hold
  | doc "creates a hold key modifier"
  # TapHold's hold is just the key_code, not a nested key.
  = fun { key_code, .. } => { hold = key_code },
}
