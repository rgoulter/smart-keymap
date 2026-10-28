#include "remap_named_layers_test_ceedling_fixture.h"

#ifdef SUITE_REMAP_NAMED_LAYERS

#include "unity.h"

#include "hid_keycodes.h"
#include "smart_keymap.h"

void setUp(void) {}

void tearDown(void) {}

void test_remapped_layered_key_acts_as_base_when_no_layer_active(void) {
  uint8_t expected_report[8] = {0, 0, KC_A, 0, 0, 0, 0, 0};
  KeymapHidReport report = {};
  KeymapHidReport *actual_report = &report;

  keymap_init();

  // act: press layered key (source index 1 → target index 2) with no layer mod
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventPress, .value = KM_LAYERED_KEY});
  keymap_tick(actual_report);

  TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_report, actual_report->keyboard, 8);
}

void test_remapped_named_layer_active_when_layer_mod_held(void) {
  uint8_t expected_report[8] = {0, 0, KC_B, 0, 0, 0, 0, 0};
  KeymapHidReport report = {};
  KeymapHidReport *actual_report = &report;

  keymap_init();

  // act: hold named-layer mod (source 0 → target 0), then layered key
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventPress, .value = KM_LAYER_MOD_KEY});
  keymap_tick(actual_report);
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventPress, .value = KM_LAYERED_KEY});
  keymap_tick(actual_report);

  TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_report, actual_report->keyboard, 8);
}

void test_remapped_gap_key_is_no_key(void) {
  uint8_t expected_report[8] = {0, 0, 0, 0, 0, 0, 0, 0};
  KeymapHidReport report = {};
  KeymapHidReport *actual_report = &report;

  keymap_init();

  // act: press the unmapped gap position inserted by from_layout
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventPress, .value = KM_NO_KEY});
  keymap_tick(actual_report);

  TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_report, actual_report->keyboard, 8);
}

#else
#error "requires SUITE_REMAP_NAMED_LAYERS"
#endif
