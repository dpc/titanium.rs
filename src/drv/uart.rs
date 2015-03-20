use core::fmt;
use core::str::StrExt;
use core::result::Result;

pub trait Uart {
    fn put(&self, ch : u8);
}

pub struct UartWriter<'a> {
    uart : &'a Uart
}


impl<'a> UartWriter<'a> {
    pub fn new(uart : &'a Uart) -> UartWriter<'a> {
        UartWriter { uart: uart }
    }
}

impl<'a> fmt::Write for UartWriter<'a> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for ch in s.bytes() {
            self.uart.put(ch);
        }
        Result::Ok(())
    }
}
