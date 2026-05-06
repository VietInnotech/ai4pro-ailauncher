param(
  [string]$RepoRoot,
  [string]$BinariesDir,
  [string]$RuntimeDir,
  [switch]$Apply,
  [switch]$DryRun,
  [switch]$Help
)

if ($Help) {
  @'
prepare-sidecars.ps1 — scaffold the planned runtime layout

Usage:
  scripts/prepare-sidecars.ps1 [-RepoRoot PATH] [-BinariesDir PATH] [-RuntimeDir PATH] [-Apply] [-DryRun]

This script is developer-facing scaffolding only. It prepares or previews the release bundle input layout for llama sidecars plus the packaged sherpa runtime root.
'@ | Write-Output
  exit 0
}

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
if (-not $RepoRoot) { $RepoRoot = (Resolve-Path (Join-Path $scriptDir '..')).Path }
if (-not $BinariesDir) { $BinariesDir = Join-Path $RepoRoot 'src-tauri/bundle/binaries' }
if (-not $RuntimeDir) { $RuntimeDir = Join-Path $RepoRoot 'src-tauri/bundle/runtime/sherpa-onnx-vit' }
if ($DryRun) { $Apply = $false }

$expected = @(
  'llama-server-aarch64-apple-darwin',
  'llama-server-x86_64-pc-windows-msvc.exe'
)

Write-Output "Llama sidecar root: $BinariesDir"
Write-Output "Sherpa runtime root: $RuntimeDir"
Write-Output ("Mode: {0}" -f $(if ($Apply) { 'apply' } else { 'dry-run' }))

if ($Apply) {
  New-Item -ItemType Directory -Force -Path $BinariesDir, $RuntimeDir | Out-Null
} else {
  Write-Output ("mkdir -p {0}" -f $BinariesDir)
  Write-Output ("mkdir -p {0}" -f $RuntimeDir)
}

foreach ($name in $expected) {
  $path = Join-Path $BinariesDir $name
  if (Test-Path $path) {
    Write-Output "present: $name"
  } else {
    Write-Output "missing: $name (manual build still required)"
  }
}

foreach ($name in @('python3', 'python.exe')) {
  $path = Join-Path $RuntimeDir $name
  if (Test-Path $path) {
    Write-Output "present: $path"
  } else {
    Write-Output "missing: $path (expected for packaged sherpa runtime)"
  }
}

Write-Output 'Run bun run validate:bundle-artifacts before packaging.'
Write-Output ("See {0} for the explicit runtime layout." -f (Join-Path $RepoRoot 'src-tauri/binaries/expected-layout.md'))
