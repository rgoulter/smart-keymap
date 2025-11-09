#ifdef KEYBOARD_MATRIX_IMPL_COL_TO_ROW
#include "debug.h"

#include "ch32x035_rcc.h"

#include "keyboard_gpio.h"
#include "keyboard_matrix.h"

void init_column(keyboard_gpio_t col) { keyboard_gpio_configure_ipu(col); }

void init_row(keyboard_gpio_t row) { keyboard_gpio_configure_output(row); }

void keyboard_matrix_init(void) {
  // NOTE: this implementation is for diode's cathodes(-) which face ROWS.
  //  i.e. that current flows from COLUMNS to ROWS.
  // This implementation configures COLS as Input (Pull Up),
  // and ROWS as Output (set low when scanned).

  RCC_APB2PeriphClockCmd(RCC_APB2Periph_GPIOA, ENABLE);
  RCC_APB2PeriphClockCmd(RCC_APB2Periph_GPIOB, ENABLE);
  RCC_APB2PeriphClockCmd(RCC_APB2Periph_GPIOC, ENABLE);

  // Rows
  for (uint8_t i = 0; i < KEYBOARD_MATRIX_ROW_COUNT; i++) {
    init_row(keyboard_matrix_rows[i]);
    keyboard_gpio_set(keyboard_matrix_rows[i]);
  }

  // Cols
  for (uint8_t i = 0; i < KEYBOARD_MATRIX_COL_COUNT; i++) {
    init_column(keyboard_matrix_cols[i]);
  }
}

void keyboard_matrix_scan_column_for_row(
    keyboard_matrix_coordinate_t index,
    bool scan_buf[KEYBOARD_MATRIX_KEY_COUNT]) {
  uint8_t row_index = index.row;
  uint8_t column_index = index.column;
  keyboard_gpio_t col = keyboard_matrix_cols[column_index];

  int16_t keymap_index = keymap_indices[row_index][column_index];
  if (keymap_index >= 0) {
    scan_buf[keymap_index] = keyboard_gpio_is_reset(col);
  }
}

void keyboard_matrix_scan_row(uint8_t row_index,
                              bool scan_buf[KEYBOARD_MATRIX_KEY_COUNT]) {
  keyboard_gpio_t row = keyboard_matrix_rows[row_index];

  keyboard_gpio_reset(row);
  Delay_Us(5);

  for (uint8_t column_index = 0; column_index < KEYBOARD_MATRIX_COL_COUNT;
       column_index++) {
    keyboard_matrix_coordinate_t index = {.row = row_index,
                                          .column = column_index};
    keyboard_matrix_scan_column_for_row(index, scan_buf);
  }

  keyboard_gpio_set(row);
}

void keyboard_matrix_scan_raw(bool scan_buf[KEYBOARD_MATRIX_KEY_COUNT]) {
  for (uint8_t row_index = 0; row_index < KEYBOARD_MATRIX_ROW_COUNT;
       row_index++) {
    keyboard_matrix_scan_row(row_index, scan_buf);
  }
}

bool keyboard_matrix_is_sw_1_1_pressed() {
  keyboard_gpio_t col = keyboard_matrix_cols[0];
  keyboard_gpio_t row = keyboard_matrix_rows[0];
  keyboard_gpio_reset(row);
  Delay_Us(5);
  return keyboard_gpio_is_reset(col);
}
#endif
