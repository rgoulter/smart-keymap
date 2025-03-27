use stm32f4xx_hal as hal;

use hal::{
    gpio::gpiob,
    pac::USART1,
    rcc::Clocks,
    serial::config::Config,
    serial::{Event, Serial},
    time::U32Ext,
    Listen,
};

use usbd_smart_keyboard::split::transport::BUFFER_LENGTH;

use crate::split::transport::{TransportReader, TransportWriter};

pub fn init_serial(
    clocks: &Clocks,
    (pb6, pb7): (gpiob::PB6, gpiob::PB7),
    usart1: USART1,
    buf: &'static mut [u8; BUFFER_LENGTH],
) -> (TransportWriter, TransportReader) {
    let pins = (pb6.into_alternate(), pb7.into_alternate());
    let mut serial = Serial::new(
        usart1,
        pins,
        Config::default().baudrate(9_600.bps()),
        &clocks,
    )
    .unwrap();
    serial.listen(Event::RxNotEmpty);

    let (tx, rx) = serial.split();
    (TransportWriter { tx }, TransportReader { buf, rx })
}
