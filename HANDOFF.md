# Session Handoff Document

## Current Status
We are actively porting the massive `bobmani` submodules into a unified `Rust` monolith. The architecture relies on translating math, boundaries, struct outlines, and parsers from legacy Python/C++ code into memory-safe Rust primitives.

## Completed Work in This Session
- **Frontend Dashboard Wiring:** Fully scaffolded the Vite + React 19 Frontend application configuration in `frontend-vite/`. Added `App.tsx` containing the logic to communicate directly with our backend `AutoChart` orchestrator securely via `POST /api/generate`. Configured `vite.config.ts` to seamlessly proxy to our local Axum node on port `3000`.
- **Web API Bootstrapping:** Imported `axum` and `tokio` (along with `serde` for serialization). Bound `src/main.rs` to serve a `POST /api/generate` endpoint, successfully exposing our `AutoChart` machine learning struct processes to external frontend queries.
- Ported the remaining components of the `arrowvortex` core parsing engine (`NoteCol.cpp` logic and `NoteUtils.h` helper loops) to `src/arrowvortex/`.
- Wrote full `integration_test.rs` to verify that `arrowvortex`, `ddc`, `ddc_onset`, and `ffr_diff_model` can be instantiated within the same memory boundaries successfully without breaking constraints.
- Ported the `autochart.py` and `autochart_lib.py` logic natively into `src/ddc/autochart.rs`, establishing the bounds for parsing logic and orchestrating the CNN mapping sequence across all difficulties.
- Ported the `util.py` logic from `ddc`, creating string-cleaning utilities in `src/ddc/util.rs`.
- Explicitly refined the `cnn.py` bounds from `ddc_onset`, creating structural Rust equivalents (`Conv2dDef`, `LinearDef`) to trace the network's layers safely before binding to ONNX or PyTorch integrations.
- Ported the `constants.py` file from `ddc_onset`, ensuring DDR difficulties and algorithmic thresholds are mapped natively to `constants.rs`.
- Ported the `util.py` logic from `ddc_onset`, adding native Rust 1D convolution, peak-finding, and thresholding into `src/ddc_onset/util.rs`. This successfully eliminates legacy dependencies on SciPy.
- Ported the `create_splits.py` logic from the `ddc` dataset pipeline into `DatasetSplitter` in `ddc/create_splits.rs`. Added the `rand` crate to support it safely and ensure deterministic paths.
- Ported the `dataset_json.py` dataset generator logic from the `ddc/ddc_stepmania` pipeline into `DatasetJsonGenerator` in `ddc/dataset_json.rs`.
- Re-established `PROJECT_MEMORY`.
- Continually tracked progress in `TODO.md` and compiled the system securely with `cargo check`, `cargo test`, and `npm run build`.
- Committed all individual logical chunks step-by-step.

## Submodules
1. `ffr-difficulty-model`: Extractor functions and SM preprocessor ported. ML prediction logic is stubbed out.
2. `ddc` (Dance Dance Convolution): Base ML structs mapped, timing `beatcalc.rs` logic translated. `Chart`, `OnsetChart`, `SymbolicChart` boundaries added. `dataset_json.rs`, `create_splits.rs`, and `util.rs` logic ported for splits and parsing. `autochart.rs` orchestrator ported.
3. `ddc_onset`: Neural network structs mapped (`PlacementCNN` layer bounds), PyTorch inferences stubbed. `util.rs` port eliminates SciPy dependencies for peak finding. `constants.rs` fully mapped.
4. `arrowvortex`: Core `Chart`, `Simfile`, `Tempo`, `GameMode`, `NoteType`, `ExpandedNote`, and `.sm` parser logic natively ported to safe Rust. Added `Segments`, `TimingData`, `NoteCol`, and `NoteUtils` completing the core structural map.

## Next Immediate Steps for the Next Session
1. Execute the Git sanitization protocol (fetch, pull, update submodules).
2. The core structures and API layers are established. The remaining massive hurdle is integrating actual native Machine Learning prediction matrices into the rust engine using crates like `tch` (PyTorch) or `tract` (ONNX).
3. Keep adhering strictly to the documented workflow rules! DON'T EVER STOP THE PARTY!

**Last Verified Build Status:** Clean `cargo check` and `npm run build` passing.
