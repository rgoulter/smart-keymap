#include "consumer_test_ceedling_fixture.h"

#ifdef SUITE_CONSUMER

#include "unity.h"

#include "hid_consumer_codes.h"
#include "smart_keymap.h"

void setUp(void) {}

void tearDown(void) {}

void test_consumer_key_press_reports_usage(void) {
  uint8_t expected_keyboard[8] = {0, 0, 0, 0, 0, 0, 0, 0};
  uint8_t expected_consumer[KEYMAP_HID_REPORT_CONSUMER_LEN] = {
      CONSUMER_PLAY_PAUSE,
      0,
      0,
      0,
  };
  KeymapHidReport report = {};
  KeymapHidReport *actual_report = &report;

  // assemble: init keymap
  keymap_init();

  // act: press Play/Pause consumer key
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventPress, .value = KM_PLAY_PAUSE_KEY});
  keymap_tick(actual_report);

  // assert: usage appears in consumer[], not keyboard[]
  TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_keyboard, actual_report->keyboard, 8);
  TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_consumer, actual_report->consumer,
                                KEYMAP_HID_REPORT_CONSUMER_LEN);
}

void test_consumer_key_release_clears_usage(void) {
  uint8_t expected_keyboard[8] = {0, 0, 0, 0, 0, 0, 0, 0};
  uint8_t expected_consumer[KEYMAP_HID_REPORT_CONSUMER_LEN] = {0, 0, 0, 0};
  KeymapHidReport report = {};
  KeymapHidReport *actual_report = &report;

  // assemble: init keymap
  keymap_init();

  // act: press then release Play/Pause
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventPress, .value = KM_PLAY_PAUSE_KEY});
  keymap_tick(actual_report);
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventRelease, .value = KM_PLAY_PAUSE_KEY});
  keymap_tick(actual_report);

  // assert: consumer report cleared; keyboard still empty
  TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_keyboard, actual_report->keyboard, 8);
  TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_consumer, actual_report->consumer,
                                KEYMAP_HID_REPORT_CONSUMER_LEN);
}

#else
#error "requires SUITE_CONSUMER"
#endif
