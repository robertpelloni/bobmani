# HANDOFF: bobmani

Current version: v0.2.31 (2026-11-19). This file was a broken symlink that GitHub could not render; it is a real file now.

## State

- Rust prototype consolidating Robert's rhythm-game fleet as 13 git submodules, with his own axum + tokio WebSocket server on top (127.0.0.1:3000).
- Code tree is v0.2.30 (2026-11-17, head before this pass: f93708f2) except the Cargo package rename `app` -> `bobmani` in 0.2.31. Docs-only honesty pass otherwise.
- 8 WebSocket actions: `ping`, `status`, `parse_sm`, `analyze_density`, `extract_audio_features`, `detect_onsets`, `predict_placement`, `calculate_ex_score`. Dashboard at `/`, plus `/health`, `/fleet`, `POST /shutdown`.
- No users, no deployment, no ONNX model shipped (`predict_placement` = labelled heuristic in normal use; real tract-onnx path is fixture-tested only).
- The individual submodule repos are still the usable entry points.

## Next step

Robert's pick: ship ddc_onset or the FFR difficulty model standalone (see TODO.md). Do not add more submodules, do not start the WASM engine, universal controller, or Bobcoin Proof-of-Play — those are vision only (VISION.md).

## Warnings

- tract-onnx debug builds blew `target/` to ~12 GB and hit the MSVC PDB limit; Cargo.toml keeps `[profile.test] debug = 1` for that reason. Do not remove it casually.
- Submodule pins have failed before (2026-11-14: bobmani's pin in the workspace repo was unpublished and had to be unstaged). Verify pins are published before staging pointers elsewhere.
