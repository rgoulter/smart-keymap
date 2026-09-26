#ifdef KEYBOARD_LED_ENABLED
#include "keyboard_led.h"

#include <stdbool.h>

#include "debug.h"

#include "ch32x035_gpio.h"

static uint16_t keyboard_led_timer = 0;
static uint8_t keyboard_led_state = 0;
static uint8_t keyboard_led_claimed = 0;

void keyboard_led_init(void) {
  GPIO_InitTypeDef GPIO_InitStructure = {0};
  GPIO_InitStructure.GPIO_Pin = KEYBOARD_LED_PIN;
  GPIO_InitStructure.GPIO_Mode = GPIO_Mode_Out_PP;
  GPIO_InitStructure.GPIO_Speed = GPIO_Speed_50MHz;
  GPIO_Init(KEYBOARD_LED_PORT, &GPIO_InitStructure);
}

void keyboard_led_claim(void) { keyboard_led_claimed = 1; }

void keyboard_led_set(uint8_t on) {
  keyboard_led_state = on ? 1 : 0;
  GPIO_WriteBit(KEYBOARD_LED_PORT, KEYBOARD_LED_PIN, keyboard_led_state);
}

void keyboard_led_tick(void) {
  // Toggle the LED every 1000 ticks, unless main has claimed the pin.
  keyboard_led_timer++;
  if (keyboard_led_claimed || keyboard_led_timer <= 1000) {
    return;
  }
  keyboard_led_timer = 0;
  keyboard_led_set(1 - keyboard_led_state);
}
#endif
