param(
  [string]$RepoRoot,
  [string]$SourceDir,
  [string]$RuntimeDir,
  [switch]$CheckOnly,
  [switch]$Help
)

if ($Help) {
  @'
build-sherpa-onnx.ps1 — packaged runtime scaffold for sherpa-onnx-vit

Usage:
  scripts/build-sherpa-onnx.ps1 [-RepoRoot PATH] [-SourceDir PATH] [-RuntimeDir PATH] [-CheckOnly]

This script is intentionally non-building scaffolding. It documents the packaged Python runtime contract and can only check for already-produced runtime artifacts.
'@ | Write-Output
  exit 0
}

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
if (-not $RepoRoot) { $RepoRoot = (Resolve-Path (Join-Path $scriptDir '..')).Path }
if (-not $RuntimeDir) { $RuntimeDir = Join-Path $RepoRoot 'src-tauri/bundle/runtime/sherpa-onnx-vit' }

Write-Output 'sherpa-onnx-vit packaged runtime scaffold'
Write-Output ("Repo root: {0}" -f $RepoRoot)
Write-Output ("Source dir: {0}" -f ($(if ($SourceDir) { $SourceDir } else { '<gh repo clone path for VietInnotech/sherpa-onnx-vit>' })))
Write-Output ("Runtime dir: {0}" -f $RuntimeDir)
Write-Output 'Pinned upstream commit: 2ce802dc045dbb306d38085423de5327d45f1d26'

if ($CheckOnly) {
  foreach ($name in @('python3', 'python.exe')) {
    $path = Join-Path $RuntimeDir $name
    if (Test-Path $path) { Write-Output "present: $path" } else { Write-Output "missing: $path" }
  }
  exit 0
}

Write-Output 'No build is performed here.'
Write-Output 'Package a Python runtime that can launch `python -m sherpa_onnx_vit` and place it under src-tauri/bundle/runtime/sherpa-onnx-vit/.'
Write-Output 'The runtime must include FastAPI, uvicorn, and the sherpa_onnx_vit package from VietInnotech/sherpa-onnx-vit at commit 2ce802dc045dbb306d38085423de5327d45f1d26.'
