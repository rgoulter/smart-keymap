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
  #include "ch32x035_gpio.h"
  #define KEYBOARD_LED_PORT GPIO%{board.led.pin.port}
  #define KEYBOARD_LED_PIN GPIO_Pin_%{std.to_string board.led.pin.pin}
    "%,
      }
    else
      {},
}
