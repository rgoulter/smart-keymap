Feature: TapHold Key (configure pending_output)

  The `pending_output` config for tap-hold keys controls speculative HID
   while the tap-vs-hold decision is still pending.

  Default `NoOutput` produces no output while pending.
  No HID is emitted until timeout, interrupt, or release settles tap vs hold.
  `Hold` emits the hold binding's HID while still pending, then keeps it
   or retracts it when tap vs hold settles. Decision logic is unchanged.
  Only the timing of hold appearance changes.

  This matches FAK `eager_decision = 'hold'`, ZMK
   `hold-while-undecided`, and QMK Speculative Hold.

  For examples of this key in other smart keyboard firmware, see e.g.:

  - [FAK's eager_decision](https://github.com/semickolon/fak)

  - [ZMK's hold-while-undecided](https://zmk.dev/docs/keymaps/behaviors/hold-tap#hold-while-undecided)

  - [QMK's Speculative Hold](https://docs.qmk.fm/tap_hold#speculative-hold)

  Background:

    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      {
        config.tap_hold.pending_output = "Hold",
        keys = [
          K.A & K.hold K.LeftCtrl,
          K.B
        ]
      }
      """

  Example: Hold with timeout shows mod from first tick and stays hold

    When the keymap registers the following input
      """
      [
        press (K.A & K.hold K.LeftCtrl),
        wait 1,
      ]
      """
    Then the HID keyboard report should equal
      """
      { modifiers = { left_ctrl = true } }
      """

  Example: Hold with quick release retracts mod and ends as tap

    When the keymap registers the following input
      """
      [
        press (K.A & K.hold K.LeftCtrl),
        wait 1,
        release (K.A & K.hold K.LeftCtrl),
      ]
      """
    Then the HID keyboard report should equal
      """
      { key_codes = [K.A] }
      """

  Example: Hold with HoldOnKeyPress interrupt shows mod already down when other key taps

    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      {
        config.tap_hold.pending_output = "Hold",
        config.tap_hold.interrupt_response = "HoldOnKeyPress",
        keys = [
          K.A & K.hold K.LeftCtrl,
          K.B
        ]
      }
      """
    When the keymap registers the following input
      """
      [
        press (K.A & K.hold K.LeftCtrl),
        wait 1,
        tap K.B,
      ]
      """
    Then the HID keyboard report should equal
      """
      { modifiers = { left_ctrl = true }, key_codes = [K.B] }
      """
