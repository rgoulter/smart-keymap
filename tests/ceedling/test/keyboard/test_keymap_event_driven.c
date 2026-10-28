#include "keyboard_test_ceedling_fixture.h"

#ifdef SUITE_KEYBOARD

#include "unity.h"

#include "hid_keycodes.h"
#include "smart_keymap.h"

void setUp(void) {}

void tearDown(void) {}

void test_event_driven_key_press(void) {
  uint8_t expected_report[8] = {0, 0, KC_A, 0, 0, 0, 0, 0};
  KeymapHidReport report = {};
  KeymapHidReport *actual_report = &report;

  keymap_init();

  keymap_register_input_after_ms(
      0,
      (struct KeymapInputEvent){.event_type = KeymapEventPress,
                                .value = KM_KEY_A},
      actual_report);

  TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_report, actual_report->keyboard, 8);
}

void test_event_driven_key_tap(void) {
  uint8_t expected_report[8] = {0, 0, 0, 0, 0, 0, 0, 0};
  KeymapHidReport report = {};
  KeymapHidReport *actual_report = &report;

  // assemble: init keymap
  keymap_init();

  // act: press then release A via event-driven API
  keymap_register_input_after_ms(
      0,
      (struct KeymapInputEvent){.event_type = KeymapEventPress,
                                .value = KM_KEY_A},
      actual_report);
  keymap_register_input_after_ms(
      0,
      (struct KeymapInputEvent){.event_type = KeymapEventRelease,
                                .value = KM_KEY_A},
      actual_report);

  // assert: empty report
  TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_report, actual_report->keyboard, 8);
}

#else
#error "requires SUITE_KEYBOARD"
#endif
