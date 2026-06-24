mod ffr_diff_model;
mod ddc;
mod ddc_onset;
mod arrowvortex;

use axum::{
    routing::{get, post},
    Router,
    Json,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Deserialize)]
struct GenerateRequest {
    audio_path: String,
}

#[derive(Serialize)]
struct GenerateResponse {
    status: String,
    message: String,
}

async fn handle_generate(Json(payload): Json<GenerateRequest>) -> Json<GenerateResponse> {
    // We instantiate the wrapper logic here safely.
    // In a real application, state would be shared across the Axum router bounds.
    let models_dir = PathBuf::from("fake_models");
    let autochart = ddc::autochart::AutoChart::new(models_dir, None, None, None);

    // Simulate generation failure cleanly since audio paths won't exist yet
    let audio_path = PathBuf::from(&payload.audio_path);
    let out_dir = PathBuf::from("output");

    match autochart.process_song(&audio_path, &out_dir) {
        Ok(_) => Json(GenerateResponse {
            status: "success".to_string(),
            message: format!("Successfully processed {}", payload.audio_path),
        }),
        Err(e) => Json(GenerateResponse {
            status: "error".to_string(),
            message: e,
        })
    }
}

async fn health_check() -> &'static str {
    "Bobmani Engine is live!"
}

#[tokio::main]
async fn main() {
    println!("Universal Rhythm Engine (Rust Monolith)");
    println!("Difficulty Predictor initialized.");

    // Define the app routing
    let app = Router::new()
        .route("/api/health", get(health_check))
        .route("/api/generate", post(handle_generate));

    // Bind and start the server
    println!("Starting Web API server on http://localhost:3000 ...");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
