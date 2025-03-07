//! App management syscalls
use crate::task::{suspend_and_run_next, exit_and_run_next};
use log::info;

/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> isize {
    info!("[kernel] Application exited with code {}", exit_code);
    exit_and_run_next();
    0
}

/// task suspends itself and submits to run next task
pub fn sys_yield() -> isize {
    info!("[kernel] Yielding to next application");
    suspend_and_run_next();
    0
}