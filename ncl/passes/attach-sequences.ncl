# Pass: attach global `sequences` as sequence / passthrough on key slots
#
# Inputs (from driver merge): sequences, chorded_keys (or layered_keys), keymap_ncl, validators
# Outputs: sequenced_keys, SequencedKeyElement
{
  sequences,
  chorded_keys,
  keymap_ncl,
  validators,

  sequenced_key_element_validator = fun k =>
    k
    |> match {
      { sequences, .. } => keymap_ncl.sequence.key_validator k,
      { sequence_passthrough, .. } => keymap_ncl.sequence_aux.key_validator k,
      "SequenceStart" => keymap_ncl.sequence_start.key_validator k,
      _ => keymap_ncl.nullable_key.key_validator k,
    },

  SequencedKeyElement = std.contract.from_validator sequenced_key_element_validator,

  sequenced_keys
    | Array SequencedKeyElement
    =
      std.array.map_with_index
        (fun key_index k =>
          # Do not rewrite keys already claimed as chorded primary/aux, or start tokens.
          k
          |> match {
            { chords = _, .. } => k,
            { passthrough = _, .. } => k,
            "SequenceStart" => k,
            _ =>
              sequences
              |> std.array.map_with_index (fun idx ch => ch & { sequence_index = idx })
              |> std.array.filter (fun { indices = [sequence_primary_key_index, ..], .. } => sequence_primary_key_index == key_index)
              |> match {
                [] =>
                  if std.array.any (fun { indices, .. } => std.array.elem key_index indices) sequences then
                    { sequence_passthrough = k }
                  else
                    k,
                sequences_ =>
                  let sequences_ = sequences_ |> std.array.map (fun { sequence_index, key, .. } => [sequence_index, key]) in
                  { sequences = sequences_, passthrough = k },
              },
          }
        )
        chorded_keys,
}
