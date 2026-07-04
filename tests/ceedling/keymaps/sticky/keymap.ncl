let K = import "keys.ncl" in

{
  keys = [
    K.sticky K.LeftShift,
    K.A,
  ],

  ceedling_fixture = {
    suite = "STICKY",
    STICKY_SHIFT_KEY = 0,
    KEY_A = 1,
  },
}
