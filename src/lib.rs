use lazy_static::lazy_static;
use std::sync::{Mutex, Once};
use sysinfo::{System, SystemExt};

pub mod api {
    pub mod cpu;
    pub mod disk;
    pub mod network;
    pub mod os;
    pub mod ram;
}

lazy_static! {
    static ref SYSTEM: Mutex<Option<System>> = {
        let mut sys = System::new_all();
        sys.refresh_all();
        Mutex::new(Some(sys))
    };
    static ref INIT: Once = Once::new();
}

pub fn get_system() -> &'static Mutex<Option<System>> {
    if let Some(sys) = SYSTEM.lock().unwrap().as_mut() {
        sys.refresh_all();
    }
    &SYSTEM
}
