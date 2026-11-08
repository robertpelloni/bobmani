# ROADMAP: Consolidated Architecture & Ecosystem Expansion

## Current Focus (v0.2.5)
- [x] Consolidate dashboard interface into single-page layout organized by priority tiers (Orchestration, Cognitive Intelligence, Supervisor/Tray, Multi-Modal I/O).
- [x] Resolve Rust compilation warnings and verify clean build in central workspace.
- [x] Validate system tray background daemon and graceful shutdown handlers.
- [x] Connect Axum WebSocket telemetry (`/ws`) to handle JSON requests (`ping`, `status`, `parse_sm`).
- [x] Expose all ported submodules (`arrowvortex`, `beatoraja`, `ddc`, `ddc_onset`, `ffr_diff_model`, `server`) across `lib.rs` and `main.rs`.
- [x] Align tract-onnx `PlacementCNN` runnable model using `Arc<TypedRunnableModel>`.
- [x] Connect real-time density, stream, and pattern analysis over Axum WebSocket (`analyze_density`).
- [x] Connect live audio sample framing and Mel feature extraction to WebSocket endpoint (`extract_audio_features`).
- [ ] Connect WebSocket streams to live frontend chart visualizer.
- [ ] Integrate ONNX model prediction triggers with live WebSocket broadcast.

## Long-Term Milestones
1. **Core-Mani WASM Engine**: Complete cross-compilation of timing, chart parsing, and judgment logic into WebAssembly.
2. **Universal Rhythm Controller**: Unify multi-controller inputs (DDR dance pads, IIDX controllers, MIDI keyboards) across all submodules.
3. **Embedded Bobcoin Proof-of-Play (PoP)**: On-chain accuracy validation and token reward integration.
