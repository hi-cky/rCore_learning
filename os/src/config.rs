
// timer
pub const INTERVAL_MS: usize = 10;

#[cfg(feature = "board_qemu")]
pub const CLOCK_FREQ: usize = 12500000;

// syscall
pub const SYSCALL_WRITE: usize = 64;
pub const SYSCALL_EXIT: usize = 93;
pub const SYSCALL_YIELD: usize = 124;
pub const SYSCALL_TEST: usize = 123;
pub const SYSCALL_GET_TIME: usize = 125;