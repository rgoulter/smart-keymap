let C = import "codegen/contracts.ncl" in

(import "matrix/col_to_row.ncl")
& {
  contracts = {
    Board = {
      matrix
        | doc "the cols/rows, and number of keys, used for generating the matrix scan code."
        | {
          cols | Array C.GpioPin,
          rows | Array C.GpioPin,
          key_count | Number,
          implementation | String | default = "col_to_row",
        },

      ..
    },
  },

  matrix_scan = {
    keyboard_matrix_init,
    keyboard_matrix_scan_raw,
    is_sw_1_1_pressed,
  },

  board
    | C.Board,

  gpio_pins
    | C.GpioPins,

  includes.keyboard_matrix = m%"
    #pragma once

    void keyboard_matrix_init(void);

    void keyboard_matrix_scan(void);
    "%,

  sources.keyboard_matrix = m%"
  #include <stdbool.h>

  #include "CH58x_common.h"

  #include "smart_keymap.h"

  bool previousScan[%{std.to_string board.matrix.key_count}] = { false };
  bool currentScan[%{std.to_string board.matrix.key_count}] = { false };

  %{matrix_scan.keyboard_matrix_init board.matrix}

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

  "%,
}
