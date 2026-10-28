let C = import "keyboard/contracts.ncl" in

{
  gpio_pins | C.GpioPins,

  board | C.Board =
    let p = gpio_pins in
    {
      matrix = {
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
          # COL11 - COL14
          p.A16,
          p.A17,
          p.A18,
          p.A19,
        ],
        rows = [
          # ROW1..ROW5
          # diode cathode -> ROW, so implementation = col_to_row
          p.A8,
          p.A9,
          p.A4,
          p.A5,
          p.A6,
        ],
        key_count = 66,
        implementation = "col_to_row",
      },

      # CH32X-60-improved: GH60-ish ANSI 60%.
      # Electrical matrix is 5×14 with 4 holes → 66 keys:
      #   R1: 14 (1u×13 + 2u Bksp)
      #   R2: 14 (1.5u Tab + 1u×12 + 1.5u)
      #   R3: 13 (1.75u Caps + 1u×11 + 2.25u Enter)
      #   R4: 12 (2.25u Shift + 1u×10 + 2.75u RShift)
      #   R5: 13 (1.25u×4 + 1u×5 + 1.25u×4 split space)
      #
      # kRCC is row R, column CC (1-based, same as SW_R_CC).
      # The let-binding indent loosely follows ANSI row-stagger;
      # keymap indices are row-wise, skipping holes.
      keymap_index_for_key = fun { column_index | Number, row_index | Number, .. } =>
        let [
          k101, k102, k103, k104, k105, k106, k107, k108, k109, k110, k111, k112, k113,   k114,
            k201, k202, k203, k204, k205, k206, k207, k208, k209, k210, k211, k212, k213, k214,
              k301, k302, k303, k304, k305, k306, k307, k308, k309, k310, k311, k312,   k313,
                k401, k402, k403, k404, k405, k406, k407, k408, k409, k410, k411,     k412,
          k501,  k502,  k503,  k504,  k505, k506, k507, k508, k509,  k510,  k511,  k512,  k513,
        ] = std.array.generate (fun i => 'Key i) matrix.key_count in
        let NO = 'NoKey in
        let matrix = [
          [k101, k102, k103, k104, k105, k106, k107, k108, k109, k110, k111, k112, k113, k114],
          [k201, k202, k203, k204, k205, k206, k207, k208, k209, k210, k211, k212, k213, k214],
          [k301, k302, k303, k304, k305, k306, k307, k308, k309, k310, k311, k312, k313,  NO],
          [k401, k402, k403, k404, k405, k406, k407, k408, k409, k410, k411, k412,  NO,  NO],
          [k501, k502, k503, k504, k505, k506, k507, k508, k509, k510, k511, k512, k513,  NO],
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
