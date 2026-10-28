#include "tap_hold_test_ceedling_fixture.h"

#ifdef SUITE_TAP_HOLD

#include "unity.h"

#include "hid_keycodes.h"
#include "smart_keymap.h"

void setUp(void) {}

void tearDown(void) {}

void test_taphold_dth_uth_is_tap(void) {
  uint8_t expected_report[8] = {0, 0, KC_C, 0, 0, 0, 0, 0};
  KeymapHidReport report = {};
  KeymapHidReport *actual_report = &report;

  // assemble: init keymap
  keymap_init();

  // act: press then release tap-hold key before hold timeout
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventPress, .value = KM_TAP_HOLD_KEY});
  keymap_tick(actual_report);
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventRelease, .value = KM_TAP_HOLD_KEY});
  keymap_tick(actual_report);

  // assert: tap key (C) appears in report
  TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_report, actual_report->keyboard, 8);
}

void test_taphold_dth_uth_eventually_clears(void) {
  uint8_t expected_report[8] = {0, 0, 0, 0, 0, 0, 0, 0};
  KeymapHidReport report = {};
  KeymapHidReport *actual_report = &report;

  // assemble: init keymap
  keymap_init();

  // act: press then release tap-hold key, then poll until tap clears
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventPress, .value = KM_TAP_HOLD_KEY});
  keymap_tick(actual_report);
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventRelease, .value = KM_TAP_HOLD_KEY});
  keymap_tick(actual_report);

  for (int i = 0; i < 50; i++) {
    keymap_tick(actual_report);
  }

  // assert: tap key released from report
  TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_report, actual_report->keyboard, 8);
}

void test_taphold_dth_eventually_holds(void) {
  uint8_t expected_report[8] = {MOD_LCTL, 0, 0, 0, 0, 0, 0, 0};
  KeymapHidReport report = {};
  KeymapHidReport *actual_report = &report;

  // assemble: init keymap
  keymap_init();

  // act: press tap-hold key and wait for hold timeout
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventPress, .value = KM_TAP_HOLD_KEY});

  for (int i = 0; i < 500; i++) {
    keymap_tick(actual_report);
  }

  // assert: hold modifier (Left Ctrl) in report
  TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_report, actual_report->keyboard, 8);
}

#else
#error "requires SUITE_TAP_HOLD"
#endif
