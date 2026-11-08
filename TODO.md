# TODO

- [x] Phase 2: Consolidate `arrowvortex` core parsing logic into `src/arrowvortex`.
- [x] Port `ffr-difficulty-model` mathematical logic into `src/ffr_diff_model`.
- [x] Implement tract-onnx execution pipeline for `ddc_onset` models in `src/ddc_onset`.
- [x] Clean up Rust compiler warnings (unused imports, unused params, unnecessary mutability).
- [x] Redesign unified dashboard UI into high-value prioritized tiers on a single consolidated page.
- [x] Verify System Tray daemon and graceful shutdown protocols.
- [x] Implement Axum WebSocket strongly-typed command handler (`ping`, `status`, `parse_sm`) in `src/server.rs`.
- [ ] Connect WebSocket streams to live frontend chart visualizer.
- [ ] Implement end-to-end integration tests between ONNX chart placement and WebSocket broadcast.
