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
        0 => 'Ok,
        1 => 'Ok,
        2 => 'Ok,
        3 => 'Ok,
        _ => 'Error "invalid USART alternate function remapping",
      }
      ),
  },
}
