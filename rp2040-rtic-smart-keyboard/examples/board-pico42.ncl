{
  gpio_pins,

  usb
    = {
      vid = 0xCAFE,
      pid = 0x0005,
      manufacturer = "rgoulter keyboard-labs",
      product = "Pico42"
    },

  matrix
    =
      let p = gpio_pins in
      {
        cols = [
          p.GP0,
          p.GP1,
          p.GP2,
          p.GP3,
          p.GP4,
          p.GP5,
          p.GP6,
          p.GP7,
          p.GP8,
          p.GP9,
          p.GP10,
          p.GP11,
        ],
        rows = [
          p.GP14,
          p.GP15,
          p.GP16,
          p.GP17,
        ],
        key_count = 42,
      },

  keymap_index_for_key = fun { column_index, row_index } =>
    let NO = null in
    let keymap_indices = [
      [ 0,  1,  2,  3,  4, NO, NO,  5,  6,  7,  8,  9],
      [10, 11, 12, 13, 14, NO, NO, 15, 16, 17, 18, 19],
      [20, 21, 22, 23, 24, NO, NO, 25, 26, 27, 28, 29],
      [30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41],
    ]
    in
    let row = std.array.at row_index keymap_indices in
    std.array.at column_index row |> match {
        idx if idx != null => 'Key idx,
        _ => 'NoKey,
    },
}
