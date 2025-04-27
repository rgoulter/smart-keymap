let C = import "contracts.ncl" in

{
  contracts = {
    UsartTxNumber =
      std.contract.from_validator (match {
        1 or 2 or 3 or 4 => 'Ok,
        _ => 'Error "invalid UART TX pin number",
      }
      ),

    UsartAlternateFunction =
      std.contract.from_validator (match {
        1 => 'Ok,
        0 or 2 or 3 => 'Error "USART alternate function remapping not implemented",
        _ => 'Error "invalid USART alternate function remapping",
      }
      ),

    Board = {
      debug = {
        tx | default | UsartTxNumber = 1,
        usart_af | optional | UsartAlternateFunction,
      },

      ..
    },
  },

  board
    | C.Board,

  cmakelists.debug =
    let debug_config = board.debug in
    let debug_def = "target_compile_definitions(keyboard_codegen INTERFACE DEBUG=%{std.to_string debug_config.tx})" in
    let debug_af_def =
      if std.record.has_field "usart_af" debug_config then
        m%"target_compile_definitions(keyboard_codegen INTERFACE DEBUG_AF=%{std.to_string debug_config.usart_af})"%
      else
        ""
    in
    [debug_def, debug_af_def] |> std.array.filter ((!=) "") |> std.string.join "\n",
}
