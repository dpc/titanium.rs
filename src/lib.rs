#![feature(no_std)]
#![feature(core)]
#![feature(lang_items)]
#![feature(asm)]
#![feature(static_assert)]
#![feature(step_by)]
#![no_std]

#![feature(trace_macros)]

#![crate_name="titanium"]

#[macro_use]
extern crate core;

#[macro_use]
pub mod macros;

#[macro_use]
pub mod arch;

pub mod lang;
pub mod drv;
pub mod consts;
pub mod hw;

pub use hw::HW;

#[macro_use]
pub mod selftest;

mod titanium {
    pub use super::selftest;
}
