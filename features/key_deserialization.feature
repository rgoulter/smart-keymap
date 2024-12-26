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
