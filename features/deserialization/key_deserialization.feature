Feature: Key Deserialization

  Example: Deserialize a keyboard::Key
    When a keyboard::Key is deserialized from the RON string
      """
      Key(key_code: 0x04)
      """
    Then the result is same value as deserializing the JSON string
      """
      { "key_code": 4 }
      """

  Example: Deserialize a tap_hold::Key
    When a tap_hold::Key is deserialized from the RON string
      """
      Key(tap: Key(key_code: 0x04), hold: Key(key_code: 0xE0))
      """
    Then the result is same value as deserializing the JSON string
      """
      { "tap": { "key_code": 4 }, "hold": { "key_code": 224 } }
      """

  Example: Deserialize a composite::Key (TapHold variant)
    When a composite::Key is deserialized from the RON string
      """
      TapHold(Key(tap: Key(key_code: 0x04), hold: Key(key_code: 0xE0)))
      """
    Then the result is same value as deserializing the JSON string
      """
      { "TapHold": { "tap": { "key_code": 4 }, "hold": { "key_code": 224 } } }
      """
