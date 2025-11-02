let C = import "contracts.ncl" in

{
  contracts = {
    Board = {
      disable_swd
        | doc "Whether to disable SWD."
        | Bool
        | default
        = false,

      ..
    },
  },

  matrix_scan = {
    is_sw_1_1_pressed,
  },

  board
    | C.Board,

  cmakelists.keyboard =
    let disable_swd_def =
      if board.disable_swd then
        ["target_compile_definitions(keyboard INTERFACE KEYBOARD_DISABLE_SWD)"]
      else
        []
    in
    disable_swd_def |> std.string.join "\n",
}
