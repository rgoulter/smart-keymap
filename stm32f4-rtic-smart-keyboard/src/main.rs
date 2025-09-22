#![no_main]
#![no_std]

#[cfg(not(custom_board))]
mod board {
    use stm32f4xx_hal as hal;

    use hal::gpio::{gpioa, Input};

    pub const COLS: usize = 1;
    pub const ROWS: usize = 1;

    pub const KEYMAP_INDICES: [[Option<u16>; COLS]; ROWS] = [[Some(0)]];

    pub use stm32f4_rtic_smart_keyboard::app_prelude::VID;

    pub const PID: u16 = 0x0005;
    pub const MANUFACTURER: &str = "smart-keyboard";
    pub const PRODUCT: &str = "STM32F4 Keyboard";

    pub type Keyboard = keyberon_smart_keyboard::input::Keyboard<COLS, ROWS, DirectPins>;

    pub type PressedKeys = keyberon_smart_keyboard::input::PressedKeys<COLS, ROWS>;

    pub struct DirectPins(pub gpioa::PA0<Input>);

    impl keyberon_smart_keyboard::input::MatrixScanner<COLS, ROWS> for DirectPins {
        fn is_boot_key_pressed(&mut self) -> bool {
            self.0.is_low()
        }

        fn get(&mut self) -> Result<[[bool; COLS]; ROWS], core::convert::Infallible> {
            Ok([[self.0.is_low(); COLS]])
        }
    }

    macro_rules! keyboard {
        ($a:ident, $b:ident) => {
            crate::board::Keyboard {
                matrix: crate::board::DirectPins($a.pa0.into_pull_up_input()),
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

#[cfg(custom_board)]
include!(concat!(env!("OUT_DIR"), "/board.rs"));

#[rtic::app(device = stm32f4xx_hal::pac, peripherals = true)]
mod app {
    // set the panic handler
    use panic_rtt_target as _;

    use rtt_target::{rprintln, rtt_init_print};
    use usb_device::bus::UsbBusAllocator;
    use usbd_human_interface_device::device::keyboard::NKROBootKeyboard;
    use usbd_human_interface_device::UsbHidError;

    use keyberon_smart_keyboard::input::smart_keymap::keymap_index_of;
    use keyberon_smart_keyboard::input::smart_keymap::KeyboardBackend;

    use stm32f4_rtic_smart_keyboard::app_prelude::*;

    use super::board;

    use board::Keyboard;
    use board::KEYMAP_INDICES;

    #[shared]
    struct SharedResources {
        usb_dev: UsbDevice,
        usb_class: UsbClass,
    }

    #[local]
    struct LocalResources {
        keyboard: Keyboard,
        backend: KeyboardBackend,
        report_success: bool,
        timer: timer::CounterUs<pac::TIM3>,
    }

    #[init(local = [
        ep_memory: [u32; 1024] = [0; 1024],
        usb_bus: Option<UsbBusAllocator<UsbBusType>> = None
    ])]
    fn init(c: init::Context) -> (SharedResources, LocalResources, init::Monotonics) {
        let rcc = c.device.RCC.constrain();
        let clocks = app_init::init_clocks(rcc);

        rtt_init_print!();
        rprintln!("init");

        let gpioa = c.device.GPIOA.split();
        let gpiob = c.device.GPIOB.split();
        let _ = gpiob;

        let usb = USB::new(
            (
                c.device.OTG_FS_GLOBAL,
                c.device.OTG_FS_DEVICE,
                c.device.OTG_FS_PWRCLK,
            ),
            (gpioa.pa11, gpioa.pa12),
            &clocks,
        );
        *c.local.usb_bus = Some(UsbBusType::new(usb, c.local.ep_memory));
        let usb_bus = c.local.usb_bus.as_ref().unwrap();

        let (usb_dev, usb_class) = app_init::init_usb_device(
            usb_bus,
            board::VID,
            board::PID,
            board::MANUFACTURER,
            board::PRODUCT,
        );

        let timer = app_init::init_timer(&clocks, c.device.TIM3);
        unsafe {
            pac::NVIC::unmask(pac::Interrupt::TIM3);
        }

        let keyboard = board::keyboard!(gpioa, gpiob);

        let backend = KeyboardBackend::new();

        (
            SharedResources { usb_dev, usb_class },
            LocalResources {
                timer,
                keyboard,
                backend,
                report_success: true,
            },
            init::Monotonics(),
        )
    }

    #[task(binds = OTG_FS, priority = 2, shared = [usb_dev,  usb_class])]
    fn usb_tx(c: usb_tx::Context) {
        let usb_tx::SharedResources { usb_dev, usb_class } = c.shared;
        (usb_dev, usb_class).lock(usb_poll);
    }

    #[task(binds = OTG_FS_WKUP, priority = 2, shared = [usb_dev,  usb_class])]
    fn usb_rx(c: usb_rx::Context) {
        let usb_rx::SharedResources { usb_dev, usb_class } = c.shared;
        (usb_dev, usb_class).lock(usb_poll);
    }

    #[task(binds = TIM3, priority = 1, shared = [usb_class], local = [keyboard, backend, report_success, timer])]
    fn tick(c: tick::Context) {
        let tick::SharedResources { mut usb_class } = c.shared;
        let tick::LocalResources {
            keyboard,
            backend,
            report_success,
            timer,
        } = c.local;

        timer.start(1.millis()).ok();

        for event in keyboard.events() {
            if let Some(event) = keymap_index_of(&KEYMAP_INDICES, event) {
                backend.event(event);
            }
        }
        if *report_success {
            backend.tick();
        }

        usb_class.lock(|k| {
            let res = k.device::<NKROBootKeyboard<'_, _>, _>().write_report(
                backend
                    .keymap_output()
                    .pressed_key_codes()
                    .iter()
                    .map(|&key| key.into()),
            );
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
