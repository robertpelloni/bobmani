# Ideas & Pivots

- **Aggressive Refactoring:** As we port old submodules, do not just copy-paste. Aggressively identify redundant code paths and merge them into high-performance, generic Rust traits and structs.
- **WASM Frontend:** Consider compiling core rust logic to WebAssembly (WASM) to maintain identical models and validation logic directly within the frontend UI.
- **Concurrency Overhaul:** Leverage Rust's fearless concurrency (`Tokio` or `Rayon`) to parallelize bottlenecks that previously slowed down the isolated submodules.