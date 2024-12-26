Feature: Key Deserialization

  Example: Deserialize a simple::Key
    When a simple::Key is deserialized from the RON string
      """
      Key(0x04)
      """
    Then the result is same value as deserializing the JSON string
      """
      4
      """

  Example: Deserialize a tap_hold::Key
    When a tap_hold::Key is deserialized from the RON string
      """
      Key(tap: 0x04, hold: 0xE0)
      """
    Then the result is same value as deserializing the JSON string
      """
      { "tap": 4, "hold": 224 }
      """
