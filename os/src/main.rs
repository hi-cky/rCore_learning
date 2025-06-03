//! The main module and entrypoint
//!
//! Various facilities of the kernels are implemented as submodules. The most
//! important ones are:
//!
//! - [`trap`]: Handles all cases of switching from userspace to the kernel
//! - [`syscall`]: System call handling and implementation
//!
//! The operating system also starts in this module. Kernel code starts
//! executing from `entry.asm`, after which [`rust_main()`] is called to
//! initialize various pieces of functionality. (See its source code for
//! details.)
//!
//! We then call [`batch::run_next_app()`] and for the first time go to
//! userspace.

#![deny(missing_docs)]
// #![deny(warnings)]
#![no_std]
#![no_main]
#![allow(dead_code)]
// #![allow(unused)]
// #![allow(unused_imports)]
// #![allow(unused_variables)]

use core::arch::global_asm;

use log::*;
#[macro_use]
mod console;
mod task;
mod lang_items;
mod logging;
mod sbi;
mod sync;
mod timer;
mod config;
pub mod syscall;
pub mod trap;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

/// clear BSS segment
fn clear_bss() {
    unsafe extern "C" {
        fn sbss();
        fn ebss();
    }
    unsafe {
        core::slice::from_raw_parts_mut(sbss as usize as *mut u8, ebss as usize - sbss as usize)
            .fill(0);
    }
}

/// the rust entry-point of os
#[no_mangle]
pub fn rust_main() -> ! {
    unsafe extern "C" {
        fn stext(); // begin addr of text segment
        fn etext(); // end addr of text segment
        fn srodata(); // start addr of Read-Only data segment
        fn erodata(); // end addr of Read-Only data ssegment
        fn sdata(); // start addr of data segment
        fn edata(); // end addr of data segment
        fn sbss(); // start addr of BSS segment
        fn ebss(); // end addr of BSS segment
        fn boot_stack_lower_bound(); // stack lower bound
        fn boot_stack_top(); // stack top
    }
    clear_bss();
    logging::init();
    println!("[kernel] Hello, world!");
    trace!(
        "[kernel] .text [{:#x}, {:#x})",
        stext as usize,
        etext as usize
    );

    //下面这段代码不加的话调试又会报错
    //更诡异了？？？
    // TODO: 找到原因
    // debug!(
    //     "[kernel] .rodata [{:#x}, {:#x})",
    //     srodata as usize, erodata as usize
    // );
    // info!(
    //     "[kernel] .data [{:#x}, {:#x})",
    //     sdata as usize, edata as usize
    // );
    // warn!(
    //     "[kernel] boot_stack top=bottom={:#x}, lower_bound={:#x}",
    //     boot_stack_top as usize, boot_stack_lower_bound as usize
    // );
    // error!("[kernel] .bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
    trap::init();


    // 加了下面的代码就编译不过，非常诡异，以后再看
    // TODO: 找到原因
    // 原因是代码太多了，导致刚好把下面的加上__switch就超代码段导致编译不进去了
    // for i in 0..app::MAX_APP_NUM {
    //     warn!("[kernel] kernel stack {:#x}", app::loader::KERNEL_STACKS[i].data.as_ptr() as usize);
    // }
    timer::start();
    task::load_apps();
    task::start();
    warn!("[kernel] Unreachable in rust_main!");
    loop {}
}
