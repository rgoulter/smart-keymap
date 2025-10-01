Feature: Automation Key (String Macro)

  Automation keys implement macro key behaviour.

  For examples of this key in other smart keyboard firmware, see e.g.:

  - [QMK's macro keys](https://docs.qmk.fm/feature_macros),

  - [ZMK's macro behaviors](https://zmk.dev/docs/keymaps/behaviors/macros)

  Example: string macro key
    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in

      let MY_MACRO = K.string_macro "hello world" in
      {
        keys = [
            MY_MACRO,
        ],
      }
      """
    When the keymap registers the following input
      """
      let K = import "keys.ncl" in
      let MY_MACRO = K.string_macro "hello world" in
      [
        press MY_MACRO,
        release MY_MACRO,
      ]
      """
    Then the output should be equivalent to output from
      """
      [
        press K.H,
        release K.H,
        press K.E,
        release K.E,
        press K.L,
        release K.L,
        press K.L,
        release K.L,
        press K.O,
        release K.O,
        press K.Space,
        release K.Space,
        press K.W,
        release K.W,
        press K.O,
        release K.O,
        press K.R,
        release K.R,
        press K.L,
        release K.L,
        press K.D,
        release K.D,
      ]
      """
