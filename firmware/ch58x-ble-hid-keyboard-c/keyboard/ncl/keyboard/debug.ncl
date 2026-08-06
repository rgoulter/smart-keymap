let C = import "contracts.ncl" in

{
  contracts = {
    Board = {
      # Optional; tx is Debug_UART0..3 (CH58x_common.h).
      debug
        | optional
        | {
          tx | Number,
          ..
        },

      ..
    },
  },

  board
    | C.Board,

  cmakelists =
    if std.record.has_field "debug" board then
      let tx = board.debug.tx in
      {
        debug =
          [
            "target_compile_definitions(keyboard INTERFACE DEBUG=%{std.to_string tx})",
            # Sleep would drop UART after the first PRINT.
            "target_compile_definitions(keyboard INTERFACE HAL_SLEEP=FALSE)",
          ]
          |> std.string.join "\n",
      }
    else
      {},
}
