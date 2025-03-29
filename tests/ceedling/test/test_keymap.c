#include "unity.h"

#include "smart_keymap.h"

#define KC_A 0x04
#define KC_B 0x05

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

// KEYMAP: [A, A, A, B]

void test_keyboard_keypress(void) {
    uint8_t expected_report[8] = {0, 0, KC_A, 0, 0, 0, 0, 0};
    uint8_t actual_report[8] = {0, 0, 0, 0, 0, 0, 0, 0};

    keymap_init();

    keymap_register_input_keypress(2); // Third key in the keymap is A

    copy_hid_boot_keyboard_report(actual_report);

    TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_report, actual_report, 8);
}

void test_keyboard_keyrelease(void) {
    uint8_t expected_report[8] = {0, 0, 0, 0, 0, 0, 0, 0};
    uint8_t actual_report[8] = {0, 0, 0, 0, 0, 0, 0, 0};

    keymap_init();

    keymap_register_input_keypress(2);
    keymap_tick(actual_report);
    keymap_register_input_keyrelease(2);

    copy_hid_boot_keyboard_report(actual_report);

    TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_report, actual_report, 8);
}

void test_keyboard_keypress_sequence_da_db(void) {
    // Pressing A, then B, should report "A B"

    uint8_t expected_report[8] = {0, 0, KC_A, KC_B, 0, 0, 0, 0};
    uint8_t actual_report[8] = {0, 0, 0, 0, 0, 0, 0, 0};

    keymap_init();

    keymap_register_input_keypress(2); // Third key in the keymap is A
    keymap_tick(actual_report);
    keymap_register_input_keypress(3); // Fourth key in the keymap is B

    copy_hid_boot_keyboard_report(actual_report);
    TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_report, actual_report, 8);
}

void test_keyboard_keypress_sequence_db_da(void) {
    // Pressing B, then A, should report "B A"

    uint8_t expected_report[8] = {0, 0, KC_B, KC_A, 0, 0, 0, 0};
    uint8_t actual_report[8] = {0, 0, 0, 0, 0, 0, 0, 0};

    keymap_init();

    keymap_register_input_keypress(3); // Fourth key in the keymap is B
    keymap_tick(actual_report);
    keymap_register_input_keypress(2); // Third key in the keymap is A

    copy_hid_boot_keyboard_report(actual_report);
    TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_report, actual_report, 8);
}

void test_keyboard_keypress_sequence_da_db_ub(void) {
    // Pressing A, then B; then releasing B, should report "A"

    uint8_t expected_report[8] = {0, 0, KC_A, 0, 0, 0, 0, 0};
    uint8_t actual_report[8] = {0, 0, 0, 0, 0, 0, 0, 0};

    keymap_init();

    keymap_register_input_keypress(2); // Third key in the keymap is A
    keymap_tick(actual_report);
    keymap_register_input_keypress(3); // Fourth key in the keymap is B
    keymap_tick(actual_report);
    keymap_register_input_keyrelease(3);

    copy_hid_boot_keyboard_report(actual_report);
    TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_report, actual_report, 8);
}

void test_keyboard_keypress_sequence_da_db_ua(void) {
    // Pressing A, then B; then releasing A, should report "B"

    uint8_t expected_report[8] = {0, 0, KC_B, 0, 0, 0, 0, 0};
    uint8_t actual_report[8] = {0, 0, 0, 0, 0, 0, 0, 0};

    keymap_init();

    keymap_register_input_keypress(2); // Third key in the keymap is A
    keymap_tick(actual_report);
    keymap_register_input_keypress(3); // Fourth key in the keymap is B
    keymap_tick(actual_report);
    keymap_register_input_keyrelease(2);

    copy_hid_boot_keyboard_report(actual_report);
    TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_report, actual_report, 8);
}

void test_keyboard_keypress_after_keyrelease(void) {
    uint8_t expected_report[8] = {0, 0, KC_A, 0, 0, 0, 0, 0};
    uint8_t actual_report[8] = {0, 0, 0, 0, 0, 0, 0, 0};

    keymap_init();

    keymap_register_input_keyrelease(2);
    keymap_tick(actual_report);
    keymap_register_input_keyrelease(2);
    keymap_tick(actual_report);
    keymap_register_input_keypress(2); // Third key in the keymap is A

    copy_hid_boot_keyboard_report(actual_report);

    TEST_ASSERT_EQUAL_UINT8_ARRAY(expected_report, actual_report, 8);
}
