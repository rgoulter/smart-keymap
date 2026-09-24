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
        {
          indices = m%"
            X _ X _
            _ X _ _
          "% |> CH.indices,
          key = K.layer_mod.set_active_layers_to [MOUSE_]
            & {
              layered =
                let B = K.layer_mod.set_active_layers_to [BASE_] in
                [B, B, B, B, B, B, B],
            },
        },

        {
          indices = m%"
            _ X _ _
            X _ X _
          "% |> CH.indices,
          key = K.layer_mod.set_active_layers_to [NAV_]
            & {
              layered =
                let B = K.layer_mod.set_active_layers_to [BASE_] in
                [B, B, B, B, B, B, B],
            },
        },

        {
          indices = m%"
            X _ _ X
            X _ _ X
          "% |> CH.indices,
          key = K.layer_mod.set_active_layers_to [BT_]
            & {
              layered =
                let B = K.layer_mod.set_active_layers_to [BASE_] in
                [B, B, B, B, B, B, B],
            },
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
          key = K.C
            & {
              layered = [
                K.NO,
                K.NO,
                K.NO,
                K.N9,
                K.NO,
                K.NO,
                K.NO,
              ],
            },
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
          key = K.F
            & {
              layered = [
                K.NO,
                K.NO,
                K.NO,
                K.N7,
                K.NO,
                K.NO,
                K.NO,
              ],
            },
        },
        {
          indices = m%"
            _ X X _
            _ _ _ _
          "% |> CH.indices,
          key = K.G
            & {
              layered = [
                K.NO,
                K.NO,
                K.NO,
                K.N8,
                K.NO,
                K.NO,
                K.NO,
              ],
            },
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
          key = K.U
            & {
              layered = [
                K.NO,
                K.NO,
                K.NO,
                K.N0,
                K.NO,
                K.NO,
                K.NO,
              ],
            },
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
          key = K.CapsLock, # Shift Lock
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
          key = K.callback 1 2, # BtClear
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

      # Bluetooth Keys
      BtProfile1 = K.callback 0 1,
      BtProfile2 = K.callback 0 2,
      BtProfile3 = K.callback 0 3,
      BtProfile4 = K.callback 0 4,
      BtProfile5 = K.callback 0 5,
      BtProfile6 = K.callback 0 6,
      BtModeBluetooth = K.callback 1 0,
      BtModeUsb = K.callback 1 1,
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
      MouseBtn1    MouseUp   MouseBtn2    MouseWheelUp
      MouseLeft    MouseDown MouseRight   MouseWheelDown
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
      Mute       Insert      VolumeUp   XXXX
      RightShift PrintScreen VolumeDown XXXX
    "%,
  ],
}
