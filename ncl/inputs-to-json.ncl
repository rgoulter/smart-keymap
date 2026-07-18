{
  inputs,

  layered_keys,

  # From keymap-ncl-to-json (merged): named-layer prepare + name→index resolution.
  prepare_keys_named_layers | default = fun keys => keys,
  named_layer_indices_from_keys | default = fun _keys => {},
  resolve_layer_modifier_names | default = fun _name_to_idx k => k,

  # From keymap-ncl-to-json: longest dense layered array after named-layer prepare.
  # Falls back to scanning layered_keys if not merged from that module.
  max_layered_length | default =
    layered_keys
    |> std.array.fold_left
      (fun acc k =>
        let len =
          k
          |> match {
            { layered, .. } if std.is_array layered => std.array.length layered,
            _ => 0,
          }
        in
        if len > acc then len else acc
      )
      0,

  # Construct [smart_keymap::input::Event(kmi)]
  # from ['Press k | 'Release k],
  # where 'k is a keymap.ncl key def
  #  (either LK, or the K of active layer).
  inputs_as_json_value_input_events =
    let LK = import "layered-key.ncl" in
    # Lower named_layers and resolve layer_mod name strings so
    # active_key / set_active_layers see dense arrays and numeric indices.
    let name_to_idx = named_layer_indices_from_keys layered_keys in
    let keys = prepare_keys_named_layers layered_keys in
    let resolve_key = fun k => resolve_layer_modifier_names name_to_idx k in
    let resolve_input = fun input =>
      input
      |> match {
        'Press k => 'Press (resolve_key k),
        'Release k => 'Release (resolve_key k),
        'Tap k => 'Tap (resolve_key k),
        other => other,
      }
    in
    let lookup_keymap_index = fun layer_state @ { active_layers, .. } k =>
      let k = resolve_key k in
      keys
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
               %{keys |> std.serialize 'Json}
               "%
      }
    in
    let input_as_json = fun layer_state @ { active_layers, .. } input =>
      input
      |> match {
        'Press k => { Press = { keymap_index = (lookup_keymap_index layer_state k) } },
        'Release k => { Release = { keymap_index = (lookup_keymap_index layer_state k) } },
        'Tap k => { Tap = { keymap_index = (lookup_keymap_index layer_state k) } },
        'PressKeymapIndex keymap_index => { Press = { include keymap_index } },
        'ReleaseKeymapIndex keymap_index => { Release = { include keymap_index } },
        'TapKeymapIndex keymap_index => { Tap = { include keymap_index } },
        'Wait duration => { Wait = { include duration } },
      }
    in
    let initial_layer_state =
      # Use max layered length across all keys (not only the first key).
      let initial_active_layers =
        std.array.generate (std.function.const false) max_layered_length
      in
      { default_layer = 0, active_layers = initial_active_layers }
    in
    let { json_value, .. } =
      std.array.fold_left
        (fun { layer_state = ls, json_value = jv } input =>
          # Assumes all `press lmod` in input_enum is
          #  where lmod is not layered.
          # e.g. unhandled case ['Press LMod 0, 'Press { layered = [LMod 1] }, ..]
          let input = resolve_input input in
          {
            layer_state = LK.update_layer_state_for_input input ls,
            json_value = jv @ [input_as_json ls input]
          }
        )
        { layer_state = initial_layer_state, json_value = [] }
        inputs
    in json_value
}
