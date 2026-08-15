{
  key_extensions.history = {
    history = {
      repeat = "Repeat",
      alt_repeat = "AltRepeat",
      # Per-site last-1 morph: { default, rules? }.
      # rules items are { prev, emit } keyboard keys (same as alt_repeat).
      adaptive = fun spec =>
        {
          default = spec."default",
          rules =
            if std.record.has_field "rules" spec then
              spec.rules
            else
              [],
        },
    },
  },
}
