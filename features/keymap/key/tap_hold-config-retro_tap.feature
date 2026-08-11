Feature: TapHold Key (configure retro_tap)

  The `retro_tap` config (ZMK `retro-tap`) means that
   timeout alone never resolves a tap-hold as hold.
   Hold activates only when another key interrupts;
   releasing alone always yields tap.

  For examples of this key in other smart keyboard firmware, see e.g.:

  - [ZMK's hold-tap, retro-tap](https://zmk.dev/docs/keymaps/behaviors/hold-tap#retro-tap)

  Background:

    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      {
        config.tap_hold.retro_tap = true,
        config.tap_hold.timeout = 200,
        config.tap_hold.interrupt_response = "HoldOnKeyPress",
        keys = [
          K.A & K.hold K.LeftCtrl,
          K.B,
        ]
      }
      """

  Example: long hold alone resolves as tap

    When the keymap registers the following input
      """
      [
        press (K.A & K.hold K.LeftCtrl),
        wait 250,
        release (K.A & K.hold K.LeftCtrl),
      ]
      """
    Then the output should be equivalent to output from
      """
      [
        tap (K.A),
      ]
      """

  Example: interrupting press still resolves as hold

    When the keymap registers the following input
      """
      [
        press (K.A & K.hold K.LeftCtrl),
        press (K.B),
      ]
      """
    Then the output should be equivalent to output from
      """
      [
        press (K.LeftCtrl),
        press (K.B),
      ]
      """
