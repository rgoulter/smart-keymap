#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_stm32::time::Hertz;
use embassy_stm32::usart::{HalfDuplexConfig, HalfDuplexReadback, Uart};
use embassy_stm32::{bind_interrupts, peripherals, usart, usb, Config};
use embassy_time::Timer;
use panic_halt as _;

use keyberon_smart_keyboard::input::smart_keymap::keymap_index_of;
use keyberon_smart_keyboard::input::MatrixScanner;

use smart_keymap::split::Message;

use board::KEYMAP_INDICES;

use core::ptr::write_volatile;
use cortex_m::peripheral::SCB;

// Constants derived from the TinyUF2 source for stm32f4
// Address: From linker script (_board_dfu_dbl_tap = _estack = end of RAM - 4)
//          RAM Base = 0x20000000, Size = 64KB = 0x10000
//          Address = 0x20000000 + 0x10000 - 4 = 0x2000FFFC
const BOOTLOADER_MAGIC_ADDR: u32 = 0x2000FFFC;

// Value: From board_api.h (DBL_TAP_MAGIC for 32-bit reg size)
const BOOTLOADER_MAGIC_VALUE: u32 = 0xf01669ef;

bind_interrupts!(struct Irqs {
    OTG_FS => usb::InterruptHandler<peripherals::USB_OTG_FS>;
    USART1 => usart::InterruptHandler<peripherals::USART1>;
});

/// Enters the TinyUF2 bootloader by writing the DBL_TAP_MAGIC value to RAM
/// and triggering a system reset.
///
/// # Safety
///
/// - This function writes to a specific memory address (`0x2000FFFC`).
///   Ensure this address is correct and accessible RAM for your target's
///   TinyUF2 configuration. It must not conflict with essential application data
///   that needs to persist across a soft reset (which is unlikely).
/// - This function triggers a system reset, abruptly terminating the
///   current application execution. Ensure system state is safe for a reset.
/// - Relies on the TinyUF2 bootloader flashed on the device correctly
///   checking this specific address and value upon startup.
pub unsafe fn enter_bootloader_tinyuf2() {
    write_volatile(BOOTLOADER_MAGIC_ADDR as *mut u32, BOOTLOADER_MAGIC_VALUE);
    SCB::sys_reset();
}

#[cfg(not(custom_board))]
mod board {
    use embassy_stm32::gpio::Input;

    pub const COLS: usize = 5;
    pub const ROWS: usize = 4;

    #[rustfmt::skip]
    pub const KEYMAP_INDICES: [[Option<u16>; COLS]; ROWS] = [
        [Some(5),  Some(6),  Some(7),  Some(8),  Some(9)],
        [Some(15), Some(16), Some(17), Some(18), Some(19)],
        [Some(25), Some(26), Some(27), Some(28), Some(29)],
        [Some(33), Some(34), Some(35),     None,     None],
    ];

    pub type Keyboard<'d> = keyberon_smart_keyboard::input::Keyboard<COLS, ROWS, DirectPins<'d>>;

    pub type PressedKeys = keyberon_smart_keyboard::input::PressedKeys<COLS, ROWS>;

    #[rustfmt::skip]
    pub struct DirectPins<'d>(
        pub Input<'d>,
        pub Input<'d>,
        pub Input<'d>,
        pub Input<'d>,
        pub Input<'d>,

        pub Input<'d>,
        pub Input<'d>,
        pub Input<'d>,
        pub Input<'d>,
        pub Input<'d>,

        pub Input<'d>,
        pub Input<'d>,
        pub Input<'d>,
        pub Input<'d>,
        pub Input<'d>,

        pub Input<'d>,
        pub Input<'d>,
        pub Input<'d>,
    );

    impl<'d> keyberon_smart_keyboard::input::MatrixScanner<COLS, ROWS> for DirectPins<'d> {
        fn is_boot_key_pressed(&mut self) -> bool {
            self.0.is_low()
        }

        fn get(&mut self) -> Result<[[bool; COLS]; ROWS], core::convert::Infallible> {
            Ok([
                [
                    self.0.is_low(),
                    self.1.is_low(),
                    self.2.is_low(),
                    self.3.is_low(),
                    self.4.is_low(),
                ],
                [
                    self.5.is_low(),
                    self.6.is_low(),
                    self.7.is_low(),
                    self.8.is_low(),
                    self.9.is_low(),
                ],
                [
                    self.10.is_low(),
                    self.11.is_low(),
                    self.12.is_low(),
                    self.13.is_low(),
                    self.14.is_low(),
                ],
                [
                    self.15.is_low(),
                    self.16.is_low(),
                    self.17.is_low(),
                    false,
                    false,
                ],
            ])
        }
    }

    macro_rules! keyboard {
        ($p:ident) => {
            crate::board::Keyboard {
                matrix: crate::board::DirectPins(
                    embassy_stm32::gpio::Input::new($p.PB3, embassy_stm32::gpio::Pull::Up),
                    embassy_stm32::gpio::Input::new($p.PA5, embassy_stm32::gpio::Pull::Up),
                    embassy_stm32::gpio::Input::new($p.PA9, embassy_stm32::gpio::Pull::Up),
                    embassy_stm32::gpio::Input::new($p.PB15, embassy_stm32::gpio::Pull::Up),
                    embassy_stm32::gpio::Input::new($p.PB12, embassy_stm32::gpio::Pull::Up),
                    embassy_stm32::gpio::Input::new($p.PB10, embassy_stm32::gpio::Pull::Up),
                    embassy_stm32::gpio::Input::new($p.PA15, embassy_stm32::gpio::Pull::Up),
                    embassy_stm32::gpio::Input::new($p.PA10, embassy_stm32::gpio::Pull::Up),
                    embassy_stm32::gpio::Input::new($p.PA8, embassy_stm32::gpio::Pull::Up),
                    embassy_stm32::gpio::Input::new($p.PB13, embassy_stm32::gpio::Pull::Up),
                    embassy_stm32::gpio::Input::new($p.PB5, embassy_stm32::gpio::Pull::Up),
                    embassy_stm32::gpio::Input::new($p.PA4, embassy_stm32::gpio::Pull::Up),
                    embassy_stm32::gpio::Input::new($p.PA6, embassy_stm32::gpio::Pull::Up),
                    embassy_stm32::gpio::Input::new($p.PB1, embassy_stm32::gpio::Pull::Up),
                    embassy_stm32::gpio::Input::new($p.PB14, embassy_stm32::gpio::Pull::Up),
                    embassy_stm32::gpio::Input::new($p.PC13, embassy_stm32::gpio::Pull::Up),
                    embassy_stm32::gpio::Input::new($p.PA0, embassy_stm32::gpio::Pull::Up),
                    embassy_stm32::gpio::Input::new($p.PA2, embassy_stm32::gpio::Pull::Up),
                ),
                debouncer: keyberon::debounce::Debouncer::new(
                    crate::board::PressedKeys::default(),
                    crate::board::PressedKeys::default(),
                    25,
                ),
            }
        };
    }

    pub(crate) use keyboard;
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut config = Config::default();
    {
        use embassy_stm32::rcc::*;
        config.rcc.hse = Some(Hse {
            freq: Hertz(25_000_000),
            mode: HseMode::Oscillator,
        });
        config.rcc.pll_src = PllSource::HSE;
        config.rcc.pll = Some(Pll {
            prediv: PllPreDiv::DIV25,  // PLL Input: 25MHz / 25 = 1MHz
            mul: PllMul::MUL336,       // VCO Output: 1MHz * 336 = 336MHz
            divp: Some(PllPDiv::DIV4), // System Clock (PLL_P): 336MHz / 4 = 84MHz.
            divq: Some(PllQDiv::DIV7), // USB/SDIO/RNG Clock (PLL_Q): 336MHz / 7 = 48MHz
            divr: None,
        });
        config.rcc.ahb_pre = AHBPrescaler::DIV1;
        config.rcc.apb1_pre = APBPrescaler::DIV2;
        config.rcc.apb2_pre = APBPrescaler::DIV1;
        config.rcc.sys = Sysclk::PLL1_P;
        config.rcc.mux.clk48sel = mux::Clk48sel::PLL1_Q;
    }
    let p = embassy_stm32::init(config);

    let mut keyboard = board::keyboard!(p);

    if keyboard.matrix.is_boot_key_pressed() {
        unsafe {
            enter_bootloader_tinyuf2();
        }
    }

    let config = usart::Config::default();
    let usart = Uart::new_half_duplex(
        p.USART1,
        p.PB6,
        Irqs,
        p.DMA2_CH7,
        p.DMA2_CH5,
        config,
        HalfDuplexReadback::NoReadback,
        HalfDuplexConfig::OpenDrainExternal,
    )
    .unwrap();
    let (mut tx, _) = usart.split();

    loop {
        Timer::after_millis(1).await;

        for event in keyboard.events() {
            if let Some(input_event) = keymap_index_of(&KEYMAP_INDICES, event) {
                let message = Message { input_event };
                let buf = message.serialize();
                tx.write(&buf).await.unwrap();
                tx.flush().await.unwrap();
            }
        }
    }
}
