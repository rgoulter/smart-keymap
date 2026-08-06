Feature: TapHold Key (configure quick_tap_ms)

  The `quick_tap_ms` config (ZMK `quick-tap-ms`) means that
   re-pressing a tap-hold key within the window of its previous press
   immediately resolves as tap.

  This is useful for hold-to-repeat on the tap behaviour
   (e.g. backspace) without waiting for the hold timeout.

  For examples of this key in other smart keyboard firmware, see e.g.:

  - [ZMK's hold-tap, quick-tap-ms](https://zmk.dev/docs/keymaps/behaviors/hold-tap#quick-tap-ms)

  Background:

    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      {
        config.tap_hold.quick_tap_ms = 175,
        config.tap_hold.timeout = 200,
        keys = [
          K.A & K.hold K.LeftCtrl,
        ]
      }
      """

  Example: re-press within quick_tap_ms forces tap even when held

    When the keymap registers the following input
      """
      [
        tap (K.A & K.hold K.LeftCtrl),
        wait 50,
        press (K.A & K.hold K.LeftCtrl),
        wait 250,
      ]
      """
    Then the output should be equivalent to output from
      """
      [
        tap (K.A),
        press (K.A),
      ]
      """

  Example: re-press after quick_tap_ms allows hold

    When the keymap registers the following input
      """
      [
        tap (K.A & K.hold K.LeftCtrl),
        wait 200,
        press (K.A & K.hold K.LeftCtrl),
        wait 250,
      ]
      """
    Then the output should be equivalent to output from
      """
      [
        tap (K.A),
        press (K.LeftCtrl),
      ]
      """
