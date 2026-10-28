#include "callback_test_ceedling_fixture.h"

#ifdef SUITE_CALLBACK

#include "unity.h"

#include "hid_keycodes.h"
#include "smart_keymap.h"

static int reset_callback_count;

static void reset_callback(void) { reset_callback_count++; }

void setUp(void) {
  reset_callback_count = 0;
  keymap_init();
  keymap_clear_callbacks();
}

void tearDown(void) {}

void test_callback_not_invoked_without_registration(void) {
  uint8_t expected_report[8] = {0, 0, 0, 0, 0, 0, 0, 0};
  KeymapHidReport report = {};
  KeymapHidReport *actual_report = &report;

  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventPress, .value = KM_RESET_KEY});
  keymap_tick(actual_report);

  TEST_ASSERT_EQUAL_INT(0, reset_callback_count);
  TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_report, actual_report->keyboard, 8);
}

void test_reset_callback_invoked_on_press(void) {
  uint8_t expected_report[8] = {0, 0, 0, 0, 0, 0, 0, 0};
  KeymapHidReport report = {};
  KeymapHidReport *actual_report = &report;

  keymap_register_callback(KEYMAP_CALLBACK_RESET, reset_callback);

  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventPress, .value = KM_RESET_KEY});
  keymap_tick(actual_report);

  TEST_ASSERT_EQUAL_INT(1, reset_callback_count);
  TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_report, actual_report->keyboard, 8);
}

void test_callback_not_invoked_on_release(void) {
  uint8_t expected_report[8] = {0, 0, 0, 0, 0, 0, 0, 0};
  KeymapHidReport report = {};
  KeymapHidReport *actual_report = &report;

  keymap_register_callback(KEYMAP_CALLBACK_RESET, reset_callback);

  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventPress, .value = KM_RESET_KEY});
  keymap_tick(actual_report);
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventRelease, .value = KM_RESET_KEY});
  keymap_tick(actual_report);

  TEST_ASSERT_EQUAL_INT(1, reset_callback_count);
  TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_report, actual_report->keyboard, 8);
}

void test_callback_invoked_on_each_press(void) {
  uint8_t expected_report[8] = {0, 0, 0, 0, 0, 0, 0, 0};
  KeymapHidReport report = {};
  KeymapHidReport *actual_report = &report;

  keymap_register_callback(KEYMAP_CALLBACK_RESET, reset_callback);

  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventPress, .value = KM_RESET_KEY});
  keymap_tick(actual_report);
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventRelease, .value = KM_RESET_KEY});
  keymap_tick(actual_report);
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventPress, .value = KM_RESET_KEY});
  keymap_tick(actual_report);

  TEST_ASSERT_EQUAL_INT(2, reset_callback_count);
  TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_report, actual_report->keyboard, 8);
}

#else
#error "requires SUITE_CALLBACK"
#endif
