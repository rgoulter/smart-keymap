#ifdef KEYBOARD_MATRIX_IMPL_TOUCHKEY_MUTUAL
#include "debug.h"

#include "ch32x035_rcc.h"

#include "keyboard_gpio.h"
#include "keyboard_matrix.h"
#include "keyboard_touchkey.h"

uint16_t tkey_baselines[KEYBOARD_MATRIX_KEY_COUNT] = {0};
uint16_t tkey_readings[KEYBOARD_MATRIX_KEY_COUNT] = {0};

// KLUDGE: hardcoded. Ideally, comes from codegen.
// for board: ch32x-tc-2x4 rev2025.1
uint8_t adc_channel_map[4] = {
    // per col
    4, // col 1
    6, // col 2
    8, // col 3
    9, // col 4
};

void init_column(keyboard_gpio_t col) {
  keyboard_touchkey_configure_sense(col);
}

void init_row(keyboard_gpio_t row) { keyboard_touchkey_configure_drive(row); }

void keyboard_matrix_scan_raw(bool scan_buf[KEYBOARD_MATRIX_KEY_COUNT]);

void keyboard_matrix_init(void) {
  // NOTE: this implementation is for 'touchkeys' using mutual capacitance,
  //  driving on rows, sensing on columns.

  RCC_APB2PeriphClockCmd(RCC_APB2Periph_GPIOA, ENABLE);
  RCC_APB2PeriphClockCmd(RCC_APB2Periph_GPIOB, ENABLE);
  RCC_APB2PeriphClockCmd(RCC_APB2Periph_GPIOC, ENABLE);

  // Sense Cols
  for (uint8_t i = 0; i < KEYBOARD_MATRIX_COL_COUNT; i++) {
    init_column(keyboard_matrix_cols[i]);
  }

  // Drive Rows
  for (uint8_t i = 0; i < KEYBOARD_MATRIX_ROW_COUNT; i++) {
    init_row(keyboard_matrix_rows[i]);
    keyboard_gpio_reset(keyboard_matrix_rows[i]);
  }

  keyboard_touchkey_init();

  // Scan once to set ADC values
  bool current_raw_scan[KEYBOARD_MATRIX_KEY_COUNT] = {false};
  keyboard_matrix_scan_raw(current_raw_scan);
}

void discharge_pin(keyboard_gpio_t col) {
  // Ensure col is discharged
  keyboard_gpio_configure_output(col);
  keyboard_gpio_reset(col);
  Delay_Us(5);
  keyboard_touchkey_configure_sense(col);
}

void keyboard_matrix_scan_column_for_row(
    keyboard_matrix_coordinate_t index,
    bool scan_buf[KEYBOARD_MATRIX_KEY_COUNT]) {
  uint8_t column_index = index.column;
  uint8_t row_index = index.row;
  keyboard_gpio_t column = keyboard_matrix_cols[column_index];

  discharge_pin(column);

  uint16_t adc_value = keyboard_touchkey_read(adc_channel_map[column_index]);

  uint8_t keymap_index = keymap_indices[row_index][column_index];

  if (keymap_index >= 0) {
    tkey_readings[keymap_index] = adc_value;

    // Initialize baseline if not set
    if (tkey_baselines[keymap_index] == 0) {
      tkey_baselines[keymap_index] = adc_value;
    }

    uint16_t threshold = 150; // TODO: make configurable
    if (adc_value < tkey_baselines[keymap_index] - threshold) {
      scan_buf[keymap_index] = true;
    } else {
      scan_buf[keymap_index] = false;

      // Update baseline
      uint8_t n = 64;
      tkey_baselines[keymap_index] =
          (tkey_baselines[keymap_index] * (n - 1) / n) + (adc_value * 1 / n);
    }
  }
}

void keyboard_matrix_scan_row(uint8_t row_index,
                              bool scan_buf[KEYBOARD_MATRIX_KEY_COUNT]) {
  keyboard_gpio_t row = keyboard_matrix_rows[row_index];

  keyboard_gpio_set(row);
  Delay_Us(5);

  for (uint8_t column_index = 0; column_index < KEYBOARD_MATRIX_COL_COUNT;
       column_index++) {
    keyboard_matrix_coordinate_t index = {.column = column_index,
                                          .row = row_index};
    keyboard_matrix_scan_column_for_row(index, scan_buf);
  }

  keyboard_gpio_reset(row);
}

void keyboard_matrix_scan_raw(bool scan_buf[KEYBOARD_MATRIX_KEY_COUNT]) {
  static uint16_t counter = 0;

  if ((++counter) > 1000) {
    printf("base: \r\n %5d %5d\r\n %5d %5d\r\n", tkey_baselines[0],
           tkey_baselines[1], tkey_baselines[2], tkey_baselines[3]);
    printf("read: \r\n %5d %5d\r\n %5d %5d\r\n", tkey_readings[0],
           tkey_readings[1], tkey_readings[2], tkey_readings[3]);
    printf("delt: \r\n %5d %5d\r\n %5d %5d\r\n",
           ((int32_t)(tkey_readings[0]) - (int32_t)(tkey_baselines[0])),
           ((int32_t)(tkey_readings[1]) - (int32_t)(tkey_baselines[1])),
           ((int32_t)(tkey_readings[2]) - (int32_t)(tkey_baselines[2])),
           ((int32_t)(tkey_readings[3]) - (int32_t)(tkey_baselines[3])));
    printf("\r\n");
    counter = 0;
  }

  for (uint8_t row_index = 0; row_index < KEYBOARD_MATRIX_ROW_COUNT;
       row_index++) {
    keyboard_matrix_scan_row(row_index, scan_buf);
  }
}

bool keyboard_matrix_is_sw_1_1_pressed() {
  return false; // TBI
}
#endif
