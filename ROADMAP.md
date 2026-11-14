# ROADMAP: Consolidated Architecture & Ecosystem Expansion

## Current Focus (v0.2.10)
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
- [ ] Connect WebSocket streams to live frontend chart visualizer.
- [ ] Integrate ONNX model prediction triggers with live WebSocket broadcast.

## Long-Term Milestones
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
