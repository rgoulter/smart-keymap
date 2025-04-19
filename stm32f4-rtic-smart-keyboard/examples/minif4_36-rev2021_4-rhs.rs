#![no_main]
#![no_std]

#[cfg(not(custom_board))]
mod board {
    pub use stm32f4_rtic_smart_keyboard::input::Input;

    pub use stm32f4_rtic_smart_keyboard::app_prelude::VID;

    pub const COLS: usize = 5;
    pub const ROWS: usize = 4;

    #[rustfmt::skip]
    pub const KEYMAP_INDICES: [[Option<u16>; COLS]; ROWS] = [
        [ Some(5),  Some(6),  Some(7),  Some(8),  Some(9)],
        [Some(15), Some(16), Some(17), Some(18), Some(19)],
        [Some(25), Some(26), Some(27), Some(28), Some(29)],
        [Some(33), Some(34), Some(35),     None,     None],
    ];

    pub const PID: u16 = 0x0005;
    pub const MANUFACTURER: &str = "smart-keyboard";
    pub const PRODUCT: &str = "MiniF4-36 rev2021.4 RHS";

    pub type Keyboard = usbd_smart_keyboard::input::Keyboard<COLS, ROWS, Matrix>;

    pub type PressedKeys = usbd_smart_keyboard::input::PressedKeys<COLS, ROWS>;

    pub type Matrix = usbd_smart_keyboard::matrix::DirectPinMatrix<Input, COLS, ROWS>;

    macro_rules! keyboard {
        ($gpioa:ident, $gpiob:ident) => {
            crate::board::Keyboard {
                matrix: crate::board::Matrix::new([
                    [
                        Some($gpioa.pa2.into_pull_up_input().erase()),
                        Some($gpioa.pa10.into_pull_up_input().erase()),
                        Some($gpioa.pa9.into_pull_up_input().erase()),
                        Some($gpioa.pa8.into_pull_up_input().erase()),
                        Some($gpiob.pb15.into_pull_up_input().erase()),
                    ],
                    [
                        Some($gpiob.pb10.into_pull_up_input().erase()),
                        Some($gpiob.pb4.into_pull_up_input().erase()),
                        Some($gpiob.pb3.into_pull_up_input().erase()),
                        Some($gpioa.pa15.into_pull_up_input().erase()),
                        Some($gpiob.pb5.into_pull_up_input().erase()),
                    ],
                    [
                        Some($gpioa.pa6.into_pull_up_input().erase()),
                        Some($gpioa.pa7.into_pull_up_input().erase()),
                        Some($gpiob.pb0.into_pull_up_input().erase()),
                        Some($gpiob.pb1.into_pull_up_input().erase()),
                        Some($gpioa.pa1.into_pull_up_input().erase()),
                    ],
                    [
                        Some($gpioa.pa3.into_pull_up_input().erase()),
                        Some($gpioa.pa4.into_pull_up_input().erase()),
                        Some($gpioa.pa5.into_pull_up_input().erase()),
                        None,
                        None,
                    ],
                ]),
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

#[rtic::app(device = stm32f4xx_hal::pac, peripherals = true, dispatchers = [SPI1])]
mod app {
    // set the panic handler
    use panic_rtt_target as _;

    use rtt_target::{rprintln, rtt_init_print};
    use usb_device::bus::UsbBusAllocator;
    use usbd_human_interface_device::device::keyboard::NKROBootKeyboard;
    use usbd_human_interface_device::UsbHidError;

    use smart_keymap::split::BUFFER_SIZE;
    use usbd_smart_keyboard::input::smart_keymap::keymap_index_of;
    use usbd_smart_keyboard::input::smart_keymap::KeyboardBackend;

    use stm32f4_rtic_smart_keyboard::split::app_prelude::*;

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
        split_conn_tx: TransportWriter,
        split_conn_rx: TransportReader,
    }

    #[init(local = [
        ep_memory: [u32; 1024] = [0; 1024],
        rx_buf: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE],
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
            // pac::NVIC::unmask(pac::Interrupt::USART1); // ??
        }

        let keyboard = board::keyboard!(gpioa, gpiob);

        let backend = {
            use smart_keymap::init;
            use smart_keymap::keymap::Keymap;
            let keymap = Keymap::new(init::KEY_DEFINITIONS, init::CONTEXT);
            KeyboardBackend::new(keymap)
        };

        let (split_conn_tx, split_conn_rx) = split_app_init::init_serial(
            &clocks,
            (gpiob.pb6, gpiob.pb7),
            c.device.USART1,
            c.local.rx_buf,
        );

        (
            SharedResources { usb_dev, usb_class },
            LocalResources {
                timer,
                keyboard,
                backend,
                report_success: true,
                split_conn_rx,
                split_conn_tx,
            },
            init::Monotonics(),
        )
    }

    #[task(binds = USART1, priority = 5, local = [split_conn_rx])]
    fn rx(c: rx::Context) {
        let rx::LocalResources { split_conn_rx } = c.local;
        if let Some(event) = split_conn_rx.read() {
            layout::spawn(LayoutMessage::Event(event)).unwrap();
        }
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

    #[task(priority = 3, capacity = 8, shared = [usb_class, usb_dev], local = [backend, report_success])]
    fn layout(c: layout::Context, message: LayoutMessage) {
        let layout::SharedResources {
            mut usb_class,
            mut usb_dev,
        } = c.shared;
        let layout::LocalResources {
            backend,
            report_success,
        } = c.local;
        match message {
            LayoutMessage::Tick => {
                if *report_success {
                    backend.tick();
                }

                if usb_dev.lock(|d| d.state()) != UsbDeviceState::Configured {
                    return;
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
            LayoutMessage::Event(event) => {
                backend.event(event);
            }
        };
    }

    #[task(binds = TIM3, priority = 1, local = [keyboard, timer, split_conn_tx])]
    fn tick(c: tick::Context) {
        let tick::LocalResources {
            keyboard,
            split_conn_tx,
            timer,
        } = c.local;

        timer.start(1.millis()).ok();

        for event in keyboard.events() {
            if let Some(event) = keymap_index_of(&KEYMAP_INDICES, event) {
                split_conn_tx.write(event);
                layout::spawn(LayoutMessage::Event(event)).unwrap();
            }
        }

        layout::spawn(LayoutMessage::Tick).unwrap();
    }
}
