use core::arch::asm;

const SYSCALL_WRITE: usize = 64;
const SYSCALL_EXIT: usize = 93;
const SYSCALL_YIELD: usize = 124;
const SYSCALL_TEST: usize = 123;
const SYSCALL_GET_TIME: usize = 125;

fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {
        asm!(
            "ecall",
            inlateout("x10") args[0] => ret,
            in("x11") args[1],
            in("x12") args[2],
            in("x17") id
        );
    }
    ret
}

pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
    syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}

pub fn sys_exit(exit_code: i32) -> isize {
    syscall(SYSCALL_EXIT, [exit_code as usize, 0, 0])
}

pub fn sys_yield() -> isize {
    syscall(SYSCALL_YIELD, [0, 0, 0])
}

pub fn sys_get_time() -> isize {
    syscall(SYSCALL_GET_TIME, [0, 0, 0])
}

pub fn sys_test() -> isize {
    // 获取当前程序计数器的值
    let pc: usize;
    unsafe {
        asm!("mv {}, a0", out(reg) pc);
    }
    // 获取当前栈指针的值
    let sp: usize;
    unsafe {
        asm!("mv {}, x2", out(reg) sp);
    }

    let mut s: [usize; 12] = [0; 12];
    unsafe {
        asm!("mv {}, s0", out(reg) s[0]);
        asm!("mv {}, s1", out(reg) s[1]);
        asm!("mv {}, s2", out(reg) s[2]);
        asm!("mv {}, s3", out(reg) s[3]);
        asm!("mv {}, s4", out(reg) s[4]);
        asm!("mv {}, s5", out(reg) s[5]);
        asm!("mv {}, s6", out(reg) s[6]);
        asm!("mv {}, s7", out(reg) s[7]);
        asm!("mv {}, s8", out(reg) s[8]);
        asm!("mv {}, s9", out(reg) s[9]);
        asm!("mv {}, s10", out(reg) s[10]);
        asm!("mv {}, s11", out(reg) s[11]);
    }
    

    syscall(SYSCALL_TEST, [pc, sp, &s as *const _ as usize])
}