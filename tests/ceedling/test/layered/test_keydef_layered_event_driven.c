#include "layered_test_ceedling_fixture.h"

#ifdef SUITE_LAYERED

#include "unity.h"

#include "hid_keycodes.h"
#include "smart_keymap.h"

void setUp(void) {}

void tearDown(void) {}

void test_event_driven_layered_key_with_layer_mod_held(void) {
  uint8_t expected_report[8] = {0, 0, KC_B, 0, 0, 0, 0, 0};
  KeymapHidReport report = {};
  KeymapHidReport *actual_report = &report;

  // assemble: init keymap
  keymap_init();

  // act: hold layer modifier and press layered key via event-driven API
  keymap_register_input_after_ms(
      0,
      (struct KeymapInputEvent){.event_type = KeymapEventPress,
                                .value = KM_LAYER_MOD_KEY},
      actual_report);
  keymap_register_input_after_ms(
      0,
      (struct KeymapInputEvent){.event_type = KeymapEventPress,
                                .value = KM_LAYERED_KEY},
      actual_report);

  // assert: active-layer key (B) in report
  TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_report, actual_report->keyboard, 8);
}

#else
#error "requires SUITE_LAYERED"
#endif
