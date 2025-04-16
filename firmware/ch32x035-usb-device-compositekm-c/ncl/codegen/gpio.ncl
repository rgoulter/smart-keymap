let C = import "contracts.ncl" in

{
  contracts = {
    GpioPin = { port | String, pin | Number },

    GpioPins = { _ | GpioPin },
  },

  gpio_pins
    | doc "Record with A1, A2, ... A23, B1, B2, ..., C24, mapped to { port, pin } records."
    | C.GpioPins
    : { _ : { port : String, pin : Number } }
    =
      let ports = ["A", "B", "C"] in
      let pins = std.array.range 0 24 in

      std.array.flat_map
        (fun port_name => std.array.map (fun pin_num => { "%{port_name}%{std.to_string pin_num}" = { "port" = port_name, "pin" = pin_num } }) pins)
        ports
      |> (std.array.fold_left (&) {}) | { _ : { port : String, pin : Number } },
}
