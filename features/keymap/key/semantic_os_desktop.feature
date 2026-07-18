Feature: Semantic OS Desktop Key

  Semantic keys resolve to different outputs depending on which named
  variant layer is active. `K.semantic` is Nickel sugar for a named-layer
  `LayeredKey` (`{ named_layers = { windows = …, linux = …, macos = … } }`).

  OS desktop switching is not a built-in key type: it is a named-layer
  *group* plus exclusive `layer_mod.set_semantic_variant_to` switches.
  The group list is the mask for `set_active_layers_amongst`; names
  resolve to indices when the keymap is compiled.

  Define a desk key once (e.g. `desk_left = K.semantic { .. }`), place it
  on a single layer row in the layout, and pick the OS variant with the
  switch keys. Named layers are assigned globally after any numbered
  layers (alphabetical order among names).

  Background:

    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      let os = ["linux", "macos", "windows"] in
      let os_windows = K.layer_mod.set_semantic_variant_to os "windows" in
      let os_linux = K.layer_mod.set_semantic_variant_to os "linux" in
      let os_macos = K.layer_mod.set_semantic_variant_to os "macos" in
      let desk_left =
        K.semantic {
          windows = K.Left & K.LGUI & K.LeftCtrl,
          linux = K.Left & K.LeftCtrl & K.LeftAlt,
          macos = K.Left & K.LeftCtrl,
        }
      in
      {
        keys = [
          os_windows,
          os_linux,
          os_macos,
          desk_left,
        ],
      }
      """

  Example: desktop left sends Windows chord when Windows OS variant is active

    When the keymap registers the following input
      """
      [
        tap (K.layer_mod.set_semantic_variant_to ["linux", "macos", "windows"] "windows"),
        press_keymap_index 3,
      ]
      """
    Then the HID keyboard report should equal
      """
      { modifiers = { left_gui = true, left_ctrl = true }, key_codes = [K.Left] }
      """

  Example: desktop left sends Linux chord when Linux OS variant is active

    When the keymap registers the following input
      """
      [
        tap (K.layer_mod.set_semantic_variant_to ["linux", "macos", "windows"] "linux"),
        press_keymap_index 3,
      ]
      """
    Then the HID keyboard report should equal
      """
      { modifiers = { left_ctrl = true, left_alt = true }, key_codes = [K.Left] }
      """

  Example: desktop left sends macOS chord when macOS OS variant is active

    When the keymap registers the following input
      """
      [
        tap (K.layer_mod.set_semantic_variant_to ["linux", "macos", "windows"] "macos"),
        press_keymap_index 3,
      ]
      """
    Then the HID keyboard report should equal
      """
      { modifiers = { left_ctrl = true }, key_codes = [K.Left] }
      """

  Example: advanced — desk keys on Fn layer with OS switch on base layer

    A small row layout: OS switches, `A` `B` `C` `D`, and `Fn`
    (hold layer 1). `C` and `D` are normal keys on base; on Fn they are
    semantic desk-left / desk-right. Numbered Fn layer and named OS
    layers coexist (OS names sit after the numbered slot).

    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      let os = ["linux", "macos", "windows"] in
      let os_windows = K.layer_mod.set_semantic_variant_to os "windows" in
      let os_linux = K.layer_mod.set_semantic_variant_to os "linux" in
      let desk_left =
        K.semantic {
          windows = K.Left & K.LGUI & K.LeftCtrl,
          linux = K.Left & K.LeftCtrl & K.LeftAlt,
          macos = K.Left & K.LeftCtrl,
        }
      in
      let desk_right =
        K.semantic {
          windows = K.Right & K.LGUI & K.LeftCtrl,
          linux = K.Right & K.LeftCtrl & K.LeftAlt,
          macos = K.Right & K.LeftCtrl,
        }
      in
      {
        layers = [
          [
            os_windows,
            os_linux,
            K.A,
            K.B,
            K.C,
            K.D,
            K.layer_mod.hold 1,
          ],
          [
            K.TTTT,
            K.TTTT,
            K.TTTT,
            K.TTTT,
            desk_left,
            desk_right,
            K.TTTT,
          ],
        ],
      }
      """

    When the keymap registers the following input
      """
      [
        press_keymap_index 4,
      ]
      """
    Then the HID keyboard report should equal
      """
      { key_codes = [K.C] }
      """

    When the keymap registers the following input
      """
      [
        release_keymap_index 4,
        release_keymap_index 6,
        press (K.layer_mod.hold 1),
        tap (K.layer_mod.set_semantic_variant_to ["linux", "macos", "windows"] "windows"),
        press_keymap_index 4,
      ]
      """
    Then the HID keyboard report should equal
      """
      { modifiers = { left_gui = true, left_ctrl = true }, key_codes = [K.Left] }
      """

    When the keymap registers the following input
      """
      [
        release_keymap_index 4,
        release_keymap_index 6,
        press (K.layer_mod.hold 1),
        tap (K.layer_mod.set_semantic_variant_to ["linux", "macos", "windows"] "linux"),
        press_keymap_index 5,
      ]
      """
    Then the HID keyboard report should equal
      """
      { modifiers = { left_ctrl = true, left_alt = true }, key_codes = [K.Right] }
      """
