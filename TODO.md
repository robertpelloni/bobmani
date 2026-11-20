# TODO: bobmani

Current version: v0.2.31 (2026-11-19). This file was a broken symlink that GitHub could not render; it is a real file now.

## Actually next (undecided — Robert's pick)

- [ ] Ship ONE tool standalone (ddc_onset or the FFR difficulty model) so a chart maker can use it without cloning the 13-submodule tree. This is the only item that makes bobmani a product.
- [ ] Decide the hymnmania question (open since 2026-10-26): fold hymnmania into bobmani, or keep it a separate repo that is only a submodule here.

## Not built

- [ ] Broadcast WebSocket results to all connected clients — replies in `src/server.rs` are per-connection only.
- [ ] Ship an ONNX placement model with the repo — `predict_placement` currently falls back to the labelled onset heuristic in normal use (real inference only when a model file is supplied; the test fixture proves that path).

## Done and verified in the code (v0.2.30 tree)

- [x] 8 WebSocket actions: `ping`, `status`, `parse_sm`, `analyze_density`, `extract_audio_features`, `detect_onsets`, `predict_placement`, `calculate_ex_score`.
- [x] Single-page dashboard at `/`, `/health`, `/fleet`, `POST /shutdown`, graceful shutdown (Ctrl-C / dashboard Quit), graceful port-conflict exit.
- [x] Verification loop: concurrent-client probe, WS edge-input tests, six-component build check.

## Vision only — do NOT check these off as planned work

- [ ] Core-Mani WASM Engine
- [ ] Universal Rhythm Controller
- [ ] Bobcoin Proof-of-Play / matchmaking / fitness telemetry
