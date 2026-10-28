let K = import "keys.ncl" in

{
  keys = [
    K.MouseBtn1,
    K.MouseLeft,
    K.MouseWheelUp,
  ],

  ceedling_fixture = {
    suite = "MOUSE",
    MOUSE_BTN1_KEY = 0,
    MOUSE_LEFT_KEY = 1,
    MOUSE_WHEEL_UP_KEY = 2,
  },
}
