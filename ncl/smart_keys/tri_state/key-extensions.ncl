{
  key_extensions.tri_state = fun keys =>
    {
      tri_state = {
        # General constructor: K.tri_state.custom { hold = K.LeftAlt, tap = K.Tab }
        custom = fun spec => { tri_state = spec },
        alt_tab = { tri_state = { hold = keys.LeftAlt, tap = keys.Tab } },
        cmd_tab = { tri_state = { hold = keys.LeftGUI, tap = keys.Tab } },
        ctrl_tab = { tri_state = { hold = keys.LeftCtrl, tap = keys.Tab } },
      },
    },
}
