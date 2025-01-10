{
  gpio_pins,

  board
    | {
      cols | Array { port | String, pin | Number },
      rows | Array { port | String, pin | Number },
      num_keys | Number,
    }
    | doc "the cols/rows, and number of keys, used for generating the matrix scan code."
    =
      let p = gpio_pins in
      {
        cols = [p.B0, p.B1],
        rows = [p.B3, p.B11],
        num_keys = 4,
      },

  keymap_index_for_key
    | doc "Returns the keymap index for the key corresponding to the (0-based) digital column_index and row_index."
    = fun { column_index | Number, row_index | Number, .. } =>
      column_index + row_index * (std.array.length board.cols),
}
