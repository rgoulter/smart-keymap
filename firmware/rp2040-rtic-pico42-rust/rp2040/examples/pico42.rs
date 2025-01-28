#![no_std]
#![no_main]

mod board {
    use rp2040_hal as hal;

    use hal::gpio::bank0;

    use rp2040_rtic_pico42_rust::matrix::Matrix;

    use rp2040_rtic_pico42_rust_rp2040::input::{Input, Output, UnconfiguredPin};

    pub const COLS: usize = 12;

    pub const ROWS: usize = 4;

    #[rustfmt::skip]
    pub const KEYMAP_INDICES: [[Option<u16>; COLS]; ROWS] = [
        [ Some(0),  Some(1),  Some(2),  Some(3),  Some(4), None,     None,      Some(5),  Some(6),  Some(7),  Some(8),  Some(9)],
        [Some(10), Some(11), Some(12), Some(13), Some(14), None,     None,     Some(15), Some(16), Some(17), Some(18), Some(19)],
        [Some(20), Some(21), Some(22), Some(23), Some(24), None,     None,     Some(25), Some(26), Some(27), Some(28), Some(29)],
        [Some(30), Some(31), Some(32), Some(33), Some(34), Some(35), Some(36), Some(37), Some(38), Some(39), Some(40), Some(41)],
    ];

    pub use rp2040_rtic_pico42_rust_rp2040::app_prelude::VID;

    pub const PID: u16 = 0x0005;
    pub const MANUFACTURER: &str = "smart-keyboard";
    pub const PRODUCT: &str = "Pico42";

    pub type Keyboard = rp2040_rtic_pico42_rust::input::Keyboard<
        COLS,
        ROWS,
        Matrix<Input, Output, COLS, ROWS, hal::Timer>,
    >;

    pub type PressedKeys = rp2040_rtic_pico42_rust::input::PressedKeys<COLS, ROWS>;

    pub fn cols(
        gp0: UnconfiguredPin<bank0::Gpio0>,
        gp1: UnconfiguredPin<bank0::Gpio1>,
        gp2: UnconfiguredPin<bank0::Gpio2>,
        gp3: UnconfiguredPin<bank0::Gpio3>,
        gp4: UnconfiguredPin<bank0::Gpio4>,
        gp5: UnconfiguredPin<bank0::Gpio5>,
        gp6: UnconfiguredPin<bank0::Gpio6>,
        gp7: UnconfiguredPin<bank0::Gpio7>,
        gp8: UnconfiguredPin<bank0::Gpio8>,
        gp9: UnconfiguredPin<bank0::Gpio9>,
        gp10: UnconfiguredPin<bank0::Gpio10>,
        gp11: UnconfiguredPin<bank0::Gpio11>,
    ) -> [Input; COLS] {
        [
            gp0.into_pull_up_input().into_dyn_pin(),
            gp1.into_pull_up_input().into_dyn_pin(),
            gp2.into_pull_up_input().into_dyn_pin(),
            gp3.into_pull_up_input().into_dyn_pin(),
            gp4.into_pull_up_input().into_dyn_pin(),
            gp5.into_pull_up_input().into_dyn_pin(),
            gp6.into_pull_up_input().into_dyn_pin(),
            gp7.into_pull_up_input().into_dyn_pin(),
            gp8.into_pull_up_input().into_dyn_pin(),
            gp9.into_pull_up_input().into_dyn_pin(),
            gp10.into_pull_up_input().into_dyn_pin(),
            gp11.into_pull_up_input().into_dyn_pin(),
        ]
    }

    pub fn rows(
        gp14: UnconfiguredPin<bank0::Gpio14>,
        gp15: UnconfiguredPin<bank0::Gpio15>,
        gp16: UnconfiguredPin<bank0::Gpio16>,
        gp17: UnconfiguredPin<bank0::Gpio17>,
    ) -> [Output; ROWS] {
        [
            gp14.into_push_pull_output().into_dyn_pin(),
            gp15.into_push_pull_output().into_dyn_pin(),
            gp16.into_push_pull_output().into_dyn_pin(),
            gp17.into_push_pull_output().into_dyn_pin(),
        ]
    }

    /// Map from [row][col] to (maybe) a row-wise keymap index.
    pub fn keymap_index_of(ev: keyberon::layout::Event) -> Option<smart_keymap::input::Event> {
        match ev {
            keyberon::layout::Event::Press(r, c) => KEYMAP_INDICES[r as usize][c as usize]
                .map(|keymap_index| smart_keymap::input::Event::Press { keymap_index }),
            keyberon::layout::Event::Release(r, c) => KEYMAP_INDICES[r as usize][c as usize]
                .map(|keymap_index| smart_keymap::input::Event::Release { keymap_index }),
        }
    }

    macro_rules! rows_and_cols {
        ($gpio_pins:expr, $cols:ident, $rows:ident) => {
            let $cols = crate::board::cols(
                $gpio_pins.gpio0,
                $gpio_pins.gpio1,
                $gpio_pins.gpio2,
                $gpio_pins.gpio3,
                $gpio_pins.gpio4,
                $gpio_pins.gpio5,
                $gpio_pins.gpio6,
                $gpio_pins.gpio7,
                $gpio_pins.gpio8,
                $gpio_pins.gpio9,
                $gpio_pins.gpio10,
                $gpio_pins.gpio11,
            );
            let $rows = crate::board::rows(
                $gpio_pins.gpio14,
                $gpio_pins.gpio15,
                $gpio_pins.gpio16,
                $gpio_pins.gpio17,
            );
        };
    }

    pub(crate) use rows_and_cols;
}

#[rtic::app(
    device = rp_pico::hal::pac,
)]
mod app {
    use panic_halt as _;

    use rp2040_rtic_pico42_rust_rp2040::app_prelude::*;

    use rp2040_rtic_pico42_rust::input::smart_keymap::KeyboardBackend;
    use rp2040_rtic_pico42_rust::matrix::Matrix as DelayedMatrix;

    use super::board;

    use board::keymap_index_of;
    use board::Keyboard;
    use board::PressedKeys;

    #[shared]
    struct Shared {
        usb_dev: UsbDevice,
        usb_class: UsbClass,
    }

    #[local]
    struct Local {
        alarm: timer::Alarm0,
        keyboard: Keyboard,
        backend: KeyboardBackend,
    }

    #[init(local=[
        usb_bus: Option<UsbBusAllocator<UsbBus>> = None
    ])]
    fn init(mut ctx: init::Context) -> (Shared, Local, init::Monotonics) {
        let (mut _watchdog, clocks) = app_init::init_clocks(
            ctx.device.WATCHDOG,
            ctx.device.XOSC,
            ctx.device.CLOCKS,
            ctx.device.PLL_SYS,
            ctx.device.PLL_USB,
            &mut ctx.device.RESETS,
        );

        let (timer, alarm) =
            app_init::init_timer(ctx.device.TIMER, &mut ctx.device.RESETS, &clocks);

        // Set up the USB driver
        *ctx.local.usb_bus = Some(UsbBusAllocator::new(hal::usb::UsbBus::new(
            ctx.device.USBCTRL_REGS,
            ctx.device.USBCTRL_DPRAM,
            clocks.usb_clock,
            true,
            &mut ctx.device.RESETS,
        )));
        let usb_bus = ctx.local.usb_bus.as_ref().unwrap();

        let (usb_dev, usb_class) =
            app_init::init_usb_device(usb_bus, board::VID, board::PID, board::MANUFACTURER, board::PRODUCT);

        unsafe {
            pac::NVIC::unmask(pac::Interrupt::USBCTRL_IRQ);
            pac::NVIC::unmask(pac::Interrupt::TIMER_IRQ_0);
        };

        let sio = Sio::new(ctx.device.SIO);
        let gpio_pins = rp_pico::Pins::new(
            ctx.device.IO_BANK0,
            ctx.device.PADS_BANK0,
            sio.gpio_bank0,
            &mut ctx.device.RESETS,
        );
        board::rows_and_cols!(gpio_pins, cols, rows);
        let matrix = DelayedMatrix::new(cols, rows, timer, 5, 5).unwrap();
        let keyboard = Keyboard {
            matrix,
            debouncer: Debouncer::new(PressedKeys::default(), PressedKeys::default(), 25),
        };

        let backend = {
            use smart_keymap::init;
            use smart_keymap::keymap::Keymap;
            let keymap = Keymap::new(init::KEY_DEFINITIONS, init::CONTEXT);
            KeyboardBackend::new(keymap)
        };

        (
            Shared { usb_dev, usb_class },
            Local {
                alarm,
                keyboard,
                backend,
            },
            init::Monotonics(),
        )
    }

    #[task(binds = USBCTRL_IRQ, priority = 2, shared = [usb_dev, usb_class])]
    fn usb_tx(c: usb_tx::Context) {
        let usb_tx::SharedResources { usb_dev, usb_class } = c.shared;
        (usb_dev, usb_class).lock(|mut ud, mut uc| usb_poll(&mut ud, &mut uc));
    }

    #[task(binds = TIMER_IRQ_0, priority = 1, shared = [usb_class], local = [keyboard, backend, alarm])]
    fn tick(c: tick::Context) {
        let tick::SharedResources { mut usb_class } = c.shared;
        let tick::LocalResources {
            alarm,
            keyboard,
            backend,
        } = c.local;

        alarm.clear_interrupt();
        alarm.schedule(1.millis()).unwrap();

        for event in keyboard.events() {
            if let Some(event) = keymap_index_of(event) {
                backend.event(event);
            }
        }
        backend.tick();

        usb_class.lock(|k| {
            backend.write_reports(k);
        });
    }
}
