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
    },

    # Layer Transparency
    TTTT = null,
  },
}
