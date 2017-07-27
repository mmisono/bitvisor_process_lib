#![crate_type = "staticlib"]
#![feature(lang_items)]
#![no_std]
#![feature(compiler_builtins_lib)]
#![feature(asm)]

extern crate rlibc;
extern crate compiler_builtins;

pub mod syscalls;
#[macro_use]
pub mod io;

use core::fmt::Write;

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str, line: u32) -> ! {
    println!("program panic at '{}', {}:{}", fmt, file, line);
    println!("process terminated.");
    syscalls::exitprocess(1);
    unreachable!();
}
