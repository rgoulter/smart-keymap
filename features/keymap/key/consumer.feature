Feature: Consumer Keys (Media keys)

  HID Consumer usage codes.

  Media keys (play/pause, volume up, volume down, etc.) have been defined.

  Example: media keys
    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      {
        keys = [
          K.PlayPause,
          K.ScanNext,
          K.ScanPrevious,
          K.Stop,
          K.VolumeDown,
          K.VolumeUp,
        ],
      }
      """
