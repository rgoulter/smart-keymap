#![no_std]
#![no_main]

#[cfg(not(custom_board))]
mod board {
    use rp2040_hal as hal;

    use hal::gpio::bank0;

    use usbd_smart_keyboard::matrix::Matrix;

    use rp2040_rtic_smart_keyboard::input::{Input, Output, UnconfiguredPin};

    pub const COLS: usize = 1;
    pub const ROWS: usize = 1;

    pub const KEYMAP_INDICES: [[Option<u16>; COLS]; ROWS] = [[Some(0)]];

    pub use rp2040_rtic_smart_keyboard::app_prelude::VID;

    pub const PID: u16 = 0x0005;
    pub const MANUFACTURER: &str = "smart-keyboard";
    pub const PRODUCT: &str = "RP2040 Keyboard";

    pub type Keyboard = usbd_smart_keyboard::input::Keyboard<
        COLS,
        ROWS,
        Matrix<Input, Output, COLS, ROWS, hal::Timer>,
    >;

    pub type PressedKeys = usbd_smart_keyboard::input::PressedKeys<COLS, ROWS>;

    pub fn cols(gp0: UnconfiguredPin<bank0::Gpio0>) -> [Input; COLS] {
        [gp0.into_pull_up_input().into_dyn_pin()]
    }

    pub fn rows(gp1: UnconfiguredPin<bank0::Gpio1>) -> [Output; ROWS] {
        [gp1.into_push_pull_output().into_dyn_pin()]
    }

    macro_rules! rows_and_cols {
        ($gpio_pins:expr, $cols:ident, $rows:ident) => {
            let $cols = crate::board::cols($gpio_pins.gpio0);
            let $rows = crate::board::rows($gpio_pins.gpio1);
        };
    }

    pub(crate) use rows_and_cols;
}

#[cfg(custom_board)]
include!(concat!(env!("OUT_DIR"), "/board.rs"));

#[rtic::app(
    device = rp_pico::hal::pac,
)]
mod app {
    use panic_halt as _;

    use hal::usb::UsbBus;

    use usbd_serial::SerialPort;

    use rp2040_rtic_smart_keyboard::app_prelude::*;

    use usbd_human_interface_device::device::keyboard::NKROBootKeyboard;

    use usbd_smart_keyboard::input::smart_keymap::keymap_index_of;
    use usbd_smart_keyboard::input::smart_keymap::KeyboardBackend;
    use usbd_smart_keyboard::input::MatrixScanner;
    use usbd_smart_keyboard::matrix::Matrix as DelayedMatrix;

    use rp2040_rtic_smart_keyboard::common::keyboard_reset_to_bootloader;

    use super::board;

    use board::Keyboard;
    use board::PressedKeys;
    use board::KEYMAP_INDICES;

    #[shared]
    struct Shared {
        usb_dev: UsbDevice,
        usb_serial: SerialPort<'static, UsbBus>,
        usb_class: UsbClass,
    }

    #[local]
    struct Local {
        alarm: timer::Alarm0,
        keyboard: Keyboard,
        backend: KeyboardBackend,
        report_success: bool,
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

        let (usb_dev, usb_serial, usb_class) = app_init::init_usb_device(
            usb_bus,
            board::VID,
            board::PID,
            board::MANUFACTURER,
            board::PRODUCT,
        );

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
        let mut matrix = DelayedMatrix::new(cols, rows, timer, 5, 5).unwrap();

        // Check if bootloader pressed
        if matrix.is_boot_key_pressed() {
            keyboard_reset_to_bootloader();
        }

        let keyboard = Keyboard {
            matrix,
            debouncer: Debouncer::new(PressedKeys::default(), PressedKeys::default(), 25),
        };

        let mut backend = {
            use smart_keymap::init;
            use smart_keymap::keymap::Keymap;
            let keymap = Keymap::new(init::KEY_DEFINITIONS, init::CONTEXT);
            KeyboardBackend::new(keymap)
        };

        backend.set_callbacks(usbd_smart_keyboard::input::smart_keymap::KeymapCallbacks {
            reset: None,
            reset_to_bootloader: Some(keyboard_reset_to_bootloader),
        });

        (
            Shared {
                usb_dev,
                usb_serial,
                usb_class,
            },
            Local {
                alarm,
                keyboard,
                backend,
                report_success: true,
            },
            init::Monotonics(),
        )
    }

    #[task(binds = USBCTRL_IRQ, priority = 2, shared = [usb_dev, usb_serial, usb_class])]
    fn usb_tx(c: usb_tx::Context) {
        let usb_tx::SharedResources {
            usb_dev,
            usb_serial,
            usb_class,
        } = c.shared;
        (usb_dev, usb_serial, usb_class).lock(usb_poll);
    }

    #[task(binds = TIMER_IRQ_0, priority = 1, shared = [usb_class, usb_serial], local = [keyboard, backend, alarm, report_success])]
    fn tick(c: tick::Context) {
        let tick::SharedResources {
            mut usb_class,
            mut usb_serial,
        } = c.shared;
        let tick::LocalResources {
            alarm,
            keyboard,
            backend,
            report_success,
        } = c.local;

        alarm.clear_interrupt();
        alarm.schedule(1.millis()).unwrap();

        for event in keyboard.events() {
            if let Some(event) = keymap_index_of(&KEYMAP_INDICES, event) {
                backend.event(event);
            }
        }
        if *report_success {
            backend.tick();
        }

        usb_class.lock(|k| {
            let res = k
                .device::<NKROBootKeyboard<'_, _>, _>()
                .write_report(backend.pressed_key_codes());
            match res {
                Err(UsbHidError::WouldBlock) => *report_success = false,
                Err(UsbHidError::UsbError(_)) => panic!(),
                Err(UsbHidError::SerializationError) => panic!(),
                Err(UsbHidError::Duplicate) => *report_success = true,
                Ok(_) => *report_success = true,
            }
        });
    }
}
