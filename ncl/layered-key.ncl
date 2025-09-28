# Functions emulating layers & layered keys.
{
  set_layer = fun b active_layers layer_idx =>
    let idx = layer_idx - 1 in
    let len = std.array.length active_layers in
    if idx < len then
      (std.array.slice 0 idx active_layers)
      @ [b]
      @ (std.array.slice (idx + 1) len active_layers)
    else
      active_layers @ (std.array.generate (std.function.const false) (idx - len)) @ [b],

  checks.check_activate_layer = {
    basic = {
      expected = [ false, false, true ],
      actual = activate_layer [ false, false, false ] 2,
    },
  },

  activate_layer = set_layer true,

  deactivate_layer = set_layer false,

  toggle_layer = fun active_layers layer_idx =>
    let idx = layer_idx - 1 in
    let len = std.array.length active_layers in
    if idx < len then
      let is_active = std.array.at idx active_layers in
      if is_active then
        set_layer false active_layers layer_idx
      else
        set_layer true active_layers layer_idx
    else
      set_layer true active_layers layer_idx,

  update_layer_state_for_input = fun input layer_state @ { default_layer, active_layers } =>
    input
    |> match {
      'Press { layer_modifier = { hold }, .. } =>
        let { active_layers = al, ..layer_state } = layer_state in
        { active_layers = activate_layer al hold } & layer_state,
      'Release { layer_modifier = { hold }, .. } =>
        let { active_layers = al, ..layer_state } = layer_state in
        { active_layers = deactivate_layer al hold } & layer_state,
      'Press { layer_modifier = { toggle }, .. } =>
        let { active_layers = al, ..layer_state } = layer_state in
        { active_layers = toggle_layer al toggle } & layer_state,
      'Press { layer_modifier = { set_active_layers_to = new_active_layer_indices }, .. } =>
        let { active_layers, ..layer_state } = layer_state in
        let layer_count = std.array.length active_layers in
        let new_active_layers =
          std.array.generate
            (fun idx =>
              let layer_index = idx + 1 in
              std.array.elem layer_index new_active_layer_indices
            )
            layer_count
        in
        { active_layers = new_active_layers } & layer_state,
      'Release { layer_modifier = { default_ }, .. } =>
        let { default_layer, ..layer_state } = layer_state in
        { default_layer = default_ } & layer_state,
      _ => layer_state,
    },

  checks.check_active_key = {
    base = {
      expected = { key_code = "A" },
      actual = active_key { key_code = "A", layered = ["B", "C", "D"], } { active_layers = [ false, false, false ] },
    },
    base_none = {
      expected = { key_code = "A" },
      actual = active_key { key_code = "A", layered = ["B", "C", "D"], } { active_layers = [], }
    },
    layered = {
      expected = "C",
      actual = active_key { key_code = "A", layered = ["B", "C", "D"], } { active_layers = [ true, true, false ] },
    },
  },

  active_key = fun lk layer_state @ { default_layer, active_layers, .. } =>
    let active_layers =
      if default_layer > 0 then
        activate_layer active_layers default_layer
      else
        active_layers
    in
    lk
    |> match {
      { passthrough, .. } =>
        active_key passthrough layer_state,
      { layered, ..base_key } =>
        let zipped_layered_keys =
          std.array.zip_with
            (fun a k => { layer_is_active = a, key = k })
            active_layers
            layered
        in
        let zipped_active_keys =
          std.array.filter
            (fun { layer_is_active, .. } => layer_is_active)
            zipped_layered_keys
        in
        let active_keys =
          std.array.map
            (fun { key, .. } => key)
            zipped_active_keys
        in
        if (std.array.length active_keys) > 0 then
          std.array.last active_keys
        else
          base_key,
      k => k,
    },
}
