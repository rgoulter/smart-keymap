#include "conditional_layers_test_ceedling_fixture.h"

#ifdef SUITE_CONDITIONAL_LAYERS

#include "unity.h"

#include "hid_keycodes.h"
#include "smart_keymap.h"

void setUp(void) {}

void tearDown(void) {}

void test_conditional_layer_letter_is_base_when_no_mods_held(void) {
  // Assemble
  uint8_t expected_report[8] = {0, 0, KC_A, 0, 0, 0, 0, 0};
  KeymapHidReport report = {};
  KeymapHidReport *actual_report = &report;
  keymap_init();

  // Act
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventPress, .value = KM_LETTER_KEY});
  keymap_tick(actual_report);

  // Assert
  TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_report, actual_report->keyboard, 8);
}

void test_conditional_layer_lower_alone_does_not_activate_adjust(void) {
  // Assemble
  uint8_t expected_report[8] = {0, 0, KC_B, 0, 0, 0, 0, 0};
  KeymapHidReport report = {};
  KeymapHidReport *actual_report = &report;
  keymap_init();

  // Act
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventPress, .value = KM_LOWER_KEY});
  keymap_tick(actual_report);
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventPress, .value = KM_LETTER_KEY});
  keymap_tick(actual_report);

  // Assert
  TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_report, actual_report->keyboard, 8);
}

void test_conditional_layer_raise_alone_does_not_activate_adjust(void) {
  // Assemble
  uint8_t expected_report[8] = {0, 0, KC_C, 0, 0, 0, 0, 0};
  KeymapHidReport report = {};
  KeymapHidReport *actual_report = &report;
  keymap_init();

  // Act
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventPress, .value = KM_RAISE_KEY});
  keymap_tick(actual_report);
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventPress, .value = KM_LETTER_KEY});
  keymap_tick(actual_report);

  // Assert
  TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_report, actual_report->keyboard, 8);
}

void test_conditional_layer_lower_and_raise_activate_adjust(void) {
  // Assemble
  uint8_t expected_report[8] = {0, 0, KC_D, 0, 0, 0, 0, 0};
  KeymapHidReport report = {};
  KeymapHidReport *actual_report = &report;
  keymap_init();

  // Act
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventPress, .value = KM_LOWER_KEY});
  keymap_tick(actual_report);
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventPress, .value = KM_RAISE_KEY});
  keymap_tick(actual_report);
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventPress, .value = KM_LETTER_KEY});
  keymap_tick(actual_report);

  // Assert
  TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_report, actual_report->keyboard, 8);
}

void test_conditional_layer_releasing_if_layer_deactivates_adjust(void) {
  // Assemble
  uint8_t expected_report[8] = {0, 0, KC_C, 0, 0, 0, 0, 0};
  KeymapHidReport report = {};
  KeymapHidReport *actual_report = &report;
  keymap_init();
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventPress, .value = KM_LOWER_KEY});
  keymap_tick(actual_report);
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventPress, .value = KM_RAISE_KEY});
  keymap_tick(actual_report);

  // Act
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventRelease, .value = KM_LOWER_KEY});
  keymap_tick(actual_report);
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventPress, .value = KM_LETTER_KEY});
  keymap_tick(actual_report);

  // Assert
  TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_report, actual_report->keyboard, 8);
}

#else
#error "requires SUITE_CONDITIONAL_LAYERS"
#endif
