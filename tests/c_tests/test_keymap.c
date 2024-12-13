#include "unity.h"

#include "smart_keymap.h"

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

int main(void) {
    UNITY_BEGIN();
    
    RUN_TEST(test_copy_hid_boot_keyboard_report);

    return UNITY_END();
}
