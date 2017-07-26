extern crate core;

#[allow(dead_code)]
pub enum MessageType {
    INT = 0,
    BUF = 1,
}

#[allow(dead_code)]
#[repr(i32)]
enum SyscallNumber {
    NOP = 1,
    MSGSETFUNC = 3,
    MSGREGISTER = 4,
    MSGOPEN = 5,
    MSGCLOSE = 6,
    MSGSENDINT = 7,
    MSGRET = 8,
    MSGSENDDESC = 9,
    NEWPROCESS = 10,
    MSGSENDBUF = 11,
    MSGUNREGISTER = 12,
    EXITPROCESS = 13,
    SETLIMIT = 14,
}

// TODO: support x86

macro_rules! syscall0 {
    ($rb:expr, $ra:expr) =>{
        unsafe {
            asm!("syscall"
                    : "={rax}" ($ra)
                    : "{rbx}" ($rb as u64)
                    : "memory", "cc", "rcx", "rdx"
                      ,"r8", "r9", "r10", "r11", "r12", "r13"
                      ,"r14", "r15"
                    : "volatile");
        }
    }
}

macro_rules! syscall1 {
    ($rb:expr, $rs:expr, $ra:expr) =>{
        unsafe {
            asm!("syscall"
                    : "={rax}" ($ra)
                    : "{rbx}" ($rb as u64),
                      "{rsi}" ($rs as u64)
                    : "memory", "cc", "rcx", "rdx"
                      ,"r8", "r9", "r10", "r11", "r12", "r13"
                      ,"r14", "r15"
                    : "volatile");
        }
    }
}

macro_rules! syscall2 {
    ($rb:expr, $rs:expr, $rd:expr, $ra:expr) =>{
        unsafe {
            asm!("syscall"
                    : "={rax}" ($ra)
                    : "{rbx}" ($rb as u64),
                      "{rsi}" ($rs as u64),
                      "{rdi}" ($rd as u64)
                    : "memory", "cc", "rcx", "rdx"
                      ,"r8", "r9", "r10", "r11", "r12", "r13"
                      ,"r14", "r15"
                    : "volatile");
        }
    }
}

// TODO: test (only few systemcalls are checked)

#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(unused_mut)]
pub fn nop() -> () {
    let mut tmp: u64;
    syscall0!(SyscallNumber::NOP, tmp);
}

#[allow(unused_mut)]
pub fn msgsetfunc(desc: i32, func: *mut u64) -> *const u64 {
    let mut tmp: u64;
    syscall2!(SyscallNumber::MSGSETFUNC, desc, func, tmp);
    return tmp as *const u64;
}

#[allow(unused_mut)]
pub fn msgregister(name: *const u8, func: *mut u64) -> *const u64 {
    let mut tmp: u64;
    syscall2!(SyscallNumber::MSGREGISTER, name, func, tmp);
    return tmp as *const u64;
}

#[allow(unused_mut)]
pub fn msgopen(name: *const u8) -> i32 {
    let mut tmp: u64;
    syscall1!(SyscallNumber::MSGOPEN, name, tmp);
    return tmp as i32;
}

#[allow(unused_mut)]
pub fn msgclose(desc: i32) -> i32 {
    let mut tmp: u64;
    syscall1!(SyscallNumber::MSGCLOSE, desc, tmp);
    return tmp as i32;
}

#[allow(unused_mut)]
pub fn msgsendint(desc: i32, data: i32) -> i32 {
    let mut tmp: u64;
    syscall2!(SyscallNumber::MSGSENDINT, desc, data, tmp);
    return tmp as i32;
}

#[allow(unused_mut)]
pub fn msgsenddesc(desc: i32, data: i32) -> i32 {
    let mut tmp: u64;
    syscall2!(SyscallNumber::MSGSENDDESC, desc, data, tmp);
    return tmp as i32;
}

#[repr(C)]
pub struct msgbuf {
    base: *mut u8,
    len: u32,
    rw: i32,
    premap_handle: i64,
}

#[repr(C)]
struct msgsendbuf_args {
    data: i32,
    bufcnt: i32,
    buf: *mut msgbuf,
}

#[allow(unused_mut)]
pub fn msgsendbuf(desc: i32, data: i32, buf: *mut msgbuf, bufcnt: i32) -> i32 {
    let mut tmp: u64;
    let mut a = msgsendbuf_args { data, bufcnt, buf };
    syscall2!(
        SyscallNumber::MSGSENDBUF,
        desc,
        core::intrinsics::transmute::<&msgsendbuf_args, u64>(&a),
        tmp
    );
    return tmp as i32;
}

#[allow(unused_mut)]
pub fn msgunregister(desc: i32) -> i32 {
    let mut tmp: u64;
    syscall1!(SyscallNumber::MSGUNREGISTER, desc, tmp);
    return tmp as i32;
}

#[allow(unused_mut)]
pub fn newprocess(name: *const u8) -> i32 {
    let mut tmp: u64;
    syscall1!(SyscallNumber::NEWPROCESS, name, tmp);
    return tmp as i32;
}

#[allow(unused_mut)]
#[allow(unused_variables)]
#[allow(unused_assignments)]
pub fn exitprocess(retval: i32) -> () {
    let mut tmp: u64;
    syscall1!(SyscallNumber::EXITPROCESS, retval, tmp);
}

#[allow(unused_mut)]
pub fn setlimit(stacksize: i32, maxstacksize: i32) -> i32 {
    let mut tmp: u64;
    syscall2!(SyscallNumber::SETLIMIT, stacksize, maxstacksize, tmp);
    return tmp as i32;
}
