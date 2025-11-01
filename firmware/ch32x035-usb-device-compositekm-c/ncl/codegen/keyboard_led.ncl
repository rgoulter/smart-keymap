let C = import "contracts.ncl" in

{
  contracts = {
    Board = {
      led = {
        enabled
          | doc "Whether to enable the LED."
          | Bool
          | default
          = false,

        pin
          | doc "The pin to use for the LED."
          | C.GpioPin,
      },

      ..
    },
  },

  board
    | C.Board,

  init_fragments =
    if board.led.enabled then
      {
        keyboard_led = m%"
          // Init LED
          {
            GPIO_InitTypeDef GPIO_InitStructure = {0};
            GPIO_InitStructure.GPIO_Pin = GPIO_Pin_%{std.to_string board.led.pin.pin};
            GPIO_InitStructure.GPIO_Mode = GPIO_Mode_Out_PP;
            GPIO_InitStructure.GPIO_Speed = GPIO_Speed_50MHz;
            GPIO_Init(GPIO%{board.led.pin.port}, &GPIO_InitStructure);
          }
          "%,
      }
    else
      {},

  cmakelists =
    if board.led.enabled then
      {
        keyboard_led = m%"
    target_compile_definitions(keyboard INTERFACE KEYBOARD_LED_ENABLED)
    "%,
      }
    else
      {},

  includes =
    if board.led.enabled then
      {
        keyboard_led = m%"
  #pragma once
  void keyboard_led_tick(void);
    "%,
      }
    else
      {},

  sources =
    if board.led.enabled then
      {
        keyboard_led = m%"
  #include <stdbool.h>

  #include "debug.h"

  #include "ch32x035_gpio.h"

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
      }
    else
      {},
}
