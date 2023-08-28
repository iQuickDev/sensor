use crate::get_system;
use axum::{routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use sysinfo::{CpuExt, SystemExt};

#[derive(Serialize, Deserialize)]
pub struct CPUInfo {
    load: f32,
    cores: usize,
    name: String,
    clock: u64,
    brand: String,
}

pub async fn cpu_all() -> Json<CPUInfo> {
    let sys_mutex = get_system();
    let mut sys = sys_mutex.lock().unwrap();
    let sys = sys.as_mut().unwrap();

    let cores = sys.cpus().len();
    let load = sys.global_cpu_info().cpu_usage();
    let name = sys.global_cpu_info().brand().to_string();
    let clock = sys.global_cpu_info().frequency();
    let brand = sys.global_cpu_info().vendor_id().to_string();

    let cpu_info = CPUInfo {
        load,
        cores,
        name,
        clock,
        brand,
    };

    Json(cpu_info)
}

pub fn cpu_router() -> Router {
    Router::new().route("/cpu", get(cpu_all))
}
