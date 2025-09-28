Feature: Mouse Keys

  HID Mouse usage codes.

  The pressed mouse codes are available through the `KeymapHidReport.mouse`
   (for `libsmart_keymap`), or `keymap::KeymapOutput::pressed_mouse_output`
   (for the `smart_keymap` crate).

  Example: mouse keys
    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      {
        keys = [
          K.MouseButton1,
          K.MouseButton2,
          K.MouseButton3,
          K.MouseLeft,
          K.MouseDown,
          K.MouseUp,
          K.MouseRight,
          K.MouseWheelDown,
          K.MouseWheelUp,
        ],
      }
      """
