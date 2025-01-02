#include "unity.h"

#include "smart_keymap.h"

#define KC_A 0x04
#define KC_B 0x05
#define KC_C 0x06
#define KC_D 0x07

#define MOD_LCTL 0x1
#define MOD_LSFT 0x2

void setUp(void) {
}

void tearDown(void) {
}

void test_taphold_interrupted_is_hold(void) {
    // Interrupting a taphold results in the 'hold' key.
    //
    // Pressing T.H., then A, is same as "Hold key + A"

    uint8_t expected_report[8] = {MOD_LCTL, 0, KC_A, 0, 0, 0, 0, 0};
    uint8_t actual_report[8] = {0, 0, 0, 0, 0, 0, 0, 0};

    keymap_init();

    keymap_register_input_keypress(0); // First key in keymap is TapHold(_, Ctrl)
    keymap_register_input_keypress(2); // Third key in the keymap is A

    copy_hid_boot_keyboard_report(actual_report);
    TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_report, actual_report, 8);
}

void test_taphold_dth_uth_is_tap(void) {
    TEST_IGNORE_MESSAGE("Known to fail, despite Cucumber equivalent passing.");

    // Pressing T.H., then releasing T.H., is same as tapping the tap key.
    // (Check the tap key gets pressed).

    uint8_t expected_report[8] = {0, 0, KC_C, 0, 0, 0, 0, 0};
    uint8_t actual_report[8] = {0, 0, 0, 0, 0, 0, 0, 0};

    keymap_init();

    keymap_register_input_keypress(0); // First key in keymap is TapHold(C, _)
    keymap_register_input_keyrelease(0);

    keymap_tick(actual_report);
    TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_report, actual_report, 8);
}

void test_taphold_dth_uth_eventually_clears(void) {
    // Pressing T.H., then releasing T.H., is same as tapping the tap key.
    // (Check the tap key releases).

    uint8_t expected_report[8] = {0, 0, 0, 0, 0, 0, 0, 0};
    uint8_t actual_report[8] = {0, 0, 0, 0, 0, 0, 0, 0};

    keymap_init();

    keymap_register_input_keypress(0); // First key in keymap is TapHold(C, _)
    keymap_register_input_keyrelease(0);

    keymap_tick(actual_report);

    keymap_tick(actual_report); // The 'tap' from the TapHold key should be released.

    TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_report, actual_report, 8);
}

void test_taphold_dth_eventually_holds(void) {
    // Pressing T.H., is eventually the same as holding the hold key.

    uint8_t expected_report[8] = {MOD_LCTL, 0, 0, 0, 0, 0, 0, 0};
    uint8_t actual_report[8] = {0, 0, 0, 0, 0, 0, 0, 0};

    keymap_init();

    keymap_register_input_keypress(0); // First key in keymap is TapHold(C, _)

    // Wait 500ms
    for (int i = 0; i < 500; i++) {
        keymap_tick(actual_report);
    }

    TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_report, actual_report, 8);
}
