# bobmani

A Rust project consolidating Robert Pelloni's rhythm-game ecosystem into one tree. At v0.2.10 ("Consolidation Phase 2"), `bobmani` pulls the whole fleet in as git submodules — `bobmania` (his StepMania fork), `itgmania`, `ddc`, `ddc_onset`, `ffr-difficulty-model`, `hymnmania`, `ksm-v2`, `leraine-studio`, `linthesia`, `pianogame`, `beatoraja`, `arrowvortex`, `Simply-Love-SM5` — and builds his own Rust layer on top of it.

The Rust crate itself (edition 2024) is an **axum + tokio** server with WebSocket support and **tract-onnx** for ONNX model inference — a local services/inference hub for the rhythm-game tooling (chart analysis/generation models are the obvious payload, given the fleet it consolidates).

## Status

**Active, early consolidation stage.** Last significant activity: **2026-11-12** — merge of a Jules feature branch (Protocol #281). Nov 8–9 work shipped v0.2.4–v0.2.10: real-time chart density/difficulty analysis, audio feature extraction and WebSocket telemetry, a TypeScript protocol bridge, onset detection, submodule sync/audit tooling, beatoraja score validation with a real-time EX-score engine, and a native in-memory simfile parser wired into the WebSocket engine.

The submodule fleet and the server skeleton exist; the unified tooling the repo is named for is still being built. If you want the individual projects, their own repos (linked as the submodules above) are the usable entry points today.

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
