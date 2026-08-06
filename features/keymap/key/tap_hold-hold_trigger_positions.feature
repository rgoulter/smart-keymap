Feature: TapHold Key (hold_trigger_key_positions / opposite-hand)

  When a tap-hold key sets `hold_trigger_key_positions`,
   only those keymap indices may resolve an interrupt as hold.
   Interrupts from other positions are ignored for the hold decision
   (ZMK positional hold-tap / opposite-hand HRM polish).

  Timeout and self-release behaviour are unchanged.

  For examples of this key in other smart keyboard firmware, see e.g.:

  - [ZMK's hold-tap, hold-trigger-key-positions](https://zmk.dev/docs/keymaps/behaviors/hold-tap#positional-hold-tap-and-hold-trigger-key-positions)

  Background:

    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      {
        config.tap_hold.interrupt_response = "HoldOnKeyPress",
        config.tap_hold.timeout = 200,
        keys = [
          K.A & K.hold K.LeftCtrl & { hold_trigger_key_positions = [2] },
          K.B,
          K.C,
        ]
      }
      """

  Example: allowed interrupt position resolves as hold

    When the keymap registers the following input
      """
      [
        press (K.A & K.hold K.LeftCtrl & { hold_trigger_key_positions = [2] }),
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
        press (K.A & K.hold K.LeftCtrl & { hold_trigger_key_positions = [2] }),
        tap (K.B),
        release (K.A & K.hold K.LeftCtrl & { hold_trigger_key_positions = [2] }),
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
