{
  key_extensions.sequence = {
    sequence = {
      # Record (not a bare string) so the key can merge with
      #  `{ layered = … }` / `{ named_layers = … }` as a layered base.
      start = { sequence = "SequenceStart" },
    },
    # Short alias matching caps_word / history style.
    sequence_start = { sequence = "SequenceStart" },
  },
}
