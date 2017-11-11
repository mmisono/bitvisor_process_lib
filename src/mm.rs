use alloc::heap::{Alloc, AllocErr, Layout};
use spin::Mutex;
use core::mem;

pub struct Allocator;

#[derive(Debug)]
pub struct Address {
    start: usize,
    end: usize,
}

#[repr(C)]
#[derive(Debug)]
struct HeapElem {
    free: u32,
    size: usize, // size includes HEADER_LEN
}

const HEADER_LEN: usize = mem::size_of::<HeapElem>();

impl HeapElem {
    pub fn free(&self) -> bool {
        self.free != 0
    }

    pub fn len(&self) -> usize {
        self.size
    }

    // split from tail
    // XXX: should return Result
    pub unsafe fn split(&mut self, size: usize, align: usize) -> Option<usize> {
        if !self.free() {
            return None;
        }

        let current_addr = self as *const _ as usize;
        let tail = current_addr + self.len();
        let start = tail - align_floor(size, align);
        let new_addr = start - HEADER_LEN;

        if new_addr < current_addr + HEADER_LEN {
            return None;
        }

        let new_elem: &mut HeapElem = mem::transmute(new_addr);
        new_elem.free = 0;
        new_elem.size = tail - new_addr;
        self.size = new_addr - current_addr;

        Some(new_addr)
    }
}

static HEAP_ADDRESS: Mutex<Option<Address>> = Mutex::new(None);

pub unsafe fn heap_init(start: usize, size: usize) {
    assert!(size >= HEADER_LEN);
    let mut elem: &mut HeapElem = mem::transmute(start);
    elem.free = 1;
    elem.size = size;
    *HEAP_ADDRESS.lock() = Some(Address {
        start: start,
        end: start + size,
    });
}

unsafe impl<'a> Alloc for &'a Allocator {
    unsafe fn alloc(&mut self, layout: Layout) -> Result<*mut u8, AllocErr> {
        if let Some(ref mut addr) = *HEAP_ADDRESS.lock() {
            let mut elem: &mut HeapElem = mem::transmute(addr.start);

            loop {
                if let Some(new_addr) = elem.split(layout.size(), layout.align()) {
                    return Ok((new_addr + HEADER_LEN) as *mut u8);
                } else {
                    let next_addr = (elem as *const _ as usize) + elem.len();
                    if next_addr + HEADER_LEN > addr.end {
                        return Err(AllocErr::Exhausted { request: layout });
                    }
                    elem = mem::transmute(next_addr);
                }
            }
        } else {
            panic!("heap in not initialized");
        }
    }

    unsafe fn dealloc(&mut self, ptr: *mut u8, _layout: Layout) {
        if let Some(ref mut addr) = *HEAP_ADDRESS.lock() {
            let a = ptr as usize;
            if a >= (addr.start + HEADER_LEN) && a <= addr.end {
                let mut elem: &mut HeapElem = mem::transmute(a - HEADER_LEN);
                elem.free = 1;

                // merge with the next elem if possible
                // (of course, it shoule be merged with the previous if possible)
                let next_addr = (elem as *const _ as usize) + elem.len();
                if (next_addr + HEADER_LEN) <= addr.end {
                    let next_elem: &mut HeapElem = mem::transmute(next_addr);
                    if next_elem.free == 1 {
                        elem.size = elem.size + next_elem.size;
                    }
                }
            } else {
                panic!("address is invalid");
            }
        } else {
            panic!("heap is not initialized");
        }
    }
}


fn align_floor(val: usize, align: usize) -> usize {
    assert!(align.is_power_of_two());
    val & (!(align - 1))
}
