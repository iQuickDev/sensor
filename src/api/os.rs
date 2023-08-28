use crate::get_system;
use axum::{routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use sysinfo::SystemExt;
use std::env::consts;

#[derive(Serialize, Deserialize)]
pub struct OsInfo {
    name: String,
    kernel: String,
    version: String,
    hostname: String,
    arch: String,
}

pub async fn os_all() -> Json<OsInfo> {
    let sys_mutex = get_system();
    let mut sys = sys_mutex.lock().unwrap();
    let sys = sys.as_mut().unwrap();
    let name = sys.name().unwrap();
    let kernel = sys.kernel_version().unwrap();
    let version = sys.long_os_version().unwrap();
    let hostname = sys.host_name().unwrap();
    let arch = consts::ARCH.to_string();

    let os_info = OsInfo {
        name,
        kernel,
        version,
        hostname,
        arch,
    };

    Json(os_info)
}

pub fn os_router() -> Router {
    Router::new().route("/os", get(os_all))
}
