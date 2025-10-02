let C = import "../ncl/codegen/contracts.ncl" in
{
  gpio_pins | { _ | C.GpioPin },

  board | C.Board = {
    usb = {
      vid = 0xCAFE,
      pid = 0x0005,
      manufacturer = "rgoulter keyboard-labs",
      product = "Pico42"
    },

    matrix =
      let p = gpio_pins in
      {
        cols = [
          p.GP0,
          p.GP1,
          p.GP2,
          p.GP3,
          p.GP4,
          p.GP5,
          p.GP6,
          p.GP7,
          p.GP8,
          p.GP9,
          p.GP10,
          p.GP11,
        ],
        rows = [
          p.GP14,
          p.GP15,
          p.GP16,
          p.GP17,
        ],
        key_count = 42,
      },

    keymap_index_for_key = fun { column_index, row_index } =>
      let [
        k00, k01, k02, k03, k04, k05, k06, k07, k08, k09,
        k10, k11, k12, k13, k14, k15, k16, k17, k18, k19,
        k20, k21, k22, k23, k24, k25, k26, k27, k28, k29,
        k30, k31, k32, k33, k34, k35, k36, k37, k38, k39,
        k40, k41,
      ] = std.array.generate (fun i => 'Key i) 42 in
      let NO = 'NoKey in
      let matrix = [
        [k00, k01, k02, k03, k04,  NO,  NO, k05, k06, k07, k08, k09],
        [k10, k11, k12, k13, k14,  NO,  NO, k15, k16, k17, k18, k19],
        [k20, k21, k22, k23, k24,  NO,  NO, k25, k26, k27, k28, k29],
        [k30, k31, k32, k33, k34, k35, k36, k37, k38, k39, k40, k41],
      ]
      in
      matrix
      |> std.array.at row_index
      |> std.array.at column_index,
  },
}
