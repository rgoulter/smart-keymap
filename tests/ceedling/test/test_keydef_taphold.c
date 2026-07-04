#include "unity.h"

#include "smart_keymap.h"

#define KC_A 0x04
#define KC_B 0x05
#define KC_C 0x06
#define KC_D 0x07

#define MOD_LCTL 0x1
#define MOD_LSFT 0x2

void setUp(void) {}

void tearDown(void) {}

void test_taphold_dth_uth_is_tap(void) {
  uint8_t expected_report[8] = {0, 0, KC_C, 0, 0, 0, 0, 0};
  KeymapHidReport report = {};
  KeymapHidReport* actual_report = &report;

  // assemble: init keymap
  keymap_init();

  // act: press then release tap-hold key before hold timeout
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventPress,
      .value = 0}); // First key in keymap is TapHold(C, _)
  keymap_tick(actual_report);
  keymap_register_input_event(
      (struct KeymapInputEvent){.event_type = KeymapEventRelease, .value = 0});
  keymap_tick(actual_report);

  // assert: tap key (C) appears in report
  TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_report, actual_report->keyboard, 8);
}

void test_taphold_dth_uth_eventually_clears(void) {
  uint8_t expected_report[8] = {0, 0, 0, 0, 0, 0, 0, 0};
  KeymapHidReport report = {};
  KeymapHidReport* actual_report = &report;

  // assemble: init keymap
  keymap_init();

  // act: press then release tap-hold key, then poll until tap clears
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventPress,
      .value = 0}); // First key in keymap is TapHold(C, _)
  keymap_tick(actual_report);
  keymap_register_input_event(
      (struct KeymapInputEvent){.event_type = KeymapEventRelease, .value = 0});
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
  KeymapHidReport* actual_report = &report;

  // assemble: init keymap
  keymap_init();

  // act: press tap-hold key and wait for hold timeout
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventPress,
      .value = 0}); // First key in keymap is TapHold(C, _)

  for (int i = 0; i < 500; i++) {
    keymap_tick(actual_report);
  }

  // assert: hold modifier (Left Ctrl) in report
  TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_report, actual_report->keyboard, 8);
}
