let K = import "keys.ncl" in

{
  keys = [
    K.reset,
    K.reset_to_bootloader,
    K.callback 3 4,
  ],

  ceedling_fixture = {
    suite = "CALLBACK",
    RESET_KEY = 0,
    BOOTLOADER_KEY = 1,
    CUSTOM_KEY = 2,
  },
}
