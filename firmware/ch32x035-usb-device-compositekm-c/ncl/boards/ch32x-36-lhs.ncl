let C = import "codegen/contracts.ncl" in

{
  gpio_pins | C.GpioPins,

  board | C.Board = {
    matrix =
      let p = gpio_pins in
      {
        cols = [p.A0, p.C3, p.A10, p.A11, p.A9],
        rows = [p.A4, p.B1, p.A7, p.A6],
        # The whole keymap is 36-keys.
        key_count = 36,
        implementation = "row_to_col",
      },

    # The CH32X-36 LHS uses a matrix of 4 rows of 5 columns,
    #  and is split with a similar RHS keyboard.
    keymap_index_for_key = fun { column_index | Number, row_index | Number, .. } =>
      let NO_KEY = 'Error "no key at this column/row index." in
      let k = fun i => 'Ok i in
      let map = [
        [k 0, k 1, k 2, k 3, k 4],
        [k 10, k 11, k 12, k 13, k 14],
        [k 20, k 21, k 22, k 23, k 24],
        [NO_KEY, NO_KEY, k 30, k 31, k 32],
      ]
      in
      let row_map = std.array.at row_index map in
      std.array.at column_index row_map,

    debug = {
      tx = 4,
      usart_af = 1,
    },
  },
}
