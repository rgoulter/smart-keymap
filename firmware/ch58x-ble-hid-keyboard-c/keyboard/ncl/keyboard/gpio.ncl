let C = import "keyboard/contracts.ncl" in

{
  contracts = {
    GpioPin = { port | String, pin | Number },

    GpioPins = { _ | GpioPin },
  },

  gpio_pins
    : { _ : { port : String, pin : Number } }
    | C.GpioPins
    | doc "Record with A1, A2, ... A23, B1, B2, ..., B24, mapped to { port, pin } records."
    =
      let ports = ["A", "B"] in
      let pins = std.array.range 0 24 in

      std.array.flat_map
        (fun port_name => std.array.map (fun pin_num => { "%{port_name}%{std.to_string pin_num}" = { "port" = port_name, "pin" = pin_num } }) pins)
        ports
      |> (std.array.fold_left (&) {}) | { _ : { port : String, pin : Number } },
}
