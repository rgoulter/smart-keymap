# Functions emulating layers & layered keys.
{
  set_layer = fun b active_layers idx =>
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

  update_active_layers_for_input = fun input active_layers =>
    input
    |> match {
      'Press { layer_modifier = { hold }, .. } => activate_layer active_layers hold,
      'Release { layer_modifier = { hold }, .. } => deactivate_layer active_layers hold,
      _ => active_layers,
    },

  checks.check_active_key = {
    base = {
      expected = { key_code = "A" },
      actual = active_key { key_code = "A", layered = ["B", "C", "D"], } [ false, false, false ],
    },
    base_none = {
      expected = { key_code = "A" },
      actual = active_key { key_code = "A", layered = ["B", "C", "D"], } [],
    },
    layered = {
      expected = "C",
      actual = active_key { key_code = "A", layered = ["B", "C", "D"], } [ true, true, false ],
    },
  },

  active_key = fun lk active_layers =>
    lk
    |> match {
      { passthrough, .. } =>
        active_key passthrough active_layers,
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
