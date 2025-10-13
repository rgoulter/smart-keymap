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

    /// Proxies [keymap::Keymap::handle_input], `tick`'ing the keymap appropriately.
    pub fn handle_input(&mut self, ev: input::Event) {
        let ObservedKeymap {
            keymap,
            distinct_reports,
        } = self;

        keymap.handle_input(ev);
        distinct_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

        keymap.tick();
        distinct_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    }

    /// Proxies [keymap::Keymap::tick], updating reports appropriately.
    pub fn tick(&mut self) {
        let ObservedKeymap {
            keymap,
            distinct_reports,
        } = self;

        keymap.tick();
        distinct_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
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
