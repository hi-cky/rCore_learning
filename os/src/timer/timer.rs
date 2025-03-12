use crate::config::{CLOCK_FREQ, INTERVAL_MS};

use riscv::register::{time, sie};
use crate::sbi::set_timer;

pub fn get_time() -> usize {
    time::read()
}

pub fn enable_timer_interrupt() {
    unsafe { sie::set_stimer(); }
}

pub fn set_next_trigger() {
    set_timer(get_time() + CLOCK_FREQ * INTERVAL_MS / 1000);
}