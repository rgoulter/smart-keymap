#ifdef KEYBOARD_MATRIX_IMPL_COL_TO_ROW
#include "CH58x_common.h"

#include "keyboard_gpio.h"
#include "keyboard_matrix.h"

void init_column(keyboard_gpio_t col) { keyboard_gpio_configure_output(col); }

void init_row(keyboard_gpio_t row) { keyboard_gpio_configure_ipd(row); }

void keyboard_matrix_configure_for_scanning(void) {
  // NOTE: this implementation is for diode's cathodes(-) which face ROWS.
  //  i.e. that current flows from COLUMNS to ROWS.
  // This implementation configures ROWS as Input (Pull Down),
  // and COLS as Output (set high when scanned).

  // Rows
  for (uint8_t i = 0; i < KEYBOARD_MATRIX_ROW_COUNT; i++) {
    init_row(keyboard_matrix_rows[i]);
  }

  // Cols
  for (uint8_t i = 0; i < KEYBOARD_MATRIX_COL_COUNT; i++) {
    init_column(keyboard_matrix_cols[i]);
    keyboard_gpio_reset(keyboard_matrix_cols[i]);
  }
}

void keyboard_matrix_configure_for_interrupt(void) {
  // Configures ROWS as Input (Pull down) and COLS as Output (High)
  // with interrupts enabled on rising edge.

  // Rows
  for (uint8_t i = 0; i < KEYBOARD_MATRIX_ROW_COUNT; i++) {
    keyboard_gpio_t row = keyboard_matrix_rows[i];
    keyboard_gpio_configure_ipd(row);
    keyboard_gpio_configure_irq_mode(row, GPIO_ITMode_RiseEdge);
  }

  // Cols
  for (uint8_t i = 0; i < KEYBOARD_MATRIX_COL_COUNT; i++) {
    keyboard_gpio_t col = keyboard_matrix_cols[i];
    keyboard_gpio_configure_output(col);
    keyboard_gpio_set(col);
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
    scan_buf[keymap_index] = keyboard_gpio_is_set(row);
  }
}

void keyboard_matrix_scan_column(uint8_t column_index,
                                 bool scan_buf[KEYBOARD_MATRIX_KEY_COUNT]) {
  keyboard_gpio_t col = keyboard_matrix_cols[column_index];

  keyboard_gpio_set(col);
  DelayUs(5);

  for (uint8_t row_index = 0; row_index < KEYBOARD_MATRIX_COL_COUNT;
       row_index++) {
    keyboard_matrix_coordinate_t index = {.row = row_index,
                                          .column = column_index};
    keyboard_matrix_scan_row_for_column(index, scan_buf);
  }

  keyboard_gpio_reset(col);
}

void keyboard_matrix_scan_raw(bool scan_buf[KEYBOARD_MATRIX_KEY_COUNT]) {
  for (uint8_t col_index = 0; col_index < KEYBOARD_MATRIX_ROW_COUNT;
       col_index++) {
    keyboard_matrix_scan_column(col_index, scan_buf);
  }
}
#endif
