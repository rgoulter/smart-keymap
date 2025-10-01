{
  gpio_pins,

  board = {
    matrix =
      let p = gpio_pins in
      {
        cols = [
          # COL1 - COL5
          p.B1,
          p.B2,
          p.B3,
          p.B4,
          p.B5,
          # COL6 - COL10
          p.B13,
          p.A13,
          p.A12,
          p.A11,
          p.A15,
          # COL11 - COL15
          p.A16,
          p.A17,
          p.A18,
          p.A19,
          p.A20,
        ],
        rows = [p.A8, p.A9, p.A4, p.A5, p.A6],
        key_count = 75,
        implementation = "col_to_row",
      },

    # The CH32X-75 a matrix of 5 rows x 15 columns.
    #
    # Want the keymap index to refer to keys row-wise.
    keymap_index_for_key = fun { column_index | Number, row_index | Number, .. } =>
      'Key (row_index * 15 + column_index),

    led = {
      enabled = true,
      pin = gpio_pins.B9,
    },

    debug.tx = 2,
  },
}
