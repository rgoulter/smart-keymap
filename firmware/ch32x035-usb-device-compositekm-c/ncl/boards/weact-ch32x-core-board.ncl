{
  gpio_pins,

  board = {
    matrix =
      let p = gpio_pins in
      {
        cols = [p.B0, p.B1],
        rows = [p.B3, p.B11],
        key_count = 4,
        implementation = "row_to_col",
      },

    keymap_index_for_key = fun { column_index | Number, row_index | Number, .. } =>
      'Key (column_index + row_index * (std.array.length matrix.cols)),
  },
}
