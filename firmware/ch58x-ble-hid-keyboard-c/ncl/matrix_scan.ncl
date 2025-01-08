let gpio_pins =
  let ports = ["A", "B"] in
  let pins = std.array.range 0 24 in

  std.array.flat_map
    (fun port_name => std.array.map (fun pin_num => { "%{port_name}%{std.to_string pin_num}" = { "port" = port_name, "pin" = pin_num } }) pins)
    ports
  |> std.array.fold_left (&) {}
in

let board =
  let gp = gpio_pins in
  {
    cols = [gp.A4, gp.A5, gp.A15, gp.A14, gp.A13, gp.A12, gp.A11, gp.A10],
    rows = [gp.A8, gp.A9, gp.B15, gp.B14, gp.B13, gp.B12, gp.B7, gp.B4],
    num_keys = 60,
  }
in

let init_col = fun { port, pin, .. } =>
  m%"
    GPIO%{port}_ModeCfg(GPIO_Pin_%{std.to_string pin}, GPIO_ModeOut_PP_5mA); // %{port}%{std.to_string pin}
"%
in

let init_row = fun { port, pin, .. } =>
  m%"
    GPIO%{port}_ModeCfg(GPIO_Pin_%{std.to_string pin}, GPIO_ModeIN_PD); // %{port}%{std.to_string pin}
"%
in

let matrix_init = fun { cols, rows, .. } =>
  m%"
void keyboard_matrix_init(void) {
    // Cols
    %{cols |> std.array.map init_col |> std.string.join "\n"}

    // Rows
    %{rows |> std.array.map init_row |>std.string.join "\n"}
}
"%
in

# The WABBLE-60 uses a digital matrix of 8x8 rows and columns,
#  forming a physical 5x12 matrix of keys.
#
# The physical matrix arranges keys column-wise.
#
# Want the keymap index to refer to keys row-wise.
#
# The given column_index and row_index refer to (digital) row/col, 0..7.
let keymap_index_for_key = fun { column_index, row_index, .. } =>
  let columnwise_index = column_index * 8 + row_index in
  let physical_column_index = std.number.floor (columnwise_index / 5) in
  let physical_row_index = columnwise_index % 5 in
  let rowwise_index = physical_row_index * 12 + physical_column_index in
  rowwise_index
in

let matrix_scan_row_for_column = fun args @ { column_index, row_index, col, row, .. } =>
  let idx = std.to_string (keymap_index_for_key args) in
  m%"
    // Read row %{std.to_string row_index} = %{row.port}%{std.to_string row.pin}
    currentScan[%{idx}] = GPIO%{row.port}_ReadPortPin(GPIO_Pin_%{std.to_string row.pin}) != 0;

    // Register presses/events based on changes
    handle_index(%{idx});
"%
in

let matrix_scan_column = fun args @ { column_index, col, rows, .. } =>
  m%"
    // Scan column %{std.to_string column_index} = %{col.port}%{std.to_string col.pin}
    GPIO%{col.port}_SetBits(GPIO_Pin_%{std.to_string col.pin});
    mDelayuS(5);

    // Read the row pins
    %{rows |> std.array.map_with_index (fun idx r => matrix_scan_row_for_column (args & { row = r, row_index = idx })) |> std.string.join "\n"}

    GPIO%{col.port}_ResetBits(GPIO_Pin_%{std.to_string col.pin});
"%
in

m%"
#include <stdbool.h>

#include "CH58x_common.h"

bool previousScan[%{std.to_string board.num_keys}] = { false };
bool currentScan[%{std.to_string board.num_keys}] = { false };

%{matrix_init board}

void handle_index(uint32_t index) {
    if (previousScan[index] != currentScan[index]) {
        if (currentScan[index]) {
            keymap_register_input_keypress(index);
        } else {
            keymap_register_input_keyrelease(index);
        }

        previousScan[index] = currentScan[index];
    }
}

void keyboard_matrix_scan(void) {
    %{board.cols |> std.array.map_with_index (fun idx c => matrix_scan_column ({ col = c, column_index = idx, rows = board.rows })) |> std.string.join "\n"}
}

"%
