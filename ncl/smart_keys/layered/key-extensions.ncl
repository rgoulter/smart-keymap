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
      set_active_layers_to = fun active_layers =>
        {
          layer_modifier = { set_active_layers_to = active_layers }
        },
    },

    # Layer Transparency
    TTTT = null,
  },
}
