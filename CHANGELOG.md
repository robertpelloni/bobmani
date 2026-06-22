# Changelog

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
