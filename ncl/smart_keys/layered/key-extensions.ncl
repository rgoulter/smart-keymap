{
  key_extensions.layered = {
    layer_mod = {
      set_default = fun layer_num =>
        {
          layer_modifier = { default_ = layer_num }
        },
      hold = fun layer_num =>
        {
          layer_modifier = { hold = layer_num }
        },
      toggle = fun layer_num =>
        {
          layer_modifier = { toggle = layer_num }
        },
      sticky = fun layer_num =>
        {
          layer_modifier = { sticky = layer_num }
        },
      set_active_layers_to = fun active_layers =>
        {
          layer_modifier = { set_active_layers_to = active_layers }
        },
      set_active_layers_amongst = fun { set_active_layers_to = set_active_layers_to_, affected_layers = affected_layers_ } =>
        {
          layer_modifier = {
            set_active_layers_to = set_active_layers_to_,
            affected_layers = affected_layers_,
          },
        },
      # Exclusive switch within a named-layer group.
      # `variant_names` is the mask (all members); `variant_name` is the one to activate.
      # Names resolve to indices when the keymap is compiled to JSON.
      set_semantic_variant_to = fun variant_names variant_name =>
        {
          layer_modifier = {
            set_active_layers_to = [variant_name],
            affected_layers = variant_names,
          },
        },
    },

    # Layer Transparency
    TTTT = null,

    # Semantic / variant key sugar: `named_layers` form of LayeredKey.
    # Unit `{}` base (keyboard noop) so callers can use `K.semantic {..}` without
    # `K.NO &`. Indices for names are assigned globally when compiling the keymap.
    semantic = fun variants => { named_layers = variants },
  },
}
