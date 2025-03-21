#![no_std]
#![feature(linkage)]
// #![allow(dead_code)]
// #![allow(unused)]
// #![allow(unused_imports)]

#[macro_use]
pub mod console;
mod lang_items;
mod syscall;

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    clear_bss();
    exit(main());
    panic!("unreachable after sys_exit!");
}

#[linkage = "weak"]
#[no_mangle]
fn main() -> i32 {
    panic!("Cannot find main!");
}

fn clear_bss() {
    extern "C" {
        fn start_bss();
        fn end_bss();
    }
    (start_bss as usize..end_bss as usize).for_each(|addr| unsafe {
        (addr as *mut u8).write_volatile(0);
    });
}

use syscall::*;

pub fn write(fd: usize, buf: &[u8]) -> isize {
    sys_write(fd, buf)
}
pub fn exit(exit_code: i32) -> isize {
    sys_exit(exit_code)
}

pub fn _yield() -> isize {
    sys_yield()
}

pub fn test() -> isize {
    sys_test()
}

pub fn get_time_ms() -> usize {
    sys_get_time() as usize
}

pub fn sleep_ms(ms: usize) {
    let start = get_time_ms();
    while get_time_ms() - start < ms {
        _yield();
    }
}