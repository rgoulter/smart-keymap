#include "layered_test_ceedling_fixture.h"

#ifdef SUITE_LAYERED

#include "unity.h"

#include "hid_keycodes.h"
#include "smart_keymap.h"

void setUp(void) {}

void tearDown(void) {}

void test_layered_key_acts_as_base_when_no_layer_active(void) {
  uint8_t expected_report[8] = {0, 0, KC_A, 0, 0, 0, 0, 0};
  KeymapHidReport report = {};
  KeymapHidReport *actual_report = &report;

  // assemble: init keymap
  keymap_init();

  // act: press layered key with no layer modifier held
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventPress, .value = KM_LAYERED_KEY});
  keymap_tick(actual_report);

  // assert: base-layer key (A) in report
  TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_report, actual_report->keyboard, 8);
}

void test_layered_key_acts_as_active_layer_when_layer_mod_held(void) {
  uint8_t expected_report[8] = {0, 0, KC_B, 0, 0, 0, 0, 0};
  KeymapHidReport report = {};
  KeymapHidReport *actual_report = &report;

  // assemble: init keymap
  keymap_init();

  // act: hold layer modifier, then press layered key
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventPress, .value = KM_LAYER_MOD_KEY});
  keymap_tick(actual_report);
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventPress, .value = KM_LAYERED_KEY});
  keymap_tick(actual_report);

  // assert: active-layer key (B) in report
  TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_report, actual_report->keyboard, 8);
}

void test_layered_keypress_retained_when_layer_mod_released(void) {
  uint8_t expected_report[8] = {0, 0, KC_B, 0, 0, 0, 0, 0};
  KeymapHidReport report = {};
  KeymapHidReport *actual_report = &report;

  // assemble: init keymap
  keymap_init();

  // act: activate layer, press layered key, then release layer modifier
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventPress, .value = KM_LAYER_MOD_KEY});
  keymap_tick(actual_report);
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventPress, .value = KM_LAYERED_KEY});
  keymap_tick(actual_report);
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventRelease, .value = KM_LAYER_MOD_KEY});
  keymap_tick(actual_report);

  // assert: keypress resolved on layer 1 is retained after mod release
  TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_report, actual_report->keyboard, 8);
}

#else
#error "requires SUITE_LAYERED"
#endif
