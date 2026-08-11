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

  apply_set_active_layers = fun new_active_layer_indices layers =>
    let layer_count = std.array.length layers in
    std.array.generate
      (fun idx =>
        let layer_index = idx + 1 in
        std.array.elem layer_index new_active_layer_indices
      )
      layer_count,

  apply_set_active_layers_masked = fun new_active_layer_indices mask_layers layers =>
    std.array.map_with_index
      (fun idx is_active =>
        let layer_index = idx + 1 in
        if std.array.elem layer_index mask_layers then
          std.array.elem layer_index new_active_layer_indices
        else
          is_active
      )
      layers,

  checks.check_set_active_layers = {
    unmasked = {
      expected = [ false, true, false ],
      actual = apply_set_active_layers [2] [ false, false, false ],
    },
    masked_preserves_outside_mask = {
      expected = [ true, false, true, false ],
      actual = apply_set_active_layers_masked [3] [3, 4] [ true, false, false, false ],
    },
    masked_switches_within_mask = {
      expected = [ false, false, false, true ],
      actual = apply_set_active_layers_masked [4] [3, 4] [ false, false, true, false ],
    },
  },

  # Highest active layer index (1-based), or null if none.
  # Note: Nickel records are recursive, so RHS field names must not shadow
  # outer bindings of the same name (e.g. avoid `{ idx = idx + 1 }`).
  highest_active_layer = fun active_layers =>
    std.array.fold_left
      (fun acc is_active =>
        let { idx = i, found = f } = acc in
        let next_idx = i + 1 in
        if is_active then
          { idx = next_idx, found = next_idx }
        else
          { idx = next_idx, found = f }
      )
      { idx = 0, found = null }
      active_layers
    |> (fun { found, .. } => found),

  # True if flags[layer_idx - 1] is set. Invalid / out-of-range indices are false.
  layer_flag_at = fun flags layer_idx =>
    let idx = layer_idx - 1 in
    if idx >= 0 && idx < std.array.length flags then
      std.array.at idx flags
    else
      false,

  # Helpers for layer_mod simulation on the inputs path.
  # Broad outer arms + has_field (not nested `{ layer_modifier = { … } }` multi-arms):
  # extra dedicated Press/Tap/Release nested patterns hang Nickel match compilation.
  apply_hold_press = fun hold layer_state =>
    let { active_layers = al, ..without_active_layers } = layer_state in
    let ll =
      if std.record.has_field "locked_layers" without_active_layers then without_active_layers.locked_layers else []
    in
    let other_fields =
      if std.record.has_field "locked_layers" without_active_layers then
        std.record.remove "locked_layers" without_active_layers
      else
        without_active_layers
    in
    if layer_flag_at ll hold then
      {
        active_layers = deactivate_layer al hold,
        locked_layers = set_layer false ll hold,
      }
      & other_fields
    else
      { active_layers = activate_layer al hold, locked_layers = ll } & other_fields,

  apply_hold_release = fun hold layer_state =>
    let { active_layers = al, ..without_active_layers } = layer_state in
    let ll =
      if std.record.has_field "locked_layers" without_active_layers then without_active_layers.locked_layers else []
    in
    let other_fields =
      if std.record.has_field "locked_layers" without_active_layers then
        std.record.remove "locked_layers" without_active_layers
      else
        without_active_layers
    in
    if layer_flag_at ll hold then
      { active_layers = al, locked_layers = ll } & other_fields
    else
      { active_layers = deactivate_layer al hold, locked_layers = ll } & other_fields,

  apply_lock_invert = fun lm layer_state =>
    let { active_layers = al, ..without_active_layers } = layer_state in
    let ll =
      if std.record.has_field "locked_layers" without_active_layers then
        without_active_layers.locked_layers
      else
        []
    in
    let other_fields =
      if std.record.has_field "locked_layers" without_active_layers then
        std.record.remove "locked_layers" without_active_layers
      else
        without_active_layers
    in
    # true = highest active layer; number = specific layer index.
    let layer =
      if lm.lock == true then
        highest_active_layer al
      else
        lm.lock
    in
    if layer == null then
      { active_layers = al, locked_layers = ll } & other_fields
    else if layer_flag_at ll layer then
      {
        active_layers = deactivate_layer al layer,
        locked_layers = set_layer false ll layer,
      }
      & other_fields
    else
      {
        active_layers = activate_layer al layer,
        locked_layers = set_layer true ll layer,
      }
      & other_fields,

  # Tap of hold: net press+release — unlock if locked, else no-op.
  apply_hold_tap = fun hold layer_state =>
    let { active_layers = al, ..without_active_layers } = layer_state in
    let ll =
      if std.record.has_field "locked_layers" without_active_layers then
        without_active_layers.locked_layers
      else
        []
    in
    let other_fields =
      if std.record.has_field "locked_layers" without_active_layers then
        std.record.remove "locked_layers" without_active_layers
      else
        without_active_layers
    in
    if layer_flag_at ll hold then
      {
        active_layers = deactivate_layer al hold,
        locked_layers = set_layer false ll hold,
      }
      & other_fields
    else
      { active_layers = al, locked_layers = ll } & other_fields,

  apply_set_active_layers_mod = fun lm layer_state =>
    let { active_layers = prior_active_layers, ..layer_state } = layer_state in
    let new_active_layers =
      if std.record.has_field "affected_layers" lm then
        apply_set_active_layers_masked lm.set_active_layers_to lm.affected_layers prior_active_layers
      else
        apply_set_active_layers lm.set_active_layers_to prior_active_layers
    in
    { active_layers = new_active_layers } & layer_state,

  # `..` keeps the pattern open so locked_layers (and future fields) are accepted.
  update_layer_state_for_input = fun input layer_state @ { default_layer, active_layers, .. } =>
    input
    |> match {
      # One broad arm per input kind + has_field on `lm` (match-tax).
      'Press { layer_modifier = lm, .. } =>
        if std.record.has_field "hold" lm then
          apply_hold_press lm.hold layer_state
        else if std.record.has_field "tap_toggle" lm then
          # Hold path: activate like MO while pressed.
          let { active_layers = al, ..layer_state } = layer_state in
          { active_layers = activate_layer al lm.tap_toggle } & layer_state
        else if std.record.has_field "toggle" lm then
          let { active_layers = al, ..layer_state } = layer_state in
          { active_layers = toggle_layer al lm.toggle } & layer_state
        else if std.record.has_field "lock" lm then
          apply_lock_invert lm layer_state
        else if std.record.has_field "set_active_layers_to" lm then
          apply_set_active_layers_mod lm layer_state
        else
          layer_state,
      'Release { layer_modifier = lm, .. } =>
        if std.record.has_field "hold" lm then
          apply_hold_release lm.hold layer_state
        else if std.record.has_field "tap_toggle" lm then
          # Hold-release path: deactivate like MO.
          # (NCL sim does not track interruption; pure taps use 'Tap below.)
          let { active_layers = al, ..layer_state } = layer_state in
          { active_layers = deactivate_layer al lm.tap_toggle } & layer_state
        else if std.record.has_field "default_" lm then
          let { default_layer, ..layer_state } = layer_state in
          { default_layer = lm.default_ } & layer_state
        else
          layer_state,
      'Tap { layer_modifier = lm, .. } =>
        if std.record.has_field "toggle" lm then
          let { active_layers = al, ..layer_state } = layer_state in
          { active_layers = toggle_layer al lm.toggle } & layer_state
        else if std.record.has_field "tap_toggle" lm then
          # Net toggle (press+release without interruption).
          let { active_layers = al, ..layer_state } = layer_state in
          { active_layers = toggle_layer al lm.tap_toggle } & layer_state
        else if std.record.has_field "lock" lm then
          apply_lock_invert lm layer_state
        else if std.record.has_field "hold" lm then
          apply_hold_tap lm.hold layer_state
        else if std.record.has_field "set_active_layers_to" lm then
          apply_set_active_layers_mod lm layer_state
        else
          layer_state,
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
