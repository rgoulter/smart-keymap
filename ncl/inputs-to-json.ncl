{
  inputs,

  layered_keys,

  # Construct [smart_keymap::input::Event(kmi)]
  # from ['Press k | 'Release k],
  # where 'k is a keymap.ncl key def
  #  (either LK, or the K of active layer).
  inputs_as_json_value_input_events =
    let LK = import "layered-key.ncl" in
    let lookup_keymap_index = fun layer_state @ { active_layers, .. } k =>
      layered_keys
      |> std.array.map_with_index (fun idx lk =>
        if lk == k then
          idx
        else if (LK.active_key lk layer_state) == k then
          idx
        else
          false
      )
      |> std.array.filter (fun idx => idx != false)
      |> match {
        [idx, ..] => idx,
        _ =>
          std.fail_with m%"
               unable to find keymap_index for:

               layer_state=
               %{layer_state |> std.serialize 'Json},

               k=
               %{k |> std.serialize 'Json}

               layered_keys=
               %{layered_keys |> std.serialize 'Json}
               "%
      }
    in
    let input_as_json = fun layer_state @ { active_layers, .. } input =>
      input
      |> match {
        'Press k => { Press = { keymap_index = (lookup_keymap_index layer_state k) } },
        'Release k => { Release = { keymap_index = (lookup_keymap_index layer_state k) } },
      }
    in
    let initial_layer_state =
      let initial_active_layers =
        layered_keys
        |> std.array.first
        |> match {
          { layered, .. } => std.array.map (std.function.const false) layered,
          _ => [],
        }
      in
      { default_layer = 0, active_layers = initial_active_layers }
    in
    let { serialized_json, .. } =
      std.array.fold_left
        (fun { layer_state = ls, serialized_json = sj } input =>
          # Assumes all `press lmod` in input_enum is
          #  where lmod is not layered.
          # e.g. unhandled case ['Press LMod 0, 'Press { layered = [LMod 1] }, ..]
          {
            layer_state = LK.update_layer_state_for_input input ls,
            serialized_json = sj @ [input_as_json ls input]
          }
        )
        { layer_state = initial_layer_state, serialized_json = [] }
        inputs
    in serialized_json
}
