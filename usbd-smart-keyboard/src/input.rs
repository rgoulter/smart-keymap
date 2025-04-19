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

/// Matrix scan result type.
pub type PressedKeys<const COLS: usize, const ROWS: usize> = [[bool; COLS]; ROWS];

// R for 'matrix get result type',
// E for 'error of matrix get result type'.
pub trait MatrixScanner<const COLS: usize, const ROWS: usize, E = Infallible> {
    /// Check whether SW_1_1 is pressed.
    fn is_boot_key_pressed(&mut self) -> bool;
    fn get(&mut self) -> Result<[[bool; COLS]; ROWS], E>;
}

/// The keyboard "frontend", manages the keyboard from the hardware matrix
/// through to keyboard events (presses/releases of coordinates on a keyboard layout).
///
/// This takes care of scanning the keyboard matrix, debouncing.
pub struct Keyboard<const COLS: usize, const ROWS: usize, M: MatrixScanner<COLS, ROWS>> {
    pub matrix: M,
    pub debouncer: Debouncer<PressedKeys<COLS, ROWS>>,
}

impl<const COLS: usize, const ROWS: usize, M: MatrixScanner<COLS, ROWS>> Keyboard<COLS, ROWS, M> {
    /// Constructs a new [Keyboard].
    pub fn new(matrix: M, debouncer: Debouncer<PressedKeys<COLS, ROWS>>) -> Self {
        Self { matrix, debouncer }
    }

    /// Scans the matrix and returns the debounced events.
    pub fn events(&mut self) -> heapless::Vec<Event, 8> {
        let key_presses = self.matrix.get().unwrap();
        self.debouncer.events(key_presses).collect()
    }
}
