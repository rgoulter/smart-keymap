Feature: Tri-state Key

  A tri-state key owns a session across later presses of the same key.
  The first press starts (typically a modifier and tap a key),
   later presses of the same key continue (tap again while the hold stays),
   and any other resolved key interrupts (releases the hold).

  Classic use is Alt-Tab (a "swapper"):
   one key holds Alt and taps Tab on the first press,
   taps Tab again on re-press,
   and releases Alt when another key is pressed.

  For examples of this key in other smart keyboard firmware, see e.g.:

  - [Getreuer's Cyclotab](https://getreuer.info/posts/keyboards/cyclotab/)

  - [DhruvinSh's ZMK tri-state](https://github.com/dhruvinsh/zmk-tri-state)

  Background:

    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      {
        keys = [
          K.tri_state.alt_tab,
          K.A,
        ]
      }
      """

  Example: first press is the Alt+Tab chord; Alt stays after release
    When the keymap registers the following input
      """
      [
        press K.tri_state.alt_tab,
      ]
      """
    Then the HID keyboard report should equal
      """
      { modifiers = { left_alt = true }, key_codes = [K.Tab] }
      """

  Example: releasing the tri-state key leaves Alt held
    When the keymap registers the following input
      """
      [
        tap K.tri_state.alt_tab,
      ]
      """
    Then the HID keyboard report should equal
      """
      { modifiers = { left_alt = true } }
      """

  Example: re-press leaves Alt held
    When the keymap registers the following input
      """
      [
        tap K.tri_state.alt_tab,
        tap K.tri_state.alt_tab,
      ]
      """
    Then the HID keyboard report should equal
      """
      { modifiers = { left_alt = true } }
      """
