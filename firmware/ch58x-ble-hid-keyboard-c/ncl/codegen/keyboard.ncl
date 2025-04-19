(import "matrix/col_to_row.ncl")
& {
  gpio_pins
    : { _ : { port : String, pin : Number } }
    | doc "Record with A1, A2, ... A23, B1, B2, ..., B24, mapped to { port, pin } records."
    =
      let ports = ["A", "B"] in
      let pins = std.array.range 0 24 in

      std.array.flat_map
        (fun port_name => std.array.map (fun pin_num => { "%{port_name}%{std.to_string pin_num}" = { "port" = port_name, "pin" = pin_num } }) pins)
        ports
      |> (std.array.fold_left (&) {}) | { _ : { port : String, pin : Number } },

  board = {
    matrix
      | {
        cols | Array { port | String, pin | Number },
        rows | Array { port | String, pin | Number },
        key_count | Number,
        implementation | String,
      }
      | doc "the cols/rows, and number of keys, used for generating the matrix scan code.",

    keymap_index_for_key
      | doc "returns the keymap index for the key corresponding to the (0-based) digital column_index and row_index.",
  },

  matrix_scan = {
    keyboard_matrix_init,
    keyboard_matrix_scan,
  },

  matrix_scan_impl,

  matrix_scan = std.record.get board.matrix.implementation matrix_scan_impl,

  keyboard_init
    | doc "C fragment with the keyboard_init function."
    = m%"
  void keyboard_init(void) {
      keyboard_matrix_init();
  }
  "%,

  output
    | doc "C code with keyboard_matrix_init and keyboard_matrix_scan functions."
    = m%"
  #include <stdbool.h>

  #include "CH58x_common.h"

  #include "smart_keymap.h"

  bool previousScan[%{std.to_string board.matrix.key_count}] = { false };
  bool currentScan[%{std.to_string board.matrix.key_count}] = { false };

  %{matrix_scan.keyboard_matrix_init board.matrix}

  %{keyboard_init}

  void handle_index(uint32_t index) {
      if (previousScan[index] != currentScan[index]) {
          if (currentScan[index]) {
              keymap_register_input_keypress(index);
          } else {
              keymap_register_input_keyrelease(index);
          }

          previousScan[index] = currentScan[index];
      }
  }

  %{matrix_scan.keyboard_matrix_scan board.matrix}

  "%
}
