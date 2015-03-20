use core::intrinsics::{transmute, size_of};
use core::iter::range_step;
use core::fmt::Write;

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

selftest!(testx (uart : &mut drv::uart::UartWriter) {
    write!(uart, "testx").unwrap();
});


selftest!(testy (uart : &mut drv::uart::UartWriter) {
    write!(uart, "testx").unwrap();
});


#[allow(dead_code)]
fn selftest_run(mut uart : &mut drv::uart::UartWriter) {

    let start : usize = &_selftest_start as *const _ as usize;
    let end : usize = &_selftest_end as *const _ as usize;

    for test in range_step(start, end, unsafe {size_of::<usize>()}) {

        let f : fn (mut uart : &mut drv::uart::UartWriter) = {
            unsafe {
                let addr : &usize = transmute(test);
                transmute(*addr)
            }
        };

        write!(uart, "Test...").unwrap();
        (f)(uart);
        write!(uart, " done\n").unwrap();
    }
}

#[cfg(feature = "selftest")]
pub fn selftest(uart : &mut drv::uart::UartWriter) {
    selftest_run(uart);
}

#[cfg(not(feature = "selftest"))]
pub fn selftest(_ : &mut drv::uart::UartWriter) {}
