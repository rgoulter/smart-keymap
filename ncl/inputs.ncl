{
  press = fun k => 'Press k,
  release = fun k => 'Release k,
  tap = fun k => 'Tap k,
  press_keymap_index = fun kmi => 'PressKeymapIndex kmi,
  release_keymap_index = fun kmi => 'ReleaseKeymapIndex kmi,
  tap_keymap_index = fun kmi => 'TapKeymapIndex kmi,
  wait = fun duration => 'Wait duration,
}
