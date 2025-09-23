{
  key_extensions.callback = {
    callback = fun i j => { keymap_callback = { Custom = [i, j] } },
    reset = { keymap_callback = "Reset" },
    reset_to_bootloader = { keymap_callback = "ResetToBootloader" },
  },
}
