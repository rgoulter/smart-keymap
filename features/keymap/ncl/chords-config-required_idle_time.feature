Feature: Chords (configure required_idle_time)

  The `required_idle_time` config for chords means that
   chords can only be activated after the required
   idle time has passed since the previous keymap input event (press/release).

  This helps prevent accidental chord activation when typing quickly.

  For examples of this key in other smart keyboard firmware, see e.g.:

  - [ZMK's combo feature, require-prior-idle-ms](https://zmk.dev/docs/keymaps/combos)

  - [fak's combo's require_prior_idle_ms](https://github.com/semickolon/fak)

  Background:

    Let's demonstrate tap-hold "required_idle_time" behaviour
    using a keymap with a keyboard key, and a tap-hold key:

    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      {
        config.chorded.required_idle_time = 100,
        chords = [
          { indices = [0, 1], key = K.C, },
        ],
        keys = [
          K.A, K.B, K.D,
        ],
      }
      """

  Example: chord doesn't resolve when pressed before required idle time

    When the keymap registers the following input
      """
      [
        tap (K.D),
        wait 50,
        press_keymap_index 0,
        press_keymap_index 1,
      ]
      """
    Then the output should be equivalent to output from
      """
      [
        tap (K.D),
        press (K.A),
        press (K.B),
      ]
      """

  Example: chord resolves pressed after required idle time

    When the keymap registers the following input
      """
      [
        tap (K.D),
        wait 150,
        press_keymap_index 0,
        press_keymap_index 1,
      ]
      """
    Then the output should be equivalent to output from
      """
      [
        tap (K.D),
        press (K.C),
      ]
      """
