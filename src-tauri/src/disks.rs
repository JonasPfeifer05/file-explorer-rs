use std::cell::OnceCell;
use serde::Serialize;
use sysinfo::{DiskExt, System, SystemExt};

static mut SYSTEM: OnceCell<System> = OnceCell::new();

#[derive(Debug,Serialize)]
pub struct DiskDescriptor {
    pub name: String
}

#[tauri::command]
pub fn disks() -> Vec<DiskDescriptor> {
    unsafe {
        SYSTEM.get_or_init(|| System::new());
        SYSTEM.get_mut().unwrap().refresh_disks_list();
        let mut disks: Vec<_> = SYSTEM.get().unwrap()
            .disks().iter()
            .map(|x| DiskDescriptor {
                name: x.mount_point().to_str().unwrap().to_string()
            })
            .collect();
        disks
    }
}