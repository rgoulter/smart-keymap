Feature: TapHold Key (configure interrupt response: hold on key press)

  The tap hold key's response to interruptions can be configured.

  "Resolves as 'Hold' when interrupted by key press" can be configured
   by setting `config.tap_hold.interrupt_response`
   to `"HoldOnKeyPress"` in `keymap.ncl`.

  Background:

    Let's demonstrate tap-hold "hold on interrupting key press" behaviour
    using a keymap with a tap-hold key, and a keyboard key:

    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      {
        config.tap_hold.interrupt_response = "HoldOnKeyPress",
        keys = [
          K.A & K.hold K.LeftCtrl,
          K.B
        ]
      }
      """

  Example: rolling key presses (press TH(A), press B)

    Rolling the tap-hold key
    with another key
    (i.e. interrupting the tap-hold key with another key press),
     for a tap-hold key configured with `interrupt_response = "HoldOnKeyPress"`
     resolves the tap-hold key as 'hold'.

    When the keymap registers the following input
      """
      [
        Press(keymap_index: 0),
        Press(keymap_index: 1),
      ]
      """
    Then the HID keyboard report should equal
      """
      let K = import "hid-usage-keyboard.ncl" in
      { modifiers = { left_ctrl = true }, key_codes = [K.B] }
      """

  Example: interrupting tap (press TH(A), press B, release B, release TH(A))

    Interrupting the tap-hold key with another key tap (press & release),
     for a tap-hold key configured with `interrupt_response = "HoldOnKeyPress"`
     resolves the key as "hold".

    When the keymap registers the following input
      """
      [
        Press(keymap_index: 0),
        Press(keymap_index: 1),
        Release(keymap_index: 1),
      ]
      """
    Then the HID keyboard report should equal
      """
      let K = import "hid-usage-keyboard.ncl" in
      { modifiers = { left_ctrl = true } }
      """
