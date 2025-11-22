let C = import "keyboard/contracts.ncl" in

{
  gpio_pins,

  board | C.Board = {
    matrix
      | doc "the cols/rows, and number of keys, used for generating the matrix scan code."
      =
        let gp = gpio_pins in
        {
          cols = [gp.A4, gp.A5, gp.A15, gp.A14, gp.A13, gp.A12, gp.A11, gp.A10],
          rows = [gp.A8, gp.A9, gp.B15, gp.B14, gp.B13, gp.B12, gp.B7, gp.B4],
          key_count = 60,
          implementation = "col_to_row",
        },

    # The WABBLE-60 uses a digital matrix of 8x8 rows and columns,
    #  forming a physical 5x12 matrix of keys.
    #
    # The physical matrix arranges keys column-wise.
    #
    # Want the keymap index to refer to keys row-wise.
    #
    # The given column_index and row_index refer to (digital) row/col, 0..7.
    keymap_index_for_key
      | doc "Returns the keymap index for the key corresponding to the (0-based) digital column_index and row_index."
      = fun { column_index | Number, row_index | Number, .. } =>
        let columnwise_index = column_index * 8 + row_index in
        if columnwise_index >= board.matrix.key_count then
          'NoKey
        else
          let physical_column_index = std.number.floor (columnwise_index / 5) in
          let physical_row_index = columnwise_index % 5 in
          let rowwise_index = physical_row_index * 12 + physical_column_index in
          'Key rowwise_index,
  },
}
