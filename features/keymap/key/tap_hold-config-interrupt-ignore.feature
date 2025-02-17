Feature: TapHold Key (configure interrupt response: ignore)

  The tap hold key's response to interruptions can be configured.

  "Interrupts ignored" can be configured by setting `config.tap_hold.interrupt_response`
   to `"Ignore"` in `keymap.ncl`.

  "Ignore interrupts" just means the key only acts as hold by holding the key
   for longer than the tap-hold timeout duration.

  Background:

    Let's demonstrate tap-hold "ignore interrupt" behaviour
    using a keymap with a tap-hold key, and a keyboard key:

    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      {
        config.tap_hold.interrupt_response = "Ignore",
        keys = [
          K.A & K.hold K.LeftCtrl,
          K.B
        ]
      }
      """

  Example: rolling key presses (press TH(A), press B, release TH(A), release B)

    Rolling the tap-hold key with another key
    (i.e. interrupting the tap-hold key with another key press),
     releasing the tap-hold key with `interrupt_response = "Ignore"`
     resolves the key as tap.

    When the keymap registers the following input
      """
      [
        press (K.A & K.hold K.LeftCtrl),
        press (K.B),
        release (K.A & K.hold K.LeftCtrl),
        release (K.B),
      ]
      """
    Then the output should be equivalent to output from
      """
      [
        press (K.A),
        press (K.B),
        release (K.A),
        release (K.B),
      ]
      """

  Example: interrupting tap (press TH(A), press B, release B, release TH(A))

    After interrupting the tap-hold key with another key tap (press & release),
     releasing the tap-hold key with `interrupt_response = "Ignore"`
     resolves the key as tap.

    When the keymap registers the following input
      """
      [
        press (K.A & K.hold K.LeftCtrl),
        press (K.B),
        release (K.B),
        release (K.A & K.hold K.LeftCtrl),
      ]
      """
    Then the output should be equivalent to output from
      """
      [
        press (K.A),
        press (K.B),
        release (K.B),
        release (K.A),
      ]
      """
