#include <stdint.h>

#include "unity.h"

#include "smart_keymap.h"

void setUp(void) {}

void tearDown(void) {}

void test_keymap_serialise_event_press(void) {
  uint8_t expected_message[4] = {0x01, 0x02, 0x04, 0x00};

  KeymapInputEvent event = {
      .event_type = KeymapEventPress,
      .value = 4,
  };
  uint8_t actual_message[4] = {0};
  keymap_serialize_event(actual_message, event);

  TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_message, actual_message, 4);
}

void test_keymap_deserialise_event_press(void) {
  KeymapInputEvent expected_event = {
      .event_type = KeymapEventPress,
      .value = 4,
  };

  uint8_t input[4] = {0x01, 0x02, 0x04, 0x00};
  uint8_t buf[4] = {0};
  KeymapInputEvent actual_event = {0};

  for (uint8_t i = 0; i < 4; i++) {
    keymap_message_buffer_receive_byte(&buf, input[i], &actual_event);
  }

  TEST_ASSERT_EQUAL_MEMORY(&expected_event, &actual_event,
                           sizeof(KeymapInputEvent));
}
