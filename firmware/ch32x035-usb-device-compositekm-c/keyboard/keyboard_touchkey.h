#pragma once

#include "keyboard_gpio.h"

void keyboard_touchkey_configure_drive(keyboard_gpio_t gpio);
void keyboard_touchkey_configure_sense(keyboard_gpio_t gpio);

void keyboard_touchkey_init(void);
uint16_t keyboard_touchkey_read(uint8_t ch);
