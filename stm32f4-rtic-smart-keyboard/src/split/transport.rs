use embedded_hal_nb::serial::Read;
use embedded_hal_nb::serial::Write;
use nb::block;
use stm32f4xx_hal as hal;

use hal::{
    pac::USART1,
    serial::{Rx, Tx},
};

use smart_keymap::input::Event;

use smart_keymap::split::{receive_byte, Message, BUFFER_SIZE};

pub struct TransportReader {
    pub buf: &'static mut [u8; BUFFER_SIZE],
    pub rx: Rx<USART1>,
}

pub struct TransportWriter {
    pub tx: Tx<USART1>,
}

impl TransportReader {
    pub fn read(&mut self) -> Option<Event> {
        self.rx
            .read()
            .ok()
            .and_then(|b: u8| receive_byte(&mut self.buf, b).ok())
            .map(|Message { input_event }: Message| input_event)
    }
}

impl TransportWriter {
    pub fn write(&mut self, input_event: Event) {
        let message = Message { input_event };
        for b in message.serialize() {
            block!(self.tx.write(b)).unwrap();
        }
        block!(self.tx.flush()).unwrap();
    }
}
