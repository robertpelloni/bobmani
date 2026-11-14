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
- [x] Implement bidirectional Rust-to-TypeScript WebSocket protocol formatting and parsing in `pianogame/typescript/src/ws_bridge.ts`.
- [x] Implement `detect_onsets` action in Axum WebSocket server using `find_peaks` and `threshold_peaks` from `ddc_onset::util`.
- [x] Create `scripts/sync_submodules.py` automated synchronization and health audit tool.
- [x] Wire `calculate_ex_score` action into Axum WebSocket server using `ScoreData` from `beatoraja::score_data`.
- [x] Implement in-memory `parse_sm_content` in `src/arrowvortex/load_sm.rs` and connect to Axum WebSocket server.
- [ ] Connect WebSocket streams to live frontend chart visualizer.
- [ ] Implement end-to-end integration tests between ONNX chart placement and WebSocket broadcast.

## 0.2.11 follow-ups
- [ ] Native tray icon + Quit wiring to Rust server shutdown endpoint.
- [ ] Chart visualiser canvas on dashboard (density heatmap).
- [ ] ONNX predict_place action + integration test.
- [ ] Fix submodule dirty trees (bobmania, hymnmania modified; ddc/ffr untracked content) — audit before committing pointers.
- [ ] Merge or close stale branches: scaffold-docs-10743658648208721759, jules-empty-repo-diagnosis-17910503964700985352 (inspect before merging; do not merge blind).
- [x] 0.2.12 Live chart visualiser canvas.
- [ ] ONNX placement action; native tray icon; submodule dirty-tree audit.
- [x] 0.2.13 predict_placement action + dashboard card.
- [ ] Ship a real placement.onnx model to exercise the ONNX branch end-to-end.
- [ ] Native tray icon; submodule dirty-tree audit. NOTE: C: free space low (~2.7GB) — cargo target is large.
- [x] 0.2.14 Real ONNX fixture + neural-branch test.
- [ ] Train/source a genuine placement model (fixture is Relu identity, not trained).
- [ ] Native tray icon; submodule dirty-tree audit.
