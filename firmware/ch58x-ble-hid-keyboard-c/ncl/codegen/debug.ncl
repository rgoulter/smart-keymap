let C = import "contracts.ncl" in

{
  contracts = {
    Board = {
      debug = {
        tx | optional | C.UartNumber,
      },

      ..
    },
  },

  board
    | C.Board,

  cmakelists.debug =
    let debug_config = board.debug in
    let debug_def =
      if std.record.has_field "tx" debug_config then
        "target_compile_definitions(sdk INTERFACE DEBUG Debug_UART%{std.to_string debug_config.tx})"
      else
        ""
    in
    [debug_def] |> std.array.filter ((!=) "") |> std.string.join "\n",
}
