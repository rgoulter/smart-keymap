let K = import "keys.ncl" in

{
  keys = [
    K.A,
    K.B,
  ],

  ceedling_fixture = {
    suite = "KEYBOARD",
    KEY_A = 0,
    KEY_B = 1,
  },
}
