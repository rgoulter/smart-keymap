//! Keymap tests that need the full-profile Vec key_system.
//!
//! These used to live in `smart_keymap::keymap` unit tests. They moved here so
//! `smart-keymap` no longer codegen/includes the full composite shell.

use smart_keymap::input;
use smart_keymap::key;
use smart_keymap::keymap::MAX_PRESSED_KEYS;

fn tap_hold_interrupt_keymap(
    interrupt_response: smart_keymap::key::tap_hold::InterruptResponse,
) -> smart_keymap::keymap::Keymap<
    [smart_keymap_full_system_std::key_system::Ref; 2],
    smart_keymap_full_system_std::key_system::Ref,
    smart_keymap_full_system_std::key_system::Context,
    smart_keymap_full_system_std::key_system::Event,
    smart_keymap_full_system_std::key_system::PendingKeyState,
    smart_keymap_full_system_std::key_system::KeyState,
    smart_keymap_full_system_std::key_system::System,
> {
    use smart_keymap_full_system_std::key_system;

    let mut config = key_system::Config::new();
    config.tap_hold.interrupt_response = interrupt_response;

    smart_keymap::keymap::Keymap::new(
        [
            key_system::Ref::TapHold(smart_keymap::key::tap_hold::Ref(0)),
            key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(0x05)),
        ],
        key_system::Context::from_config(config),
        key_system::System::new(
            smart_keymap::key::automation::System::new(Vec::new()),
            smart_keymap::key::callback::System::new(Vec::new()),
            smart_keymap::key::chorded::System::new(Vec::new(), Vec::new()),
            smart_keymap::key::consumer::System::new(Vec::new()),
            smart_keymap::key::keyboard::System::new(vec![smart_keymap::key::keyboard::Key {
                key_code: 0x05,
                modifiers: smart_keymap::key::KeyboardModifiers::new(),
            }]),
            smart_keymap::key::layered::System::new(Vec::new(), Vec::new()),
            smart_keymap::key::mod_conditioned::System::new(Vec::new()),
            smart_keymap::key::mouse::System::new(Vec::new()),
            smart_keymap::key::sequence::System::new(Vec::new(), Vec::new()),
            smart_keymap::key::sticky::System::new(Vec::new()),
            smart_keymap::key::tap_dance::System::new(Vec::new()),
            smart_keymap::key::tap_hold::System::new(vec![smart_keymap::key::tap_hold::Key::new(
                key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(0x04)),
                key_system::Ref::Keyboard(smart_keymap::key::keyboard::Ref::KeyCode(0xE0)),
            )]),
        ),
    )
}

macro_rules! simple_keyboard_keymap {
    () => {{
        use smart_keymap_full_system_std::key_system;

        use key_system::Context;
        use key_system::Ref;
        const KEY_COUNT: usize = 1;
        const KEY_REFS: [Ref; KEY_COUNT] = [key_system::Ref::Keyboard(
            smart_keymap::key::keyboard::Ref::KeyCode(0x04),
        )];
        const CONTEXT: Context = Context::from_config(key_system::Config::new());

        smart_keymap::keymap::Keymap::new(
            KEY_REFS,
            CONTEXT,
            key_system::System::new(
                smart_keymap::key::automation::System::new(Vec::new()),
                smart_keymap::key::callback::System::new(Vec::new()),
                smart_keymap::key::chorded::System::new(Vec::new(), Vec::new()),
                smart_keymap::key::consumer::System::new(Vec::new()),
                smart_keymap::key::keyboard::System::new(Vec::new()),
                smart_keymap::key::layered::System::new(Vec::new(), Vec::new()),
                smart_keymap::key::mod_conditioned::System::new(Vec::new()),
                smart_keymap::key::mouse::System::new(Vec::new()),
                smart_keymap::key::sequence::System::new(Vec::new(), Vec::new()),
                smart_keymap::key::sticky::System::new(Vec::new()),
                smart_keymap::key::tap_dance::System::new(Vec::new()),
                smart_keymap::key::tap_hold::System::new(Vec::new()),
            ),
        )
    }};
}

fn tap_hold_timeout_event() -> key::Event<smart_keymap_full_system_std::key_system::Event> {
    key::Event::Key {
        keymap_index: 0,
        key_event: smart_keymap_full_system_std::key_system::Event::TapHold(
            smart_keymap::key::tap_hold::Event::TapHoldTimeout,
        ),
    }
}

/// `queued_events` gets one entry per processed input;
///  tick delay defers the second physical `handle_input` until `tick()`.
///
/// Motivating smart key: **tap-hold** (control for #578).
#[test]
fn physical_input_during_pending_records_once_in_queued_events() {
    use smart_keymap::key::tap_hold::InterruptResponse;

    // Assemble
    let mut keymap = tap_hold_interrupt_keymap(InterruptResponse::Ignore);
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    let baseline = keymap
        .test_pending_queued_events_len()
        .expect("tap-hold pending");

    // Act -- deferred interrupt press, then pace it in.
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    assert_eq!(Some(baseline), keymap.test_pending_queued_events_len());
    // First tick clears delay; second tick dequeues the deferred press.
    keymap.tick();
    keymap.tick();

    // Assert
    assert_eq!(Some(baseline + 1), keymap.test_pending_queued_events_len());
}

/// Scheduled `Event::Input` during pending:
///  `handle_event` calls `update_pending_state` then `process_input`,
///  which applies pending state again.
/// With `HoldOnKeyPress`,
///  the interrupt should resolve the tap-hold to hold
///  without also pressing the interrupting key.
///
/// Motivating smart key: **tap-hold** (`HoldOnKeyPress` home-row mods; #578).
#[test]
fn scheduled_input_during_pending_does_not_reprocess_as_physical_press() {
    use smart_keymap::key::tap_hold::InterruptResponse;

    // Assemble
    let mut keymap = tap_hold_interrupt_keymap(InterruptResponse::HoldOnKeyPress);
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    assert!(keymap.test_pending_queued_events_len().is_some());

    // Act
    keymap.test_handle_scheduled_key_event(key::Event::Input(input::Event::Press {
        keymap_index: 1,
    }));

    // Assert -- hold only; interrupt key not pressed (#578).
    let hold = key::KeyOutput::from_key_code(0xE0);
    let interrupt_key = key::KeyOutput::from_key_code(0x05);
    assert_eq!(
        heapless::Vec::<key::KeyOutput, { MAX_PRESSED_KEYS }>::from_slice(&[hold]).unwrap(),
        keymap.pressed_keys()
    );
    assert!(!keymap.pressed_keys().contains(&interrupt_key));
}

/// Creating a pending key sets the delay
///  so the *next* physical input is deferred.
///
/// Motivating smart key: **tap-hold**
///  (`key_uninterrupted_tap_is_reported` / interrupt pacing).
#[test]
fn pending_creation_defers_next_physical_input_by_one_delay() {
    use smart_keymap::key::tap_hold::InterruptResponse;

    // Assemble
    let mut keymap = tap_hold_interrupt_keymap(InterruptResponse::Ignore);
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    assert!(keymap.test_is_pending());
    assert!(keymap.test_input_queue_delay());

    // Act -- interrupt without waiting for ticks.
    keymap.handle_input(input::Event::Press { keymap_index: 1 });

    // Assert -- still only in the delay line, not the session log.
    assert_eq!(1, keymap.test_input_queue_len());
    assert_eq!(Some(0), keymap.test_pending_queued_events_len());
    assert!(keymap.pressed_keys().is_empty());
}

/// When a press creates pending while later inputs are already
///  sitting in the global queue, those leftovers move into the new
///  pending delay line.
///
/// Without that transfer, ticks only drain the local ingest queue and
///  the stranded global events never pace while pending
///  (`tap_th_then_tap_th`, rolling nested HoldOnKeyTap cases).
#[test]
fn pending_creation_moves_global_queue_tail_into_local_delay_line() {
    use smart_keymap::key::tap_hold::InterruptResponse;

    // Assemble -- not pending; delay armed; backlog Press(TH)+Release(TH).
    let mut keymap = tap_hold_interrupt_keymap(InterruptResponse::Ignore);
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    assert!(!keymap.test_is_pending());
    assert_eq!(2, keymap.test_input_queue_len());
    assert!(keymap.test_input_queue_delay());

    // Act -- clear delay and process Press(0) → pending; Release(0) transfers.
    keymap.tick(); // clear delay
    keymap.tick(); // process Press(0), create pending, move Release(0) local

    // Assert
    assert!(keymap.test_is_pending());
    assert_eq!(
        1,
        keymap.test_input_queue_len(),
        "Release(0) must sit in the pending delay line after creation"
    );
    assert_eq!(Some(0), keymap.test_pending_queued_events_len());
}

/// Physical inputs while the delay gate is armed sit in the delay line
///  and are not yet recorded in the session log.
///
/// Motivating smart key: **tap-hold**
///  (interrupts must not enter the session log until paced).
#[test]
fn physical_inputs_while_delay_active_stay_in_delay_line() {
    use smart_keymap::key::tap_hold::InterruptResponse;

    // Assemble -- pending with delay set.
    let mut keymap = tap_hold_interrupt_keymap(InterruptResponse::Ignore);
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    assert!(keymap.test_input_queue_delay());

    // Act
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.handle_input(input::Event::Release { keymap_index: 1 });

    // Assert
    assert_eq!(2, keymap.test_input_queue_len());
    assert_eq!(Some(0), keymap.test_pending_queued_events_len());
}

/// While pending, the first tick after a delay only clears the delay;
///  it does not dequeue the next delay-line input.
///
/// Motivating smart key: **tap-hold**
///  (one-input-per-tick spacing for interrupt detection).
#[test]
fn first_tick_while_pending_only_clears_delay() {
    use smart_keymap::key::tap_hold::InterruptResponse;

    // Assemble -- pending with two events waiting in the delay line.
    let mut keymap = tap_hold_interrupt_keymap(InterruptResponse::Ignore);
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.handle_input(input::Event::Release { keymap_index: 1 });
    assert_eq!(2, keymap.test_input_queue_len());

    // Act
    keymap.tick();

    // Assert
    assert_eq!(2, keymap.test_input_queue_len());
    assert_eq!(Some(0), keymap.test_pending_queued_events_len());
    assert!(!keymap.test_input_queue_delay());
}

/// When delay is already cleared, a tick moves exactly one delay-line input
///  into the session log.
///
/// Motivating smart key: **tap-hold**
///  (paced interrupt press then release land in separate ticks).
#[test]
fn tick_when_delay_zero_moves_one_input_into_session_log() {
    use smart_keymap::key::tap_hold::InterruptResponse;

    // Assemble -- delay cleared; Press(1) and Release(1) still queued.
    let mut keymap = tap_hold_interrupt_keymap(InterruptResponse::Ignore);
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.handle_input(input::Event::Release { keymap_index: 1 });
    keymap.tick(); // clear delay
    assert!(!keymap.test_input_queue_delay());
    assert_eq!(2, keymap.test_input_queue_len());

    // Act
    keymap.tick();

    // Assert -- Press(1) entered the session log; Release still delayed.
    assert_eq!(1, keymap.test_input_queue_len());
    assert_eq!(Some(1), keymap.test_pending_queued_events_len());
    let log = keymap.test_pending_session_log_inputs().unwrap();
    assert_eq!(&[input::Event::Press { keymap_index: 1 }], log.as_slice());
}

/// Resolve via timeout does not drain the delay line:
///  never-logged inputs stay queued for post-resolve pacing.
///
/// Motivating smart key: **tap-hold**
///  (timeout-to-hold while an interrupt is still only delayed).
#[test]
fn resolve_by_timeout_leaves_delay_line_inputs_queued() {
    use smart_keymap::key::tap_hold::InterruptResponse;

    // Assemble -- interrupt still only in the delay line (not session log).
    let mut keymap = tap_hold_interrupt_keymap(InterruptResponse::Ignore);
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    assert_eq!(1, keymap.test_input_queue_len());
    assert_eq!(Some(0), keymap.test_pending_queued_events_len());

    // Act
    keymap.test_handle_scheduled_key_event(tap_hold_timeout_event());

    // Assert -- hold; delay-line Press(1) still queued, not applied as a press.
    assert!(!keymap.test_is_pending());
    assert_eq!(1, keymap.test_input_queue_len());
    let hold = key::KeyOutput::from_key_code(0xE0);
    assert_eq!(
        heapless::Vec::<key::KeyOutput, { MAX_PRESSED_KEYS }>::from_slice(&[hold]).unwrap(),
        keymap.pressed_keys()
    );
    assert!(!keymap
        .pressed_keys()
        .contains(&key::KeyOutput::from_key_code(0x05)));
}

/// After resolve, paced ticks drain the former delay-line input
///  as a normal press.
///
/// Motivating smart key: **tap-hold**
///  (post-hold interrupt key still registers as a normal press).
#[test]
fn post_resolve_ticks_drain_delay_line_as_normal_press() {
    use smart_keymap::key::tap_hold::InterruptResponse;

    // Assemble -- resolve while Press(1) is still only in the delay line.
    let mut keymap = tap_hold_interrupt_keymap(InterruptResponse::Ignore);
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.test_handle_scheduled_key_event(tap_hold_timeout_event());
    assert!(!keymap.test_is_pending());
    assert_eq!(1, keymap.test_input_queue_len());

    // Act -- post-resolve pacing.
    keymap.tick();
    keymap.tick();
    if keymap.test_input_queue_len() > 0 {
        keymap.tick();
        keymap.tick();
    }

    // Assert
    assert_eq!(0, keymap.test_input_queue_len());
    assert!(keymap
        .pressed_keys()
        .contains(&key::KeyOutput::from_key_code(0x05)));
}

/// Inputs still only in the delay line at resolve time
///  are excluded from the session log,
///  so they are not absorbed into resolve replay.
///
/// After a processing `tick`,
///  delay ends cleared (`set_delay` then `tick_delay` in the same tick),
///  so the next queued event is ready but not yet popped
///  until the next `tick`/`handle_input`.
///
/// Motivating smart key: **tap-hold**
///  (partially paced interrupt release must survive resolve).
#[test]
fn resolve_leaves_never_logged_delay_line_inputs_queued() {
    use smart_keymap::key::tap_hold::InterruptResponse;

    // Assemble -- pace Press(1) into the session log; leave Release(1) queued.
    let mut keymap = tap_hold_interrupt_keymap(InterruptResponse::Ignore);
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.handle_input(input::Event::Release { keymap_index: 1 });
    keymap.tick(); // clear delay
    keymap.tick(); // process Press(1)
    assert_eq!(Some(1), keymap.test_pending_queued_events_len());
    assert_eq!(1, keymap.test_input_queue_len());
    assert!(!keymap.test_input_queue_delay());

    // Act -- resolve without processing remaining Release(1).
    keymap.test_handle_scheduled_key_event(tap_hold_timeout_event());

    // Assert -- never-logged Release(1) remains queued
    //  (session log may also prepend Press(1) ahead of it).
    assert!(!keymap.test_is_pending());
    assert!(
        keymap.test_input_queue_len() >= 1,
        "at least the never-logged Release(1) should remain queued after resolve"
    );
}

/// A scheduled `Event::Input` during pending is applied immediately
///  and recorded when still pending,
///  while a concurrent physical input stays in the delay line.
///
/// Motivating smart key: **tap-hold**
///  (scheduled vs physical dual paths during pending; #578 family).
#[test]
fn scheduled_input_during_pending_records_while_physical_stays_delayed() {
    use smart_keymap::key::tap_hold::InterruptResponse;

    // Assemble -- pending; physical interrupt already in the delay line.
    // Ignore so the scheduled interrupt does not resolve.
    let mut keymap = tap_hold_interrupt_keymap(InterruptResponse::Ignore);
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    assert_eq!(1, keymap.test_input_queue_len());
    assert_eq!(Some(0), keymap.test_pending_queued_events_len());

    // Act
    keymap.test_handle_scheduled_key_event(key::Event::Input(input::Event::Press {
        keymap_index: 1,
    }));

    // Assert -- scheduled recorded; physical still waiting (not double-processed).
    assert!(keymap.test_is_pending());
    assert_eq!(Some(1), keymap.test_pending_queued_events_len());
    assert_eq!(1, keymap.test_input_queue_len());
}

/// When pending resolves inside a `tick` that pops from the delay line,
///  `tick` ends with `set_delay` then `tick_delay`,
///  so delay is cleared after resolve.
/// Resolve also prepends filtered session-log inputs onto the queue.
///
/// Motivating smart key: **tap-hold**
///  (release-as-tap path; delay-after-resolve timing for observed reports).
#[test]
fn resolve_via_tick_leaves_delay_zero_and_queues_replay() {
    use smart_keymap::key::tap_hold::InterruptResponse;

    // Assemble -- pending release waiting in the delay line.
    let mut keymap = tap_hold_interrupt_keymap(InterruptResponse::Ignore);
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    assert!(keymap.test_is_pending());
    keymap.tick(); // clear delay

    // Act -- process release → resolve as tap; prepend last self Release.
    keymap.tick();

    // Assert
    assert!(!keymap.test_is_pending());
    assert!(
        !keymap.test_input_queue_delay(),
        "tick sets delay then tick_delay in the same call"
    );
    assert!(
        keymap.test_input_queue_len() >= 1,
        "resolve prepends filtered session-log inputs onto the delay line"
    );
}

/// When resolve happens inside `handle_input`
///  (HoldOnKeyPress interrupt popped in that call),
///  delay is set without a same-call `tick_delay`,
///  so the next physical input is deferred —
///  a different delay state than resolve-via-tick.
///
/// Motivating smart key: **tap-hold** (`HoldOnKeyPress`; #578).
#[test]
fn resolve_inside_handle_input_sets_delay_for_next_physical() {
    use smart_keymap::key::tap_hold::InterruptResponse;

    // Assemble -- pending; delay cleared so next handle_input can resolve.
    let mut keymap = tap_hold_interrupt_keymap(InterruptResponse::HoldOnKeyPress);
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    assert!(keymap.test_is_pending());
    keymap.tick();
    assert!(!keymap.test_input_queue_delay());

    // Act -- interrupt resolves hold inside handle_input.
    keymap.handle_input(input::Event::Press { keymap_index: 1 });

    // Assert -- delay armed again; interrupt not applied as a normal press (#578).
    assert!(!keymap.test_is_pending());
    assert!(keymap.test_input_queue_delay());
    assert!(!keymap
        .pressed_keys()
        .contains(&key::KeyOutput::from_key_code(0x05)));
}

/// After resolve-inside-`handle_input`, the next physical event
///  is deferred onto the delay line because delay is still armed.
///
/// Motivating smart key: **tap-hold** (`HoldOnKeyPress`; #578).
#[test]
fn post_resolve_inside_handle_input_defers_next_physical() {
    use smart_keymap::key::tap_hold::InterruptResponse;

    // Assemble -- resolve via HoldOnKeyPress interrupt inside handle_input.
    let mut keymap = tap_hold_interrupt_keymap(InterruptResponse::HoldOnKeyPress);
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.tick();
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    assert!(!keymap.test_is_pending());
    assert!(keymap.test_input_queue_delay());
    // Session log may have prepended Press(1); queue non-empty is ok.
    let queue_after_resolve = keymap.test_input_queue_len();

    // Act
    keymap.handle_input(input::Event::Release { keymap_index: 1 });

    // Assert
    assert_eq!(
        queue_after_resolve + 1,
        keymap.test_input_queue_len(),
        "post-resolve delay defers the next physical event onto the delay line"
    );
}

#[test]
fn test_keymap_input_queue_processes_events_one_per_tick_delay() {
    let mut keymap = simple_keyboard_keymap!();

    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    assert_eq!(
        heapless::Vec::<key::KeyOutput, { MAX_PRESSED_KEYS }>::from_slice(&[
            key::KeyOutput::from_key_code(0x04)
        ])
        .unwrap(),
        keymap.pressed_keys()
    );

    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    assert_eq!(
        heapless::Vec::<key::KeyOutput, { MAX_PRESSED_KEYS }>::from_slice(&[
            key::KeyOutput::from_key_code(0x04)
        ])
        .unwrap(),
        keymap.pressed_keys()
    );

    keymap.tick();
    keymap.tick();
    assert!(keymap.pressed_keys().is_empty());
}

#[test]
fn test_keymap_init_clears_pressed_keys_and_input_queue() {
    let mut keymap = simple_keyboard_keymap!();

    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    keymap.init();

    assert!(keymap.pressed_keys().is_empty());
    assert!(!keymap.requires_polling());
}

#[test]
fn test_keymap_virtual_key_press_and_release() {
    let mut keymap = simple_keyboard_keymap!();
    let key_output = key::KeyOutput::from_key_code(0x05);

    keymap.handle_input(input::Event::VirtualKeyPress { key_output });
    assert_eq!(
        heapless::Vec::<key::KeyOutput, { MAX_PRESSED_KEYS }>::from_slice(&[key_output]).unwrap(),
        keymap.pressed_keys()
    );

    keymap.handle_input(input::Event::VirtualKeyRelease { key_output });
    keymap.tick();
    keymap.tick();
    assert!(keymap.pressed_keys().is_empty());
}

#[test]
fn test_keymap_many_input_events_without_tick_or_report() {
    // Assemble
    let mut keymap = {
        use smart_keymap_full_system_std::key_system;

        use key_system::Context;
        use key_system::Ref;
        const KEY_COUNT: usize = 1;
        const KEY_REFS: [Ref; KEY_COUNT] = [key_system::Ref::Keyboard(
            smart_keymap::key::keyboard::Ref::KeyCode(0x04),
        )];
        const CONTEXT: Context = Context::from_config(key_system::Config::new());

        smart_keymap::keymap::Keymap::new(
            KEY_REFS,
            CONTEXT,
            key_system::System::new(
                smart_keymap::key::automation::System::new(Vec::new()),
                smart_keymap::key::callback::System::new(Vec::new()),
                smart_keymap::key::chorded::System::new(Vec::new(), Vec::new()),
                smart_keymap::key::consumer::System::new(Vec::new()),
                smart_keymap::key::keyboard::System::new(Vec::new()),
                smart_keymap::key::layered::System::new(Vec::new(), Vec::new()),
                smart_keymap::key::mod_conditioned::System::new(Vec::new()),
                smart_keymap::key::mouse::System::new(Vec::new()),
                smart_keymap::key::sequence::System::new(Vec::new(), Vec::new()),
                smart_keymap::key::sticky::System::new(Vec::new()),
                smart_keymap::key::tap_dance::System::new(Vec::new()),
                smart_keymap::key::tap_hold::System::new(Vec::new()),
            ),
        )
    };

    // Act
    for _ in 0..100 {
        keymap.handle_input(input::Event::Press { keymap_index: 0 });
        keymap.handle_input(input::Event::Release { keymap_index: 0 });
    }

    // Assert
    // (expect no panics)
}
