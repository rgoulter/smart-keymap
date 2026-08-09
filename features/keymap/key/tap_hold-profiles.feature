Feature: TapHold Key (behavior profiles)

  Tap-hold keys use behavior **profiles**. Profile 0 is the default profile
  (`config.tap_hold` knobs: timeout, interrupt_response, required_idle_time —
  stored as `Config.default_profile`). Extra profiles are authored as
  `config.tap_hold.profiles = { name = { … }, … }` and selected per key with
  `tap_hold_profile` (name string or numeric index).

  Names lower to indices 1.. in record field order (JSON array on config).

  For similar ideas in other firmware, see e.g.:

  - [ZMK custom hold-tap behaviors](https://zmk.dev/docs/keymaps/behaviors/hold-tap#custom-hold-tap-examples)
  - FAK `hold_tap_behaviors[]` + behavior index on the key

  Background:

    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      {
        config.tap_hold = {
          interrupt_response = "Ignore",
          timeout = 200,
          profiles = {
            hold_press = {
              interrupt_response = "HoldOnKeyPress",
              timeout = 200,
            },
          },
        },
        keys = [
          K.A & K.hold K.LeftCtrl,
          K.B,
          K.C & K.hold K.LeftShift & K.tap_hold_profile "hold_press",
        ],
      }
      """

  Example: default profile ignores interrupt (hold only on timeout)

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

  Example: named profile hold_press resolves interrupt as hold

    When the keymap registers the following input
      """
      [
        press (K.C & K.hold K.LeftShift & K.tap_hold_profile "hold_press"),
        press (K.B),
        release (K.C & K.hold K.LeftShift & K.tap_hold_profile "hold_press"),
        release (K.B),
      ]
      """
    Then the output should be equivalent to output from
      """
      [
        press (K.LeftShift),
        press (K.B),
        release (K.LeftShift),
        release (K.B),
      ]
      """
