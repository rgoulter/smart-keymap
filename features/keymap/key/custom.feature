Feature: Custom Keys
  Custom keys are reported as arbitrary codes `0-255` in the keymap output.

  Their functionality is arbitrary, defined by the keyboard firmware implementation.

  The pressed custom codes are available through the `KeymapHidReport.custom`
   (for `libsmart_keymap`), or `keymap::KeymapOutput::pressed_custom_codes`
   (for the `smart_keymap` crate).

  Example: custom key
    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      let my_custom_key = K.custom 255 in
      { keys = [ my_custom_key ] }
      """
