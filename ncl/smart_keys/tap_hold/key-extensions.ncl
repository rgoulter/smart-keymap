{
  key_extensions.tap_hold = fun keys =>
    {
      hold
        | doc "creates a hold key modifier"
        = fun key => { hold = key },

      # Autoshift: tap → key, hold past timeout → key with LeftShift.
      # Implemented as tap-hold sugar; timeout / interrupt come from
      # `config.tap_hold` (see features/keymap/key/autoshift.feature).
      autoshift
        | doc m%"
            Autoshift a keyboard key: tap for the unshifted key,
            hold past `config.tap_hold.timeout` for Shift+key.

            Equivalent to `k & hold (k & LeftShift)`.
          "%
        = fun k => k & hold (k & keys.LeftShift),

      autoshift_as
        | doc m%"
            Autoshift with an explicit hold key (custom shifted value).

            Equivalent to `tap & hold hold_key`.
          "%
        = fun tap hold_key => tap & hold hold_key,

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
