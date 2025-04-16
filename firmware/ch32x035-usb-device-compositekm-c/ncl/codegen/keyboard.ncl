let C = import "contracts.ncl" in

(import "matrix-row_to_col.ncl")
& (import "matrix-col_to_row.ncl")
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

      disable_swd
        | doc "Whether to disable SWD."
        | Bool
        | default
        = false,

      ..
    },
  },
  
  matrix_scan = {
    keyboard_matrix_init,
    keyboard_matrix_scan_raw,
    is_sw_1_1_pressed,
  },

  matrix_scan_impl,

  matrix_scan = std.record.get board.matrix.implementation matrix_scan_impl,

  board
   | C.Board,

  gpio_pins
    | C.GpioPins,

  enter_bootloader_check = m%"
    %{matrix_scan.is_sw_1_1_pressed}
    if (sw_1_1_is_pressed) {
        keyboard_reset_to_bootloader();
    }
    "%,

  disable_swd_fragment =
    if board.disable_swd then
      m%"
        // Disable SWD
        GPIO_PinRemapConfig(GPIO_Remap_SWJ_Disable, ENABLE);
        "%
    else
      "",

  init_fragments,

  led_init_fragment =
    if std.record.has_field "keyboard_led" init_fragments then
      init_fragments.keyboard_led
    else
      "",

  keyboard_init
    | doc "C fragment with the keyboard_init function."
    = m%"
  void keyboard_reset_to_bootloader(void) {
      SystemReset_StartMode(Start_Mode_BOOT);
      NVIC_SystemReset();
  }

  void keyboard_init(void) {
      keyboard_matrix_init();

      %{enter_bootloader_check}

      keymap_register_callback(KEYMAP_CALLBACK_BOOTLOADER, keyboard_reset_to_bootloader);

      %{disable_swd_fragment}

      %{led_init_fragment}
  }
  "%,

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

  cmakelists.keyboard
    = m%"
    target_sources(keyboard_codegen INTERFACE ${CMAKE_CURRENT_SOURCE_DIR}/keyboard.c)
    "%,

  sources.keyboard
    | doc "C code with keyboard_matrix_init and keyboard_matrix_scan functions."
    = m%"
  #include <stdbool.h>

  #include "debug.h"

  #include "ch32x035_flash.h"
  #include "ch32x035_gpio.h"
  #include "ch32x035_rcc.h"

  #include "smart_keymap.h"

  %{matrix_scan.keyboard_matrix_init board.matrix}

  %{keyboard_init}

  void key_state_changed(uint32_t index, bool new_state) {
      if (new_state) {
          keymap_register_input_keypress(index);
      } else {
          keymap_register_input_keyrelease(index);
      }
  }

  %{keyboard_matrix_scan board.matrix}
  "%,
}
