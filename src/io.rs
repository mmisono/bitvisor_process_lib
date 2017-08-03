extern crate core;
use core::fmt;
use core::str;
use core::str::Utf8Error;

use syscalls;

fn putchar(c: u8) {
    syscalls::msgsendint(1, c as i32);
}

pub struct Writer;

impl Writer {
    pub fn write_byte(b: u8) {
        putchar(b);
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for b in s.bytes() {
            Writer::write_byte(b);
        }

        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    () => ();
    ($fmt:expr) => {
        write!(::io::Writer, $fmt).unwrap();
    };
    ($fmt:expr, $($arg:tt)*) => {
        write!(::io::Writer, $fmt, $($arg)*).unwrap();
    };
}

#[macro_export]
macro_rules! println {
    () => {
        print!("\n")
    };
    ($fmt:expr) => {
        print!(concat!($fmt, "\n"))
    };
    ($fmt:expr, $($arg:tt)*) => {
        print!(concat!($fmt, "\n"), $($arg)*)
    };
}

pub fn lineinput_desc(kbd: i32, dsp: i32, buf: &mut [u8]) -> Result<&str, Utf8Error> {
    let mut i: usize = 0;
    loop {
        let c = syscalls::msgsendint(kbd, 0);
        match c as u8 {
            b'\x08' => {
                // backspace
                if i > 0 {
                    i -= 1;
                    syscalls::msgsendint(dsp, c);
                    syscalls::msgsendint(dsp, ' ' as i32);
                    syscalls::msgsendint(dsp, c);
                }
            }
            b'\r' | b'\n' => {
                syscalls::msgsendint(dsp, c);
                break;
            }
            b' '...b'~' => {
                if i < buf.len() {
                    buf[i] = c as u8;
                }
                i += 1;
                syscalls::msgsendint(dsp, c);
            }
            _ => (),
        }
    }
    if i > buf.len() {
        i = buf.len();
    }
    return str::from_utf8(&buf[..i]);
}

pub fn lineinput(buf: &mut [u8]) -> Result<&str, Utf8Error> {
    return lineinput_desc(0, 1, buf);
}
