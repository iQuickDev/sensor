use axum::Router;
use sensor::api::{cpu, disk, network, os, ram};
use tower_http::cors::CorsLayer;

#[tokio::main]
pub async fn main() {
    let app = Router::new()
        .merge(cpu::cpu_router())
        .merge(disk::disk_router())
        .merge(network::network_router())
        .merge(os::os_router())
        .merge(ram::ram_router())
        .layer(CorsLayer::very_permissive());

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
