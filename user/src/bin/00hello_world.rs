#![no_std]
#![no_main]

use user_lib::*;
    
#[no_mangle]
fn main() -> i32 {
    let len = 10;
    for i in 0..10 {
        println!("program 0, iteration {}/{}", i+1, len);
        sleep_ms(500);
    }
    0
}
