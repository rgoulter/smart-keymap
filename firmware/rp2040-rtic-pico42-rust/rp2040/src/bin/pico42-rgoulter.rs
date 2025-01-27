#![no_std]
#![no_main]

/// Map from [row][col] to (maybe) a row-wise keymap index.
const PICO42_KEYMAP_INDICES: [[Option<u16>; 12]; 4] = [
    [ Some(0),  Some(1),  Some(2),  Some(3),  Some(4), None,     None,      Some(5),  Some(6),  Some(7),  Some(8),  Some(9)],
    [Some(10), Some(11), Some(12), Some(13), Some(14), None,     None,     Some(15), Some(16), Some(17), Some(18), Some(19)],
    [Some(20), Some(21), Some(22), Some(23), Some(24), None,     None,     Some(25), Some(26), Some(27), Some(28), Some(29)],
    [Some(30), Some(31), Some(32), Some(33), Some(34), Some(35), Some(36), Some(37), Some(38), Some(39), Some(40), Some(41)],
];

fn keymap_index_of(ev: keyberon::layout::Event) -> Option<smart_keymap::input::Event> {
    match ev {
        keyberon::layout::Event::Press(r, c) => {
            PICO42_KEYMAP_INDICES[r as usize][c as usize].map(|keymap_index| smart_keymap::input::Event::Press { keymap_index })
        }
        keyberon::layout::Event::Release(r, c) => {
            PICO42_KEYMAP_INDICES[r as usize][c as usize].map(|keymap_index| smart_keymap::input::Event::Release { keymap_index })
        }
    }
}

#[rtic::app(
    device = rp_pico::hal::pac,
)]
mod app {
    use panic_halt as _;

    use rp2040_rtic_pico42_rust_rp2040::app_prelude::*;

    use rp2040_rtic_pico42_rust::input::smart_keymap::KeyboardBackend;
    use rp2040_rtic_pico42_rust::input::PressedKeys12x4;
    use rp2040_rtic_pico42_rust::layouts::split_3x5_3::rgoulter::matrix4x12::{CHORDS, NUM_CHORDS};
    use rp2040_rtic_pico42_rust::matrix::Matrix as DelayedMatrix;
    use rp2040_rtic_pico42_rust_rp2040::keyboards::pykey40;

    #[shared]
    struct Shared {
        usb_dev: UsbDevice,
        usb_class: UsbClass,
    }

    #[local]
    struct Local {
        alarm: timer::Alarm0,
        keyboard: pykey40::Keyboard<NUM_CHORDS>,
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
            app_init::init_usb_device(usb_bus, VID, 0x0005, MANUFACTURER, "Pico42 Keyboard");

        unsafe {
            pac::NVIC::unmask(pac::Interrupt::USBCTRL_IRQ);
            pac::NVIC::unmask(pac::Interrupt::TIMER_IRQ_0);
        };

        let sio = Sio::new(ctx.device.SIO);
        let gpio0 = rp_pico::Pins::new(
            ctx.device.IO_BANK0,
            ctx.device.PADS_BANK0,
            sio.gpio_bank0,
            &mut ctx.device.RESETS,
        );
        let rp_pico::Pins {
            gpio0,
            gpio1,
            gpio2,
            gpio3,
            gpio4,
            gpio5,
            gpio6,
            gpio7,
            gpio8,
            gpio9,
            gpio10,
            gpio11,
            gpio14,
            gpio15,
            gpio16,
            gpio17,
            ..
        } = gpio0;
        let cols = pykey40::cols(
            gpio0, gpio1, gpio2, gpio3, gpio4, gpio5, gpio6, gpio7, gpio8, gpio9, gpio10, gpio11,
        );
        let rows = pykey40::rows(gpio14, gpio15, gpio16, gpio17);
        let matrix = DelayedMatrix::new(cols, rows, timer, 5, 5).unwrap();
        let keyboard = pykey40::Keyboard {
            matrix,
            debouncer: Debouncer::new(PressedKeys12x4::default(), PressedKeys12x4::default(), 25),
            chording: Chording::new(&CHORDS),
        };

        let backend = {
            use smart_keymap::keymap::Keymap;
            use smart_keymap::init;
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
            if let Some(event) = crate::keymap_index_of(event) {
                backend.event(event);
            }
        }
        backend.tick();

        usb_class.lock(|k| {
            backend.write_reports(k);
        });
    }
}
