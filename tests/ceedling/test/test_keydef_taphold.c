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
    // Pressing T.H, then A, is same as "Hold key + A"

    uint8_t expected_report[8] = {MOD_LCTL, 0, KC_A, 0, 0, 0, 0, 0};
    uint8_t actual_report[8] = {0, 0, 0, 0, 0, 0, 0, 0};

    keymap_init();

    keymap_register_input_keypress(0); // First key in keymap is TapHold(_, Ctrl)
    keymap_register_input_keypress(2); // Third key in the keymap is A

    copy_hid_boot_keyboard_report(actual_report);
    TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_report, actual_report, 8);
}
