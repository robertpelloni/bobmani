# bobmani unified build — Rust hub + Go harness. Safe: never overwrites docs.
# Usage: pwsh scripts/build.ps1 [-Release]
# Every step is exit-checked individually: the first version of this script
# checked only the last cargo command and printed OK while `cargo test`
# had failed (LNK1180, disk full). That false-green is why this exists.
param([switch]$Release)
$ErrorActionPreference = 'Stop'
$root = Split-Path $PSScriptRoot -Parent

function Step([string]$name, [scriptblock]$cmd) {
  Write-Host "== $name"
  & $cmd
  if ($LASTEXITCODE -ne 0) { throw "$name failed (exit $LASTEXITCODE)" }
}

Push-Location $root
try {
  $ver = (Get-Content VERSION.md -TotalCount 1) -replace ' .*',''
  Write-Host "bobmani $ver"
  # Disk guard: stale per-hash test PDBs accumulate in target/ (~150MB
  # each) and filled C: to 70MB free in 0.2.20, failing links with
  # LNK1180. Below 4GB free, drop only this package's artifacts first.
  $freeGB = [math]::Round((Get-PSDrive C).Free / 1GB, 2)
  Write-Host "C: free ${freeGB}GB"
  if ($freeGB -lt 4) {
    Write-Host "Low disk: cargo clean -p app (regenerable artifacts only)"
    cargo clean -p app
  }
  if ($Release) {
    Step "cargo test --release" { cargo test --release }
    Step "cargo build --release" { cargo build --release }
  } else {
    Step "cargo test" { cargo test }
    Step "cargo build" { cargo build }
  }
  Push-Location pianogame\go
  try { Step "go test ./..." { go test ./... } } finally { Pop-Location }
  Push-Location pianogame\typescript
  try {
    # Node's default TLS is rejected by registry.npmjs.org in this env
    # (alert 70); --tls-min-v1.2 is the documented fix (see HANDOFF 0.2.21).
    if (-not (Test-Path node_modules)) {
      $env:NODE_OPTIONS = '--tls-min-v1.2'
      try { Step "npm install" { npm install --ignore-scripts --no-audit --no-fund } }
      finally { Remove-Item Env:NODE_OPTIONS -ErrorAction SilentlyContinue }
    }
    Step "tsc" { npx tsc }
    Step "typescript tests" {
      Get-ChildItem dist\test_*.js | ForEach-Object {
        node $_.FullName | Out-Host
        if ($LASTEXITCODE -ne 0) { throw "TS test failed: $($_.Name)" }
      }
    }
  } finally { Pop-Location }
  $bin = if ($Release) { 'release' } else { 'debug' }
  Write-Host "OK. Run: .\target\$bin\app.exe  ->  http://127.0.0.1:3000/"
} finally { Pop-Location }
