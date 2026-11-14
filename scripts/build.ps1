# bobmani unified build — Rust hub + Go harness. Safe: never overwrites docs.
# Usage: pwsh scripts/build.ps1 [-Release]
param([switch]$Release)
$ErrorActionPreference = 'Stop'
$root = Split-Path $PSScriptRoot -Parent
Push-Location $root
try {
  $ver = (Get-Content VERSION.md -TotalCount 1) -replace ' .*',''
  Write-Host "bobmani $ver — cargo test"
  if ($Release) { cargo test --release; cargo build --release } else { cargo test; cargo build }
  if ($LASTEXITCODE -ne 0) { throw "cargo failed" }
  Write-Host "Go harness tests (incl. tray supervisor)"
  Push-Location pianogame\go; go test ./...; if ($LASTEXITCODE -ne 0) { throw "go test failed" }; Pop-Location
  Write-Host "OK. Run: .\target\$(if($Release){'release'}else{'debug'})\app.exe  ->  http://127.0.0.1:3000/"
} finally { Pop-Location }
