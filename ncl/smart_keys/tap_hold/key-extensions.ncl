{
  key_extensions.tap_hold = {
    keys,

    hold
      | doc "creates a hold key modifier"
      = fun key => { hold = key },

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
