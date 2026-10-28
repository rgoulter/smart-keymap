{
  key_extensions.history = {
    history = {
      # Records (not bare strings) so the keys can merge with
      #  `{ layered = … }` / `{ named_layers = … }` as a layered base.
      repeat = { history = "Repeat" },
      alt_repeat = { history = "AltRepeat" },
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
