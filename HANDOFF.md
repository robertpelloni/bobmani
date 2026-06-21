# Handoff

## Session Summary (Rust Porting Phase 2)
During this session, the project transitioned out of its uninitialized state. The core `bobmani` meta-repo was synchronized, bringing in all 13 legacy submodules (such as `Simply-Love-SM5`, `beatoraja`, `ffr-difficulty-model`, etc.).

With the source code available, the directive to "port all submodules into a massive rust program" actively began.

The first target was the `ffr-difficulty-model`.
- Scaffolding for `src/ffr_diff_model/` was created.
- The Python mathematical feature extractors `HorizontalDensity`, `VerticalDensity`, `StreamDetector`, and `PatternDetector` were successfully translated into performant, memory-safe Rust structs.
- A custom Rust `SMChartPreprocessor` was written to parse `.sm` files using the `regex` crate, effectively replacing the Python `simfile` dependency.
- The entire pipeline was wired into `main.rs` and executed successfully against a test file.

The second target is the `ddc` (Dance Dance Convolution) model.
- Scaffolding for `src/ddc/mod.rs` and `src/ddc/autochart.rs` was created.
- The `AutoChart` struct was implemented as an empty boundary ready to ingest the ML onset-detection logic.
- The core math logic from `ddc/learn/beatcalc.py` was translated into `src/ddc/beatcalc.rs`, handling complex timing intersections, 'stops', and epsilon-based sub-divisions required for rhythm game timing.
- The boundary structure for audio processing (`src/ddc/extract_feats.rs`) was scaffolded. It correctly houses the constants required by `librosa`'s Mel Spectrogram extraction, establishing the correct 3D array inputs necessary for the DDC neural networks.

## Current State
The project has a firm Rust monolith foundation. However, code review has flagged that the actual ML model inference (for the difficulty predictor) and raw audio processing (for DDC) are currently stubbed.

**Next Immediate Steps for Successor Models:**
1. Focus on `src/ddc/extract_feats.rs`. You must replace the `librosa` stub logic with actual Rust audio processing. We recommend utilizing `hound` (to parse `.wav` files) and `rustfft` (to calculate the Short-Time Fourier Transform).
2. For the `ffr-difficulty-model`, you need to implement a parser for the scikit-learn `.p` pickle weights or port the model to ONNX so `predict()` yields a real difficulty integer rather than a placeholder map.
3. Maintain the unified version number (currently `0.1.2`) and update `CHANGELOG.md` accordingly. Ensure commits are made after every major step.
