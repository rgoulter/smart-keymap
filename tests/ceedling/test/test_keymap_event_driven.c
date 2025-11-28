#include "unity.h"

#include "smart_keymap.h"

#define KC_A 0x04
#define KC_B 0x05
#define KC_C 0x06

void setUp(void) {}

void tearDown(void) {}

// KEYMAP: [C & TH.LCtrl, D & TH.LSft, A, B]

void test_event_driven_key_press(void) {
  uint8_t expected_report[8] = {0, 0, KC_A, 0, 0, 0, 0, 0};
  KeymapHidReport report = {};
  KeymapHidReport* actual_report = &report;

  keymap_init();

  keymap_register_input_after_ms(
      0,
      (struct KeymapInputEvent){.event_type = KeymapEventPress, .value = 2},
      actual_report); // Third key in the keymap is A

  TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_report, actual_report->keyboard, 8);
}

void test_event_driven_key_tap(void) {
  uint8_t expected_report[8] = {0, 0, 0, 0, 0, 0, 0, 0};
  KeymapHidReport report = {};
  KeymapHidReport* actual_report = &report;

  keymap_init();

  keymap_register_input_after_ms(
      0,
      (struct KeymapInputEvent){.event_type = KeymapEventPress, .value = 2},
      actual_report); // Third key in the keymap is A
  keymap_register_input_after_ms(
      0,
      (struct KeymapInputEvent){.event_type = KeymapEventRelease, .value = 2},
      actual_report); // Third key in the keymap is A

  TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_report, actual_report->keyboard, 8);
}

void test_event_driven_tap_hold_key_tap(void) {
  uint8_t expected_report[8] = {0, 0, KC_C, 0, 0, 0, 0, 0};
  KeymapHidReport report = {};
  KeymapHidReport* actual_report = &report;

  keymap_init();

  // Tap the C & TH.LCtrl
  uint16_t keymap_index = 0;

  // Press (at time 0)
  // - schedules event at time 200,
  // - no key output.
  {
    uint32_t actual_next_ev_ms = keymap_register_input_after_ms(
       0,
       (struct KeymapInputEvent){.event_type = KeymapEventPress, .value = keymap_index},
       actual_report);

    TEST_ASSERT_EQUAL_UINT32(200, actual_next_ev_ms);
    TEST_ASSERT_EQUAL_UINT8_ARRAY(((uint8_t[8]){0, 0, 0, 0, 0, 0, 0, 0}), actual_report->keyboard, 8);
  }

  // Release (at time 150)
  // - next ev is 50 ms later (at time 200),
  // - should have 'C' output.
  {
    uint32_t actual_next_ev_ms = keymap_register_input_after_ms(
       150,
       (struct KeymapInputEvent){.event_type = KeymapEventRelease, .value = keymap_index},
       actual_report);

    TEST_ASSERT_EQUAL_UINT32(50, actual_next_ev_ms);
    TEST_ASSERT_EQUAL_UINT8_ARRAY(((uint8_t[8]){0, 0, KC_C, 0, 0, 0, 0, 0}), actual_report->keyboard, 8);
  }
}

void test_event_driven_tap_hold_key_tap_release_reported(void) {
  uint8_t expected_report[8] = {0, 0, KC_C, 0, 0, 0, 0, 0};
  KeymapHidReport report = {};
  KeymapHidReport* actual_report = &report;

  keymap_init();

  // Tap the C & TH.LCtrl
  uint16_t keymap_index = 0;

  // Press (at time 0)
  // - schedules event at time 200,
  // - no key output.
  {
    uint32_t actual_next_ev_ms = keymap_register_input_after_ms(
       0,
       (struct KeymapInputEvent){.event_type = KeymapEventPress, .value = keymap_index},
       actual_report);

    TEST_ASSERT_EQUAL_UINT32(200, actual_next_ev_ms);
    TEST_ASSERT_EQUAL_UINT8_ARRAY(((uint8_t[8]){0, 0, 0, 0, 0, 0, 0, 0}), actual_report->keyboard, 8);

    // Keymap doesn't require polling yet.
    TEST_ASSERT_FALSE(keymap_requires_polling());
  }

  // Release (at time 150)
  // - next ev is 50 ms later (at time 200),
  // - should have 'C' output.
  {
    uint32_t actual_next_ev_ms = keymap_register_input_after_ms(
       150,
       (struct KeymapInputEvent){.event_type = KeymapEventRelease, .value = keymap_index},
       actual_report);

    TEST_ASSERT_EQUAL_UINT32(50, actual_next_ev_ms);
    TEST_ASSERT_EQUAL_UINT8_ARRAY(((uint8_t[8]){0, 0, KC_C, 0, 0, 0, 0, 0}), actual_report->keyboard, 8);

    // Keymap requires polling until the 'tap' is finished.
    TEST_ASSERT_TRUE(keymap_requires_polling());
  }

  // Next tick: still has 'C' output (the tap is reported)
  {
    keymap_tick(actual_report);
    TEST_ASSERT_EQUAL_UINT8_ARRAY(((uint8_t[8]){0, 0, KC_C, 0, 0, 0, 0, 0}), actual_report->keyboard, 8);
    TEST_ASSERT_TRUE(keymap_requires_polling());
  }

  // Next tick: no output (the tap is cleared)
  {
    keymap_tick(actual_report);
    TEST_ASSERT_EQUAL_UINT8_ARRAY(((uint8_t[8]){0, 0, 0, 0, 0, 0, 0, 0}), actual_report->keyboard, 8);
    TEST_ASSERT_FALSE(keymap_requires_polling());
  }
}
