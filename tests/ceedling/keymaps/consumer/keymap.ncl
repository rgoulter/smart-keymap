let K = import "keys.ncl" in

{
  keys = [
    K.PlayPause,
  ],

  ceedling_fixture = {
    suite = "CONSUMER",
    PLAY_PAUSE_KEY = 0,
  },
}
