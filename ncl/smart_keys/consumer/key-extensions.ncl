{
  key_extensions = {
    consumer =
      (import "hid-usage-consumer.ncl")
      |> std.record.map_values (fun uc => { consumer_code = uc }),

    consumer_aliases = fun keys =>
      {
        VolUp = keys.VolumeUp,
        VolDown = keys.VolumeDown,
      },
  },
}
