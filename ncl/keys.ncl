let hid_codes = import "hid-usage-keyboard.ncl" in
hid_codes
|> std.record.map_values (fun kc => { key_code = kc })
