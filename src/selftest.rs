use core::intrinsics::{transmute, size_of};
use core::fmt::Write;

use arch;

pub use drv;


extern {
    static _selftest_start: u8;
    static _selftest_end: u8;
}

pub type TestFn = fn (mut uart : &mut drv::uart::UartWriter) -> bool;

#[packed]
pub struct TestEntry {
    pub func : TestFn,
    pub name : &'static str,
}

#[macro_export]
#[cfg(feature = "selftest")]
macro_rules! selftest {
    ($name:ident ($uart:ident : $uart_t:ty) $code:block) => {

        fn $name($uart: $uart_t) -> bool {

            use ::titanium::selftest::TestEntry;

            static TEST_ENTRY : TestEntry = TestEntry {
                func: $name,
                name: stringify!($name),
            };

            #[allow(dead_code)]
            #[link_section = ".titanium.selftest"]
            static TEST_ENTRY_P : &'static TestEntry = &TEST_ENTRY;

            $code
        }
    }
}

#[macro_export]
#[cfg(not(feature = "selftest"))]
macro_rules! selftest {
    ($name:ident ($uart:ident : $uart_t:ty) $code:block) => {

        #[link_section = ".titanium.discard"]
        fn $name($uart: $uart_t) -> bool {

            use ::titanium::selftest::TestEntry;

            static TEST_ENTRY : TestEntry = TestEntry {
                func: $name,
                name: stringify!($name),
            };

            #[allow(dead_code)]
            #[link_section = ".titanium.discard"]
            static TEST_ENTRY_P : &'static TestEntry = &TEST_ENTRY;

            $code
        }
    }
}

/// Start selftest
#[allow(dead_code)]
fn selftest_run(mut uart : &mut drv::uart::UartWriter) {

    let start : usize = &_selftest_start as *const _ as usize;
    let end : usize = &_selftest_end as *const _ as usize;

    let mut err_i = 0u32;
    let mut ok_i = 0u32;

    write!(uart, "SELFTEST START\n").unwrap();
    for addr in (start..end).step_by(unsafe {size_of::<usize>()}) {

        let t : &TestEntry = {
            unsafe {
                let addr : &usize = transmute(addr);
                transmute(*addr)
            }
        };

        write!(uart, "{}...", t.name).unwrap();
        let ret = (t.func)(uart);
        if ret {
            ok_i += 1;
            write!(uart, " OK\n").unwrap();
        } else {
            err_i += 1;
            write!(uart, " FAIL\n").unwrap();
        }

    }
    write!(uart, "SELFTEST END: {} ERR; {} OK; {} TOTAL\n", err_i, ok_i, ok_i + err_i).unwrap();

    arch::semihosting::exit();
}

#[cfg(feature = "selftest")]
pub fn selftest(uart : &mut drv::uart::UartWriter) {
    selftest_run(uart);
}

#[cfg(not(feature = "selftest"))]
pub fn selftest(_ : &mut drv::uart::UartWriter) {}
