use axum::{
    extract::State,
    routing::get,
    Json, Router,
};
use serde::Serialize;
use std::sync::{Arc, Mutex};
use std::net::SocketAddr;
use crate::ring_buffer::ZenithRingBuffer;
use crate::wasm_host::WasmPlugin;

#[derive(Clone)]
pub struct AdminState {
    pub buffer: ZenithRingBuffer,
    pub plugins: Arc<Mutex<Vec<WasmPlugin>>>,
}

#[derive(Serialize)]
struct StatusResponse {
    status: String,
    buffer_len: usize,
    plugin_count: usize,
}

#[derive(Serialize)]
struct PluginResponse {
    id: usize,
    status: String,
}

async fn get_status(State(state): State<AdminState>) -> Json<StatusResponse> {
    let plugins = state.plugins.lock().unwrap();
    Json(StatusResponse {
        status: "running".to_string(),
        buffer_len: state.buffer.len(),
        plugin_count: plugins.len(),
    })
}

async fn get_plugins(State(state): State<AdminState>) -> Json<Vec<PluginResponse>> {
    let plugins = state.plugins.lock().unwrap();
    let list = plugins.iter().enumerate().map(|(i, _)| PluginResponse {
        id: i,
        status: "loaded".to_string(),
    }).collect();
    Json(list)
}

pub async fn start_admin_server(state: AdminState, port: u16) {
    let app = Router::new()
        .route("/status", get(get_status))
        .route("/plugins", get(get_plugins))
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("Zenith Admin API listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
