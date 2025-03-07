mod timer;

const INTERVAL_MS: usize = 10;

#[cfg(feature = "board_qemu")]
pub const CLOCK_FREQ: usize = 12500000;

pub fn get_time_ms() -> usize {
    timer::get_time() / (CLOCK_FREQ / 1000)
}

pub fn start(){
    timer::enable_timer_interrupt();
    timer::set_next_trigger();
}

pub fn go_on() {
    timer::set_next_trigger();
}