let C = import "keyboard/contracts.ncl" in

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
      # The keyboard has 36 keys,
      #  but the LHS only has half of these.
      let [
        k00, k01, k02, k03, k04, k05, k06, k07, k08, k09,
        k10, k11, k12, k13, k14, k15, k16, k17, k18, k19,
        k20, k21, k22, k23, k24, k25, k26, k27, k28, k29,
                  k30, k31, k32, k33, k34, k35,
      ] = std.array.generate (fun i => 'Key i) matrix.key_count in
      let NO = 'NoKey in
      let matrix = [
        [k00, k01, k02, k03, k04],
        [k10, k11, k12, k13, k14],
        [k20, k21, k22, k23, k24],
        [ NO,  NO, k30, k31, k32],
      ]
      in
      matrix
      |> std.array.at row_index
      |> std.array.at column_index,

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
