#pragma once

#include <stdbool.h>
#include <stdint.h>

#include "ch32x035.h"

typedef struct {
  // Port Source
  uint8_t port;
  // Pin Source
  uint8_t pin;
} keyboard_gpio_t;

typedef struct {
  GPIO_TypeDef *port;
  uint32_t pin;
} ch32x_gpio_t;

ch32x_gpio_t to_ch32x_gpio(keyboard_gpio_t gpio_source);

void keyboard_gpio_configure_ipu(keyboard_gpio_t gpio);
void keyboard_gpio_configure_output(keyboard_gpio_t gpio);

void keyboard_gpio_set(keyboard_gpio_t gpio);
void keyboard_gpio_reset(keyboard_gpio_t gpio);
bool keyboard_gpio_is_set(keyboard_gpio_t gpio);
bool keyboard_gpio_is_reset(keyboard_gpio_t gpio);
