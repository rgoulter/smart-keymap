Feature: TapHold Key (configure resolve-as-hold timeout)

  The "timeout" before a tap-hold is considered as held
  can be configured by the field `config.tap_hold.timeout`
   in `keymap.ncl`.

  Example: timeout configured to a low value
    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      {
        config.tap_hold.timeout = 50,
        keys = [ K.A & K.hold K.LeftCtrl ],
      }
      """
    When the keymap registers the following input
      """
      [
        Press(keymap_index: 0)
      ]
      """
    And the keymap ticks 60 times
    Then the HID keyboard report should equal
      """
      let K = import "hid-usage-keyboard.ncl" in
      { modifiers = { left_ctrl = true } }
      """


  Example: timeout configured to a high value

    e.g. by setting the timeout to a very high value,
     the key still won't resolve as "held" until that
     the timeout has been reached.

    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      {
        config.tap_hold.timeout = 30000,
        keys = [ K.A & K.hold K.LeftCtrl ],
      }
      """
    When the keymap registers the following input
      """
      [
        Press(keymap_index: 0)
      ]
      """
    And the keymap ticks 10000 times
    Then the HID keyboard report should equal
      """
      { modifiers = {}, key_codes = [] }
      """
