Feature: Autoshift Key

  Autoshift makes a key emit its *shifted* form when held past a timeout,
  and the unshifted form when tapped (released before the timeout).

  In smart-keymap, autoshift is **tap-hold sugar**, not a separate key family:

  - `K.autoshift K.A` is equivalent to `K.A & K.hold (K.A & K.LeftShift)`
  - `K.autoshift_as tap hold` is equivalent to `tap & K.hold hold`
    (custom shifted value; same idea as QMK custom auto-shifted keys)

  Timing and interrupt policy use the shared tap-hold config:

  - `config.tap_hold.timeout` — hold threshold (default 200ms; role of QMK
    `AUTO_SHIFT_TIMEOUT`)
  - `config.tap_hold.interrupt_response` — default `Ignore` is a reasonable
    autoshift fit among current options (other keys do not force hold)
  - `config.tap_hold.required_idle_time` — optional “flow” guard

  For examples of this idea in other smart keyboard firmware, see e.g.:

  - [QMK Auto Shift](https://docs.qmk.fm/features/auto_shift)

  Background:

    Let's use a keymap with an autoshift letter key.

    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      {
        keys = [
          K.autoshift K.A,
          K.B,
        ]
      }
      """

  Example: autoshift key acts as unshifted when tapped
    When the keymap registers the following input
      """
      [
        tap (K.autoshift K.A),
      ]
      """
    Then the HID keyboard report should equal
      """
      { key_codes = [K.A] }
      """

  Example: autoshift key acts as shifted when held past timeout

    When the keymap registers the following input
      """
      [
        press (K.autoshift K.A),
      ]
      """
    And the keymap ticks 500 times
    Then the HID keyboard report should equal
      """
      { modifiers = { left_shift = true }, key_codes = [K.A] }
      """

  Example: custom shifted value with autoshift_as

    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      {
        keys = [
          K.autoshift_as K.Comma K.Semicolon,
        ]
      }
      """
    When the keymap registers the following input
      """
      [
        press (K.autoshift_as K.Comma K.Semicolon),
      ]
      """
    And the keymap ticks 500 times
    Then the HID keyboard report should equal
      """
      { key_codes = [K.Semicolon] }
      """
