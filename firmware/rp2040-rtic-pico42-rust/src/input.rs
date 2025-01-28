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

