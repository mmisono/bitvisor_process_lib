#![crate_type = "staticlib"]
#![no_std]
#![feature(asm)]
#![feature(alloc)]
#![feature(allocator_api)]
#![feature(compiler_builtins_lib)]
#![feature(const_fn)]
#![feature(const_size_of)]
#![feature(lang_items)]

extern crate rlibc;
extern crate compiler_builtins;
extern crate spin;
extern crate alloc;

pub mod syscalls;
#[macro_use]
pub mod io;
pub mod mm;

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

