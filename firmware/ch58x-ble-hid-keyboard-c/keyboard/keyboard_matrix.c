#include "keyboard_matrix.h"

#include <stdint.h>

#include "smart_keymap.h"

// defined in matrix implementation
void keyboard_matrix_scan_raw(bool scan_buf[KEYBOARD_MATRIX_KEY_COUNT]);

void key_state_changed(uint32_t index, bool new_state) {
  KeymapInputEvent ev = {.event_type = 0, .value = index};
  if (new_state) {
    ev.event_type = KeymapEventPress;
  } else {
    ev.event_type = KeymapEventRelease;
  }
  keymap_register_input_event(ev);

#ifdef KEYBOARD_SPLIT
  keyboard_split_write(ev);
#endif
}

void keyboard_matrix_scan(void) {
  static bool debounced_state[KEYBOARD_MATRIX_KEY_COUNT] = {false};
  static bool previous_raw_scan[KEYBOARD_MATRIX_KEY_COUNT] = {false};
  static bool current_raw_scan[KEYBOARD_MATRIX_KEY_COUNT] = {false};
  static uint8_t debounce_counter[KEYBOARD_MATRIX_KEY_COUNT] = {0};

  keyboard_matrix_scan_raw(current_raw_scan);

  for (uint32_t i = 0; i < KEYBOARD_MATRIX_KEY_COUNT; i++) {
    if (current_raw_scan[i] == debounced_state[i]) {
      debounce_counter[i] = 0;
    } else {
      if (current_raw_scan[i] != previous_raw_scan[i]) {
        debounce_counter[i] = 0;
      } else {
        debounce_counter[i]++;
      }

      if (debounce_counter[i] >= 5) {
        key_state_changed(i, current_raw_scan[i]);
        debounced_state[i] = current_raw_scan[i];
      }
    }

    previous_raw_scan[i] = current_raw_scan[i];
  }
}
