let K = import "keys.ncl" in

{
  keys = [
    # TapHold (tap: Keyboard, hold: LayerMod)
    #   needs NK = composite::Key<Keyboard>
    K.A & K.hold (K.layer_mod.hold 1),
    K.B & { layered = [ K.C ] },
  ],
}
