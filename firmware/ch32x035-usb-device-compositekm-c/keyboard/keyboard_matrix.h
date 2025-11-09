#pragma once
#include "keyboard_gpio.h"
#include <stdbool.h>
#include <stdint.h>
extern const keyboard_gpio_t keyboard_matrix_cols[KEYBOARD_MATRIX_COL_COUNT];
extern const keyboard_gpio_t keyboard_matrix_rows[KEYBOARD_MATRIX_ROW_COUNT];
extern const int16_t keymap_indices[KEYBOARD_MATRIX_ROW_COUNT]
                                   [KEYBOARD_MATRIX_COL_COUNT];
typedef struct {
  uint8_t row;
  uint8_t column;
} keyboard_matrix_coordinate_t;
bool keyboard_matrix_is_sw_1_1_pressed();
void keyboard_matrix_init(void);
void keyboard_matrix_scan(void);
