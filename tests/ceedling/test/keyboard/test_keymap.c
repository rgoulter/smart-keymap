#include "keyboard_test_ceedling_fixture.h"

#ifdef SUITE_KEYBOARD

#include "unity.h"

#include "hid_keycodes.h"
#include "smart_keymap.h"

void setUp(void) {}

void tearDown(void) {}

void test_copy_hid_boot_keyboard_report(void) {
  uint8_t expected_report[8] = {0, 0, 0, 0, 0, 0, 0, 0};
  KeymapHidReport report = {};
  KeymapHidReport *actual_report = &report;

  keymap_init();
  keymap_tick(actual_report);

  TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_report, actual_report->keyboard, 8);
}

void test_keyboard_keypress(void) {
  uint8_t expected_report[8] = {0, 0, KC_A, 0, 0, 0, 0, 0};
  KeymapHidReport report = {};
  KeymapHidReport *actual_report = &report;

  keymap_init();

  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventPress, .value = KM_KEY_A});
  keymap_tick(actual_report);

  TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_report, actual_report->keyboard, 8);
}

void test_keyboard_keyrelease(void) {
  uint8_t expected_report[8] = {0, 0, 0, 0, 0, 0, 0, 0};
  KeymapHidReport report = {};
  KeymapHidReport *actual_report = &report;

  // assemble: init keymap
  keymap_init();

  // act: press then release key A
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventPress, .value = KM_KEY_A});
  keymap_tick(actual_report);
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventRelease, .value = KM_KEY_A});
  keymap_tick(actual_report);

  // assert: empty report
  TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_report, actual_report->keyboard, 8);
}

void test_keyboard_keypress_sequence_da_db(void) {
  uint8_t expected_report[8] = {0, 0, KC_A, KC_B, 0, 0, 0, 0};
  KeymapHidReport report = {};
  KeymapHidReport *actual_report = &report;

  // assemble: init keymap
  keymap_init();

  // act: press A then B
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventPress, .value = KM_KEY_A});
  keymap_tick(actual_report);
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventPress, .value = KM_KEY_B});
  keymap_tick(actual_report);

  // assert: report shows A then B
  TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_report, actual_report->keyboard, 8);
}

void test_keyboard_keypress_sequence_db_da(void) {
  uint8_t expected_report[8] = {0, 0, KC_B, KC_A, 0, 0, 0, 0};
  KeymapHidReport report = {};
  KeymapHidReport *actual_report = &report;

  // assemble: init keymap
  keymap_init();

  // act: press B then A
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventPress, .value = KM_KEY_B});
  keymap_tick(actual_report);
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventPress, .value = KM_KEY_A});
  keymap_tick(actual_report);

  // assert: report shows B then A
  TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_report, actual_report->keyboard, 8);
}

void test_keyboard_keypress_sequence_da_db_ub(void) {
  uint8_t expected_report[8] = {0, 0, KC_A, 0, 0, 0, 0, 0};
  KeymapHidReport report = {};
  KeymapHidReport *actual_report = &report;

  // assemble: init keymap
  keymap_init();

  // act: press A, press B, release B
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventPress, .value = KM_KEY_A});
  keymap_tick(actual_report);
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventPress, .value = KM_KEY_B});
  keymap_tick(actual_report);
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventRelease, .value = KM_KEY_B});
  keymap_tick(actual_report);

  // assert: only A remains in report
  TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_report, actual_report->keyboard, 8);
}

void test_keyboard_keypress_sequence_da_db_ua(void) {
  uint8_t expected_report[8] = {0, 0, KC_B, 0, 0, 0, 0, 0};
  KeymapHidReport report = {};
  KeymapHidReport *actual_report = &report;

  // assemble: init keymap
  keymap_init();

  // act: press A, press B, release A
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventPress, .value = KM_KEY_A});
  keymap_tick(actual_report);
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventPress, .value = KM_KEY_B});
  keymap_tick(actual_report);
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventRelease, .value = KM_KEY_A});
  keymap_tick(actual_report);

  // assert: only B remains in report
  TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_report, actual_report->keyboard, 8);
}

void test_keyboard_double_keypress(void) {
  uint8_t expected_report[8] = {0, 0, KC_A, 0, 0, 0, 0, 0};
  KeymapHidReport report = {};
  KeymapHidReport *actual_report = &report;

  // assemble: init keymap
  keymap_init();

  // act: press A twice
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventPress, .value = KM_KEY_A});
  keymap_tick(actual_report);
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventPress, .value = KM_KEY_A});
  keymap_tick(actual_report);

  // assert: report still shows a single A
  TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_report, actual_report->keyboard, 8);
}

#else
#error "requires SUITE_KEYBOARD"
#endif
