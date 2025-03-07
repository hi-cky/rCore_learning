use core::arch::asm;
use super::{APP_BASE_ADDRESS, APP_SIZE_LIMIT, MAX_APP_NUM, APP_MANAGER};
use log::info;

pub struct AppManager {
    pub num_app: usize,
    pub app_start: [usize; MAX_APP_NUM + 1],
}

impl AppManager {
    pub fn load_app(&self, app_id: usize) {
        let base_address = get_base_i(app_id);
        // clear region
        (base_address..base_address + APP_SIZE_LIMIT)
            .for_each(|addr| unsafe { (addr as *mut u8).write_volatile(0) });

        unsafe {
            let src = core::slice::from_raw_parts(
                self.app_start[app_id] as *const u8,
                self.app_start[app_id + 1] - self.app_start[app_id],
            );

            let dst = core::slice::from_raw_parts_mut(
                base_address as *mut u8,
                src.len(),
            );
            dst.copy_from_slice(src);

            info!("[kernel] load app {} to {:#x} - {:#x}", app_id, base_address, base_address + src.len());

            asm!("fence.i");
        }
    }

}

pub fn get_base_i(app_id: usize) -> usize {
    APP_BASE_ADDRESS + app_id * APP_SIZE_LIMIT
}


pub fn load_apps() {
    let app_manager = APP_MANAGER.exclusive_access();
    for i in 0..app_manager.num_app {
        app_manager.load_app(i);
    }
}