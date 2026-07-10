Feature: TapHold Key (configure no resolve-as-hold timeout)

  The "timeout" before a tap-hold is considered as held
  can be disabled by setting `config.tap_hold.timeout`
   to `null` in `keymap.ncl`.

  When timeout is `null`, the tap/hold decision does not timeout:
   the key remains pending until released (as tap) or interrupted
   (depending on interrupt_response).

  Without a timeout, hold is only useful when interruptions
   resolve as hold (e.g. `interrupt_response = "HoldOnKeyPress"`).

  Background:

    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      {
        config.tap_hold.timeout = null,
        config.tap_hold.interrupt_response = "HoldOnKeyPress",
        keys = [
          K.A & K.hold K.LeftCtrl,
          K.B,
        ],
      }
      """

  Example: holding a long time then releasing is still tap

    With no timeout, waiting does not promote the key to hold.
    Releasing after a long press still resolves as tap.

    When the keymap registers the following input
      """
      [
        press (K.A & K.hold K.LeftCtrl),
        wait 500,
        release (K.A & K.hold K.LeftCtrl),
      ]
      """
    Then the HID keyboard report should equal
      """
      { key_codes = [K.A] }
      """

  Example: holding a long time then interrupting is hold

    Hold is still reachable via interruption:
     press another key while the tap-hold is still pending.

    When the keymap registers the following input
      """
      [
        press (K.A & K.hold K.LeftCtrl),
        wait 500,
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
