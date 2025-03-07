use log::info;


/// 测试系统调用后寄存器信息是否正常
pub fn sys_test(ra: usize, sp: usize, s: usize) -> isize {
    info!("[kernel] test syscall called");
    unsafe {
        let _s = core::slice::from_raw_parts(s as *const u8, 12);
        info!("[kernel] ra: {:#x}, sp: {:#x}, s: {:#?}", ra, sp, _s);
    }
    0
}