use core::fmt::Debug;
use core::ops::Index;

use crate::input;
use crate::key;
use crate::keymap;

use keymap::Keymap;
use keymap::SetKeymapContext;

/// Wrapper around a [crate::keymap::Keymap] that also tracks distinct HID reports.
#[derive(Debug)]
pub struct ObservedKeymap<I: Index<usize, Output = R>, R, Ctx, Ev: Debug, PKS, KS, S> {
    keymap: Keymap<I, R, Ctx, Ev, PKS, KS, S>,
    distinct_reports: keymap::DistinctReports,
}

impl<
        I: Debug + Index<usize, Output = R>,
        R: Copy + Debug,
        Ctx: Debug + key::Context<Event = Ev> + SetKeymapContext,
        Ev: Copy + Debug,
        PKS: Debug,
        KS: Copy + Debug + From<key::NoOpKeyState>,
        S: key::System<R, Ref = R, Context = Ctx, Event = Ev, PendingKeyState = PKS, KeyState = KS>,
    > ObservedKeymap<I, R, Ctx, Ev, PKS, KS, S>
{
    /// Constructs an observed keymap with a new [keymap::DistinctReports].
    pub fn new(keymap: Keymap<I, R, Ctx, Ev, PKS, KS, S>) -> Self {
        ObservedKeymap {
            keymap,
            distinct_reports: keymap::DistinctReports::new(),
        }
    }

    /// Proxies [keymap::Keymap::handle_input_after_time], `tick`'ing the keymap appropriately.
    pub fn handle_input_after_time(&mut self, delta_ms: u32, ev: input::Event) -> Option<u32> {
        let ObservedKeymap {
            keymap,
            distinct_reports,
        } = self;

        let next_ev = keymap.handle_input_after_time(delta_ms, ev);

        distinct_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

        for _ in 0..keymap::INPUT_QUEUE_TICK_DELAY {
            keymap.tick();
            distinct_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
        }

        next_ev
    }

    /// Proxies [keymap::Keymap::tick_to_next_scheduled_event], updating reports appropriately.
    pub fn tick_to_next_scheduled_event(&mut self) -> Option<u32> {
        let ObservedKeymap {
            keymap,
            distinct_reports,
        } = self;

        let next_ev = keymap.tick_to_next_scheduled_event();

        distinct_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

        next_ev
    }

    /// Proxies [keymap::Keymap::boot_keyboard_report].
    pub fn boot_keyboard_report(&self) -> [u8; 8] {
        let ObservedKeymap { keymap, .. } = self;

        keymap::KeymapOutput::new(keymap.pressed_keys()).as_hid_boot_keyboard_report()
    }

    /// Reference to distinct reports.
    pub fn distinct_reports(&self) -> &keymap::DistinctReports {
        &self.distinct_reports
    }

    /// Ticks the keymap until there are no scheduled events, updating reports appropriately.
    pub fn tick_until_no_scheduled_events(&mut self) {
        let ObservedKeymap {
            keymap,
            distinct_reports,
        } = self;

        while keymap.has_scheduled_events() {
            keymap.tick();
            distinct_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
        }
    }
}
