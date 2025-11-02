#pragma once
#ifdef KEYBOARD_LED_ENABLED
#include "generated/keyboard_led.h" // IWYU pragma: export
void keyboard_led_init(void);
void keyboard_led_tick(void);
#endif
