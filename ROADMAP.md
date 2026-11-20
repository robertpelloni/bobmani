# ROADMAP: Consolidated Architecture & Ecosystem Expansion

## Current Focus (v0.2.31)
- [x] Consolidate dashboard interface into single-page layout organized by priority tiers (Orchestration, Cognitive Intelligence, Supervisor/Tray, Multi-Modal I/O).
- [x] Resolve Rust compilation warnings and verify clean build in central workspace.
- [x] Validate system tray background daemon and graceful shutdown handlers.
- [x] Connect Axum WebSocket telemetry (`/ws`) to handle JSON requests (`ping`, `status`, `parse_sm`).
- [x] Expose all ported submodules (`arrowvortex`, `beatoraja`, `ddc`, `ddc_onset`, `ffr_diff_model`, `server`) across `lib.rs` and `main.rs`.
- [x] Align tract-onnx `PlacementCNN` runnable model using `Arc<TypedRunnableModel>`.
- [x] Connect real-time density, stream, and pattern analysis over Axum WebSocket (`analyze_density`).
- [x] Connect live audio sample framing and Mel feature extraction to WebSocket endpoint (`extract_audio_features`).
- [x] Implement bidirectional protocol bridge and TypeScript unit testing in `pianogame/typescript/src/ws_bridge.ts`.
- [x] Connect audio onset peak-finding and threshold filtering over Axum WebSocket (`detect_onsets`).
- [x] Implement automated submodule synchronization and health audit tool (`scripts/sync_submodules.py`).
- [x] Connect `beatoraja` score judgment and EX score calculation over Axum WebSocket (`calculate_ex_score`).
- [x] Connect native `arrowvortex` in-memory simfile parser (`parse_sm_content`) to WebSocket `parse_sm` endpoint.
- [x] Connect WebSocket streams to live frontend chart visualizer. (Done in 0.2.12 — dashboard canvas visualiser; see Update 0.2.11 below. This top list was stale.)
- [x] Integrate ONNX model prediction as a WebSocket action. (Done in 0.2.13/0.2.14 — `predict_placement`: real tract-onnx inference when a model file is present, labelled heuristic fallback otherwise; no model ships with the repo.)
- [ ] Broadcast WebSocket results to all connected clients. (NOT built — `src/server.rs` replies only to the requesting connection. The old wording of the item above implied broadcast; that part remains undone.)

## Long-Term Vision — NOT planned work

The three items below are vision, not a schedule, not milestones anyone is working toward, and not capabilities of this repo. They stay here only as a record of the direction once imagined.
1. **Core-Mani WASM Engine**: Complete cross-compilation of timing, chart parsing, and judgment logic into WebAssembly.
2. **Universal Rhythm Controller**: Unify multi-controller inputs (DDR dance pads, IIDX controllers, MIDI keyboards) across all submodules.
3. **Embedded Bobcoin Proof-of-Play (PoP)**: On-chain accuracy validation and token reward integration.

## Update 0.2.11 (2026-11-14)
- [x] Serve unified dashboard from Rust hub at `/` + `/health` (previously docs claimed a dashboard the root server never served).
- [x] Tray clarified: Go console supervisor only, no native OS tray icon yet.
- [ ] Native OS tray icon (Windows shell) with Quit that stops the Rust server.
- [ ] Live chart visualiser canvas fed by analyze_density stream.
- [ ] ONNX placement inference action over WebSocket.
- [x] 0.2.12 Connect live chart visualiser canvas to density input/stream.
- [x] 0.2.13 ONNX placement action over WebSocket (heuristic fallback labelled).
- [x] 0.2.14 End-to-end ONNX placement test with real model fixture.
- [x] 0.2.15 Graceful shutdown (Ctrl-C, /shutdown, dashboard Quit).
- [x] 0.2.16 Live fleet audit endpoint + dashboard.
- [x] 0.2.17 Native OS tray icon with graceful server quit.
- [x] 0.2.23 Consolidate/inventory all legacy dashboards onto the canonical page (Tier 4).
- [x] 0.2.30 Sustained verification loop: build + live protocol re-verified with zero regressions.

## Update 0.2.31 (2026-11-19)
- [x] Honesty pass: README status reconciled to the actual v0.2.30 code tree (Nov 17); stale top-list items above checked against `src/server.rs` and the 0.2.11 update section.
- [x] VISION.md bannered as aspirational / not implemented.
- [x] TODO.md and HANDOFF.md replaced — both were broken symlinks GitHub could not render.
- [x] Cargo package renamed `app` -> `bobmani` (no code change).
