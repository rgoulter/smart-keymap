let C = import "keyboard/contracts.ncl" in

{
  contracts = {
    Board = {
      matrix
        | doc "the cols/rows, and number of keys, used for generating the matrix scan code."
        | {
          cols | Array C.GpioPin,
          rows | Array C.GpioPin,
          key_count | Number,
          implementation | String,
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

  cmakelists.keyboard_matrix =
    [
      "target_compile_definitions(keyboard INTERFACE KEYBOARD_MATRIX_IMPL_%{board.matrix.implementation |> std.string.uppercase})",
      "target_compile_definitions(keyboard INTERFACE KEYBOARD_MATRIX_KEY_COUNT=%{board.matrix.key_count |> std.to_string})",
      "target_compile_definitions(keyboard INTERFACE KEYBOARD_MATRIX_ROW_COUNT=%{board.matrix.rows |> std.array.length |> std.to_string})",
      "target_compile_definitions(keyboard INTERFACE KEYBOARD_MATRIX_COL_COUNT=%{board.matrix.cols |> std.array.length |> std.to_string})",
    ]
    |> std.string.join "\n",

  includes.keyboard_matrix = m%"
    #pragma once
    "%,

  sources.keyboard_matrix = m%"
  #include <stdbool.h>

  #include "debug.h"

  #include "ch32x035_gpio.h"
  #include "ch32x035_rcc.h"

  #include "smart_keymap.h"

  #ifdef KEYBOARD_SPLIT
  #include "keyboard_split.h"
  #endif

  %{matrix_scan.keyboard_matrix_init board.matrix}

  bool keyboard_matrix_is_sw_1_1_pressed() {
      %{matrix_scan.is_sw_1_1_pressed}
      return sw_1_1_is_pressed;
  }

  %{matrix_scan.keyboard_matrix_scan_raw board.matrix}
  "%,
}
