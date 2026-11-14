# Deployment & Environment Setup

## Build Instructions
This project has transitioned into a single, comprehensive Rust application with multi-engine harness support.

To build the project for deployment:
```bash
cargo build --release
```

The resulting binary will be located at `./target/release/app`.

## Running the Services
1. **Rust Monolith & WebSocket Server**:
   ```bash
   cargo run --release
   ```
   Listens on `127.0.0.1:3000` for WebSocket connections (`/ws`).

2. **Go Desktop Supervisor & System Tray**:
   ```bash
   (cd pianogame/go && go run main.go)
   ```

3. **StepMania / Bobmania Dashboard Audit**:
   ```bash
   (cd bobmania && python3 scripts/audit_dashboard.py)
   ```

## Environment
- **Language:** Rust (2024 Edition), Go (1.24+), TypeScript / Node.js
- **Dependency Manager:** Cargo, Go modules, npm

## Dashboard (0.2.11)
```powershell
cargo run            # or .\target\debug\app.exe
# Open http://127.0.0.1:3000/  (dashboard)  ·  /health  ·  ws://127.0.0.1:3000/ws
```
Unified build/test: `pwsh scripts/build.ps1`. Tray supervisor: `cd pianogame\go; go run .` (console only, Ctrl-C quits; no OS tray icon yet).
