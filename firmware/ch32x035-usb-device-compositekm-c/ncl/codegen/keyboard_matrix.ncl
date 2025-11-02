let C = import "codegen/contracts.ncl" in

(import "matrix/row_to_col.ncl")
& (import "matrix/col_to_row.ncl")
& {
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

  keyboard_matrix_scan
    | doc "Generates C fragment with the keyboard_matrix_scan function, for the given cols/rows."
    = fun args @ { cols, rows, .. } =>
      m%"
  %{matrix_scan.keyboard_matrix_scan_raw args}

  void keyboard_matrix_scan(void) {
      static bool debounced_state[%{std.to_string board.matrix.key_count}] = { false };
      static bool previous_raw_scan[%{std.to_string board.matrix.key_count}] = { false };
      static bool current_raw_scan[%{std.to_string board.matrix.key_count}] = { false };
      static uint8_t debounce_counter[%{std.to_string board.matrix.key_count}] = { 0 };

      keyboard_matrix_scan_raw(current_raw_scan);

      for (uint32_t i = 0; i < %{std.to_string board.matrix.key_count}; i++) {
          if (current_raw_scan[i] == debounced_state[i]) {
              debounce_counter[i] = 0;
          } else {
              if (current_raw_scan[i] != previous_raw_scan[i]) {
                  debounce_counter[i] = 0;
              } else {
                  debounce_counter[i]++;
              }

              if (debounce_counter[i] >= 5) {
                  key_state_changed(i, current_raw_scan[i]);
                  debounced_state[i] = current_raw_scan[i];
              }
          }

          previous_raw_scan[i] = current_raw_scan[i];
      }
  }
  "%,

  includes.keyboard_matrix = m%"
    #pragma once

    void keyboard_matrix_init(void);

    void keyboard_matrix_scan(void);
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

  void key_state_changed(uint32_t index, bool new_state) {
      KeymapInputEvent ev = { .event_type = 0, .value = index };
      if (new_state) {
          ev.event_type = KeymapEventPress;
      } else {
          ev.event_type = KeymapEventRelease;
      }
      keymap_register_input_event(ev);

      #ifdef KEYBOARD_SPLIT
      keyboard_split_write(ev);
      #endif
  }

  %{keyboard_matrix_scan board.matrix}
  "%,
}
