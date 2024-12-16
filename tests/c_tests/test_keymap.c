#include "unity.h"

#include "smart_keymap.h"

#define KC_A 0x04

void setUp(void) {
}

void tearDown(void) {
}

void test_copy_hid_boot_keyboard_report(void) {
    uint8_t expected_report[8] = {0, 0, 0, 0, 0, 0, 0, 0};

    uint8_t actual_report[8] = {1, 2, 3, 4, 5, 6, 7, 8};
    copy_hid_boot_keyboard_report(actual_report);
    
    TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_report, actual_report, 8);
}

// KEYMAP: [A, A, A]

void test_simple_keypress(void) {
    uint8_t expected_report[8] = {0, 0, KC_A, 0, 0, 0, 0, 0};
    uint8_t actual_report[8] = {0, 0, 0, 0, 0, 0, 0, 0};

    keymap_init();

    keymap_register_input_keypress(2); // Third key in the keymap is A

    copy_hid_boot_keyboard_report(actual_report);

    TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_report, actual_report, 8);
}

void test_simple_keyrelease(void) {
    uint8_t expected_report[8] = {0, 0, 0, 0, 0, 0, 0, 0};
    uint8_t actual_report[8] = {0, 0, 0, 0, 0, 0, 0, 0};

    keymap_init();

    keymap_register_input_keypress(2);
    keymap_register_input_keyrelease(2);

    copy_hid_boot_keyboard_report(actual_report);

    TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_report, actual_report, 8);
}

int main(void) {
    UNITY_BEGIN();
    
    RUN_TEST(test_copy_hid_boot_keyboard_report);

    RUN_TEST(test_simple_keypress);
    RUN_TEST(test_simple_keyrelease);

    return UNITY_END();
}
