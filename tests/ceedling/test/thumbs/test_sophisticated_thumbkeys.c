#include "thumbs_test_ceedling_fixture.h"

#ifdef SUITE_THUMBS

#include "hid_keycodes.h"
#include "smart_keymap.h"
#include "unity.h"

void setUp(void) {}

void tearDown(void) {}

static void press_key(uint16_t key) {
  keymap_register_input_event(
      (struct KeymapInputEvent){.event_type = KeymapEventPress, .value = key});
}

static void release_key(uint16_t key) {
  keymap_register_input_event((struct KeymapInputEvent){
      .event_type = KeymapEventRelease, .value = key});
}

static void tick_n(KeymapHidReport *report, int n) {
  for (int i = 0; i < n; i++) {
    keymap_tick(report);
  }
}

static void assert_kc(const KeymapHidReport *report, uint8_t kc) {
  uint8_t expected[8] = {0, 0, kc, 0, 0, 0, 0, 0};
  TEST_ASSERT_EQUAL_UINT8_ARRAY(expected, report->keyboard, 8);
}

static void tick_until_kc(KeymapHidReport *report, uint8_t kc, int max_ticks) {
  for (int i = 0; i < max_ticks; i++) {
    if (report->keyboard[2] == kc) {
      return;
    }
    keymap_tick(report);
  }
  assert_kc(report, kc);
}

static void tap_thumb(uint16_t thumb, KeymapHidReport *report) {
  press_key(thumb);
  keymap_tick(report);
  release_key(thumb);
  /* One tick resolves the tap; more ticks clear it from the report. */
  keymap_tick(report);
}

static void idle_then_hold_probe(uint16_t thumb, KeymapHidReport *report) {
  tick_n(report, 2000);
  press_key(thumb);
  /* Chord wait (200) + tap-hold timeout (200), plus pacing slack. */
  tick_n(report, 500);
  press_key(KM_PROBE_KEY);
  tick_n(report, 20);
}

void test_thumb_tab_taps(void) {
  KeymapHidReport report = {};

  // assemble
  keymap_init();

  // act: single tap of Tab/mouse thumb
  tap_thumb(KM_TAB_MOUR, &report);

  // assert: tap is Tab, not the mouse layer
  assert_kc(&report, KC_TAB);
}

void test_thumb_esc_taps(void) {
  KeymapHidReport report = {};

  // assemble
  keymap_init();

  // act: single tap of Esc/media thumb
  tap_thumb(KM_ESC_MEDR, &report);

  // assert: tap is Escape, not the media layer
  assert_kc(&report, KC_ESCAPE);
}

void test_thumb_spc_taps(void) {
  KeymapHidReport report = {};

  // assemble
  keymap_init();

  // act: single tap of Space/nav thumb
  tap_thumb(KM_SPC_NAVR, &report);

  // assert: tap is Space, not the nav layer
  assert_kc(&report, KC_SPACE);
}

void test_thumb_ent_taps(void) {
  KeymapHidReport report = {};

  // assemble
  keymap_init();

  // act: single tap of Enter/sym thumb
  tap_thumb(KM_ENT_NSSL, &report);

  // assert: tap is Return, not the sym layer
  assert_kc(&report, KC_RETURN);
}

void test_thumb_bksp_taps(void) {
  KeymapHidReport report = {};

  // assemble
  keymap_init();

  // act: single tap of Backspace/num thumb
  tap_thumb(KM_BKSP_NSL, &report);

  // assert: tap is Backspace, not the num layer
  assert_kc(&report, KC_BACKSPACE);
}

void test_thumb_del_taps(void) {
  KeymapHidReport report = {};

  // assemble
  keymap_init();

  // act: single tap of Delete/fun thumb
  tap_thumb(KM_DEL_FUNL, &report);

  // assert: tap is Delete, not the fun layer
  assert_kc(&report, KC_DELETE);
}

void test_thumb_tab_hold_activates_mou(void) {
  KeymapHidReport report = {};

  // assemble
  keymap_init();

  // act: hold Tab/mouse past timeout, then press the probe key
  idle_then_hold_probe(KM_TAB_MOUR, &report);

  // assert: hold is the mouse layer (probe is B)
  assert_kc(&report, KC_B);
}

void test_thumb_esc_hold_activates_med(void) {
  KeymapHidReport report = {};

  // assemble
  keymap_init();

  // act: hold Esc/media past timeout, then press the probe key
  idle_then_hold_probe(KM_ESC_MEDR, &report);

  // assert: hold is the media layer (probe is C)
  assert_kc(&report, KC_C);
}

void test_thumb_spc_hold_activates_nav(void) {
  KeymapHidReport report = {};

  // assemble
  keymap_init();

  // act: hold Space/nav past timeout, then press the probe key
  idle_then_hold_probe(KM_SPC_NAVR, &report);

  // assert: hold is the nav layer (probe is Left)
  assert_kc(&report, KC_LEFT);
}

void test_thumb_ent_hold_activates_sym(void) {
  KeymapHidReport report = {};

  // assemble
  keymap_init();

  // act: hold Enter/sym past timeout, then press the probe key
  idle_then_hold_probe(KM_ENT_NSSL, &report);

  // assert: hold is the sym layer (probe is D)
  assert_kc(&report, KC_D);
}

void test_thumb_bksp_hold_activates_num(void) {
  KeymapHidReport report = {};

  // assemble
  keymap_init();

  // act: hold Backspace/num past timeout, then press the probe key
  idle_then_hold_probe(KM_BKSP_NSL, &report);

  // assert: hold is the num layer (probe is 1)
  assert_kc(&report, KC_N1);
}

void test_thumb_del_hold_activates_fun(void) {
  KeymapHidReport report = {};

  // assemble
  keymap_init();

  // act: hold Delete/fun past timeout, then press the probe key
  idle_then_hold_probe(KM_DEL_FUNL, &report);

  // assert: hold is the fun layer (probe is F1)
  assert_kc(&report, KC_F1);
}

void test_thumb_bksp_interrupt_tap_resolves_hold(void) {
  KeymapHidReport report = {};

  // assemble
  keymap_init();

  // act: press Backspace/num, then tap the probe key (HoldOnKeyTap)
  press_key(KM_BKSP_NSL);
  keymap_tick(&report);
  press_key(KM_PROBE_KEY);
  keymap_tick(&report);
  release_key(KM_PROBE_KEY);
  tick_until_kc(&report, KC_N1, 50);

  // assert: interrupting tap resolves the thumb as hold; probe is 1 from num
  assert_kc(&report, KC_N1);
}

void test_chord_esc_spc_taps_tab(void) {
  KeymapHidReport report = {};

  // assemble
  keymap_init();
  tick_n(&report, 2000);

  // act: tap the Esc+Space chord (press both, release both)
  press_key(KM_ESC_MEDR);
  keymap_tick(&report);
  press_key(KM_SPC_NAVR);
  keymap_tick(&report);
  release_key(KM_ESC_MEDR);
  keymap_tick(&report);
  release_key(KM_SPC_NAVR);
  tick_until_kc(&report, KC_TAB, 50);

  // assert: chord tap is Tab, not Esc or Space
  assert_kc(&report, KC_TAB);
}

void test_chord_ent_bksp_taps_delete(void) {
  KeymapHidReport report = {};

  // assemble
  keymap_init();
  tick_n(&report, 2000);

  // act: tap the Enter+Backspace chord (press both, release both)
  press_key(KM_ENT_NSSL);
  keymap_tick(&report);
  press_key(KM_BKSP_NSL);
  keymap_tick(&report);
  release_key(KM_ENT_NSSL);
  keymap_tick(&report);
  release_key(KM_BKSP_NSL);
  tick_until_kc(&report, KC_DELETE, 50);

  // assert: chord tap is Delete, not Enter or Backspace
  assert_kc(&report, KC_DELETE);
}

void test_chord_esc_spc_hold_activates_mou(void) {
  KeymapHidReport report = {};

  // assemble
  keymap_init();
  tick_n(&report, 2000);

  // act: hold the Esc+Space chord past timeout, then press the probe key
  press_key(KM_ESC_MEDR);
  keymap_tick(&report);
  press_key(KM_SPC_NAVR);
  tick_n(&report, 500);
  press_key(KM_PROBE_KEY);
  tick_n(&report, 20);

  // assert: chord hold is the mouse layer (probe is B)
  assert_kc(&report, KC_B);
}

void test_chord_ent_bksp_hold_activates_fun(void) {
  KeymapHidReport report = {};

  // assemble
  keymap_init();
  tick_n(&report, 2000);

  // act: hold the Enter+Backspace chord past timeout, then press the probe key
  press_key(KM_ENT_NSSL);
  keymap_tick(&report);
  press_key(KM_BKSP_NSL);
  tick_n(&report, 500);
  press_key(KM_PROBE_KEY);
  tick_n(&report, 20);

  // assert: chord hold is the fun layer (probe is F1)
  assert_kc(&report, KC_F1);
}

#else
#error "requires SUITE_THUMBS"
#endif
