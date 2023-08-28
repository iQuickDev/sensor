use crate::get_system;
use axum::{routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use sysinfo::{NetworkExt, SystemExt};

#[derive(Serialize, Deserialize)]
pub struct Interfaces {
    interfaces: Vec<InterfaceInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct InterfaceInfo {
    name: String,
    upload: u64,
    download: u64,
    address: String,
}

pub async fn interfaces_all() -> Json<Interfaces> {
    let sys_mutex = get_system();
    let mut sys = sys_mutex.lock().unwrap();
    let sys = sys.as_mut().unwrap();

    let mut interfaces_vec = vec![];
    for (interface_name, data) in sys.networks() {
        let interface_info = InterfaceInfo {
            name: interface_name.to_string(),
            upload: data.transmitted(),
            download: data.received(),
            address: data.mac_address().to_string(),
        };

        interfaces_vec.push(interface_info);
    }

    let disks = Interfaces {
        interfaces: interfaces_vec,
    };

    Json(disks)
}

pub fn network_router() -> Router {
    Router::new().route("/network", get(interfaces_all))
}
