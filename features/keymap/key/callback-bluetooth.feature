Feature: Callback Keys (Bluetooth Profile callbacks)
  Predefined callbacks for Bluetooth profile management have been defined.
  (It's up to the implementing keyboard firmware to implement functionality).

  - [QMK's Bluetooth keycodes for its Wireless feature](https://docs.qmk.fm/features/wireless#bluetooth-keycodes),

  - [ZMK's Bluetooth behaviors](https://zmk.dev/docs/keymaps/behaviors/bluetooth)

  Example: Bluetooth profile management keys
    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      {
        keys = [
          K.bluetooth_profile.clear,
          K.bluetooth_profile.clear_all,
          K.bluetooth_profile.disconnect,
          K.bluetooth_profile.next,
          K.bluetooth_profile.previous,
          K.bluetooth_profile.select 0,
          K.bluetooth_profile.select 1,
          K.bluetooth_profile.select 2,
        ]
      }
      """
