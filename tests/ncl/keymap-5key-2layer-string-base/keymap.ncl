# Leaf keys authored as records (caps-word toggle, key lock,
#  history repeat, sequence start) as layered bases with keys above them.
# Previously these were bare strings, which cannot merge with
#  `{ layered = … }` ("non mergeable terms").
# c.f. https://github.com/rgoulter/smart-keymap/issues/729
let K = import "keys.ncl" in
{
  layers = [
    [K.layer_mod.hold 1, K.caps_word.toggle, K.key_lock, K.history.repeat, K.sequence_start],
    [K.TTTT, K.A, K.B, K.C, K.D],
  ],
}
