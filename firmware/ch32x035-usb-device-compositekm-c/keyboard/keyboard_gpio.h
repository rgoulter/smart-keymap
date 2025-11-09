#pragma once

#include <stdbool.h>
#include <stdint.h>

typedef struct {
  // Port Source
  uint8_t port;
  // Pin Source
  uint8_t pin;
} keyboard_gpio_t;

void keyboard_gpio_configure_ipu(keyboard_gpio_t gpio);
void keyboard_gpio_configure_output(keyboard_gpio_t gpio);

void keyboard_gpio_set(keyboard_gpio_t gpio);
void keyboard_gpio_reset(keyboard_gpio_t gpio);
bool keyboard_gpio_is_set(keyboard_gpio_t gpio);
bool keyboard_gpio_is_reset(keyboard_gpio_t gpio);
