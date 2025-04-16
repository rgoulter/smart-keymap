let C = import "contracts.ncl" in

(import "codegen-matrix-row_to_col.ncl")
& (import "codegen-matrix-col_to_row.ncl")
& {
  contracts = {
    GpioPin = { port | String, pin | Number },

    GpioPins = { _ | GpioPin },

    Board = {
      matrix
        | doc "the cols/rows, and number of keys, used for generating the matrix scan code."
        | {
          cols | Array GpioPin,
          rows | Array GpioPin,
          key_count | Number,
          implementation | String,
        },

      disable_swd
        | doc "Whether to disable SWD."
        | Bool
        | default
        = false,

      led = {
        enabled
          | doc "Whether to enable the LED."
          | Bool
          | default
          = false,

        pin
          | doc "The pin to use for the LED."
          | GpioPin,
      },

      ..
    },
  },
  
  gpio_pins
    | doc "Record with A1, A2, ... A23, B1, B2, ..., C24, mapped to { port, pin } records."
    | C.GpioPins
    : { _ : { port : String, pin : Number } }
    =
      let ports = ["A", "B", "C"] in
      let pins = std.array.range 0 24 in

      std.array.flat_map
        (fun port_name => std.array.map (fun pin_num => { "%{port_name}%{std.to_string pin_num}" = { "port" = port_name, "pin" = pin_num } }) pins)
        ports
      |> (std.array.fold_left (&) {}) | { _ : { port : String, pin : Number } },

  matrix_scan = {
    keyboard_matrix_init,
    keyboard_matrix_scan_raw,
    is_sw_1_1_pressed,
  },

  matrix_scan_impl,

  matrix_scan = std.record.get board.matrix.implementation matrix_scan_impl,

  board
   | C.Board
   ,

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

  led_init_fragment =
    if board.led.enabled then
      m%"
        // Init LED
        {
          GPIO_InitTypeDef GPIO_InitStructure = {0};
          GPIO_InitStructure.GPIO_Pin = GPIO_Pin_%{std.to_string board.led.pin.pin};
          GPIO_InitStructure.GPIO_Mode = GPIO_Mode_Out_PP;
          GPIO_InitStructure.GPIO_Speed = GPIO_Speed_50MHz;
          GPIO_Init(GPIO%{board.led.pin.port}, &GPIO_InitStructure);
        }
        "%
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

  # Function which blinks the LED
  keyboard_led_tick =
    if board.led.enabled then
      m%"
        static uint16_t keyboard_led_timer = 0;
        static uint8_t keyboard_led_state = 0;

        void keyboard_led_tick(void) {
            // Toggle the LED every 1000 ticks
            keyboard_led_timer++;
            if (keyboard_led_timer > 1000) {
                keyboard_led_timer = 0;
                keyboard_led_state = 1 - keyboard_led_state;

                GPIO_WriteBit(GPIO%{board.led.pin.port}, GPIO_Pin_%{std.to_string board.led.pin.pin}, keyboard_led_state);
            }
        }
        "%
    else
      "",

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

  %{keyboard_led_tick}
  "%
}
