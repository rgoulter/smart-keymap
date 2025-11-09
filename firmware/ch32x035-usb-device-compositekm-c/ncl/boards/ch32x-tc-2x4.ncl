let C = import "keyboard/contracts.ncl" in

{
  gpio_pins | C.GpioPins,

  board | C.Board = {
    matrix =
      let p = gpio_pins in
      {
        # Sense columns
        cols = [p.A4, p.A6, p.A7, p.B1],
        # Drive rows
        rows = [
            p.A5,
            # p.A10,
        ],
        key_count = 4,

        implementation = "touchkey_mutual",
      },

    keymap_index_for_key = fun { column_index | Number, row_index | Number, .. } =>
      let [
        k00, k01, k02, k03,
        # k04, k05, k06, k07,
      ] = std.array.generate (fun i => 'Key i) matrix.key_count in
      let NO = 'NoKey in
      let matrix = [
        [k00, k01, k02, k03],
        # [k04, k05, k06, k07],
      ]
      in
      matrix
      |> std.array.at row_index
      |> std.array.at column_index,

    led = {
      enabled = true,
      pin = gpio_pins.A3,
    },

    # TX4 available if Col3 not used.
    debug.tx = 1,
  },
}
