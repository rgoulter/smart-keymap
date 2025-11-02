let C = import "contracts.ncl" in

{
  contracts = {
    Board = {
      disable_swd
        | doc "Whether to disable SWD."
        | Bool
        | default
        = false,

      ..
    },
  },

  matrix_scan = {
    is_sw_1_1_pressed,
  },

  board
    | C.Board,

  disable_swd_fragment =
    if board.disable_swd then
      m%"
        // Disable SWD
        RCC_APB2PeriphClockCmd(RCC_APB2Periph_AFIO, ENABLE);
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

  sources.keyboard = m%"
  #include <stdbool.h>

  #include "debug.h"

  #include "ch32x035_flash.h"
  #include "ch32x035_gpio.h"
  #include "ch32x035_rcc.h"

  #include "smart_keymap.h"

  #include "keyboard_matrix.h"

  #ifdef KEYBOARD_LED_ENABLED
  #include "keyboard_led.h"
  #endif
  #ifdef KEYBOARD_SPLIT
  #include "keyboard_split.h"
  #endif

  void keyboard_reset_to_bootloader(void) {
      SystemReset_StartMode(Start_Mode_BOOT);
      NVIC_SystemReset();
  }

  void keyboard_init(void) {
      keyboard_matrix_init();

      %{matrix_scan.is_sw_1_1_pressed}
      if (sw_1_1_is_pressed) {
          keyboard_reset_to_bootloader();
      }

      keymap_register_callback(KEYMAP_CALLBACK_BOOTLOADER, keyboard_reset_to_bootloader);

      %{disable_swd_fragment}

      %{led_init_fragment}

  #ifdef KEYBOARD_SPLIT
      keyboard_split_init();
  #endif
  }
  "%,
}
