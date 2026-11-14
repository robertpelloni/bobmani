# Changelog

## [0.2.10] - 2026-11-08
### Added
- Implemented `parse_sm_content` in `src/arrowvortex/load_sm.rs` to parse in-memory StepMania Simfiles without disk I/O bottlenecks.
- Wired native `arrowvortex` simfile and chart parsing directly into `WsRequest::ParseSm` in `src/server.rs`.
- Added unit testing for in-memory StepMania simfile parsing (`test_parse_sm_content_basic`).

### Changed
- Bumped version to `0.2.10` in `VERSION.md`.
- Synchronized `ROADMAP.md`, `TODO.md`, `MEMORY.md`, and `HANDOFF.md`.

## [0.2.9] - 2026-11-08
### Added
- Integrated `beatoraja::score_data::ScoreData` into Axum WebSocket server via `CalculateExScore` command (`src/server.rs`).
- Added `ScoreAnalysis` response returning calculated EX score, maximum achievable score, accuracy percentage, and integrity validation state.
- Added comprehensive unit testing for real-time EX score calculation (`test_process_calculate_exscore`).

### Changed
- Bumped version to `0.2.9` in `VERSION.md`.
- Synchronized `ROADMAP.md`, `TODO.md`, `MEMORY.md`, and `HANDOFF.md`.

## [0.2.8] - 2026-11-08
### Added
- Created `scripts/sync_submodules.py` providing automated verification and synchronization checks across all 13 project submodules.
- Validated clean working trees, published commit pins, and remote branch tracking across all submodules.

### Changed
- Bumped version to `0.2.8` in `VERSION.md`.
- Synchronized `ROADMAP.md`, `TODO.md`, `MEMORY.md`, and `HANDOFF.md`.

## [0.2.7] - 2026-11-08
### Added
- Extended Axum WebSocket command pipeline with `DetectOnsets` request action and `OnsetDetections` response telemetry (`src/server.rs`).
- Integrated `find_peaks` and `threshold_peaks` from `crate::ddc_onset::util` to detect local note onset maxima and filter by threshold dynamically.
- Added comprehensive unit testing for real-time onset peak detection (`test_process_detect_onsets`).

### Changed
- Bumped version to `0.2.7` in `VERSION.md`.
- Synchronized `ROADMAP.md`, `TODO.md`, `MEMORY.md`, and `HANDOFF.md`.

## [0.2.6] - 2026-11-08
### Added
- Extended `WsBridge` in `pianogame/typescript/src/ws_bridge.ts` with `formatRustRequest`, `parseRustResponse`, and `broadcastMessage`.
- Aligned TypeScript frontend bridge protocol with all Rust Axum `WsRequest` actions (`ping`, `status`, `parse_sm`, `analyze_density`, `extract_audio_features`).
- Added unit test suite `pianogame/typescript/src/test_ws_bridge.ts` compiled via `tsc` and executed via `node dist/test_ws_bridge.js`.

### Changed
- Bumped version to `0.2.6` in `VERSION.md`.
- Synchronized `ROADMAP.md`, `TODO.md`, `MEMORY.md`, and `HANDOFF.md`.

## [0.2.5] - 2026-11-08
### Added
- Implemented audio sample framing, root-mean-square energy calculation, and log-Mel feature generation in `MelExtractor::extract` (`src/ddc/extract_feats.rs`).
- Implemented multi-channel log-Mel spectrogram generation with 3 FFT frame lengths in `SpectrogramExtractor::forward` (`src/ddc_onset/spectral.rs`).
- Added `ExtractAudioFeatures` request action and `AudioFeatures` response telemetry to Axum WebSocket command pipeline (`src/server.rs`).
- Unit tests for audio feature generation and WebSocket command routing (`test_mel_extractor_dimensions`, `test_spectrogram_extractor_forward_batch_shape`, `test_process_extract_audio_features`).

### Changed
- Bumped version to `0.2.5` in `VERSION.md`.
- Synchronized `ROADMAP.md`, `TODO.md`, `MEMORY.md`, and `HANDOFF.md`.

## [0.2.4] - 2026-11-08
### Added
- Extended Axum WebSocket command pipeline with `AnalyzeDensity` request action and `DensityAnalysis` response telemetry.
- Integrated `HorizontalDensity`, `StreamDetector`, and `PatternDetector` difficulty features directly into `src/server.rs` message routing.
- Added comprehensive unit testing for real-time density and pattern analysis over WebSocket (`test_process_analyze_density`).
- Dynamic version extraction in WebSocket status handler using `env!("CARGO_PKG_VERSION")`.

### Changed
- Bumped version to `0.2.4` in `VERSION.md`.
- Synchronized `ROADMAP.md`, `TODO.md`, `MEMORY.md`, and `HANDOFF.md`.

## [0.2.3] - 2026-11-08
### Added
- Exposed complete module hierarchy (`arrowvortex`, `beatoraja`, `ddc`, `ddc_onset`, `ffr_diff_model`, `server`) across library root `src/lib.rs` and application root `src/main.rs`.
- Exposed internal onset components (`cnn`, `constants`, `spectral`, `util`) in `src/ddc_onset/mod.rs`.
- Standardized `tract-onnx` neural model representation using `Arc<TypedRunnableModel>` in `src/ddc_onset/cnn.rs`.

### Fixed
- Cleaned compiler warnings in `src/ddc/` (`beatcalc.rs`, `chart.rs`, `dataset_json.rs`, `extract_feats.rs`), achieving zero-warning clean compilation with `cargo check` and `cargo test`.

### Changed
- Bumped version to `0.2.3` in `VERSION.md`.
- Updated `ROADMAP.md`, `TODO.md`, `MEMORY.md`, and `HANDOFF.md`.

## [0.2.2] - 2026-11-08
### Added
- Implemented strongly typed Axum WebSocket command dispatch in `src/server.rs` with `serde` and `serde_json`.
- Added request parsing for `ping` (Pong response with UNIX timestamp), `status` (reporting active submodules and healthy state), and `parse_sm` (extracting StepMania Simfile title, artist, and chart count).
- Added comprehensive unit tests in `src/server.rs` covering all command types and error handling.
- Added automated browser-based end-to-end WebSocket integration test suite using Playwright.

### Changed
- Bumped version to `0.2.2` in `VERSION.md`.
- Updated `ROADMAP.md`, `TODO.md`, `MEMORY.md`, and `HANDOFF.md`.

## [0.2.1] - 2026-11-08
### Added
- Redesigned and consolidated `pianogame` control surface into a unified single-page mission control dashboard organized into 4 high-value prioritized tiers (Autonomous Orchestration, Cognitive Intelligence, Supervisor & System Tray, Multi-Modal I/O) plus streaming telemetry.
- Comprehensive tooltips, labels, and interactive forms across all 17 interactive cards with version badge synchronized to v5.91.0.
- Verified and audited `bobmania` unified StepMania dashboard overlay (`ScreenUnifiedDashboard overlay.lua`) covering all 13 core modules.

### Fixed
- Fixed 5 compiler warnings in the root Rust workspace (`load_sm.rs`, `note_list.rs`, `timing_data.rs`, `chart.rs`, `preprocessor.rs`), restoring zero-warning clean compilation with `cargo check` and `cargo test`.
- Verified Go test suite and desktop system tray supervisor lifecycle with clean health polling and graceful server termination.

### Changed
- Updated system documentation: `ROADMAP.md`, `TODO.md`, `VISION.md`, `MEMORY.md`, `DEPLOY.md`, `IDEAS.md`, `VERSION.md` (bumped to 0.2.1), and `HANDOFF.md`.

## [0.1.1] - Massive Rust Port Initialization
- Resolved missing `Cargo.toml` error by officially initializing the central Cargo workspace.
- Updated core project directives (ROADMAP, TODO, VISION) to mandate the translation and consolidation of all previous submodules into this massive Rust app.

## [0.1.0] - Initial Setup
- Created baseline documentation scaffolding.
## [0.1.2] - FFR Difficulty Model Scaffolding
- Synced submodules to local workspace.
- Initialized Rust port for the `ffr-difficulty-model`.
- Scaffolded predictor and feature calculation interfaces.

## [0.1.3] - 2024-06-21
### Added
- Ported `NoteList` and `NoteSet` structures from `arrowvortex` to safe Rust.
- Ported `TimingData`, `Event`, and `Signature` structures from `arrowvortex`.
### Fixed
- Fixed the `.sm` file parser (`load_sm.rs`) to use the encapsulated `NoteList::append` method.
- Resolved compilation recovery errors with the Segment code logic.

## [0.1.4] - 2024-06-21
### Added
- Ported `dataset_json.py` from `ddc` to `DatasetJsonGenerator` in native Rust.
- Included the `rand` crate to handle random dataset shuffling natively.

## [0.1.5] - 2024-06-21
### Added
- Ported `create_splits.py` from `ddc` to `DatasetSplitter` in native Rust `create_splits.rs`.

## [0.1.6] - 2024-06-21
### Added
- Ported `util.py` logic from `ddc_onset` into `src/ddc_onset/util.rs`.
- Implemented native Rust peak-finding algorithms and 1D Hamming convolution (replacing `scipy` dependencies).

## [0.1.7] - 2024-06-21
### Added
- Ported `constants.py` from `ddc_onset` into native Rust `constants.rs`.

## [0.1.8] - 2024-06-22
### Refined
- Replaced stub CNN logic in `src/ddc_onset/cnn.rs` with explicitly typed `Conv2dDef`, `MaxPool2dDef`, and `LinearDef` structural outlines for `PlacementCNN` boundary iteration without PyTorch dependency bloat.

## [0.1.9] - 2024-06-22
### Added
- Ported `util.py` logic from `ddc` into native Rust string sanitizers (`ezname`) in `src/ddc/util.rs`.

## [0.1.17] - 2024-06-22
### Fixed
- Reconciled and sanitized the `git` submodule pointers across branches to ensure safe merging of `main` without destroying detached heads (`beatoraja`, `bobmania`, `hymnmania`).

## [1.0.0] - 2024-06-24
### Changed
- Major architectural pivot: Transitioned core orchestration structure to a **Go-first architecture** representing "Jules Autopilot".
- Overrode legacy monolithic tracking; established `backend-go` entry point serving API/JSON manifest parameters.

## [1.0.1] - 2024-06-25
### Added
- Linked the `backend-go` runtime directly to a local `SQLite` driver instance utilizing `gorm.io/gorm`.
- Auto-migrated `MemoryChunk` and `Session` database struct bounds per the dual-layer RAG indexing roadmap requirements.
## [0.2.0] - 2024-05-XX
- Resolved Rust compilation errors.
- Successfully integrated tract-onnx bindings.

## [0.2.11] - 2026-11-14
### Added
- Single-page dashboard served by the Rust hub at `/` (src/dashboard.html, embedded via include_str!): labelled forms + tooltips for all 7 actions (ping, status, parse_sm, analyze_density, extract_audio_features, detect_onsets, calculate_ex_score), priority tiers, live telemetry log, fleet and tray cards. No subpages.
- `GET /health` JSON (ok, version, actions). Cargo.toml version aligned 0.1.0 -> 0.2.11 (was stale vs VERSION.md).
- `scripts/build.ps1` unified build/test; archived destructive `update_docs.sh` to `scripts/archive/` and replaced it with a deprecation stub.
### Verified
- cargo test: 28 passed (14 lib + 14 bin, incl. new test_dashboard_covers_all_actions). Live BelowNormal run: /health 200, / 200 (11076 bytes), WS pong/status/parse_sm correct. Go: go test ./... ok.

## [0.2.12] - 2026-11-14
### Added
- Live chart visualiser on the single dashboard page: canvas rendering notes (4 lanes x time) live from the density input, with rows/notes/span/notes-per-second meta and server NPS/stream/jack annotation on analyze_density response.
### Verified
- cargo test 28 pass, node --check on dashboard JS OK, live / 200 with canvas+drawChart present, /health reports 0.2.12 after rebuild.

## [0.2.13] - 2026-11-14
### Added
- `predict_placement` WS action (8th): PlacementCNN::predict_salience via tract-ONNX when models/placement.onnx or $BOBMANI_ONNX_MODEL exists; otherwise labelled onset-heuristic placements (source=model_loaded flags say which). Dashboard placement card in Tier 2.
### Verified
- cargo test 32 pass (incl. heuristic thresholds + WS no-model path). Live WS: placements [2,5], model_loaded false, source onset_heuristic_no_model.

## [0.2.14] - 2026-11-14
### Added
- Real ONNX fixture tests/fixtures/placement_identity.onnx (Relu, dynamic length; regenerate via scripts/make_onnx_fixture.py) proving the tract-ONNX branch: placement_with_model refactor for testability, live WS with $BOBMANI_ONNX_MODEL returns model_loaded:true, source:onnx.
### Verified
- cargo test 34 pass. Live: placements [0,1,2] on decreasing salience via ONNX (heuristic would return only [0]).
