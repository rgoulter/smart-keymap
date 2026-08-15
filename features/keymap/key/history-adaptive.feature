Feature: History Adaptive Key

  An Adaptive key emits a configured output based on the last resolved
  key output, and otherwise emits a per-key default (Hands Down / ZMK
  adaptive-key style).

  Unlike Alternate Repeat, the rule table and default live on the key
  itself, so different letter sites can have different morphs.

  For examples of this key in other smart keyboard firmware, see e.g.:

  - [urob's zmk-adaptive-key](https://github.com/urob/zmk-adaptive-key)

  Background:

    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      let H = K.history.adaptive {
        default = K.H,
        rules = [
          { prev = K.A, emit = K.U },
          { prev = K.E, emit = K.O },
        ],
      } in
      {
        keys = [
          K.A,
          K.B,
          H,
        ]
      }
      """

  Example: adaptive H after A types U
    When the keymap registers the following input
      """
      let K = import "keys.ncl" in
      let H = K.history.adaptive {
        default = K.H,
        rules = [
          { prev = K.A, emit = K.U },
          { prev = K.E, emit = K.O },
        ],
      } in
      [
        tap K.A,
        tap H,
      ]
      """
    Then the output should be equivalent to output from
      """
      [
        tap K.A,
        tap K.U,
      ]
      """

  Example: adaptive H after unmapped B types H
    When the keymap registers the following input
      """
      let K = import "keys.ncl" in
      let H = K.history.adaptive {
        default = K.H,
        rules = [
          { prev = K.A, emit = K.U },
          { prev = K.E, emit = K.O },
        ],
      } in
      [
        tap K.B,
        tap H,
      ]
      """
    Then the output should be equivalent to output from
      """
      [
        tap K.B,
        tap K.H,
      ]
      """

  Example: two adaptive keys keep independent defaults and rules
    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      let H = K.history.adaptive {
        default = K.H,
        rules = [
          { prev = K.A, emit = K.U },
        ],
      } in
      let M = K.history.adaptive {
        default = K.M,
        rules = [
          { prev = K.G, emit = K.L },
        ],
      } in
      {
        keys = [
          K.A,
          K.G,
          H,
          M,
        ]
      }
      """
    When the keymap registers the following input
      """
      let K = import "keys.ncl" in
      let H = K.history.adaptive {
        default = K.H,
        rules = [
          { prev = K.A, emit = K.U },
        ],
      } in
      let M = K.history.adaptive {
        default = K.M,
        rules = [
          { prev = K.G, emit = K.L },
        ],
      } in
      [
        tap K.A,
        tap H,
        tap K.G,
        tap M,
        tap H,
      ]
      """
    Then the output should be equivalent to output from
      """
      [
        tap K.A,
        tap K.U,
        tap K.G,
        tap K.L,
        tap K.H,
      ]
      """
