{
  contracts = {
    UsartNumber =
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
  },
}
