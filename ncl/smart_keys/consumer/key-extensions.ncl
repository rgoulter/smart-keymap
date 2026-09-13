{
  key_extensions = {
    consumer =
      (import "hid-usage-consumer.ncl")
      |> std.record.map_values (fun uc => { consumer_code = uc }),

    consumer_aliases = fun keys =>
      {
        VolUp = keys.VolumeUp,
        VolDown = keys.VolumeDown,

        # Four-letter aliases for layer-string alignment
        MUTE = keys.Mute,
        VOLU = keys.VolumeUp,
        VOLD = keys.VolumeDown,
        MPLY = keys.PlayPause,
      },
  },
}
