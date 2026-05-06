param(
  [string]$RepoRoot,
  [string]$SourceDir,
  [string]$OutputDir,
  [switch]$CheckOnly,
  [switch]$Help
)

if ($Help) {
  @'
build-llama-cpp.ps1 — manual build scaffold for llama.cpp

Usage:
  scripts/build-llama-cpp.ps1 [-RepoRoot PATH] [-SourceDir PATH] [-OutputDir PATH] [-CheckOnly]

This script is intentionally non-building scaffolding. It describes the manual
build step and can only check for already-produced sidecars.
'@ | Write-Output
  exit 0
}

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
if (-not $RepoRoot) { $RepoRoot = (Resolve-Path (Join-Path $scriptDir '..')).Path }
if (-not $OutputDir) { $OutputDir = Join-Path $RepoRoot 'src-tauri/bundle/binaries' }

Write-Output 'llama.cpp build scaffold'
Write-Output ("Repo root: {0}" -f $RepoRoot)
Write-Output ("Source dir: {0}" -f ($(if ($SourceDir) { $SourceDir } else { '<manual checkout path>' })))
Write-Output ("Output dir: {0}" -f $OutputDir)

if ($CheckOnly) {
  foreach ($name in @(
    'llama-server-aarch64-apple-darwin',
    'llama-server-x86_64-pc-windows-msvc.exe'
  )) {
    $path = Join-Path $OutputDir $name
    if (Test-Path $path) { Write-Output "present: $name" } else { Write-Output "missing: $name" }
  }
  exit 0
}

Write-Output 'No build is performed here.'
Write-Output 'Manually build llama.cpp for the target platform, then place the resulting llama-server binary into src-tauri/bundle/binaries/.'
