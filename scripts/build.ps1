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
  if ($Release) {
    Step "cargo test --release" { cargo test --release }
    Step "cargo build --release" { cargo build --release }
  } else {
    Step "cargo test" { cargo test }
    Step "cargo build" { cargo build }
  }
  Push-Location pianogame\go
  try { Step "go test ./..." { go test ./... } } finally { Pop-Location }
  $bin = if ($Release) { 'release' } else { 'debug' }
  Write-Host "OK. Run: .\target\$bin\app.exe  ->  http://127.0.0.1:3000/"
} finally { Pop-Location }
