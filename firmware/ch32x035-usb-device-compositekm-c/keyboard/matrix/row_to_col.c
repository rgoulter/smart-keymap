#ifdef KEYBOARD_MATRIX_IMPL_ROW_TO_COL
#include "debug.h"

#include "ch32x035_rcc.h"

#include "keyboard_gpio.h"
#include "keyboard_matrix.h"

void init_column(keyboard_gpio_t col) { keyboard_gpio_configure_output(col); }

void init_row(keyboard_gpio_t row) { keyboard_gpio_configure_ipu(row); }

void keyboard_matrix_init(void) {
  // NOTE: this implementation is for diode's cathodes(-) which face COLUMNS.
  //  i.e. that current flows from ROWS to COLUMNS.
  // This implementation configures ROWS as Input (Pull Up),
  // and COLS as Output (reset to low when scanned).

  RCC_APB2PeriphClockCmd(RCC_APB2Periph_GPIOA, ENABLE);
  RCC_APB2PeriphClockCmd(RCC_APB2Periph_GPIOB, ENABLE);
  RCC_APB2PeriphClockCmd(RCC_APB2Periph_GPIOC, ENABLE);

  // Cols
  for (uint8_t i = 0; i < KEYBOARD_MATRIX_COL_COUNT; i++) {
    init_column(keyboard_matrix_cols[i]);
    keyboard_gpio_set(keyboard_matrix_cols[i]);
  }

  // Rows
  for (uint8_t i = 0; i < KEYBOARD_MATRIX_ROW_COUNT; i++) {
    init_row(keyboard_matrix_rows[i]);
  }
}

void keyboard_matrix_scan_row_for_column(
    keyboard_matrix_coordinate_t index,
    bool scan_buf[KEYBOARD_MATRIX_KEY_COUNT]) {
  uint8_t row_index = index.row;
  uint8_t column_index = index.column;
  keyboard_gpio_t row = keyboard_matrix_rows[row_index];

  int16_t keymap_index = keymap_indices[row_index][column_index];
  if (keymap_index >= 0) {
    scan_buf[keymap_index] = keyboard_gpio_is_reset(row);
  }
}

void keyboard_matrix_scan_column(uint8_t column_index,
                                 bool scan_buf[KEYBOARD_MATRIX_KEY_COUNT]) {
  keyboard_gpio_t col = keyboard_matrix_cols[column_index];

  keyboard_gpio_reset(col);
  Delay_Us(5);

  for (uint8_t row_index = 0; row_index < KEYBOARD_MATRIX_ROW_COUNT;
       row_index++) {
    keyboard_matrix_coordinate_t index = {.column = column_index,
                                          .row = row_index};
    keyboard_matrix_scan_row_for_column(index, scan_buf);
  }

  keyboard_gpio_set(col);
}

void keyboard_matrix_scan_raw(bool scan_buf[KEYBOARD_MATRIX_KEY_COUNT]) {
  for (uint8_t col_index = 0; col_index < KEYBOARD_MATRIX_COL_COUNT;
       col_index++) {
    keyboard_matrix_scan_column(col_index, scan_buf);
  }
}

bool keyboard_matrix_is_sw_1_1_pressed() {
  keyboard_gpio_t col = keyboard_matrix_cols[0];
  keyboard_gpio_t row = keyboard_matrix_rows[0];
  keyboard_gpio_reset(col);
  Delay_Us(5);
  return keyboard_gpio_is_reset(row);
}
#endif
