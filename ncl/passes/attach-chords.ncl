# Pass: attach global `chords` as chorded / passthrough on key slots
#
# Inputs (from driver merge): chords, layered_keys, keymap_ncl, validators
# Outputs: chorded_keys, ChordedKeyElement
{
  chords,
  layered_keys,
  keymap_ncl,
  validators,

  chorded_key_element_validator = fun k =>
    k
    |> match {
      { chords, .. } => keymap_ncl.chorded.key_validator k,
      { passthrough, .. } => keymap_ncl.chorded_aux.key_validator k,
      _ => keymap_ncl.nullable_key.key_validator k,
    },

  ChordedKeyElement = std.contract.from_validator chorded_key_element_validator,

  chorded_keys
    | Array ChordedKeyElement
    =
      std.array.map_with_index
        (fun key_index k =>
          chords
          |> std.array.map_with_index (fun idx ch => ch & { chord_index = idx })
          |> std.array.filter (fun { indices = [chord_primary_key_index, ..], .. } => chord_primary_key_index == key_index)
          |> match {
            [] =>
              if std.array.any (fun { indices, .. } => std.array.elem key_index indices) chords then
                { passthrough = k }
              else
                k,
            chords_ =>
              let chords_ = chords_ |> std.array.map (fun { chord_index, key, .. } => [chord_index, key]) in
              { chords = chords_, passthrough = k },
          }
        )
        layered_keys,

  checks.check_chorded_key_element =
    let K = import "keys.ncl" in
    {
      check_plain_key_ok =
        chorded_key_element_validator K.A == 'Ok,

      check_null_ok =
        chorded_key_element_validator null == 'Ok,

      check_string_token_ok =
        chorded_key_element_validator K.caps_word.toggle == 'Ok,

      check_passthrough_ok =
        chorded_key_element_validator { passthrough = K.A } == 'Ok,

      check_full_chord_ok =
        chorded_key_element_validator {
          chords = [[0, K.C]],
          passthrough = K.A,
        } == 'Ok,

      check_passthrough_invalid_inner_is_error =
        chorded_key_element_validator { passthrough = { not_a_key = true } }
        |> validators.check_is_error,
    },
}
