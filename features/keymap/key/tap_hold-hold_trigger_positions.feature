Feature: TapHold Key (hold_trigger_key_positions / positional)

  When a tap-hold **profile** sets `hold_trigger_key_positions`, only those
  keymap indices may resolve an interrupt as hold. Interrupts from other
  positions are ignored for the hold decision (ZMK positional hold-tap).

  Profile 0 is `config.tap_hold` (including optional
  `hold_trigger_key_positions`). Named extras may set the field on their
  profile record. Keys stay thin: `tap` + `hold` + optional profile selector.

  Timeout and self-release behaviour are unchanged.

  For examples of this key in other smart keyboard firmware, see e.g.:

  - [ZMK's hold-tap, hold-trigger-key-positions](https://zmk.dev/docs/keymaps/behaviors/hold-tap#positional-hold-tap-and-hold-trigger-key-positions)

  Background:

    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      {
        config.tap_hold = {
          interrupt_response = "HoldOnKeyPress",
          timeout = 200,
          hold_trigger_key_positions = [2],
        },
        keys = [
          K.A & K.hold K.LeftCtrl,
          K.B,
          K.C,
        ]
      }
      """

  Example: interrupt from a hold trigger position resolves as hold

    When the keymap registers the following input
      """
      [
        press (K.A & K.hold K.LeftCtrl),
        press (K.C),
      ]
      """
    Then the output should be equivalent to output from
      """
      [
        press (K.LeftCtrl),
        press (K.C),
      ]
      """

  Example: non-trigger interrupt does not force hold

    Interrupting with a non-trigger key (press & release), then releasing
     the tap-hold, resolves as tap rather than hold.

    When the keymap registers the following input
      """
      [
        press (K.A & K.hold K.LeftCtrl),
        tap (K.B),
        release (K.A & K.hold K.LeftCtrl),
      ]
      """
    Then the output should be equivalent to output from
      """
      [
        press (K.A),
        tap (K.B),
        release (K.A),
      ]
      """

  Example: named profile with hold_trigger_key_positions

    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      {
        config.tap_hold = {
          interrupt_response = "Ignore",
          timeout = 200,
          profiles = {
            opposite = {
              interrupt_response = "HoldOnKeyPress",
              hold_trigger_key_positions = [2],
            },
          },
        },
        keys = [
          K.A & K.hold K.LeftCtrl & K.tap_hold_profile "opposite",
          K.B,
          K.C,
        ]
      }
      """

    When the keymap registers the following input
      """
      [
        press (K.A & K.hold K.LeftCtrl & K.tap_hold_profile "opposite"),
        press (K.C),
      ]
      """
    Then the output should be equivalent to output from
      """
      [
        press (K.LeftCtrl),
        press (K.C),
      ]
      """
