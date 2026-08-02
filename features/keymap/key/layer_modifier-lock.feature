Feature: Layer Modifier: Lock

  The `K.layer_mod.lock` key locks the highest currently active layer so it stays
  on after the momentary (hold) layer modifier is released.

  The `K.layer_mod.lock_layer` key locks or unlocks a specific layer (activates
  the layer when locking; deactivates when unlocking).

  For examples of this feature in other smart keyboard firmware, see e.g.:

  - [QMK's Layer Lock](https://docs.qmk.fm/features/layer_lock),

  - [Miryoku's CURR / OPP layer lock](https://github.com/manna-harbour/miryoku/tree/master/docs/reference)

  Example: locking a held layer keeps it active after hold is released

    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      {
        layers = [
          [
            K.layer_mod.hold 1,
            K.layer_mod.lock,
            K.A,
          ],
          [
            K.TTTT,
            K.TTTT,
            K.B,
          ],
        ],
      }
      """
    When the keymap registers the following input
      """
      [
        press (K.layer_mod.hold 1),
        tap (K.layer_mod.lock),
        release (K.layer_mod.hold 1),
        press (K.B),
      ]
      """
    Then the HID keyboard report should equal
      """
      { key_codes = [K.B] }
      """

  Example: tapping lock again unlocks and deactivates the layer

    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      {
        layers = [
          [
            K.layer_mod.hold 1,
            K.layer_mod.lock,
            K.A,
          ],
          [
            K.TTTT,
            K.TTTT,
            K.B,
          ],
        ],
      }
      """
    When the keymap registers the following input
      """
      [
        press (K.layer_mod.hold 1),
        tap (K.layer_mod.lock),
        release (K.layer_mod.hold 1),
        tap (K.layer_mod.lock),
        press (K.A),
      ]
      """
    Then the HID keyboard report should equal
      """
      { key_codes = [K.A] }
      """

  Example: pressing hold again while locked unlocks the layer

    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      {
        layers = [
          [
            K.layer_mod.hold 1,
            K.layer_mod.lock,
            K.A,
          ],
          [
            K.TTTT,
            K.TTTT,
            K.B,
          ],
        ],
      }
      """
    When the keymap registers the following input
      """
      [
        press (K.layer_mod.hold 1),
        tap (K.layer_mod.lock),
        release (K.layer_mod.hold 1),
        tap (K.layer_mod.hold 1),
        press (K.A),
      ]
      """
    Then the HID keyboard report should equal
      """
      { key_codes = [K.A] }
      """

  Example: lock_layer activates an inactive layer

    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      {
        layers = [
          [
            K.layer_mod.lock_layer 1,
            K.A,
          ],
          [
            K.TTTT,
            K.B,
          ],
        ],
      }
      """
    When the keymap registers the following input
      """
      [
        tap (K.layer_mod.lock_layer 1),
        press (K.B),
      ]
      """
    Then the HID keyboard report should equal
      """
      { key_codes = [K.B] }
      """

  Example: lock_layer a second time deactivates the layer

    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      {
        layers = [
          [
            K.layer_mod.lock_layer 1,
            K.A,
          ],
          [
            K.TTTT,
            K.B,
          ],
        ],
      }
      """
    When the keymap registers the following input
      """
      [
        tap (K.layer_mod.lock_layer 1),
        tap (K.layer_mod.lock_layer 1),
        press (K.A),
      ]
      """
    Then the HID keyboard report should equal
      """
      { key_codes = [K.A] }
      """
