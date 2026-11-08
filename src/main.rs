pub mod arrowvortex;
pub mod beatoraja;
pub mod ddc;
pub mod ddc_onset;
pub mod ffr_diff_model;
pub mod server;

#[tokio::main]
async fn main() {
    println!("Phase 2 Consolidation - Unified Rust Backend Starting");

    // Start WebSocket Server
    server::run_server().await;
}
