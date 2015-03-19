use core::intrinsics::{transmute, size_of};
use core::iter::range_step;

pub use drv;

extern {
    static _selftest_start: u64;
    static _selftest_end: u64;
}


#[macro_export]
macro_rules! selftest {
    ($name:ident ($uart:ident : $uart_t:ty) $code:block) => {

        fn $name($uart: $uart_t) {

            #[allow(dead_code)]
            #[link_section = ".selftest"]
            static POINTER : fn (mut uart : $uart_t) = $name;

            $code
        }
    }
}

selftest!(testx (uart : &mut drv::Uart) {
    uart.put('x' as u8);
});


selftest!(testy (uart : &mut drv::Uart) {
    uart.put('y' as u8);
});


#[allow(dead_code)]
fn selftest_run(mut uart : &mut drv::Uart) {

    let start : usize = &_selftest_start as *const _ as usize;
    let end : usize = &_selftest_end as *const _ as usize;

    for test in range_step(start, end, unsafe {size_of::<usize>()}) {

        let f : fn (mut uart : &mut drv::Uart) = {
            unsafe {
                let addr : &usize = transmute(test);
                transmute(*addr)
            }
        };

        (f)(uart);
    }
}

#[cfg(feature = "selftest")]
pub fn selftest(uart : &mut drv::Uart) {
    selftest_run(uart);
}

#[cfg(not(feature = "selftest"))]
pub fn selftest(_ : &mut drv::Uart) {}
