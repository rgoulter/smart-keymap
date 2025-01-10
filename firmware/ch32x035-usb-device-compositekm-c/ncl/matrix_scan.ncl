{
  gpio_pins
    : { _ : { port : String, pin : Number } }
    | doc "Record with A1, A2, ... A23, B1, B2, ..., C24, mapped to { port, pin } records."
    =
      let ports = ["A", "B", "C"] in
      let pins = std.array.range 0 24 in

      std.array.flat_map
        (fun port_name => std.array.map (fun pin_num => { "%{port_name}%{std.to_string pin_num}" = { "port" = port_name, "pin" = pin_num } }) pins)
        ports
      |> (std.array.fold_left (&) {}) | { _ : { port : String, pin : Number } },

  board
    | {
      cols | Array { port | String, pin | Number },
      rows | Array { port | String, pin | Number },
      num_keys | Number,
    }
    | doc "the cols/rows, and number of keys, used for generating the matrix scan code.",

  keymap_index_for_key
    | doc "Returns the keymap index for the key corresponding to the (0-based) digital column_index and row_index.",

  init_col_fragment
    | doc "Generates C fragment for initializing the given column pin, for keyboard_matrix_init."
    = fun { port, pin, .. } =>
      m%"
      // %{port}%{std.to_string pin}
      {
        GPIO_InitTypeDef GPIO_InitStructure = { 0 };
        GPIO_InitStructure.GPIO_Pin = GPIO_Pin_%{std.to_string pin};
        GPIO_InitStructure.GPIO_Mode = GPIO_Mode_Out_PP;
        GPIO_InitStructure.GPIO_Speed = GPIO_Speed_50MHz;
        GPIO_Init( GPIO%{port}, &GPIO_InitStructure );
      }
  "%,

  init_row_fragment
    | doc "Generates C fragment for initializing the given row pin, for keyboard_matrix_init"
    = fun { port, pin, .. } =>
      m%"
      // %{port}%{std.to_string pin}
      {
        GPIO_InitTypeDef GPIO_InitStructure = { 0 };
        GPIO_InitStructure.GPIO_Pin = GPIO_Pin_%{std.to_string pin};
        GPIO_InitStructure.GPIO_Mode = GPIO_Mode_IPU;
        GPIO_InitStructure.GPIO_Speed = GPIO_Speed_50MHz;
        GPIO_Init( GPIO%{port}, &GPIO_InitStructure );
      }
  "%,

  keyboard_matrix_init
    | doc "Generates C fragment with the keyboard_matrix_init function, for the given cols/rows."
    = fun { cols, rows, .. } =>
      m%"
  void keyboard_matrix_init(void) {
      // NOTE: this implementation assumes diode's cathodes(-) face COLUMNS.
      //  i.e. that current flows from ROWS to COLUMNS.
      // This implementation configures ROWS as Input (Pull Up),
      // and COLS as Output (set low when scanned).

      RCC_APB2PeriphClockCmd( RCC_APB2Periph_GPIOA, ENABLE );
      RCC_APB2PeriphClockCmd( RCC_APB2Periph_GPIOB, ENABLE );
      RCC_APB2PeriphClockCmd( RCC_APB2Periph_GPIOC, ENABLE );

      // Cols
      %{cols |> std.array.map init_col_fragment |> std.string.join "\n"}

      // Rows
      %{rows |> std.array.map init_row_fragment |>std.string.join "\n"}
  }
  "%,

  matrix_scan_row_for_column
    | doc "Generates C fragment for reading a row, as part of COL2ROW scanning."
    = fun args @ { column_index, row_index, col, row, .. } =>
      let maybe_idx = keymap_index_for_key args in
      maybe_idx |> match {
        'Error err => "",
        'Ok idx_num =>
          let idx = std.to_string idx_num in
          m%"
          // Read row %{std.to_string row_index} = %{row.port}%{std.to_string row.pin}
          // SW_r_c = SW_%{std.to_string (1 + row_index)}_%{std.to_string (1 + column_index)}
          scan_buf[%{idx}] = GPIO_ReadInputDataBit(GPIO%{row.port}, GPIO_Pin_%{std.to_string row.pin}) != 1;
  "%},

  matrix_scan_column
    | doc "Generates C fragment for reading a column, as part of COL2ROW scanning."
    = fun args @ { column_index, col, rows, .. } =>
      m%"
      // Scan column %{std.to_string column_index} = %{col.port}%{std.to_string col.pin}
      GPIO_ResetBits(GPIO%{col.port}, GPIO_Pin_%{std.to_string col.pin});
      Delay_Us(5);

      // Read the row pins
      %{rows |> std.array.map_with_index (fun idx r => matrix_scan_row_for_column (args & { row = r, row_index = idx })) |> std.string.join "\n"}

      GPIO_SetBits(GPIO%{col.port}, GPIO_Pin_%{std.to_string col.pin});
  "%,

  keyboard_matrix_scan_raw
    | doc "Generates C fragment with the keyboard_matrix_scan function, for the given cols/rows."
    = fun { cols, rows, .. } =>
      m%"
  void keyboard_matrix_scan_raw(bool scan_buf[%{std.to_string board.num_keys}]) {
      %{board.cols |> std.array.map_with_index (fun idx c => matrix_scan_column ({ col = c, column_index = idx, rows = board.rows })) |> std.string.join "\n"}
  }
  "%,

  keyboard_matrix_scan
    | doc "Generates C fragment with the keyboard_matrix_scan function, for the given cols/rows."
    = fun args@{ cols, rows, .. } =>
      m%"
  %{keyboard_matrix_scan_raw args}

  void keyboard_matrix_scan(void) {
      static bool debounced_state[%{std.to_string board.num_keys}] = { false };
      static bool previous_raw_scan[%{std.to_string board.num_keys}] = { false };
      static bool current_raw_scan[%{std.to_string board.num_keys}] = { false };
      static uint8_t debounce_counter[%{std.to_string board.num_keys}] = { 0 };

      keyboard_matrix_scan_raw(current_raw_scan);

      for (uint32_t i = 0; i < %{std.to_string board.num_keys}; i++) {
          if (current_raw_scan[i] == debounced_state[i]) {
              debounce_counter[i] = 0;
          } else {
              if (current_raw_scan[i] != previous_raw_scan[i]) {
                  debounce_counter[i] = 0;
              } else {
                  debounce_counter[i]++;
              }

              if (debounce_counter[i] >= 5) {
                  key_state_changed(i, current_raw_scan[i]);
                  debounced_state[i] = current_raw_scan[i];
              }
          }

          previous_raw_scan[i] = current_raw_scan[i];
      }
  }
  "%,

  output
    | doc "C code with keyboard_matrix_init and keyboard_matrix_scan functions."
    = m%"
  #include <stdbool.h>

  #include "ch32x035.h"

  #include "smart_keymap.h"

  %{keyboard_matrix_init board}

  void key_state_changed(uint32_t index, bool new_state) {
      if (new_state) {
          keymap_register_input_keypress(index);
      } else {
          keymap_register_input_keyrelease(index);
      }
  }

  %{keyboard_matrix_scan board}

  "%
}
