let C = import "codegen/contracts.ncl" in

# codegen for matrix scan,
# diode direction: current from column to row
# impl.: pull-down rows
{
  board | C.Board = {
    matrix,
    keymap_index_for_key,
  },

  matrix_scan =
    if board.matrix.implementation == "col_to_row" then
      {
        init_col_fragment
            | doc "Generates C fragment for initializing the given column pin, for keyboard_matrix_init."
            = fun { port, pin, .. } =>
            m%"
            GPIO%{port}_ModeCfg(GPIO_Pin_%{std.to_string pin}, GPIO_ModeOut_PP_5mA); // %{port}%{std.to_string pin}
        "%,

        init_row_fragment
            | doc "Generates C fragment for initializing the given row pin, for keyboard_matrix_init"
            = fun { port, pin, .. } =>
            m%"
            GPIO%{port}_ModeCfg(GPIO_Pin_%{std.to_string pin}, GPIO_ModeIN_PD); // %{port}%{std.to_string pin}
        "%,

        keyboard_matrix_init
            | doc "Generates C fragment with the keyboard_matrix_init function, for the given cols/rows."
            = fun { cols, rows, .. } =>
            m%"
        void keyboard_matrix_init(void) {
            // NOTE: this implementation assumes diode's cathodes(-) face ROWS.
            //  i.e. that current flows from COLUMNS to ROWS.
            // This implementation configures ROWS as Input (Pull Down),
            // and COLS as Output (set high when scanned).

            // Cols
            %{cols |> std.array.map init_col_fragment |> std.string.join "\n"}

            // Rows
            %{rows |> std.array.map init_row_fragment |>std.string.join "\n"}
        }
        "%,

        matrix_scan_row_for_column
            | doc "Generates C fragment for reading a row, as part of COL2ROW scanning."
            = fun args @ { column_index, row_index, col, row, .. } =>
            let maybe_idx = board.keymap_index_for_key args in
            maybe_idx
            |> match {
                'Error err => "",
                'Ok idx_num =>
                let idx = std.to_string idx_num in
                m%"
                // Read row %{std.to_string row_index} = %{row.port}%{std.to_string row.pin}
                // SW_r_c = SW_%{std.to_string (1 + row_index)}_%{std.to_string (1 + column_index)}
                currentScan[%{idx}] = GPIO%{row.port}_ReadPortPin(GPIO_Pin_%{std.to_string row.pin}) != 0;
                // Register presses/events based on changes
                handle_index(%{idx});

        "%
            },

        matrix_scan_column
            | doc "Generates C fragment for reading a column, as part of COL2ROW scanning."
            = fun args @ { column_index, col, rows, .. } =>
            m%"
            // Scan column %{std.to_string column_index} = %{col.port}%{std.to_string col.pin}
            GPIO%{col.port}_SetBits(GPIO_Pin_%{std.to_string col.pin});
            mDelayuS(5);

            // Read the row pins
            %{rows |> std.array.map_with_index (fun idx r => matrix_scan_row_for_column (args & { row = r, row_index = idx })) |> std.string.join "\n"}

            GPIO%{col.port}_ResetBits(GPIO_Pin_%{std.to_string col.pin});
        "%,

        keyboard_matrix_scan
            | doc "Generates C fragment with the keyboard_matrix_scan function, for the given cols/rows."
            = fun { cols, rows, .. } =>
            m%"
        void keyboard_matrix_scan(void) {
            %{board.matrix.cols |> std.array.map_with_index (fun idx c => matrix_scan_column ({ col = c, column_index = idx, rows = board.matrix.rows })) |> std.string.join "\n"}
        }
        "%,
      }
    else
      {}
}
