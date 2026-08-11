Feature: Layer Modifier: Tap Toggle

  The `K.layer_mod.tap_toggle` key combines momentary and toggle layer
  access (QMK `TT` with `TAPPING_TOGGLE = 1`):

  - holding the key (interrupted by another key) activates the layer while
    held, like `K.layer_mod.hold`;
  - tapping the key toggles whether the layer is active, like
    `K.layer_mod.toggle`.

  For examples of this feature in other smart keyboard firmware, see e.g.:

  - [QMK's TT(layer)](https://docs.qmk.fm/feature_layers#switching-and-toggling-layers),

  Background:

    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      {
        layers = [
          [
            K.layer_mod.tap_toggle 1,
            K.A,
          ],
          [
            K.TTTT,
            K.B,
          ],
        ],
      }
      """

  Example: holding the tap-toggle key activates the layer
    When the keymap registers the following input
      """
      [
        press (K.layer_mod.tap_toggle 1),
        press (K.B),
      ]
      """
    Then the HID keyboard report should equal
      """
      { key_codes = [K.B] }
      """

  Example: releasing a held tap-toggle key deactivates the layer
    When the keymap registers the following input
      """
      [
        press (K.layer_mod.tap_toggle 1),
        press_keymap_index 1,
        release_keymap_index 1,
        release (K.layer_mod.tap_toggle 1),
        press (K.A),
      ]
      """
    Then the HID keyboard report should equal
      """
      { key_codes = [K.A] }
      """

  Example: tapping the tap-toggle key toggles the layer on
    When the keymap registers the following input
      """
      [
        tap (K.layer_mod.tap_toggle 1),
        press (K.B),
      ]
      """
    Then the HID keyboard report should equal
      """
      { key_codes = [K.B] }
      """

  Example: tapping the tap-toggle key a second time toggles the layer off
    When the keymap registers the following input
      """
      [
        tap (K.layer_mod.tap_toggle 1),
        tap (K.layer_mod.tap_toggle 1),
        press (K.A),
      ]
      """
    Then the HID keyboard report should equal
      """
      { key_codes = [K.A] }
      """
