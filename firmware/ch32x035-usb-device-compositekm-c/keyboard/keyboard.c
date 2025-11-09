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

  if (keyboard_matrix_is_sw_1_1_pressed()) {
    keyboard_reset_to_bootloader();
  }

  keymap_register_callback(KEYMAP_CALLBACK_BOOTLOADER,
                           keyboard_reset_to_bootloader);

#ifdef KEYBOARD_DISABLE_SWD
  // Disable SWD
  RCC_APB2PeriphClockCmd(RCC_APB2Periph_AFIO, ENABLE);
  GPIO_PinRemapConfig(GPIO_Remap_SWJ_Disable, ENABLE);
#endif

#ifdef KEYBOARD_LED_ENABLED
  keyboard_led_init();
#endif

#ifdef KEYBOARD_SPLIT
  keyboard_split_init();
#endif
}
