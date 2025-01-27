use core::convert::Infallible;
use keyberon::debounce::Debouncer;
use keyberon::layout::Event;
use usb_device::bus::UsbBus;
use usb_device::UsbError;
use usbd_human_interface_device::device::consumer::{ConsumerControl, MultipleConsumerReport};
use usbd_human_interface_device::device::keyboard::NKROBootKeyboard;
use usbd_human_interface_device::page;
use usbd_human_interface_device::UsbHidError;

use crate::common;

/// For input from the smart_keymap crate.
pub mod smart_keymap;

pub type PressedKeys<const COLS: usize, const ROWS: usize> = [[bool; COLS]; ROWS];

pub type PressedKeys1x1 = PressedKeys<1, 1>;
pub type PressedKeys5x4 = PressedKeys<5, 4>;
pub type PressedKeys12x4 = PressedKeys<12, 4>;
pub type PressedKeys12x5 = PressedKeys<12, 5>;

// R for 'matrix get result type',
// E for 'error of matrix get result type'.
pub trait MatrixScanner<const COLS: usize, const ROWS: usize, E = Infallible> {
    fn get(&mut self) -> Result<[[bool; COLS]; ROWS], E>;
}

/// The keyboard "frontend", manages the keyboard from the hardware matrix
/// through to keyboard events (presses/releases of coordinates on a keyboard layout).
///
/// This takes care of scanning the keyboard matrix, debouncing, and handling matrix chords.
pub struct Keyboard<
    const COLS: usize,
    const ROWS: usize,
    M: MatrixScanner<COLS, ROWS>,
> {
    pub matrix: M,
    pub debouncer: Debouncer<PressedKeys<COLS, ROWS>>,
}

impl<
        const COLS: usize,
        const ROWS: usize,
        M: MatrixScanner<COLS, ROWS>,
    > Keyboard<COLS, ROWS, M>
{
    pub fn new(
        matrix: M,
        debouncer: Debouncer<PressedKeys<COLS, ROWS>>,
    ) -> Self {
        Self {
            matrix,
            debouncer,
        }
    }

    pub fn events(&mut self) -> heapless::Vec<Event, 8> {
        let key_presses = self.matrix.get().unwrap();
        self.debouncer.events(key_presses).collect()
    }
}

/// Simplified interface of the keyberon's Layout.
///
/// I: Iteratable for keycodes.
/// T: Custom action type
/// K: Keycode type
pub trait LayoutEngine<T, K> {
    type KeycodeIterator<'a>: IntoIterator<Item = K>;

    /// Register a key event.
    fn event(&mut self, event: keyberon::layout::Event);

    /// Iterates on the key codes of the current state.
    fn keycodes<'a>(&self) -> Self::KeycodeIterator<'a>;

    /// A time event.
    ///
    /// This method must be called regularly, typically every millisecond.
    ///
    /// Returns the corresponding `CustomEvent`, allowing to manage
    /// custom actions thanks to the `Action::Custom` variant.
    fn tick(&mut self) -> keyberon::layout::CustomEvent<T>;
}

// C: number of columns
// R: number of rows
// L: number of layers
// T: custom action type
// K: keycode type
impl<const C: usize, const R: usize, const L: usize, T: 'static, K: 'static + Copy>
    LayoutEngine<T, K> for keyberon::layout::Layout<C, R, L, T, K>
{
    type KeycodeIterator<'a> = heapless::Vec<K, 8>;

    fn event(&mut self, event: keyberon::layout::Event) {
        self.event(event);
    }

    fn keycodes<'a>(&self) -> Self::KeycodeIterator<'a> {
        // keyberon's keycodes() has signature which returns `impl Iterator<Item = K>`;
        // Currently, can't use `impl Trait` in associated types,
        // so collect the keycodes into a Vec.
        self.keycodes().collect()
    }

    fn tick(&mut self) -> keyberon::layout::CustomEvent<T> {
        self.tick()
    }
}

pub trait HIDReporter<K, C, KE, CE> {
    fn write_keyboard_report(&mut self, report: impl IntoIterator<Item = K>) -> Result<(), KE>;

    fn write_consumer_report(&mut self, report: impl IntoIterator<Item = C>) -> Result<(), CE>;
}

impl<B> HIDReporter<page::Keyboard, page::Consumer, UsbHidError, UsbError> for common::UsbClass<B>
where
    B: UsbBus,
{
    fn write_keyboard_report(
        &mut self,
        iter: impl IntoIterator<Item = page::Keyboard>,
    ) -> Result<(), UsbHidError> {
        self.device::<NKROBootKeyboard<'_, _>, _>()
            .write_report(iter)
    }

    fn write_consumer_report(
        &mut self,
        iter: impl IntoIterator<Item = page::Consumer>,
    ) -> Result<(), UsbError> {
        let codes: [page::Consumer; 4] = iter
            .into_iter()
            .chain(core::iter::repeat(page::Consumer::Unassigned))
            .take(4)
            .collect::<heapless::Vec<_, 4>>()
            .into_array()
            .unwrap();
        let report = MultipleConsumerReport { codes };
        self.device::<ConsumerControl<'_, _>, _>()
            .write_report(&report)
            .map(|_| ())
    }
}

