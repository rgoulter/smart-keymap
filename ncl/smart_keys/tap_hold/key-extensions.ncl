{
  key_extensions.tap_hold = fun keys =>
    {
      hold
        | doc "creates a hold key modifier"
        = fun key => { hold = key },

      # Select a named (or numeric) tap-hold behavior profile.
      # Names resolve against config.tap_hold.profiles in keymap.ncl → json.
      # Example: K.A & K.hold K.LeftCtrl & K.tap_hold_profile "hml"
      tap_hold_profile
        | doc "selects a tap-hold behavior profile by name or index"
        = fun name_or_index => { tap_hold_profile = name_or_index },

      H_LCtrl = hold keys.LeftCtrl,
      H_LShift = hold keys.LeftShift,
      H_LAlt = hold keys.LeftAlt,
      H_LGUI = hold keys.LeftGUI,
      H_RCtrl = hold keys.RightCtrl,
      H_RShift = hold keys.RightShift,
      H_RAlt = hold keys.RightAlt,
      H_RGUI = hold keys.RightGUI,
    },
}
