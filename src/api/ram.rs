use crate::get_system;
use axum::{routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use sysinfo::SystemExt;

#[derive(Serialize, Deserialize)]
pub struct RamInfo {
    total: u64,
    used: u64,
    total_swap: u64,
    used_swap: u64,
}

pub async fn ram_all() -> Json<RamInfo> {
    let sys_mutex = get_system();
    let mut sys = sys_mutex.lock().unwrap();
    let sys = sys.as_mut().unwrap();
    let total = sys.total_memory();
    let used = sys.used_memory();
    let total_swap = sys.total_swap();
    let used_swap = sys.used_swap();

    let ram_info = RamInfo {
        total,
        used,
        total_swap,
        used_swap,
    };

    Json(ram_info)
}

pub fn ram_router() -> Router {
    Router::new().route("/ram", get(ram_all))
}
