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

  update_layer_state_for_input = fun input layer_state @ { active_layers, .. } =>
    input
    |> match {
      'Press { layer_modifier = { hold }, .. } =>
        let { active_layers = al, ..layer_state } = layer_state in
        { active_layers = activate_layer al hold } & layer_state,
      'Release { layer_modifier = { hold }, .. } =>
        let { active_layers = al, ..layer_state } = layer_state in
        { active_layers = deactivate_layer al hold } & layer_state,
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

  active_key = fun lk layer_state @ { active_layers, .. } =>
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
