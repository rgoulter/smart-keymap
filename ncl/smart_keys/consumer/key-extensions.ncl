{
  key_extensions = {
    consumer =
      (import "hid-usage-consumer.ncl")
      |> std.record.map_values (fun uc => { consumer_code = uc }),

    aliases = {
      keys,
      VolUp = keys.VolumeUp,
      VolDown = keys.VolumeDown,
    },
  },
}
