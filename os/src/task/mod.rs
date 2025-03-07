use lazy_static::lazy_static;
use crate::sync::UPSafeCell;
use core::arch::global_asm;

pub mod loader;
mod task;
mod stack;

use loader::{AppManager, get_base_i};
use stack::{UserStack, KernelStack};
use task::{TaskManager, TaskControlBlock, TaskContext, TaskStatus, TaskManagerInner};
use crate::trap::TrapContext;

pub const MAX_APP_NUM: usize = 4;
const APP_BASE_ADDRESS: usize = 0x80400000;
const APP_SIZE_LIMIT: usize = 0x20000;

const USER_STACK_SIZE: usize = 4096 * 2;
const KERNEL_STACK_SIZE: usize = 4096 * 2;

global_asm!(include_str!("switch.S"));

pub static KERNEL_STACKS: [KernelStack; MAX_APP_NUM] = [KernelStack {
    data: [0; KERNEL_STACK_SIZE],
}; MAX_APP_NUM];


pub static USER_STACKS: [UserStack; MAX_APP_NUM] = [UserStack {
    data: [0; USER_STACK_SIZE],
}; MAX_APP_NUM];

lazy_static! {
    static ref APP_MANAGER: UPSafeCell<AppManager> = unsafe {
        UPSafeCell::new({
            extern "C" {
                fn _num_app();
            }
            let num_app_ptr = _num_app as usize as *const usize;
            let num_app = num_app_ptr.read_volatile();
            let mut app_start: [usize; MAX_APP_NUM + 1] = [0; MAX_APP_NUM + 1];
            let app_start_raw: &[usize] =
                core::slice::from_raw_parts(num_app_ptr.add(1), num_app + 1);
            app_start[..=num_app].copy_from_slice(app_start_raw);
            AppManager {
                num_app,
                app_start,
            }
        })
    };
}

lazy_static! {
    static ref TASK_MANAGER: TaskManager = unsafe {
        extern "C" {
            fn __init();
        }

        let app_manager = APP_MANAGER.exclusive_access();
        let app_num = app_manager.num_app;
        
        let mut tcbs = [TaskControlBlock::uninit(); MAX_APP_NUM];

        for i in 0..app_num {      
            tcbs[i].task_status = TaskStatus::Ready;
            tcbs[i].task_cx = TaskContext::new(
                __init as usize,
                KERNEL_STACKS[i].push_context(TrapContext::app_init_context(get_base_i(i), USER_STACKS[i].get_sp())) as *const _ as usize,
                [0; 12],
            );
            // warn!("kernel stack for app {}", i);
        }

        TaskManager {
            num: app_num,
            inner: UPSafeCell::new(TaskManagerInner {
                tcbs: tcbs,
                current_task: 0,
            }),
        }
    };
}


pub fn load_apps() {
    loader::load_apps();
}


pub fn suspend_and_run_next() {
    let current_task = TASK_MANAGER.get_current_app_id();
    TASK_MANAGER.suspend_task(current_task);
    TASK_MANAGER.run_next_task();
}

pub fn exit_and_run_next() {
    let current_task = TASK_MANAGER.get_current_app_id();
    TASK_MANAGER.exit_task(current_task);
    TASK_MANAGER.run_next_task();
}

pub fn start() {
    TASK_MANAGER.run_first_task();
}