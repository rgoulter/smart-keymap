#include "sticky_test_ceedling_fixture.h"

#ifdef SUITE_STICKY

#include "unity.h"

#include "hid_keycodes.h"
#include "smart_keymap.h"

void setUp(void) {}

void tearDown(void) {}

static void tap_key(uint16_t key, KeymapHidReport *report) {
  keymap_register_input_event(
      (struct KeymapInputEvent){.event_type = KeymapEventPress, .value = key});
  keymap_tick(report);
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventRelease, .value = key});
  keymap_tick(report);
}

void test_sticky_mod_modifies_next_keyboard_key(void) {
  uint8_t expected_report[8] = {MOD_LSHFT, 0, KC_A, 0, 0, 0, 0, 0};
  KeymapHidReport report = {};
  KeymapHidReport *actual_report = &report;

  // assemble: init keymap
  keymap_init();

  // act: tap sticky shift, then press A
  tap_key(KM_STICKY_SHIFT_KEY, actual_report);
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventPress, .value = KM_KEY_A});
  keymap_tick(actual_report);

  // assert: sticky shift applies to the next key only
  TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_report, actual_report->keyboard, 8);
}

void test_sticky_mod_only_affects_next_keyboard_key(void) {
  uint8_t expected_shifted[8] = {MOD_LSHFT, 0, KC_A, 0, 0, 0, 0, 0};
  uint8_t expected_unshifted[8] = {0, 0, KC_A, 0, 0, 0, 0, 0};
  KeymapHidReport report = {};
  KeymapHidReport *actual_report = &report;

  // assemble: init keymap
  keymap_init();

  // act: consume sticky shift on a tap of A
  tap_key(KM_STICKY_SHIFT_KEY, actual_report);
  tap_key(KM_KEY_A, actual_report);
  for (int i = 0; i < 10; i++) {
    keymap_tick(actual_report);
  }

  // act: press A again without re-arming sticky shift
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventPress, .value = KM_KEY_A});
  keymap_tick(actual_report);

  // assert: sticky modifier was consumed; A is unshifted
  TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_unshifted, actual_report->keyboard, 8);

  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventRelease, .value = KM_KEY_A});
  keymap_tick(actual_report);

  // act: re-arm sticky shift and press A once more
  tap_key(KM_STICKY_SHIFT_KEY, actual_report);
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventPress, .value = KM_KEY_A});
  keymap_tick(actual_report);

  // assert: newly armed sticky shift applies again
  TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_shifted, actual_report->keyboard, 8);
}

#else
#error "requires SUITE_STICKY"
#endif
