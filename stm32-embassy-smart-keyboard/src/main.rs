#![no_std]
#![no_main]

use core::sync::atomic::{AtomicBool, Ordering};

use defmt::*;
use embassy_executor::Spawner;
use embassy_futures::join::join;
use embassy_stm32::time::Hertz;
use embassy_stm32::usb::Driver;
use embassy_stm32::{bind_interrupts, peripherals, usb, Config};
use embassy_time::Timer;
use embassy_usb::class::hid::{HidReaderWriter, ReportId, RequestHandler, State};
use embassy_usb::control::OutResponse;
use embassy_usb::{Builder, Handler};
use static_cell::StaticCell;
use usbd_hid::descriptor::{
    KeyboardReport, MediaKeyboardReport, MouseReport, SerializedDescriptor,
};
use {defmt_rtt as _, panic_probe as _};

use keyberon_smart_keyboard::input::smart_keymap::keymap_index_of;
use keyberon_smart_keyboard::input::smart_keymap::KeyboardBackend;
use keyberon_smart_keyboard::smart_keymap::key;

use board::KEYMAP_INDICES;

static EP_OUT_BUFFER: StaticCell<[u8; 256]> = StaticCell::new();

static CONFIG_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();
static BOS_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();
static MS_OS_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();
static CONTROL_BUF: StaticCell<[u8; 64]> = StaticCell::new();

static KEYBOARD_BACKEND: StaticCell<KeyboardBackend> = StaticCell::new();

static HID_STATE_MOUSE: StaticCell<State> = StaticCell::new();
static HID_STATE_CONSUMER: StaticCell<State> = StaticCell::new();

bind_interrupts!(struct Irqs {
    OTG_FS => usb::InterruptHandler<peripherals::USB_OTG_FS>;
});

#[cfg(not(custom_board))]
mod board {
    use embassy_stm32::gpio::Input;

    pub const COLS: usize = 1;
    pub const ROWS: usize = 1;

    pub const KEYMAP_INDICES: [[Option<u16>; COLS]; ROWS] = [[Some(0)]];

    pub type Keyboard<'d> = keyberon_smart_keyboard::input::Keyboard<COLS, ROWS, DirectPins<'d>>;

    pub type PressedKeys = keyberon_smart_keyboard::input::PressedKeys<COLS, ROWS>;

    pub struct DirectPins<'d>(pub Input<'d>);

    impl<'d> keyberon_smart_keyboard::input::MatrixScanner<COLS, ROWS> for DirectPins<'d> {
        fn is_boot_key_pressed(&mut self) -> bool {
            self.0.is_low()
        }

        fn get(&mut self) -> Result<[[bool; COLS]; ROWS], core::convert::Infallible> {
            Ok([[self.0.is_low(); COLS]])
        }
    }

    macro_rules! keyboard {
        ($p:ident) => {
            crate::board::Keyboard {
                matrix: crate::board::DirectPins(embassy_stm32::gpio::Input::new(
                    $p.PA0,
                    embassy_stm32::gpio::Pull::Up,
                )),
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

    let ep_out_buffer: &'static mut [u8; 256] = EP_OUT_BUFFER.init([0u8; 256]);
    let mut config = embassy_stm32::usb::Config::default();

    config.vbus_detection = false;

    let driver = Driver::new_fs(p.USB_OTG_FS, Irqs, p.PA12, p.PA11, ep_out_buffer, config);

    let mut config = embassy_usb::Config::new(0xc0de, 0xcafe);
    config.manufacturer = Some("Embassy");
    config.product = Some("HID keyboard example");
    config.serial_number = Some("12345678");
    config.max_power = 100;
    config.max_packet_size_0 = 64;

    let config_descriptor = CONFIG_DESCRIPTOR.init([0; 256]);
    let bos_descriptor = BOS_DESCRIPTOR.init([0; 256]);
    let ms_os_descriptor = MS_OS_DESCRIPTOR.init([0; 256]);
    let control_buf = CONTROL_BUF.init([0; 64]);

    let mut device_handler = MyDeviceHandler::new();

    let mut state_kbd = State::new();
    let state_mouse = HID_STATE_MOUSE.init(State::new());
    let state_consumer = HID_STATE_CONSUMER.init(State::new());

    let mut builder = Builder::new(
        driver,
        config,
        config_descriptor,
        bos_descriptor,
        ms_os_descriptor,
        control_buf,
    );

    builder.handler(&mut device_handler);

    let config = embassy_usb::class::hid::Config {
        report_descriptor: KeyboardReport::desc(),
        request_handler: None,
        poll_ms: 1,
        max_packet_size: 8,
    };
    let hid_kbd = HidReaderWriter::<_, 1, 8>::new(&mut builder, &mut state_kbd, config);

    let config_mouse = embassy_usb::class::hid::Config {
        report_descriptor: MouseReport::desc(),
        request_handler: None,
        poll_ms: 1,
        max_packet_size: 8,
    };
    let hid_mouse = HidReaderWriter::<_, 1, 8>::new(&mut builder, state_mouse, config_mouse);

    let config_consumer = embassy_usb::class::hid::Config {
        report_descriptor: MediaKeyboardReport::desc(),
        request_handler: None,
        poll_ms: 1,
        max_packet_size: 2,
    };
    let hid_consumer =
        HidReaderWriter::<_, 1, 2>::new(&mut builder, state_consumer, config_consumer);

    let mut usb = builder.build();

    let usb_fut = usb.run();

    let (reader_kbd, mut writer_kbd) = hid_kbd.split();
    let (reader_mouse, mut writer_mouse) = hid_mouse.split();
    let (reader_consumer, mut writer_consumer) = hid_consumer.split();

    let mut keyboard = board::keyboard!(p);

    let backend = KEYBOARD_BACKEND.init(KeyboardBackend::new());

    let mut report_success = true;

    let mut last_mouse_report = MouseReport {
        buttons: 0,
        x: 0,
        y: 0,
        wheel: 0,
        pan: 0,
    };
    let mut last_consumer_report = MediaKeyboardReport { usage_id: 0 };

    let in_fut = async {
        loop {
            Timer::after_millis(1).await;

            for event in keyboard.events() {
                if let Some(event) = keymap_index_of(&KEYMAP_INDICES, event) {
                    backend.event(event);
                }
            }
            if report_success {
                backend.tick();
            }

            let report = backend.keymap_output().as_hid_boot_keyboard_report();
            match writer_kbd.write(&report).await {
                Ok(()) => {
                    report_success = true;
                }
                Err(e) => {
                    warn!("Failed to send report: {:?}", e);
                    report_success = false;
                }
            };

            let mouse_output = backend.keymap_output().pressed_mouse_output();
            let mouse_report = MouseReport {
                buttons: mouse_output.pressed_buttons,
                x: mouse_output.x,
                y: mouse_output.y,
                wheel: mouse_output.vertical_scroll,
                pan: mouse_output.horizontal_scroll,
            };
            if mouse_report != last_mouse_report || mouse_output != key::MouseOutput::NO_OUTPUT {
                let buf = [
                    mouse_report.buttons,
                    mouse_report.x as u8,
                    mouse_report.y as u8,
                    mouse_report.wheel as u8,
                    mouse_report.pan as u8,
                ];
                if writer_mouse.write(&buf).await.is_ok() {
                    last_mouse_report = mouse_report;
                }
            }

            let consumer_code = backend
                .keymap_output()
                .pressed_consumer_codes()
                .get(0)
                .copied()
                .unwrap_or(0);
            let consumer_report = MediaKeyboardReport {
                usage_id: consumer_code as u16,
            };
            if consumer_report.usage_id != last_consumer_report.usage_id {
                let buf = consumer_report.usage_id.to_le_bytes();
                if writer_consumer.write(&buf).await.is_ok() {
                    last_consumer_report = consumer_report;
                }
            }
        }
    };

    let out_fut = async {
        let mut request_handler_kbd = MyRequestHandler {};
        let mut request_handler_mouse = MyRequestHandler {};
        let mut request_handler_consumer = MyRequestHandler {};
        join(
            reader_kbd.run(false, &mut request_handler_kbd),
            join(
                reader_mouse.run(false, &mut request_handler_mouse),
                reader_consumer.run(false, &mut request_handler_consumer),
            ),
        )
        .await;
    };

    join(usb_fut, join(in_fut, out_fut)).await;
}

struct MyRequestHandler {}

impl RequestHandler for MyRequestHandler {
    fn get_report(&mut self, id: ReportId, _buf: &mut [u8]) -> Option<usize> {
        info!("Get report for {:?}", id);
        None
    }

    fn set_report(&mut self, id: ReportId, data: &[u8]) -> OutResponse {
        info!("Set report for {:?}: {=[u8]}", id, data);
        OutResponse::Accepted
    }

    fn set_idle_ms(&mut self, id: Option<ReportId>, dur: u32) {
        info!("Set idle rate for {:?} to {:?}", id, dur);
    }

    fn get_idle_ms(&mut self, id: Option<ReportId>) -> Option<u32> {
        info!("Get idle rate for {:?}", id);
        None
    }
}

struct MyDeviceHandler {
    configured: AtomicBool,
}

impl MyDeviceHandler {
    fn new() -> Self {
        MyDeviceHandler {
            configured: AtomicBool::new(false),
        }
    }
}

impl Handler for MyDeviceHandler {
    fn enabled(&mut self, enabled: bool) {
        self.configured.store(false, Ordering::Relaxed);
        if enabled {
            info!("Device enabled");
        } else {
            info!("Device disabled");
        }
    }

    fn reset(&mut self) {
        self.configured.store(false, Ordering::Relaxed);
        info!("Bus reset, the Vbus current limit is 100mA");
    }

    fn addressed(&mut self, addr: u8) {
        self.configured.store(false, Ordering::Relaxed);
        info!("USB address set to: {}", addr);
    }

    fn configured(&mut self, configured: bool) {
        self.configured.store(configured, Ordering::Relaxed);
        if configured {
            info!(
                "Device configured, it may now draw up to the configured current limit from Vbus."
            )
        } else {
            info!("Device is no longer configured, the Vbus current limit is 100mA.");
        }
    }
}
