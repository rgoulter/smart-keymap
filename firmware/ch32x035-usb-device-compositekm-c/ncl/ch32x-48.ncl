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
        cols = [p.B1, p.A6, p.A9, p.A0, p.C3, p.A10, p.A11],
        rows = [p.A4, p.C19, p.A3, p.A2, p.A1, p.C18, p.A7],
        num_keys = 48,
      },

  # The CH32X-48 uses a digital matrix of 7x7 rows and columns,
  #  forming a physical 4x12 matrix of keys.
  #
  # The physical matrix arranges keys column-wise.
  #
  # Want the keymap index to refer to keys row-wise.
  #
  # The given column_index and row_index refer to (digital) row/col, 0..6.
  keymap_index_for_key
    | doc "Returns the keymap index for the key corresponding to the (0-based) digital column_index and row_index."
    = fun { column_index | Number, row_index | Number, .. } =>
      let columnwise_index = column_index * 7 + row_index in
      let physical_column_index = std.number.floor (columnwise_index / 4) in
      let physical_row_index = columnwise_index % 4 in
      let rowwise_index = physical_row_index * 12 + physical_column_index in
      rowwise_index,
}
