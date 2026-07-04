#include "callback_test_ceedling_fixture.h"

#ifdef SUITE_CALLBACK

#include "unity.h"

#include "hid_keycodes.h"
#include "smart_keymap.h"

static int bootloader_callback_count;
static int custom_callback_count;

static void bootloader_callback(void) { bootloader_callback_count++; }

static void custom_callback(void) { custom_callback_count++; }

void setUp(void) {
  bootloader_callback_count = 0;
  custom_callback_count = 0;
  keymap_init();
  keymap_clear_callbacks();
}

void tearDown(void) {}

void test_bootloader_callback_invoked_on_press(void) {
  uint8_t expected_report[8] = {0, 0, 0, 0, 0, 0, 0, 0};
  KeymapHidReport report = {};
  KeymapHidReport *actual_report = &report;

  keymap_register_callback(KEYMAP_CALLBACK_BOOTLOADER, bootloader_callback);

  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventPress, .value = KM_BOOTLOADER_KEY});
  keymap_tick(actual_report);

  TEST_ASSERT_EQUAL_INT(1, bootloader_callback_count);
  TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_report, actual_report->keyboard, 8);
}

void test_custom_callback_invoked_on_press(void) {
  uint8_t expected_report[8] = {0, 0, 0, 0, 0, 0, 0, 0};
  KeymapHidReport report = {};
  KeymapHidReport *actual_report = &report;

  keymap_register_custom_callback(3, 4, custom_callback);

  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventPress, .value = KM_CUSTOM_KEY});
  keymap_tick(actual_report);

  TEST_ASSERT_EQUAL_INT(1, custom_callback_count);
  TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_report, actual_report->keyboard, 8);
}

#else
#error "requires SUITE_CALLBACK"
#endif
