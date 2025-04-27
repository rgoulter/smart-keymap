let C = import "codegen/contracts.ncl" in

{
  gpio_pins | C.GpioPins,

  board | C.Board = {
    matrix =
      let p = gpio_pins in
      {
        cols = [p.B1, p.A6, p.A9, p.A0, p.C3, p.A10, p.A11],
        rows = [p.A4, p.C19, p.A3, p.A2, p.A1, p.C18, p.A7],
        key_count = 48,
        implementation = "row_to_col",
      },

    # The CH32X-48 uses a digital matrix of 7x7 rows and columns,
    #  forming a physical 4x12 matrix of keys.
    #
    # The physical matrix arranges keys column-wise.
    #
    # Want the keymap index to refer to keys row-wise.
    #
    # The given column_index and row_index refer to (digital) row/col, 0..6.
    keymap_index_for_key = fun { column_index | Number, row_index | Number, .. } =>
      let columnwise_index = column_index * 7 + row_index in
      if columnwise_index >= matrix.key_count then
        'Error "no key at this column/row index."
      else
        let physical_column_index = std.number.floor (columnwise_index / 4) in
        let physical_row_index = columnwise_index % 4 in
        let rowwise_index = physical_row_index * 12 + physical_column_index in
        'Ok rowwise_index,

    # CH32X-48 uses CH32X033F8P6's PC18/PC19 (DIO/DCK) for matrix rows.
    # Hence, disable SWD to avoid conflict.
    disable_swd = true,

    led = {
      enabled = true,
      pin = gpio_pins.A5,
    },

    debug.tx = 2,
  },
}
