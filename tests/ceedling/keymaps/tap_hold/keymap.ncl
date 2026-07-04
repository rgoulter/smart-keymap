let K = import "keys.ncl" in

{
  keys = [
    K.C & K.hold K.LeftCtrl,
  ],

  ceedling_fixture = {
    suite = "TAP_HOLD",
    TAP_HOLD_KEY = 0,
  },
}
