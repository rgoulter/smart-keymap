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
        tap MY_MACRO,
      ]
      """
    Then the output should be equivalent to output from
      """
      [
        tap K.H,
        tap K.E,
        tap K.L,
        tap K.L,
        tap K.O,
        tap K.Space,
        tap K.W,
        tap K.O,
        tap K.R,
        tap K.L,
        tap K.D,
      ]
      """
