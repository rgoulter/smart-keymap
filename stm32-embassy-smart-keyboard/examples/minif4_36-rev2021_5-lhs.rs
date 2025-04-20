#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_futures::join::join;
use embassy_stm32::mode::Async;
use embassy_stm32::time::Hertz;
use embassy_stm32::usart::{HalfDuplexConfig, HalfDuplexReadback, Uart};
use embassy_stm32::usb::Driver;
use embassy_stm32::{bind_interrupts, peripherals, usart, usb, Config};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::channel::{Channel, Sender};
use embassy_time::Timer;
use embassy_usb::class::hid::{
    HidReader, HidReaderWriter, HidWriter, ReportId, RequestHandler, State,
};
use embassy_usb::control::OutResponse;
use embassy_usb::Builder;
use panic_halt as _;
use static_cell::StaticCell;
use usbd_hid::descriptor::{KeyboardReport, SerializedDescriptor};

use keyberon_smart_keyboard::input::smart_keymap::keymap_index_of;
use keyberon_smart_keyboard::input::smart_keymap::KeyboardBackend;
use keyberon_smart_keyboard::input::MatrixScanner;
use keyberon_smart_keyboard::split::BackendMessage;

use smart_keymap::input::Event;
use smart_keymap::split::{Message, BUFFER_SIZE};

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

static EP_OUT_BUFFER: StaticCell<[u8; 256]> = StaticCell::new();

static CONFIG_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();
static BOS_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();
static MS_OS_DESCRIPTOR: StaticCell<[u8; 256]> = StaticCell::new();
static CONTROL_BUF: StaticCell<[u8; 64]> = StaticCell::new();

static HID_STATE: StaticCell<State> = StaticCell::new();

static KEYBOARD_BACKEND: StaticCell<KeyboardBackend> = StaticCell::new();

static BACKEND_CHANNEL: Channel<ThreadModeRawMutex, BackendMessage, 8> = Channel::new();

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
        [ Some(0),  Some(1),  Some(2),  Some(3),  Some(4)],
        [Some(10), Some(11), Some(12), Some(13), Some(14)],
        [Some(20), Some(21), Some(22), Some(23), Some(24)],
        [Some(30), Some(31), Some(32),     None,     None],
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
                    embassy_stm32::gpio::Input::new($p.PB12, embassy_stm32::gpio::Pull::Up),
                    embassy_stm32::gpio::Input::new($p.PB15, embassy_stm32::gpio::Pull::Up),
                    embassy_stm32::gpio::Input::new($p.PA9, embassy_stm32::gpio::Pull::Up),
                    embassy_stm32::gpio::Input::new($p.PA5, embassy_stm32::gpio::Pull::Up),
                    embassy_stm32::gpio::Input::new($p.PB3, embassy_stm32::gpio::Pull::Up),
                    embassy_stm32::gpio::Input::new($p.PB13, embassy_stm32::gpio::Pull::Up),
                    embassy_stm32::gpio::Input::new($p.PA8, embassy_stm32::gpio::Pull::Up),
                    embassy_stm32::gpio::Input::new($p.PA10, embassy_stm32::gpio::Pull::Up),
                    embassy_stm32::gpio::Input::new($p.PA15, embassy_stm32::gpio::Pull::Up),
                    embassy_stm32::gpio::Input::new($p.PB10, embassy_stm32::gpio::Pull::Up),
                    embassy_stm32::gpio::Input::new($p.PB14, embassy_stm32::gpio::Pull::Up),
                    embassy_stm32::gpio::Input::new($p.PB1, embassy_stm32::gpio::Pull::Up),
                    embassy_stm32::gpio::Input::new($p.PA6, embassy_stm32::gpio::Pull::Up),
                    embassy_stm32::gpio::Input::new($p.PA4, embassy_stm32::gpio::Pull::Up),
                    embassy_stm32::gpio::Input::new($p.PB5, embassy_stm32::gpio::Pull::Up),
                    embassy_stm32::gpio::Input::new($p.PA2, embassy_stm32::gpio::Pull::Up),
                    embassy_stm32::gpio::Input::new($p.PA0, embassy_stm32::gpio::Pull::Up),
                    embassy_stm32::gpio::Input::new($p.PC13, embassy_stm32::gpio::Pull::Up),
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
async fn main(spawner: Spawner) {
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

    let driver: Driver<'static, _> =
        Driver::new_fs(p.USB_OTG_FS, Irqs, p.PA12, p.PA11, ep_out_buffer, config);

    let mut config = embassy_usb::Config::new(0xc0de, 0xcafe);
    config.manufacturer = Some("Embassy");
    config.product = Some("STM32F4 rev2021.5 LHS");
    config.serial_number = Some("12345678");
    config.max_power = 100;
    config.max_packet_size_0 = 64;

    let config_descriptor = CONFIG_DESCRIPTOR.init([0; 256]);
    let bos_descriptor = BOS_DESCRIPTOR.init([0; 256]);
    let ms_os_descriptor = MS_OS_DESCRIPTOR.init([0; 256]);
    let control_buf = CONTROL_BUF.init([0; 64]);

    let mut request_handler = MyRequestHandler {};

    let state = HID_STATE.init(State::new());

    let mut builder = Builder::new(
        driver,
        config,
        config_descriptor,
        bos_descriptor,
        ms_os_descriptor,
        control_buf,
    );

    let config = embassy_usb::class::hid::Config {
        report_descriptor: KeyboardReport::desc(),
        request_handler: None,
        poll_ms: 1,
        max_packet_size: 8,
    };

    let hid = HidReaderWriter::<_, 1, 8>::new(&mut builder, state, config);

    let mut usb = builder.build();

    let usb_fut = usb.run();

    let (reader, writer): (
        HidReader<'static, Driver<'static, _>, 1>,
        HidWriter<'static, Driver<'static, _>, 8>,
    ) = hid.split();

    let mut keyboard = board::keyboard!(p);

    if keyboard.matrix.is_boot_key_pressed() {
        unsafe {
            enter_bootloader_tinyuf2();
        }
    }

    let matrix_scan_sender: Sender<'static, ThreadModeRawMutex, BackendMessage, 8> =
        BACKEND_CHANNEL.sender();

    let config = usart::Config::default();
    let uart = Uart::new_half_duplex(
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

    let out_fut = async {
        reader.run(false, &mut request_handler).await;
    };

    spawner.spawn(keyboard_backend(writer)).unwrap();
    spawner
        .spawn(keyboard_matrix_scan(matrix_scan_sender, keyboard))
        .unwrap();
    spawner
        .spawn(keyboard_split_rx(matrix_scan_sender, uart))
        .unwrap();

    join(usb_fut, out_fut).await;
}

#[embassy_executor::task]
async fn keyboard_backend(
    mut writer: HidWriter<'static, Driver<'static, peripherals::USB_OTG_FS>, 8>,
) {
    let backend = KEYBOARD_BACKEND.init({
        use smart_keymap::init;
        use smart_keymap::keymap::Keymap;
        let keymap = Keymap::new(init::KEY_DEFINITIONS, init::CONTEXT);
        KeyboardBackend::new(keymap)
    });

    let mut report_success = true;

    loop {
        match BACKEND_CHANNEL.receive().await {
            BackendMessage::Event(event) => {
                backend.event(event);
            }
            BackendMessage::Tick => {
                if report_success {
                    backend.tick();
                }

                let report = backend.keymap_output().as_hid_boot_keyboard_report();
                match writer.write(&report).await {
                    Ok(()) => {
                        report_success = true;
                    }
                    Err(_e) => {
                        report_success = false;
                    }
                };
            }
        }
    }
}

#[embassy_executor::task]
async fn keyboard_matrix_scan(
    matrix_scan_sender: Sender<'static, ThreadModeRawMutex, BackendMessage, 8>,
    mut keyboard: board::Keyboard<'static>,
) {
    loop {
        Timer::after_millis(1).await;

        for event in keyboard.events() {
            if let Some(event) = keymap_index_of(&KEYMAP_INDICES, event) {
                matrix_scan_sender.send(BackendMessage::Event(event)).await;
            }
        }
        matrix_scan_sender.send(BackendMessage::Tick).await;
    }
}

#[embassy_executor::task]
async fn keyboard_split_rx(
    matrix_scan_sender: Sender<'static, ThreadModeRawMutex, BackendMessage, 8>,
    uart: Uart<'static, Async>,
) {
    let mut transport_reader = TransportReader::new(uart);
    loop {
        if let Some(event) = transport_reader.read().await {
            matrix_scan_sender.send(BackendMessage::Event(event)).await;
        }
    }
}

struct MyRequestHandler {}

impl RequestHandler for MyRequestHandler {
    fn get_report(&mut self, _id: ReportId, _buf: &mut [u8]) -> Option<usize> {
        None
    }

    fn set_report(&mut self, _id: ReportId, _data: &[u8]) -> OutResponse {
        OutResponse::Accepted
    }

    fn set_idle_ms(&mut self, _id: Option<ReportId>, _dur: u32) {}

    fn get_idle_ms(&mut self, _id: Option<ReportId>) -> Option<u32> {
        None
    }
}

struct TransportReader<'d> {
    pub uart: usart::Uart<'d, Async>,
}

impl<'d> TransportReader<'d> {
    pub fn new(rx: usart::Uart<'d, Async>) -> Self {
        TransportReader { uart: rx }
    }

    pub async fn read(&mut self) -> Option<Event> {
        let mut buf: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
        if let Ok(_) = self.uart.read(&mut buf).await {
            Message::deserialize(&buf)
                .ok()
                .map(|Message { input_event }: Message| input_event)
        } else {
            None
        }
    }
}
