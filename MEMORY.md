# MEMORY: Architectural Insights & System Observations

- **Rust Consolidation**: Ported `arrowvortex` Simfile/NoteList/TimingData structures and `.sm` parsing logic into native Rust. Ported `ffr-difficulty-model` density calculation and `ddc_onset` peak-finding/tract-onnx inference pipeline.
- **Tract-ONNX Runnable Model**: `PlacementCNN` in `src/ddc_onset/cnn.rs` utilizes `Arc<TypedRunnableModel>` (from `tract_onnx::prelude::*`) matching the output of `.into_runnable()?` without extraneous generic bounds.
- **Module Exposure**: All submodules (`arrowvortex`, `beatoraja`, `ddc`, `ddc_onset`, `ffr_diff_model`, `server`) are exposed publicly through both `src/lib.rs` and `src/main.rs`. Internal onset features (`cnn`, `constants`, `spectral`, `util`) are exposed via `src/ddc_onset/mod.rs`.
- **WebSocket Protocol**: The monolithic Rust backend implements an Axum (0.8.x) WebSocket server on `127.0.0.1:3000/ws` parsing strongly typed `WsRequest` (Ping, Status, ParseSm, AnalyzeDensity, ExtractAudioFeatures, DetectOnsets, CalculateExScore) and responding with `WsResponse` (Pong, Status, SmMetadata, DensityAnalysis, AudioFeatures, OnsetDetections, ScoreAnalysis, Error) via `serde` and `serde_json`.
- **Real-Time Difficulty, Score & Spectral Telemetry**: `AnalyzeDensity` calculates NPS/Length, stream, and jack metrics. `ExtractAudioFeatures` runs `MelExtractor::extract` for multi-channel log-Mel spectrograms. `DetectOnsets` runs `find_peaks` and `threshold_peaks` on onset salience vectors. `CalculateExScore` instantiates `beatoraja::score_data::ScoreData` to evaluate EX scores and pass note integrity.
- **Automated Submodule Audit Script**: `scripts/sync_submodules.py` verifies recursive initialization, clean status, and published commit hashes across all 13 submodules.
- **TypeScript WsBridge Protocol Support**: `pianogame/typescript/src/ws_bridge.ts` contains static formatters (`formatRustRequest`) and parsers (`parseRustResponse`) mirroring Rust's `WsRequest` and `WsResponse` schemas.
- **Compiler Hygiene**: All warnings eliminated across `src/arrowvortex/`, `src/ffr_diff_model/`, `src/ddc/`, and `src/server.rs`.
- **Unified Single-Page Dashboard**: Dashboard interfaces are structured around high-value prioritization tiers:
  - Tier 1: Core Autonomous Orchestration & Autopilot (Routers, Orchestrator, Adaptive Routing, Worktree Racing).
  - Tier 2: Cognitive Intelligence, RAG & Vector Knowledge (AST Parser, RAG Search, Memory Store, Prompt Templates).
  - Tier 3: Process Supervisor, System Tray & Protocol Telemetry (Multi-engine Tray Supervisor, Health Heartbeats, MCP Server, Webview Container).
  - Tier 4: Storage, Browser Automation & Multi-Modal I/O (SQLite Persistent Store, Chrome CDP, Email/Webhooks, MIDI Analyzer).
- **System Tray Protocol**: The Go desktop launcher and background supervisor provide process monitoring for 5 engines, notification balloon alerts, and graceful multi-engine teardown on `quit`.
- **Bobmania Theme Dashboard**: Bobmania's `ScreenUnifiedDashboard overlay.lua` exposes 13 high-value modules tested via `scripts/audit_dashboard.py`.
