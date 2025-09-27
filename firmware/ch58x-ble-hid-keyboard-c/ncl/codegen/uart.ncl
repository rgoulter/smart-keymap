{
  contracts = {
    UartNumber =
      std.contract.from_validator (match {
        0 or 1 or 2 or 3 => 'Ok,
        _ => 'Error "invalid UART number",
      }
      ),
  },
}
