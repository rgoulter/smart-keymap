Feature: Callback Keys
  Callback keys invoke keymap callbacks, allowing the implementing firmware
   to execute arbitrary behaviour when the key is pressed.

  Callbacks for resetting the keyboard and entering the bootloader
   have been defined.

  Example: reset callback
    Firmware should register the callback with `void keymap_register_callback(uint8_t callback_id, void (*callback_fn)(void));`
    using `KEYMAP_CALLBACK_RESET`
    (for `libsmart_keymap`), or `keymap::Keymap::set_callback` (when using the `smart_keymap` crate).

    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      { keys = [ K.reset ] }
      """

  Example: reset to bootloader callback
    Firmware should register the callback with `void keymap_register_callback(uint8_t callback_id, void (*callback_fn)(void));`
    using `KEYMAP_CALLBACK_BOOTLOADER`
    (for `libsmart_keymap`), or `keymap::Keymap::set_callback` (when using the `smart_keymap` crate).

    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      { keys = [ K.reset_to_bootloader ] }
      """

  Example: arbitrary keymap callback
    Arbitrary callbacks to the keyboard firmware are indicated
     as a pair of numbers 0-255.

    Firmware should register the callback with `void keymap_register_custom_callback(uint8_t custom_0, uint8_t custom_1, void (*callback_fn)(void))`
    (for `libsmart_keymap`), or `keymap::Keymap::set_callback` (when using the `smart_keymap` crate).

    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      let my_callback = K.callback 0 255 in
      { keys = [ my_callback ] }
      """
