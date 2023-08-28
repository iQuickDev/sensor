use crate::get_system;
use axum::{routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use sysinfo::{DiskExt, SystemExt};

#[derive(Serialize, Deserialize)]
pub struct Disks {
    disks: Vec<DiskInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct DiskInfo {
    name: String,
    filesystem: String,
    mountpoint: String,
    capacity: u64,
}

pub async fn disk_all() -> Json<Disks> {
    let sys_mutex = get_system();
    let mut sys = sys_mutex.lock().unwrap();
    let sys = sys.as_mut().unwrap();

    let mut disks_vec = vec![];
    for disk in sys.disks() {
        let disk_info = DiskInfo {
            name: disk.name().to_string_lossy().into_owned(),
            filesystem: std::str::from_utf8(disk.file_system()).unwrap().to_string(),
            mountpoint: disk.mount_point().to_str().unwrap().to_string(),
            capacity: disk.total_space(),
        };

        disks_vec.push(disk_info);
    }

    let disks = Disks { disks: disks_vec };

    Json(disks)
}

pub fn disk_router() -> Router {
    Router::new().route("/disk", get(disk_all))
}
