#pragma once
#include "generated/keyboard_matrix.h" // IWYU pragma: export
#include <stdbool.h>
bool keyboard_matrix_is_sw_1_1_pressed();
void keyboard_matrix_init(void);
void keyboard_matrix_scan(void);
