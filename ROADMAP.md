# ROADMAP: Consolidated Architecture & Ecosystem Expansion

## Current Focus (v0.2.2)
- [x] Consolidate dashboard interface into single-page layout organized by priority tiers (Orchestration, Cognitive Intelligence, Supervisor/Tray, Multi-Modal I/O).
- [x] Resolve Rust compilation warnings and verify clean build in central workspace.
- [x] Validate system tray background daemon and graceful shutdown handlers.
- [x] Connect Axum WebSocket telemetry (`/ws`) to handle JSON requests (`ping`, `status`, `parse_sm`).
- [ ] Connect WebSocket streams to live frontend chart waveform and pattern inspection components.
- [ ] Implement ONNX model inference triggering over WebSocket command channel.

## Long-Term Milestones
1. **Core-Mani WASM Engine**: Complete cross-compilation of timing, chart parsing, and judgment logic into WebAssembly.
2. **Universal Rhythm Controller**: Unify multi-controller inputs (DDR dance pads, IIDX controllers, MIDI keyboards) across all submodules.
3. **Embedded Bobcoin Proof-of-Play (PoP)**: On-chain accuracy validation and token reward integration.
