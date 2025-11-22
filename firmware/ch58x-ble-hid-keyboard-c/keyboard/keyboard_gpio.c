#include "keyboard_gpio.h"

// SysTick_IRQn used by core_riscv.h, but not included.
// R32_PA_PIN, R32_PB_PIN used by CH58x_gpio, but not included.
#include "CH583SFR.h"

// FunctionalState used by CH58x_gpio, but not included.
#include "core_riscv.h"

#include "CH58x_gpio.h"

uint32_t to_ch58x_pin(keyboard_gpio_t gpio_source) {
  return 1 << gpio_source.pin;
}

void keyboard_gpio_configure_ipd(keyboard_gpio_t gpio_source) {
  uint32_t pin = to_ch58x_pin(gpio_source);
  switch (gpio_source.port) {
  case KEYBOARD_GPIO_PORT_A:
    GPIOA_ModeCfg(pin, GPIO_ModeIN_PD);
    break;
  case KEYBOARD_GPIO_PORT_B:
    GPIOB_ModeCfg(pin, GPIO_ModeIN_PD);
    break;
  default:
    break;
  }
}

void keyboard_gpio_configure_output(keyboard_gpio_t gpio_source) {
  uint32_t pin = to_ch58x_pin(gpio_source);
  switch (gpio_source.port) {
  case KEYBOARD_GPIO_PORT_A:
    GPIOA_ModeCfg(pin, GPIO_ModeOut_PP_5mA);
    break;
  case KEYBOARD_GPIO_PORT_B:
    GPIOB_ModeCfg(pin, GPIO_ModeOut_PP_5mA);
    break;
  default:
    break;
  }
}

void keyboard_gpio_set(keyboard_gpio_t gpio_source) {
  uint32_t pin = to_ch58x_pin(gpio_source);
  switch (gpio_source.port) {
  case KEYBOARD_GPIO_PORT_A:
    GPIOA_SetBits(pin);
    break;
  case KEYBOARD_GPIO_PORT_B:
    GPIOB_SetBits(pin);
    break;
  default:
    break;
  }
}

void keyboard_gpio_reset(keyboard_gpio_t gpio_source) {
  uint32_t pin = to_ch58x_pin(gpio_source);
  switch (gpio_source.port) {
  case KEYBOARD_GPIO_PORT_A:
    GPIOA_ResetBits(pin);
    break;
  case KEYBOARD_GPIO_PORT_B:
    GPIOB_ResetBits(pin);
    break;
  default:
    break;
  }
}

bool keyboard_gpio_is_set(keyboard_gpio_t gpio_source) {
  uint32_t pin = to_ch58x_pin(gpio_source);
  switch (gpio_source.port) {
  case KEYBOARD_GPIO_PORT_A:
    return GPIOA_ReadPortPin(pin) != 0;
  case KEYBOARD_GPIO_PORT_B:
    return GPIOB_ReadPortPin(pin) != 0;
  default:
    return false;
  }
}

bool keyboard_gpio_is_reset(keyboard_gpio_t gpio_source) {
  uint32_t pin = to_ch58x_pin(gpio_source);
  switch (gpio_source.port) {
  case KEYBOARD_GPIO_PORT_A:
    return GPIOA_ReadPortPin(pin) == 0;
  case KEYBOARD_GPIO_PORT_B:
    return GPIOB_ReadPortPin(pin) == 0;
  default:
    return false;
  }
}
