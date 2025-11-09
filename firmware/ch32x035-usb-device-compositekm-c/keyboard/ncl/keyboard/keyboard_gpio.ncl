let C = import "contracts.ncl" in

{
  contracts = {
    GpioPin = {
      port | String,
      pin | Number,
      as_keyboard_gpio_expr =
        let port_source = port |> match {
          "A" => 0,
          "B" => 1,
          "C" => 2,
          invalid_port => std.fail_with "invalid port %{invalid_port}"
        } in
        "{ .port = %{port_source |> std.to_string}, .pin = %{pin |> std.to_string} }",
    },
  },
}
