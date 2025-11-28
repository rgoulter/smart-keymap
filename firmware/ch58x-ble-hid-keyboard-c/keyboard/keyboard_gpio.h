#pragma once

#include <stdbool.h>
#include <stdint.h>

// SysTick_IRQn used by core_riscv.h, but not included.
// R32_PA_PIN, R32_PB_PIN used by CH58x_gpio, but not included.
#include "CH583SFR.h"

// FunctionalState used by CH58x_gpio, but not included.
#include "core_riscv.h"

#include "CH58x_gpio.h"

typedef enum {
  KEYBOARD_GPIO_PORT_A = 0,
  KEYBOARD_GPIO_PORT_B = 1,
} keyboard_gpio_port_t;

typedef struct {
  // Port Source (0 = GPIOA, 1 = GPIOB)
  uint8_t port;
  // Pin Source
  uint8_t pin;
} keyboard_gpio_t;

void keyboard_gpio_configure_ipd(keyboard_gpio_t gpio);
void keyboard_gpio_configure_output(keyboard_gpio_t gpio);

void keyboard_gpio_configure_irq_mode(keyboard_gpio_t gpio,
                                      GPIOITModeTpDef mode);

void keyboard_gpio_set(keyboard_gpio_t gpio);
void keyboard_gpio_reset(keyboard_gpio_t gpio);
bool keyboard_gpio_is_set(keyboard_gpio_t gpio);
bool keyboard_gpio_is_reset(keyboard_gpio_t gpio);
