Feature: TapHold Key

  The TapHold key can behave differently depending on
   whether the key has been tapped or held.

  e.g.

  - Quickly pressing and releasing a TapHold key results in a the 'tap' behaviour.

  - Pressing the TapHold key for a long enough period results in the 'hold' behaviour.

  - Pressing another key while the TapHold key is pressed "interrupts" the TapHold key,
     resulting in the 'hold' behaviour.

  For examples of this key in other smart keyboard firmware, see e.g.:

  - [QMK's mod-tap keys](https://docs.qmk.fm/keycodes#mod-tap-keys),

  - [ZMK's hold-tap keymap behaviors](https://zmk.dev/docs/keymaps/behaviors/hold-tap)

  Background:

    Let's use a keymap with a tap-hold key, and a keyboard key.

    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      {
        keys = [
          K.A & K.hold K.LeftCtrl,
          K.B
        ]
      }
      """

  Example: tap hold key acts as 'tap' when tapped
    When the keymap registers the following input
      """
      [
        press (K.A & K.hold K.LeftCtrl),
        release (K.A & K.hold K.LeftCtrl),
      ]
      """
    Then the HID keyboard report should equal
      """
      { key_codes = [K.A] }
      """

  Example: tap hold key acts as 'hold' when held

    When the keymap registers the following input
      """
      [
        press (K.A & K.hold K.LeftCtrl),
      ]
      """
    And the keymap ticks 500 times
    Then the HID keyboard report should equal
      """
      { modifiers = { left_ctrl = true } }
      """
