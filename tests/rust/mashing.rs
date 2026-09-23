use smart_keymap::input;
use smart_keymap::keymap::ObservedKeymap;

use crate::hid_keycodes::*;
use smart_keymap_macros::keymap;

/// Mashing many keys at once must not panic or corrupt HID.
///
/// The CH32X-60-improved board has 66 keys; a palm mashing every key
///  and a firmware starved of `keymap_tick` could overflow fixed-capacity
///  queues. The contract is "lots of input => ignore what can't handle".
#[test]
fn mashing_many_keys_does_not_panic_and_stays_bounded() {
    // Assemble -- 66 plain keys (A repeated for brevity, distinct keycodes irrelevant for overflow).
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            let keys_66 = std.array.replicate 66 K.A in
            {
                keys = keys_66,
            }
        "#
    ));

    // Act -- press all 66 without interleaving ticks (simulates ISR burst before USB drains).
    for idx in 0..66u16 {
        keymap.handle_input(input::Event::Press { keymap_index: idx });
    }

    // Drain with ticks; pending logic should pace one-per-tick.
    for _ in 0..80 {
        keymap.tick();
    }

    // Release many.
    for idx in 0..66u16 {
        keymap.handle_input(input::Event::Release { keymap_index: idx });
    }
    for _ in 0..80 {
        keymap.tick();
    }

    // Assert -- still produces a sane report (no panic), and reportable keys are bounded by MAX_PRESSED_KEYS.
    let report = keymap.boot_keyboard_report();
    // Report is 8 bytes, modifiers + 6 keycodes; should be valid.
    assert_eq!(8, report.len());
    // At most 6 non-zero keycodes in the boot report even if 16 pressed.
    let non_zero = report[2..].iter().filter(|&&kc| kc != 0).count();
    assert!(non_zero <= 6);
}

/// Pressing more than `MAX_PRESSED_KEYS` (16) plus a pending tap-hold
///  must not panic in `pressed_keys()` (previously used `collect()` which aborts on overflow).
#[test]
fn more_than_16_pressed_plus_pending_does_not_panic() {
    // Assemble -- first key is tap-hold (pending), rest are plain.
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                config.tap_hold.timeout = 200,
                keys = [
                    K.A & K.hold K.LeftCtrl,
                ] @ std.array.replicate 20 K.B,
            }
        "#
    ));

    // Act -- press tap-hold, then flood 16 more to exceed capacity.
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    // Need to pace: each handle_input may be delayed; interleave ticks to drain queue.
    for idx in 1..21u16 {
        keymap.handle_input(input::Event::Press { keymap_index: idx });
        keymap.tick();
    }
    // Let tap-hold timeout resolve to hold while many keys remain pressed.
    for _ in 0..210 {
        keymap.tick();
    }

    // Assert -- no panic, boot report bounded (pressed_keys truncation).
    let report = keymap.boot_keyboard_report();
    assert_eq!(8, report.len());
    let non_zero = report[2..].iter().filter(|&&kc| kc != 0).count();
    assert!(non_zero <= 6);
}

/// Input queue overflow must be silent (push_back_or_ignore) not panic.
///
/// Rapidly pushing 64 events into a 32-capacity queue should drop excess.
#[test]
fn input_queue_overflow_is_ignored_not_panic() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            {
                keys = [
                    { key_code = 4 },
                ],
            }
        "#
    ));

    // Act -- flood with alternating press/release without ticks.
    for i in 0..64u16 {
        let ev = if i % 2 == 0 {
            input::Event::Press { keymap_index: 0 }
        } else {
            input::Event::Release { keymap_index: 0 }
        };
        keymap.handle_input(ev);
    }
    keymap.tick_until_no_scheduled_events();

    // Assert -- still a valid report.
    let report = keymap.boot_keyboard_report();
    assert_eq!(8, report.len());
}

/// Out-of-range indices are ignored (C side also guards, but core should not abort if they slip through).
/// This simulates a corrupted split byte deserializing to 500.
#[test]
fn out_of_range_keymap_index_is_ignored() {
    // Assemble -- single key map.
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            {
                keys = [
                    { key_code = 4 },
                ],
            }
        "#
    ));

    // Act -- press valid, then out-of-range.
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 500 });
    keymap.handle_input(input::Event::Release { keymap_index: 500 });
    keymap.tick_until_no_scheduled_events();

    // Assert -- valid key still reports, no panic.
    let reports = keymap.distinct_reports();
    // Should have had at least one report with A.
    assert!(reports.reports().iter().any(|r| r[2] == KC_A));
}
