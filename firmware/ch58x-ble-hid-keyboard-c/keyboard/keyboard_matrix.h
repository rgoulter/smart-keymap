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

#define KEYBOARD_MATRIX_KEY_PRESSED (1)
#define KEYBOARD_MATRIX_KEY_RELEASED (-1)

void keyboard_matrix_scan(int8_t new_states[KEYBOARD_MATRIX_KEY_COUNT]);
void keyboard_matrix_configure_for_scanning(void);
void keyboard_matrix_configure_for_interrupt(void);

uint8_t keyboard_matrix_pressed_keys_count();
