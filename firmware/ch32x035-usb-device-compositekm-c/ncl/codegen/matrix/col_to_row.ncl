let C = import "codegen/contracts.ncl" in

# codegen for matrix scan,
# diode direction: current from column to row
# impl.: pull-up cols
{
  board | C.Board
    = {
      matrix,

      keymap_index_for_key,
    },

  matrix_scan =
    if board.matrix.implementation == "col_to_row" then
      {
        init_col_fragment = fun { port, pin, .. } =>
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

        init_row_fragment = fun { port, pin, .. } =>
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

        keyboard_matrix_init
          | doc "Generates C fragment with the keyboard_matrix_init function, for the given cols/rows."
          = fun { cols, rows, .. } =>
            m%"
      void keyboard_matrix_init(void) {
          // NOTE: this implementation is for diode's cathodes(-) which face ROWS.
          //  i.e. that current flows from COLUMNS to ROWS.
          // This implementation configures COLS as Input (Pull Up),
          // and ROWS as Output (set low when scanned).

          RCC_APB2PeriphClockCmd( RCC_APB2Periph_GPIOA, ENABLE );
          RCC_APB2PeriphClockCmd( RCC_APB2Periph_GPIOB, ENABLE );
          RCC_APB2PeriphClockCmd( RCC_APB2Periph_GPIOC, ENABLE );

          // Rows
          %{rows |> std.array.map init_row_fragment |>std.string.join "\n"}

          // Cols
          %{cols |> std.array.map init_col_fragment |> std.string.join "\n"}
      }
      "%,

        matrix_scan_column_for_row = fun args @ { column_index, row_index, col, row, .. } =>
          let maybe_idx = board.keymap_index_for_key args in
          maybe_idx
          |> match {
            'Error err => "",
            'Ok idx_num =>
              let idx = std.to_string idx_num in
              m%"
              // Read col %{std.to_string column_index} = %{col.port}%{std.to_string col.pin}
              // SW_r_c = SW_%{std.to_string (1 + row_index)}_%{std.to_string (1 + column_index)}
              scan_buf[%{idx}] = GPIO_ReadInputDataBit(GPIO%{col.port}, GPIO_Pin_%{std.to_string col.pin}) != 1;
      "%
          },

        matrix_scan_row = fun args @ { row_index, row, cols, .. } =>
          m%"
          // Scan row %{std.to_string row_index} = %{row.port}%{std.to_string row.pin}
          GPIO_ResetBits(GPIO%{row.port}, GPIO_Pin_%{std.to_string row.pin});
          Delay_Us(5);

          // Read the col pins
          %{cols |> std.array.map_with_index (fun idx c => matrix_scan_column_for_row (args & { col = c, column_index = idx })) |> std.string.join "\n"}

          GPIO_SetBits(GPIO%{row.port}, GPIO_Pin_%{std.to_string row.pin});
      "%,

        keyboard_matrix_scan_raw
          | doc "Generates C fragment with the keyboard_matrix_scan function, for the given cols/rows."
          = fun { cols, rows, .. } =>
            m%"
      void keyboard_matrix_scan_raw(bool scan_buf[%{std.to_string board.matrix.key_count}]) {
          %{board.matrix.rows |> std.array.map_with_index (fun idx r => matrix_scan_row ({ row = r, row_index = idx, cols = board.matrix.cols })) |> std.string.join "\n"}
      }
      "%,

        is_sw_1_1_pressed =
          let row = std.array.at 0 board.matrix.rows in
          let col = std.array.at 0 board.matrix.cols in
          m%"
        GPIO_ResetBits(GPIO%{row.port}, GPIO_Pin_%{std.to_string row.pin}); // row 0
        Delay_Us(5);
        bool sw_1_1_is_pressed = GPIO_ReadInputDataBit(GPIO%{col.port}, GPIO_Pin_%{std.to_string col.pin}) != 1; // col 0
        "%,
      }
    else
      {},
}
