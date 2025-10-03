Feature: Automation Key

  Automation keys implement macro key behaviour.

  For common use cases, it's simpler to use higher-level wrappers
   around the key, such as `K.string_macro` (documented below),
   or otherwise use Nickel's expressive language to build abstractions
   above it.

  For examples of this key in other smart keyboard firmware, see e.g.:

  - [fak's macros](https://github.com/semickolon/fak?tab=readme-ov-file#macros)

  - [QMK's macro keys](https://docs.qmk.fm/feature_macros),

  - [ZMK's macro behaviors](https://zmk.dev/docs/keymaps/behaviors/macros)

  Example: automation key
    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in

      let MY_MACRO = {
          automation_instructions.on_press = [
              { Press = { key_code = { Keyboard = 0x04 } } },
              { Release = { key_code = { Keyboard = 0x04 } } },
              { Press = { key_code = { Keyboard = 0x05 } } },
              { Release = { key_code = { Keyboard = 0x05 } } },
              { Press = { key_code = { Keyboard = 0x06 } } },
              { Release = { key_code = { Keyboard = 0x06 } } },
          ],
      } in
      {
        keys = [
            MY_MACRO,
        ],
      }
      """
    When the keymap registers the following input
      """
      [
        tap_keymap_index 0,
      ]
      """
    Then the output should be equivalent to output from
      """
      [
        tap K.A,
        tap K.B,
        tap K.C,
      ]
      """

  Example: on_press, while_pressed, on_release
    The automation key's key instructions support `on_press`,
     `while_pressed`, `on_release` instructions:

    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      let { string_to_instructions, .. } = import "smart_keys/automation/lib.ncl" in

      let MY_MACRO = {
          automation_instructions = {
              on_press = "ab" |> string_to_instructions,
              while_pressed = [
                    { Tap = { key_code = { Keyboard = 0x06 } } },
                    { Wait = 1000 },
              ],
              on_release = "de" |> string_to_instructions,
          },
      } in
      {
        keys = [
            MY_MACRO,
        ],
      }
      """
