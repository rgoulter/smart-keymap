let C = import "keyboard/contracts.ncl" in

{
  gpio_pins | C.GpioPins,

  board | C.Board = {
    matrix =
      let p = gpio_pins in
      {
        cols = [p.B11, p.B10, p.B9, p.B8, p.B7, p.B6, p.A23, p.A22, p.A20, p.A19, p.A18, p.A17],
        rows = [p.A8, p.A2, p.A4, p.A6],
        key_count = 48,
        implementation = "col_to_row",
      },

    keymap_index_for_key = fun { column_index | Number, row_index | Number, .. } =>
      # Since this is an ortholinear keyboard,
      #  this *can* be implemented concisely such as with:
      #
      #    'Key (row_index * 12 + column_index),
      #
      # The following implementation pattern illustrates an approach which
      #  works for more keyboard layouts:
      let [
        k00, k01, k02, k03, k04, k05, k06, k07, k08, k09, k10, k11,
        k12, k13, k14, k15, k16, k17, k18, k19, k20, k21, k22, k23,
        k24, k25, k26, k27, k28, k29, k30, k31, k32, k33, k34, k35,
        k36, k37, k38, k39, k40, k41, k42, k43, k44, k45, k46, k47,
      ] = std.array.generate (fun i => 'Key i) matrix.key_count in
      let NO = 'NoKey in
      let matrix = [
        [k00, k01, k02, k03, k04, k05, k06, k07, k08, k09, k10, k11],
        [k12, k13, k14, k15, k16, k17, k18, k19, k20, k21, k22, k23],
        [k24, k25, k26, k27, k28, k29, k30, k31, k32, k33, k34, k35],
        [k36, k37, k38, k39, k40, k41, k42, k43, k44, k45, k46, k47],
      ]
      in
      matrix
      |> std.array.at row_index
      |> std.array.at column_index,

    led = {
      enabled = true,
      pin = gpio_pins.B12,
    },

    debug = {
      tx = 2,
      usart_af = 2,
    },
  },
}
