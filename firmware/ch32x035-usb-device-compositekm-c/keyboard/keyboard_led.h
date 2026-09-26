#pragma once
#ifdef KEYBOARD_LED_ENABLED
#include "generated/keyboard_led.h" // IWYU pragma: export
void keyboard_led_init(void);
void keyboard_led_tick(void);
/* Main owns the pin after this. keyboard_led_tick no longer writes it. */
void keyboard_led_claim(void);
void keyboard_led_set(uint8_t on);
#endif
