//! Hardware pin switch matrix handling.

use core::fmt::Debug;

use embedded_hal::delay::DelayNs;
use embedded_hal::digital::{InputPin, OutputPin};

/// Newtype wrapper around [keyberon::matrix::DirectPinMatrix]
///  to implement [crate::input::MatrixScanner] for it.
pub struct DirectPinMatrix<P: InputPin, const CS: usize, const RS: usize>(
    pub keyberon::matrix::DirectPinMatrix<P, CS, RS>,
);

impl<P, const CS: usize, const RS: usize, E: Debug> DirectPinMatrix<P, CS, RS>
where
    P: InputPin<Error = E>,
{
    pub fn new(pins: [[Option<P>; CS]; RS]) -> Self
    where
        P: InputPin<Error = E>,
    {
        Self(keyberon::matrix::DirectPinMatrix::new(pins).unwrap())
    }
}

impl<P, const CS: usize, const RS: usize> crate::input::MatrixScanner<CS, RS>
    for DirectPinMatrix<P, CS, RS>
where
    P: InputPin<Error = core::convert::Infallible>,
{
    fn is_boot_key_pressed(&mut self) -> bool {
        self.0.get().unwrap()[0][0]
    }

    fn get(&mut self) -> Result<[[bool; CS]; RS], core::convert::Infallible> {
        self.0.get()
    }
}

/// Describes the hardware-level matrix of switches.
///
/// Generic parameters are in order: The type of column pins,
/// the type of row pins, the number of columns and rows.
/// **NOTE:** In order to be able to put different pin structs
/// in an array they have to be downgraded (stripped of their
/// numbers etc.). Most HAL-s have a method of downgrading pins
/// to a common (erased) struct. (for example see
/// [stm32f0xx_hal::gpio::PA0::downgrade](https://docs.rs/stm32f0xx-hal/0.17.1/stm32f0xx_hal/gpio/gpioa/struct.PA0.html#method.downgrade))
///
/// TIM5 is used to provide a delay during the matrix scanning.
pub struct Matrix<C, R, const CS: usize, const RS: usize, D>
where
    C: InputPin,
    R: OutputPin,
    D: DelayNs,
{
    cols: [C; CS],
    rows: [R; RS],
    delay: D,
    select_delay_us: u32,
    unselect_delay_us: u32,
}

impl<C, R, const CS: usize, const RS: usize, D> Matrix<C, R, CS, RS, D>
where
    C: InputPin,
    R: OutputPin,
    D: DelayNs,
{
    /// Creates a new Matrix.
    ///
    /// Assumes columns are pull-up inputs,
    /// and rows are output pins which are set high when not being scanned.
    pub fn new<E>(
        cols: [C; CS],
        rows: [R; RS],
        delay: D,
        select_delay_us: u32,
        unselect_delay_us: u32,
    ) -> Result<Self, E>
    where
        C: InputPin<Error = E>,
        R: OutputPin<Error = E>,
    {
        let mut res = Self {
            cols,
            rows,
            delay,
            select_delay_us,
            unselect_delay_us,
        };
        res.clear()?;
        Ok(res)
    }
    fn clear<E>(&mut self) -> Result<(), E>
    where
        C: InputPin<Error = E>,
        R: OutputPin<Error = E>,
    {
        for r in self.rows.iter_mut() {
            r.set_high()?;
        }
        Ok(())
    }
}

impl<C, R, const CS: usize, const RS: usize, D, E: Debug> crate::input::MatrixScanner<CS, RS, E>
    for Matrix<C, R, CS, RS, D>
where
    C: InputPin<Error = E>,
    R: OutputPin<Error = E>,
    D: DelayNs,
{
    fn is_boot_key_pressed(&mut self) -> bool {
        self.rows[0].set_low().unwrap();
        self.delay.delay_us(self.select_delay_us);

        let is_pressed = self.cols[0].is_low().unwrap();

        self.rows[0].set_high().unwrap();
        self.delay.delay_us(self.unselect_delay_us);

        is_pressed
    }

    /// Scans the matrix and checks which keys are pressed.
    ///
    /// Every row pin in order is pulled low, and then each column
    /// pin is tested; if it's low, the key is marked as pressed.
    ///
    /// Delays for a bit after setting each pin, and after clearing
    /// each pin.
    fn get(&mut self) -> Result<[[bool; CS]; RS], E> {
        let mut keys = [[false; CS]; RS];

        for (ri, row) in self.rows.iter_mut().enumerate() {
            row.set_low()?;
            // Delay after setting the pin low.
            // Using a timer for this is probably overkill.
            self.delay.delay_us(self.select_delay_us);
            for (ci, col) in self.cols.iter_mut().enumerate() {
                if col.is_low()? {
                    keys[ri][ci] = true;
                }
            }
            row.set_high()?;
            // Delay after setting the pin high.
            // Using a timer for this is probably overkill.
            self.delay.delay_us(self.unselect_delay_us);
        }
        Ok(keys)
    }
}
