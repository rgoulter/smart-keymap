let C = import "contracts.ncl" in
{
  contracts = {
    Board = {
      usb
        | {
          vid | Number,
          pid | Number,
          manufacturer | String,
          product | String,
        },

      matrix
        | {
          cols | Array C.GpioPin,
          rows | Array C.GpioPin,
          key_count | Number,
        }
        | doc "the cols/rows, and number of keys, used for generating the matrix scan code.",

      keymap_index_for_key
        | doc "Returns the keymap index for the key corresponding to the (0-based) digital column_index and row_index.",
    },
  },

  gpio_pins | { _ | C.GpioPin },

  board | C.Board,

  board_rs =
    let col_count = board.matrix.cols |> std.array.length in
    let const_cols_expr = col_count |> std.to_string in
    let row_count = board.matrix.rows |> std.array.length in
    let const_rows_expr = row_count |> std.to_string in
    let keymap_indices_expr =
      let keymap_index_expr = match {
        'Key idx => "Some(%{std.to_string idx})",
        'NoKey => "None",
      }
      in
      let cols_expr = fun row_idx =>
        std.array.generate
          (fun col_idx =>
            board.keymap_index_for_key {
              row_index = row_idx,
              column_index = col_idx,
            }
            |> keymap_index_expr
          )
          col_count
      in
      let row_expr = fun row_idx =>
        "[ %{std.string.join ", " (cols_expr row_idx)} ]"
      in
      let row_exprs = std.array.generate row_expr row_count in
      "[ %{std.string.join ", " row_exprs} ]"
    in
    let macro_cols_expr =
      let macro_col_expr = fun col_idx =>
        let { id, .. } = std.array.at col_idx board.matrix.cols in
        "$gpio_pins.%{id}.into_pull_up_input().into_dyn_pin()"
      in
      let macro_col_exprs = std.array.generate macro_col_expr col_count in
      "[%{std.string.join "," macro_col_exprs}]"
    in
    let macro_rows_expr =
      let macro_row_expr = fun row_idx =>
        let { id, .. } = std.array.at row_idx board.matrix.rows in
        "$gpio_pins.%{id}.into_push_pull_output().into_dyn_pin()"
      in
      let macro_row_exprs = std.array.generate macro_row_expr row_count in
      "[%{std.string.join "," macro_row_exprs}]"
    in

    let vid_expr = board.usb.vid |> std.to_string in
    let pid_expr = board.usb.pid |> std.to_string in
    let manufacturer_expr = "\"%{board.usb.manufacturer}\"" in
    let product_expr = "\"%{board.usb.product}\"" in
    m%"
mod board {
    use rp2040_hal as hal;

    use keyberon_smart_keyboard::matrix::Matrix;

    use rp2040_rtic_smart_keyboard::input::{Input, Output};

    pub const COLS: usize = %{const_cols_expr};

    pub const ROWS: usize = %{const_rows_expr};

    pub const KEYMAP_INDICES: [[Option<u16>; COLS]; ROWS] = %{keymap_indices_expr};

    pub const VID: u16 = %{vid_expr};
    pub const PID: u16 = %{pid_expr};
    pub const MANUFACTURER: &str = %{manufacturer_expr};
    pub const PRODUCT: &str = %{product_expr};

    pub type Keyboard = keyberon_smart_keyboard::input::Keyboard<
        COLS,
        ROWS,
        Matrix<Input, Output, COLS, ROWS, hal::Timer>,
    >;

    pub type PressedKeys = keyberon_smart_keyboard::input::PressedKeys<COLS, ROWS>;

    macro_rules! rows_and_cols {
        ($gpio_pins:expr, $cols:ident, $rows:ident) => {
            let $cols = %{macro_cols_expr};
            let $rows = %{macro_rows_expr};
        };
    }

    pub(crate) use rows_and_cols;
}
"%,
}
