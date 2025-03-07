use crate::trap::TrapContext;
use super::{KERNEL_STACK_SIZE, USER_STACK_SIZE};

#[repr(align(4096))]
#[derive(Clone, Copy)]
pub struct KernelStack {
    pub data: [u8; KERNEL_STACK_SIZE],
}

#[repr(align(4096))]
#[derive(Clone, Copy)]
pub struct UserStack {
    pub data: [u8; USER_STACK_SIZE],
}

impl KernelStack {
    pub fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + KERNEL_STACK_SIZE
    }
    pub fn push_context(&self, cx: TrapContext) -> &'static mut TrapContext {
        let cx_ptr = (self.get_sp() - core::mem::size_of::<TrapContext>()) as *mut TrapContext;
        unsafe {
            *cx_ptr = cx;
        }
        unsafe { cx_ptr.as_mut().unwrap() }
    }
}

impl UserStack {
    pub fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
}
