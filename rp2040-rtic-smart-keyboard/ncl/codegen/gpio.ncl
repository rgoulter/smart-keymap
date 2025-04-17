let C = import "contracts.ncl" in
{
  contracts = {
    GpioPin = { id | String, type | String, },
  },

  gpio_pins
    : { _ | C.GpioPin }
    | doc "Record with GP0, GP1, .., GP29, mapped to { id, type } records."
    =
      let ports = ["GP"] in
      let pins = std.array.range 0 30 in

      std.array.flat_map
        (fun port_name => # GP
          std.array.map
            (fun pin_num => # 0, 1, .. 29
              {
                # GP0 = ...
                "%{port_name}%{std.to_string pin_num}" = {
                  "id" = "gpio%{std.to_string pin_num}",
                  "type" = "bank0::Gpio%{std.to_string pin_num}",
                },
              }
            )
            pins
        )
        ports
      |> (std.array.fold_left (&) {}) | { _ : C.GpioPin },
}
