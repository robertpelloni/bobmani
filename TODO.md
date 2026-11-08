# TODO

- [x] Phase 2: Consolidate `arrowvortex` core parsing logic into `src/arrowvortex`.
- [x] Port `ffr-difficulty-model` mathematical logic into `src/ffr_diff_model`.
- [x] Implement tract-onnx execution pipeline for `ddc_onset` models in `src/ddc_onset`.
- [x] Clean up Rust compiler warnings (unused imports, unused params, unnecessary mutability) across `arrowvortex`, `ffr_diff_model`, and `ddc`.
- [x] Redesign unified dashboard UI into high-value prioritized tiers on a single consolidated page.
- [x] Verify System Tray daemon and graceful shutdown protocols.
- [x] Implement Axum WebSocket strongly-typed command handler (`ping`, `status`, `parse_sm`) in `src/server.rs`.
- [x] Expose all ported submodules uniformly across `src/lib.rs`, `src/main.rs`, and `src/ddc_onset/mod.rs`.
- [x] Add real-time density, stream, and pattern analysis (`analyze_density`) to the Axum WebSocket server in `src/server.rs`.
- [x] Implement audio framing and multi-channel log-Mel feature generation in `src/ddc/extract_feats.rs` and `src/ddc_onset/spectral.rs`.
- [x] Wire `extract_audio_features` action into Axum WebSocket server (`src/server.rs`).
- [ ] Connect WebSocket streams to live frontend chart visualizer.
- [ ] Implement end-to-end integration tests between ONNX chart placement and WebSocket broadcast.
