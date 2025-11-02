#pragma once
#ifdef KEYBOARD_SPLIT
#include "generated/keyboard_split.h" // IWYU pragma: export
#include "smart_keymap.h"
void keyboard_split_init(void);
int keyboard_split_write(KeymapInputEvent ev);
#endif
