let C = import "codegen/contracts.ncl" in

{
  gpio_pins | C.GpioPins,

  board | C.Board = {
    matrix =
      let p = gpio_pins in
      {
        cols = [p.A0, p.A1, p.A4, p.B1, p.A6] |> std.array.reverse,
        rows = [p.A11, p.A10, p.C3, p.A9],
        # The whole keymap is 36-keys.
        key_count = 36,
        implementation = "row_to_col",
      },

    # The CH32X-36 RHS uses a matrix of 4 rows of 5 columns,
    #  and is split with a similar RHS keyboard.
    keymap_index_for_key = fun { column_index | Number, row_index | Number, .. } =>
      let NO_KEY = 'Error "no key at this column/row index." in
      let k = fun i => 'Ok i in
      let map = [
        [k 5,  k  6, k  7, k  8, k  9],
        [k 15, k 16, k 17, k 18, k 19],
        [k 25, k 26, k 27, k 28, k 29],
        [k 33, k 34, k 35, NO_KEY, NO_KEY],
      ]
      in
      let row_map = std.array.at row_index map in
      std.array.at column_index row_map,

    debug = {
      tx = 4,
      usart_af = 1,
    },

    split = {
      enabled = true,
      usart = 2,
    },
  },
}
