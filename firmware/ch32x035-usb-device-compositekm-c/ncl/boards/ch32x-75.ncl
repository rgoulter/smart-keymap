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
      # Since this is an ortholinear keyboard,
      #  this *can* be implemented concisely such as with:
      #
      #    'Key (row_index * 15 + column_index),
      #
      # The following implementation pattern illustrates an approach which
      #  works for more keyboard layouts:
      let [
        k00, k01, k02, k03, k04, k05, k06, k07, k08, k09, k10, k11, k12, k13, k14,
        k15, k16, k17, k18, k19, k20, k21, k22, k23, k24, k25, k26, k27, k28, k29,
        k30, k31, k32, k33, k34, k35, k36, k37, k38, k39, k40, k41, k42, k43, k44,
        k45, k46, k47, k48, k49, k50, k51, k52, k53, k54, k55, k56, k57, k58, k59,
        k60, k61, k62, k63, k64, k65, k66, k67, k68, k69, k70, k71, k72, k73, k74,
      ] = std.array.generate (fun i => 'Key i) matrix.key_count in
      let NO = 'NoKey in
      let matrix = [
        [k00, k01, k02, k03, k04, k05, k06, k07, k08, k09, k10, k11, k12, k13, k14],
        [k15, k16, k17, k18, k19, k20, k21, k22, k23, k24, k25, k26, k27, k28, k29],
        [k30, k31, k32, k33, k34, k35, k36, k37, k38, k39, k40, k41, k42, k43, k44],
        [k45, k46, k47, k48, k49, k50, k51, k52, k53, k54, k55, k56, k57, k58, k59],
        [k60, k61, k62, k63, k64, k65, k66, k67, k68, k69, k70, k71, k72, k73, k74],
      ]
      in
      matrix
      |> std.array.at row_index
      |> std.array.at column_index,

    led = {
      enabled = true,
      pin = gpio_pins.B9,
    },

    debug.tx = 2,
  },
}
