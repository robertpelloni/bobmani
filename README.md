# bobmani

A Rust project consolidating Robert Pelloni's rhythm-game ecosystem into one tree. At v0.2.0 ("Consolidation Phase 2"), `bobmani` pulls the whole fleet in as git submodules — `bobmania` (his StepMania fork), `itgmania`, `ddc`, `ddc_onset`, `ffr-difficulty-model`, `hymnmania`, `ksm-v2`, `leraine-studio`, `linthesia`, `pianogame`, `beatoraja`, `arrowvortex`, `Simply-Love-SM5` — and builds his own Rust layer on top of it.

The Rust crate itself (v0.1.0, edition 2024) is an **axum + tokio** server with WebSocket support and **tract-onnx** for ONNX model inference — the shape of a local services/inference hub for the rhythm-game tooling (chart analysis/generation models are the obvious payload, given the fleet it consolidates).

## Status

Early consolidation stage: the submodule fleet and the server skeleton exist; the unified tooling the repo is named for is still being built. If you want the individual projects, their own repos (linked as the submodules above) are the usable entry points today.
