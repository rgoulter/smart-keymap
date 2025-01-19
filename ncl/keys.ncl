let key_extensions = {
  keyboard
    = fun K =>
      (import "hid-usage-keyboard.ncl")
      |> std.record.map_values
           (fun kc =>
             kc |> match {
               _ if 0xE0 <= kc && kc <= 0xE7 => {
                 modifiers = kc |> match {
                   0xE0 => { left_ctrl = true },
                   0xE1 => { left_shift = true },
                   0xE2 => { left_alt = true },
                   0xE3 => { left_gui = true },
                   0xE4 => { right_ctrl = true },
                   0xE5 => { right_shift = true },
                   0xE6 => { right_alt = true },
                   0xE7 => { right_gui = true },
                 },
               },

               kc => { key_code = kc },
             }),

  tap_hold
    = fun K => {
      hold
      | doc "creates a hold key modifier"
      = fun key => { hold = key },

      H_LCtrl = hold K.LeftCtrl,
      H_LShift = hold K.LeftShift,
      H_LAlt = hold K.LeftAlt,
      H_LGUI = hold K.LeftGUI,
      H_RCtrl = hold K.RightCtrl,
      H_RShift = hold K.RightShift,
      H_RAlt = hold K.RightAlt,
      H_RGUI = hold K.RightGUI,
    },

  layered
    = fun K => {
      layer_mod = {
        hold = fun layer_num => {
          layer_modifier = { hold = layer_num }
        },
      },

      # Layer Transparency
      TTTT = null,
    },
} in

let extend_keys = fun K key_module =>
  K & key_module K in

std.array.fold_left
  extend_keys
  {}
  [
    key_extensions.keyboard,
    key_extensions.tap_hold,
    key_extensions.layered,
  ]
