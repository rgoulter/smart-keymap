let C = import "contracts.ncl" in

{
  contracts = {
    Board = {
      debug = {
        tx | default | C.UsartNumber = 1,
        usart_af | optional | C.UsartAlternateFunction,
      },

      ..
    },
  },

  board
    | C.Board,

  cmakelists.debug =
    let debug_config = board.debug in
    let debug_def = "target_compile_definitions(keyboard INTERFACE DEBUG=%{std.to_string debug_config.tx})" in
    let debug_af_def =
      if std.record.has_field "usart_af" debug_config then
        m%"target_compile_definitions(keyboard INTERFACE DEBUG_AF=%{std.to_string debug_config.usart_af})"%
      else
        ""
    in
    [debug_def, debug_af_def] |> std.array.filter ((!=) "") |> std.string.join "\n",
}
