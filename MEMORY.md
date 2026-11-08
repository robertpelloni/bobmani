# MEMORY: Architectural Insights & System Observations

- **Rust Consolidation**: Ported `arrowvortex` Simfile/NoteList/TimingData structures and `.sm` parsing logic into native Rust. Ported `ffr-difficulty-model` density calculation and `ddc_onset` peak-finding/tract-onnx inference pipeline.
- **WebSocket Protocol**: The monolithic Rust backend implements an Axum (0.8.x) WebSocket server on `127.0.0.1:3000/ws` parsing strongly typed `WsRequest` (Ping, Status, ParseSm) and responding with `WsResponse` (Pong, Status, SmMetadata, Error) via `serde` and `serde_json`.
- **Compiler Hygiene**: All warnings eliminated across `src/arrowvortex/` (`chart.rs`, `load_sm.rs`, `note_list.rs`, `timing_data.rs`), `src/ffr_diff_model/` (`preprocessor.rs`), and `src/server.rs`.
- **Unified Single-Page Dashboard**: Dashboard interfaces are structured around high-value prioritization tiers:
  - Tier 1: Core Autonomous Orchestration & Autopilot (Routers, Orchestrator, Adaptive Routing, Worktree Racing).
  - Tier 2: Cognitive Intelligence, RAG & Vector Knowledge (AST Parser, RAG Search, Memory Store, Prompt Templates).
  - Tier 3: Process Supervisor, System Tray & Protocol Telemetry (Multi-engine Tray Supervisor, Health Heartbeats, MCP Server, Webview Container).
  - Tier 4: Storage, Browser Automation & Multi-Modal I/O (SQLite Persistent Store, Chrome CDP, Email/Webhooks, MIDI Analyzer).
- **System Tray Protocol**: The Go desktop launcher and background supervisor provide process monitoring for 5 engines, notification balloon alerts, and graceful multi-engine teardown on `quit`.
- **Bobmania Theme Dashboard**: Bobmania's `ScreenUnifiedDashboard overlay.lua` exposes 13 high-value modules tested via `scripts/audit_dashboard.py`.
