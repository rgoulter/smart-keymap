Feature: Mod-conditioned keys

  Dual-binding keys that pick a nested binding based on held modifiers,
  optionally suppressing those modifiers from the HID report
  (ZMK mod-morph / QMK key-override style).

  A mod-conditioned key is a record with `base`, `morphed`, and `mods`
  (and optional `keep_mods` for trigger modifiers that should remain in the
  report). Named keys such as `K.gresc` and `K.bspc_del` are particular
  examples of this pattern.

  Background:

    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      let my_bspc_del = {
        base = K.Backspace,
        morphed = K.Delete,
        mods = { left_shift = true },
      } in
      {
        keys = [
          my_bspc_del,
          K.LeftShift,
          K.RightShift,
        ]
      }
      """

  Example: custom mod-conditioned key alone sends the base binding
    When the keymap registers the following input
      """
      let K = import "keys.ncl" in
      let my_bspc_del = {
        base = K.Backspace,
        morphed = K.Delete,
        mods = { left_shift = true },
      } in
      [
        press my_bspc_del,
      ]
      """
    Then the output should be equivalent to output from
      """
      [
        press K.Backspace,
      ]
      """

  Example: custom mod-conditioned key with trigger mod sends morphed binding without the mod
    When the keymap registers the following input
      """
      let K = import "keys.ncl" in
      let my_bspc_del = {
        base = K.Backspace,
        morphed = K.Delete,
        mods = { left_shift = true },
      } in
      [
        press K.LeftShift,
        press my_bspc_del,
      ]
      """
    Then the output should be equivalent to output from
      """
      [
        press K.LeftShift,
        press K.Delete,
        release K.LeftShift,
      ]
      """

  Example: custom keep_mods leaves listed mods in the report
    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      let my_bspc_del = {
        base = K.Backspace,
        morphed = K.Delete,
        mods = {
          left_shift = true,
          right_shift = true,
        },
        keep_mods = {
          right_shift = true,
        },
      } in
      {
        keys = [
          my_bspc_del,
          K.LeftShift,
          K.RightShift,
        ]
      }
      """
    When the keymap registers the following input
      """
      let K = import "keys.ncl" in
      let my_bspc_del = {
        base = K.Backspace,
        morphed = K.Delete,
        mods = {
          left_shift = true,
          right_shift = true,
        },
        keep_mods = {
          right_shift = true,
        },
      } in
      [
        press K.RightShift,
        press my_bspc_del,
      ]
      """
    Then the output should be equivalent to output from
      """
      [
        press K.RightShift,
        press K.Delete,
      ]
      """

  Example: gresc alone sends Escape
    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      {
        keys = [
          K.gresc,
          K.LeftShift,
        ]
      }
      """
    When the keymap registers the following input
      """
      [
        press K.gresc,
      ]
      """
    Then the output should be equivalent to output from
      """
      [
        press K.Escape,
      ]
      """

  Example: gresc with Shift sends Grave without Shift in the report
    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      {
        keys = [
          K.gresc,
          K.LeftShift,
        ]
      }
      """
    When the keymap registers the following input
      """
      [
        press K.LeftShift,
        press K.gresc,
      ]
      """
    Then the output should be equivalent to output from
      """
      [
        press K.LeftShift,
        press K.Grave,
        release K.LeftShift,
      ]
      """

  Example: bspc_del with LeftShift sends Delete without LeftShift
    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      {
        keys = [
          K.bspc_del,
          K.LeftShift,
          K.RightShift,
        ]
      }
      """
    When the keymap registers the following input
      """
      [
        press K.LeftShift,
        press K.bspc_del,
      ]
      """
    Then the output should be equivalent to output from
      """
      [
        press K.LeftShift,
        press K.Delete,
        release K.LeftShift,
      ]
      """

  Example: bspc_del with RightShift keeps RightShift
    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      {
        keys = [
          K.bspc_del,
          K.LeftShift,
          K.RightShift,
        ]
      }
      """
    When the keymap registers the following input
      """
      [
        press K.RightShift,
        press K.bspc_del,
      ]
      """
    Then the output should be equivalent to output from
      """
      [
        press K.RightShift,
        press K.Delete,
      ]
      """
