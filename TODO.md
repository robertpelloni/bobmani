# TODO

- [x] Initialize Rust Cargo application
- [x] Define global overarching vision and documentation
- [x] Sync upstream to bring in legacy submodules
- [x] Begin porting `ffr-difficulty-model` (Horizontal/Vertical Density)
- [x] Complete port of `ffr-difficulty-model` (Stream/Pattern Detectors, Preprocessor)
- [ ] Implement Rust ML Model Inference (ONNX or manual weights) for ffr-difficulty-model
- [x] Begin porting `arrowvortex` (C++ structs and .sm loading logic, Tempo, Segments, NoteList, NoteSet, TimingData)
- [x] Complete port of `arrowvortex` (Remaining elements: NoteCol, NoteUtils)
- [x] Begin porting `ddc` and `ddc_onset` (Data models, basic logic, dataset parsing)
- [x] Complete port of `ddc` and `ddc_onset` dataset JSON logic and split tools
- [x] Complete port of `ddc_onset` util logic (peak finding, thresholds, constants)
- [x] Refine `ddc_onset` CNN logic bounds into explicit safe Rust struct parameters
- [x] Port `ddc` utilities (`ezname` formatters)
- [x] Scaffold Vite + React Frontend
- [x] Complete port of `ddc` and `ddc_onset` remaining logic (autochart)
- [x] Write integration tests for the unified architecture
- [ ] Link backend engine to frontend API (via lightweight web server like axum)
