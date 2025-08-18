# Port of ARTSEYIO layout, v0.8.1, RHS.
# - https://artsey.io/

# Layer indices:
let BASE_ = 0 in
let NAV_ = 1 in
let MOUSE_ = 2 in
let BT_ = 3 in

let NUM_ = 4 in
let PAREN_ = 5 in
let SYM_ = 6 in
let MEDIA_ = 7 in

{
  chords =
    let K = import "keys.ncl" in
    let CH = import "chording.ncl" in
    [
        # Set Layer chords
        # XXX: will need to be able to set *back* to base
        {
          indices = m%"
            X _ X _
            _ X _ _
          "% |> CH.indices,
          key = K.layer_mod.set_active_layers_to [MOUSE_],
        },

        {
          indices = m%"
            _ X _ _
            X _ X _
          "% |> CH.indices,
          key = K.layer_mod.set_active_layers_to [NAV_],
        },

        {
          indices = m%"
            X _ _ X
            X _ _ X
          "% |> CH.indices,
          key = K.layer_mod.set_active_layers_to [BT_],
        },

        # Base (& Num) chords
        {
          indices = m%"
            _ _ _ _
            X _ _ X
          "% |> CH.indices,
          key = K.B,
        },
        {
          indices = m%"
            _ _ _ _
            X X _ _
          "% |> CH.indices,
          key = K.C, # XXX ... and num
        },
        {
          indices = m%"
            X X X _
            _ _ _ _
          "% |> CH.indices,
          key = K.D,
        },
        {
          indices = m%"
            X X _ _
            _ _ _ _
          "% |> CH.indices,
          key = K.F, # XXX ... and num
        },
        {
          indices = m%"
            _ X X _
            _ _ _ _
          "% |> CH.indices,
          key = K.G, # XXX ... and num
        },
        {
          indices = m%"
            _ _ _ _
            X _ X _
          "% |> CH.indices,
          key = K.H,
        },
        {
          indices = m%"
            _ _ X X
            _ _ _ _
          "% |> CH.indices,
          key = K.J,
        },
        {
          indices = m%"
            _ _ _ _
            _ X _ X
          "% |> CH.indices,
          key = K.K,
        },
        {
          indices = m%"
            _ _ _ _
            X X X _
          "% |> CH.indices,
          key = K.L,
        },
        {
          indices = m%"
            _ _ _ _
            _ X X X
          "% |> CH.indices,
          key = K.M,
        },
        {
          indices = m%"
            _ _ _ _
            _ _ X X
          "% |> CH.indices,
          key = K.N,
        },
        {
          indices = m%"
            _ _ _ _
            X _ X X
          "% |> CH.indices,
          key = K.P,
        },
        {
          indices = m%"
            X _ X X
            _ _ _ _
          "% |> CH.indices,
          key = K.Q,
        },
        {
          indices = m%"
            _ _ _ _
            _ X X _
          "% |> CH.indices,
          key = K.U, # XXX ... and num
        },
        {
          indices = m%"
            _ X _ X
            _ _ _ _
          "% |> CH.indices,
          key = K.V,
        },
        {
          indices = m%"
            X _ _ X
            _ _ _ _
          "% |> CH.indices,
          key = K.W,
        },
        {
          indices = m%"
            _ X X X
            _ _ _ _
          "% |> CH.indices,
          key = K.X,
        },
        {
          indices = m%"
            X X X X
            _ _ _ _
          "% |> CH.indices,
          key = K.Z,
        },
        {
          indices = m%"
            X _ _ _
            X _ _ _
          "% |> CH.indices,
          key = K.Return,
        },
        {
          indices = m%"
            X _ _ _
            _ X X _
          "% |> CH.indices,
          key = K.Quote,
        },
        {
          indices = m%"
            X _ _ _
            _ X _ _
          "% |> CH.indices,
          key = K.Dot,
        },
        {
          indices = m%"
            X _ _ _
            _ _ X _
          "% |> CH.indices,
          key = K.Comma,
        },
        {
          indices = m%"
            X _ _ _
            _ _ _ X
          "% |> CH.indices,
          key = K.Slash,
        },
        {
          indices = m%"
            _ _ X _
            _ _ X _
          "% |> CH.indices,
          key = K.Exclaim,
        },
        {
          indices = m%"
            _ _ _ _
            X X X X
          "% |> CH.indices,
          key = K.Space,
        },
        {
          indices = m%"
            _ X _ _
            X _ _ _
          "% |> CH.indices,
          key = K.Backspace,
        },
        {
          indices = m%"
            _ X _ _
            _ _ X _
          "% |> CH.indices,
          key = K.Delete,
        },
        {
          indices = m%"
            X X _ _
            _ _ _ X
          "% |> CH.indices,
          key = K.Escape,
        },
        {
          indices = m%"
            X X X _
            _ _ _ X
          "% |> CH.indices,
          key = K.Tab,
        },
        {
          indices = m%"
            _ _ _ X
            X _ _ _
          "% |> CH.indices,
          key = K.sticky K.LeftCtrl,
        },
        {
          indices = m%"
            _ _ _ X
            _ X _ _
          "% |> CH.indices,
          key = K.sticky K.LeftGUI,
        },
        {
          indices = m%"
            _ _ _ X
            _ _ X _
          "% |> CH.indices,
          key = K.sticky K.LeftAlt,
        },
        {
          indices = m%"
            _ X X X
            X _ _ _
          "% |> CH.indices,
          key = K.sticky K.LeftShift,
        },
        {
          indices = m%"
            _ X _ _
            _ X _ _
          "% |> CH.indices,
          key = K.NO, # TBI: Shift Lock?
        },
        {
          indices = m%"
            X _ _ _
            _ X X X
          "% |> CH.indices,
          key = K.CapsLock,
        },
        {
          indices = m%"
            _ X X _
            _ X X _
          "% |> CH.indices,
          key = K.NO, # TBI: Clear bluetooth
        },
    ],
  custom_keys = fun K =>
    let HoldLayerMod = fun layer_index => K.hold (K.layer_mod.hold layer_index) in
    {
      a = K.A & HoldLayerMod PAREN_,
      r = K.R,
      t = K.T,
      s = K.S & HoldLayerMod NUM_,

      e = K.E & HoldLayerMod SYM_,
      y = K.Y,
      i = K.I,
      o = K.O & HoldLayerMod MEDIA_,

      # Mouse Keys (TBI)
      MouseButton1 = K.NO,
      MouseButton2 = K.NO,
      MouseLeft = K.NO,
      MouseDown = K.NO,
      MouseUp = K.NO,
      MouseRight = K.NO,
      MouseScrollDown = K.NO,
      MouseScrollUp = K.NO,

      # Bluetooth Keys (TBI)
      BtProfile1 = K.NO,
      BtProfile2 = K.NO,
      BtProfile3 = K.NO,
      BtProfile4 = K.NO,
      BtProfile5 = K.NO,
      BtProfile6 = K.NO,
      BtModeBluetooth = K.NO,
      BtModeUsb = K.NO,

      # Media Keys (TBI)
      MediaMute = K.NO,
      MediaVolUp = K.NO,
      MediaVolDown = K.NO,
    },

  layers = [
    # 0: BASE_
    m%"
      a r t s
      e y i o
    "%,

    # 1: NAV_
    m%"
      HOME UP   END  PGUP
      LEFT DOWN RGHT PGDN
    "%,

    # 2: MOUSE_
    m%"
      MouseButton1 MouseUp   MouseButton2 MouseScrollUp
      MouseLeft    MouseDown MouseRight   MouseScrollDown
    "%,

    # 3: BT_
    m%"
      BtProfile1 BtProfile2 BtProfile3 BtModeBluetooth
      BtProfile4 BtProfile5 BtProfile6 BtModeUsb
    "%,

    # 4: NUM_
    m%"
      1 2 3 XXXX
      4 5 6 XXXX
    "%,

    # 5: PAREN_
    m%"
      XXXX ( ) {
      XXXX [ ] }
    "%,

    # 6: SYM_
    m%"
      !    \ ; `
      XXXX ? - =
    "%,

    # 7: MEDIA_
    m%"
      MediaMute  Insert      MediaVolUp   XXXX
      RightShift PrintScreen MediaVolDown XXXX
    "%,
  ],
}
