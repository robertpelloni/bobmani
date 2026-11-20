# bobmani

A Rust project consolidating Robert Pelloni's rhythm-game ecosystem into one tree. At v0.2.31 ("Consolidation Phase 2"), `bobmani` pulls the whole fleet in as git submodules — `bobmania` (his StepMania fork), `itgmania`, `ddc`, `ddc_onset`, `ffr-difficulty-model`, `hymnmania`, `ksm-v2`, `leraine-studio`, `linthesia`, `pianogame`, `beatoraja`, `arrowvortex`, `Simply-Love-SM5` — and builds his own Rust layer on top of it.

The Rust crate itself (edition 2024) is an **axum + tokio** server with WebSocket support and **tract-onnx** for ONNX model inference — a local services/inference hub for the rhythm-game tooling (chart analysis/generation models are the obvious payload, given the fleet it consolidates).

## Status

**Prototype, early consolidation stage — no users, no deployment.** Current version: **v0.2.31** (2026-11-19, honesty/hygiene pass; code is the v0.2.30 tree of 2026-11-17 except the Cargo package rename from `app` to `bobmani`). Nov 8–17 work shipped v0.2.4–v0.2.30: real-time chart density/difficulty analysis, audio feature extraction and WebSocket telemetry, a TypeScript protocol bridge, onset detection, submodule sync/audit tooling, beatoraja score validation with a real-time EX-score engine, a native in-memory simfile parser wired into the WebSocket engine, a single-page dashboard at `/` with a chart-visualiser canvas, graceful shutdown, and a sustained verification loop (concurrent-client probe, six-component build check).

The local server (127.0.0.1:3000) exposes exactly 8 WebSocket actions — `ping`, `status`, `parse_sm`, `analyze_density`, `extract_audio_features`, `detect_onsets`, `predict_placement`, `calculate_ex_score` — plus `/health`, `/fleet`, and `POST /shutdown`. Replies are per-connection only; there is no broadcast to other clients. `predict_placement` runs real tract-onnx inference only when a model file is present (a test fixture proves that path); **no model ships with this repo**, so in normal use it falls back to a labelled onset heuristic, and the response's `source` field says which ran.

The submodule fleet and the server exist; the unified tooling the repo is named for is still being built. The 13 submodules are checkouts of other projects, not code owned by this repo. If you want the individual projects, their own repos (linked as the submodules above) are the usable entry points today.

## Layout

- `src/server.rs` — the axum WebSocket server; `src/main.rs`, `src/lib.rs` — entry points
- `src/arrowvortex/`, `src/beatoraja/`, `src/ddc/`, `src/ddc_onset/`, `src/ffr_diff_model/` — per-tool integration modules
- `scripts/` — automation, including submodule sync/audit tooling
- Submodule checkouts: `bobmania`, `itgmania`, `ddc`, `ddc_onset`, `ffr-difficulty-model`, `hymnmania`, `ksm-v2`, `leraine-studio`, `linthesia`, `pianogame`, `beatoraja`, `arrowvortex`, `Simply-Love-SM5`

## Running

```bash
cargo build
cargo run
```
