{
  key_extensions.callback = {
    bluetooth_profile = {
      clear = { keymap_callback = { Bluetooth = "Clear" } },
      clear_all = { keymap_callback = { Bluetooth = "ClearAll" } },
      disconnect = { keymap_callback = { Bluetooth = "Disconnect" } },
      next = { keymap_callback = { Bluetooth = "Next" } },
      previous = { keymap_callback = { Bluetooth = "Previous" } },
      select = fun i => { keymap_callback = { Bluetooth = { Select = i } } },
    },
    callback = fun i j => { keymap_callback = { Custom = [i, j] } },
    reset = { keymap_callback = "Reset" },
    reset_to_bootloader = { keymap_callback = "ResetToBootloader" },
  },
}
