use core::fmt::Debug;
use core::ops::Index;

use smart_keymap::input;
use smart_keymap::key;
use smart_keymap::keymap::{self, Keymap, KeymapOutput, SetKeymapContext};

/// Callbacks for the keymap.
pub struct KeymapCallbacks {
    /// Callback for resetting keyboard state.
    pub reset: Option<fn() -> ()>,
    /// Callback for entering the bootloader.
    pub reset_to_bootloader: Option<fn() -> ()>,
}

/// The keyboard "backend", manages the keyboard from the events received
/// (presses/releases of coordinates on a keyboard layout).
/// through to listing HID scancodes to report using HIDs.
///
/// Type parameters match [Keymap].
#[derive(Debug)]
pub struct KeyboardBackend<
    I: Index<usize, Output = R> = [smart_keymap::init::Ref; smart_keymap::init::KEY_COUNT],
    R = smart_keymap::init::Ref,
    Ctx = smart_keymap::init::Context,
    Ev: Debug = smart_keymap::init::Event,
    PKS = smart_keymap::init::PendingKeyState,
    KS = smart_keymap::init::KeyState,
    S = smart_keymap::init::System,
> {
    keymap: Keymap<I, R, Ctx, Ev, PKS, KS, S>,
    keymap_output: KeymapOutput,
}

impl Default for KeyboardBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl KeyboardBackend {
    /// Constructs a new keyboard backend using the board keymap
    ///  ([smart_keymap::new_keymap]).
    pub fn new() -> Self {
        Self::new_with_keymap(smart_keymap::new_keymap())
    }
}

impl<I, R, Ctx, Ev, PKS, KS, S> KeyboardBackend<I, R, Ctx, Ev, PKS, KS, S>
where
    I: Index<usize, Output = R>,
    Ev: Debug,
{
    /// Constructs a new keyboard backend with the given keymap.
    pub fn new_with_keymap(keymap: Keymap<I, R, Ctx, Ev, PKS, KS, S>) -> Self {
        Self {
            keymap,
            keymap_output: KeymapOutput::default(),
        }
    }
}

impl<I, R, Ctx, Ev, PKS, KS, S> KeyboardBackend<I, R, Ctx, Ev, PKS, KS, S>
where
    I: Debug + Index<usize, Output = R>,
    R: Copy + Debug,
    Ctx: Debug + key::Context<Event = Ev> + SetKeymapContext,
    Ev: Copy + Debug,
    PKS: Debug,
    KS: Copy + Debug + From<key::NoOpKeyState>,
    S: key::System<R, Ref = R, Context = Ctx, Event = Ev, PendingKeyState = PKS, KeyState = KS>,
{
    /// Set the keymap callbacks.
    pub fn set_callbacks(&mut self, callbacks: KeymapCallbacks) {
        use keymap::KeymapCallback;
        if let Some(callback_fn) = callbacks.reset {
            self.keymap.set_callback(KeymapCallback::Reset, callback_fn);
        }
        if let Some(callback_fn) = callbacks.reset_to_bootloader {
            self.keymap
                .set_callback(KeymapCallback::ResetToBootloader, callback_fn);
        }
    }

    /// Register a key event.
    pub fn event(&mut self, event: input::Event) {
        self.keymap.handle_input(event);
    }

    /// A time event.
    ///
    /// This method must be called regularly, typically every millisecond.
    ///
    /// Returns true if the pressed_key_codes have changed.
    pub fn tick(&mut self) -> bool {
        self.keymap.tick();

        let keymap_output = self.keymap.report_output();

        let old_keymap_output = core::mem::replace(&mut self.keymap_output, keymap_output);

        old_keymap_output != self.keymap_output
    }

    /// Returns the current keymap output.
    pub fn keymap_output(&self) -> &KeymapOutput {
        &self.keymap_output
    }
}

/// Constructs a [input::Event] from a [keyberon::layout::Event],
///  using a map from row, column to (maybe) keymap index.
pub fn keymap_index_of<const COLS: usize, const ROWS: usize>(
    indices: &[[Option<u16>; COLS]; ROWS],
    ev: keyberon::layout::Event,
) -> Option<input::Event> {
    match ev {
        keyberon::layout::Event::Press(r, c) => {
            indices[r as usize][c as usize].map(|keymap_index| input::Event::Press { keymap_index })
        }
        keyberon::layout::Event::Release(r, c) => indices[r as usize][c as usize]
            .map(|keymap_index| input::Event::Release { keymap_index }),
    }
}
