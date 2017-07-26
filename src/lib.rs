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

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn eh_personality() {}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn panic_fmt() -> ! {
    loop {}
}
