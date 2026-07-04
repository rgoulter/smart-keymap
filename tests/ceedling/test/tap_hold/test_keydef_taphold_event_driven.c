#include "tap_hold_test_ceedling_fixture.h"

#ifdef SUITE_TAP_HOLD

#include "unity.h"

#include "hid_keycodes.h"
#include "smart_keymap.h"

void setUp(void) {}

void tearDown(void) {}

void test_event_driven_tap_hold_key_tap(void) {
  KeymapHidReport report = {};
  KeymapHidReport *actual_report = &report;

  // assemble: init keymap
  keymap_init();

  // act: press tap-hold key at t=0
  {
    uint32_t actual_next_ev_ms = keymap_register_input_after_ms(
        0,
        (struct KeymapInputEvent){.event_type = KeymapEventPress,
                                  .value = KM_TAP_HOLD_KEY},
        actual_report);

    // assert: hold timeout scheduled, no key output yet
    TEST_ASSERT_EQUAL_UINT32(200, actual_next_ev_ms);
    TEST_ASSERT_EQUAL_UINT8_ARRAY(((uint8_t[8]){0, 0, 0, 0, 0, 0, 0, 0}),
                                  actual_report->keyboard, 8);
  }

  // act: release tap-hold key at t=150
  {
    uint32_t actual_next_ev_ms = keymap_register_input_after_ms(
        150,
        (struct KeymapInputEvent){.event_type = KeymapEventRelease,
                                  .value = KM_TAP_HOLD_KEY},
        actual_report);

    // assert: tap fires immediately, next event in 50ms
    TEST_ASSERT_EQUAL_UINT32(50, actual_next_ev_ms);
    TEST_ASSERT_EQUAL_UINT8_ARRAY(((uint8_t[8]){0, 0, KC_C, 0, 0, 0, 0, 0}),
                                  actual_report->keyboard, 8);
  }
}

void test_event_driven_tap_hold_key_tap_release_reported(void) {
  KeymapHidReport report = {};
  KeymapHidReport *actual_report = &report;

  // assemble: init keymap
  keymap_init();

  // act: press tap-hold key at t=0
  {
    uint32_t actual_next_ev_ms = keymap_register_input_after_ms(
        0,
        (struct KeymapInputEvent){.event_type = KeymapEventPress,
                                  .value = KM_TAP_HOLD_KEY},
        actual_report);

    // assert: hold timeout scheduled, polling not required yet
    TEST_ASSERT_EQUAL_UINT32(200, actual_next_ev_ms);
    TEST_ASSERT_EQUAL_UINT8_ARRAY(((uint8_t[8]){0, 0, 0, 0, 0, 0, 0, 0}),
                                  actual_report->keyboard, 8);
    TEST_ASSERT_FALSE(keymap_requires_polling());
  }

  // act: release tap-hold key at t=150
  {
    uint32_t actual_next_ev_ms = keymap_register_input_after_ms(
        150,
        (struct KeymapInputEvent){.event_type = KeymapEventRelease,
                                  .value = KM_TAP_HOLD_KEY},
        actual_report);

    // assert: tap reported, polling required until tap clears
    TEST_ASSERT_EQUAL_UINT32(50, actual_next_ev_ms);
    TEST_ASSERT_EQUAL_UINT8_ARRAY(((uint8_t[8]){0, 0, KC_C, 0, 0, 0, 0, 0}),
                                  actual_report->keyboard, 8);
    TEST_ASSERT_TRUE(keymap_requires_polling());
  }

  // act: first tick after tap
  {
    keymap_tick(actual_report);

    // assert: tap still held, polling still required
    TEST_ASSERT_EQUAL_UINT8_ARRAY(((uint8_t[8]){0, 0, KC_C, 0, 0, 0, 0, 0}),
                                  actual_report->keyboard, 8);
    TEST_ASSERT_TRUE(keymap_requires_polling());
  }

  // act: second tick clears tap
  {
    keymap_tick(actual_report);

    // assert: report empty, polling no longer required
    TEST_ASSERT_EQUAL_UINT8_ARRAY(((uint8_t[8]){0, 0, 0, 0, 0, 0, 0, 0}),
                                  actual_report->keyboard, 8);
    TEST_ASSERT_FALSE(keymap_requires_polling());
  }
}

#else
#error "requires SUITE_TAP_HOLD"
#endif
