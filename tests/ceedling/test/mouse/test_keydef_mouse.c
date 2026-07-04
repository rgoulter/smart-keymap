#include "mouse_test_ceedling_fixture.h"

#ifdef SUITE_MOUSE

#include "unity.h"

#include "hid_keycodes.h"
#include "smart_keymap.h"

void setUp(void) {}

void tearDown(void) {}

void test_mouse_button_press_reports_in_mouse_field(void) {
  uint8_t expected_keyboard[8] = {0, 0, 0, 0, 0, 0, 0, 0};
  KeymapHidReport report = {};
  KeymapHidReport *actual_report = &report;

  // assemble: init keymap
  keymap_init();

  // act: press mouse button 1
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventPress, .value = KM_MOUSE_BTN1_KEY});
  keymap_tick(actual_report);

  // assert: button state in mouse report; keyboard[] unused
  TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_keyboard, actual_report->keyboard, 8);
  TEST_ASSERT_EQUAL_UINT8(1, actual_report->mouse.pressed_buttons);
  TEST_ASSERT_EQUAL_INT8(0, actual_report->mouse.x);
  TEST_ASSERT_EQUAL_INT8(0, actual_report->mouse.y);
  TEST_ASSERT_EQUAL_INT8(0, actual_report->mouse.vertical_scroll);
  TEST_ASSERT_EQUAL_INT8(0, actual_report->mouse.horizontal_scroll);
}

void test_mouse_button_release_clears_mouse_field(void) {
  uint8_t expected_keyboard[8] = {0, 0, 0, 0, 0, 0, 0, 0};
  KeymapHidReport report = {};
  KeymapHidReport *actual_report = &report;

  // assemble: init keymap
  keymap_init();

  // act: press then release mouse button 1
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventPress, .value = KM_MOUSE_BTN1_KEY});
  keymap_tick(actual_report);
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventRelease, .value = KM_MOUSE_BTN1_KEY});
  keymap_tick(actual_report);

  // assert: mouse report cleared
  TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_keyboard, actual_report->keyboard, 8);
  TEST_ASSERT_EQUAL_UINT8(0, actual_report->mouse.pressed_buttons);
  TEST_ASSERT_EQUAL_INT8(0, actual_report->mouse.x);
  TEST_ASSERT_EQUAL_INT8(0, actual_report->mouse.y);
  TEST_ASSERT_EQUAL_INT8(0, actual_report->mouse.vertical_scroll);
  TEST_ASSERT_EQUAL_INT8(0, actual_report->mouse.horizontal_scroll);
}

void test_mouse_keys_combine_in_mouse_field(void) {
  uint8_t expected_keyboard[8] = {0, 0, 0, 0, 0, 0, 0, 0};
  KeymapHidReport report = {};
  KeymapHidReport *actual_report = &report;

  // assemble: init keymap
  keymap_init();

  // act: press button, cursor-left, and wheel-up keys together
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventPress, .value = KM_MOUSE_BTN1_KEY});
  keymap_tick(actual_report);
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventPress, .value = KM_MOUSE_LEFT_KEY});
  keymap_tick(actual_report);
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventPress, .value = KM_MOUSE_WHEEL_UP_KEY});
  keymap_tick(actual_report);

  // assert: button, movement, and scroll accumulate in one mouse report
  TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_keyboard, actual_report->keyboard, 8);
  TEST_ASSERT_EQUAL_UINT8(1, actual_report->mouse.pressed_buttons);
  TEST_ASSERT_EQUAL_INT8(-5, actual_report->mouse.x);
  TEST_ASSERT_EQUAL_INT8(0, actual_report->mouse.y);
  TEST_ASSERT_EQUAL_INT8(1, actual_report->mouse.vertical_scroll);
  TEST_ASSERT_EQUAL_INT8(0, actual_report->mouse.horizontal_scroll);
}

#else
#error "requires SUITE_MOUSE"
#endif
