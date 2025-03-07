use crate::sync::UPSafeCell;
use super::MAX_APP_NUM;
use log::{info, warn};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}


#[derive(Clone, Copy)]
#[repr(C)]
pub struct TaskContext {
    ra: usize,
    sp: usize,
    s: [usize; 12],
}

impl TaskContext {
    pub fn new(ra : usize, sp : usize, s : [usize; 12]) -> Self {
        Self {
            ra: ra,
            sp: sp,
            s: s,
        }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct TaskControlBlock {
    pub task_status: TaskStatus,
    pub task_cx: TaskContext,
}


impl TaskControlBlock {
    pub fn uninit() -> Self {
        Self {
            task_status: TaskStatus::UnInit,
            task_cx: TaskContext::new(0, 0, [0; 12]),
        }
    }
}

pub struct TaskManagerInner {
    pub current_task: usize,
    pub tcbs: [TaskControlBlock; MAX_APP_NUM],
}

pub struct TaskManager {
    pub num: usize,
    pub inner: UPSafeCell<TaskManagerInner>,
}

impl TaskManager {

    pub fn get_current_app_id(&self) -> usize {
        let inner = self.inner.exclusive_access();
        inner.current_task
    }

    pub fn find_a_task(&self) -> Option<usize> {
        let inner = self.inner.exclusive_access();
        
        let mut curr = (inner.current_task + 1) % self.num;
        while inner.tcbs[curr].task_status != TaskStatus::Ready {
            curr = (curr + 1) % self.num;
            if curr == inner.current_task {
                return None;
            }
        }

        info!("[kernel] found a task {} to run", curr);
        Some(curr)
    }

    pub fn run_first_task(&self) {
        extern "C" {
            pub fn __switch(
                current_task_cx_ptr: *mut TaskContext,
                next_task_cx_ptr: *const TaskContext
            );
        }
        warn!("[kernal] __switch: {:#x}", __switch as usize);

        let mut inner = self.inner.exclusive_access();
        inner.current_task = 0;
        inner.tcbs[0].task_status = TaskStatus::Running;

        let mut unused_task_cx = TaskContext {
            ra: 0,
            sp: 0,
            s: [0; 12],
        };
        let next_task_cx = &inner.tcbs[0].task_cx as *const _ as *const TaskContext;

        drop(inner);

        info!("[kernel] running first task");
        // for i in 0..super::MAX_APP_NUM {
        //     info!("[kernel] kernel stack {}: {:#p}", i, super::loader::KERNEL_STACKS[i].data.as_ptr());
        //     info!("[kernel] user stack {}: {:#p}", i, super::loader::USER_STACKS[i].data.as_ptr());
        // }

        unsafe {
            __switch(
                &mut unused_task_cx as *mut _ as *mut TaskContext,
                next_task_cx,
            )
        }
    }

    pub fn run_next_task(&self) {
        extern "C" {
            pub fn __switch(
                current_task_cx_ptr: *mut TaskContext,
                next_task_cx_ptr: *const TaskContext
            );
        }

        let next_task = self.find_a_task();

        let mut inner = self.inner.exclusive_access();

        let current_task = inner.current_task;
        match next_task {
            Some(task_id) => {
                inner.current_task = task_id;
                inner.tcbs[task_id].task_status = TaskStatus::Running;

                let current_task_cx = &inner.tcbs[current_task].task_cx as *const _ as *const TaskContext;
                let next_task_cx = &inner.tcbs[task_id].task_cx as *const _ as *const TaskContext;

                drop(inner);

                unsafe {
                    __switch(
                        current_task_cx as *mut TaskContext,
                        next_task_cx,
                    )
                }
            }
            None => {
                // no more task to run
                crate::sbi::shutdown(false);
            }
        }
    }

    pub fn suspend_task(&self, task_id: usize) {
        let mut inner = self.inner.exclusive_access();
        inner.tcbs[task_id].task_status = TaskStatus::Ready;
    }

    pub fn exit_task(&self, task_id: usize) {
        let mut inner = self.inner.exclusive_access();
        inner.tcbs[task_id].task_status = TaskStatus::Exited;
    }
}