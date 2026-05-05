param(
  [string]$AppDataRoot,
  [switch]$Apply,
  [switch]$DryRun,
  [switch]$Help
)

if ($Help) {
  @'
initialize-machine-config.ps1 — prepare machine directories and report readiness

Usage:
  scripts/initialize-machine-config.ps1 [-AppDataRoot PATH] [-Apply] [-DryRun]
'@ | Write-Output
  exit 0
}

if (-not $AppDataRoot) { $AppDataRoot = $env:LOCAL_AI_APP_DATA_ROOT }
if (-not $AppDataRoot) { $AppDataRoot = Join-Path $env:LOCALAPPDATA 'AI4Pro/AILauncher' }
if ($DryRun) { $Apply = $false }

$ModelsDir = Join-Path $AppDataRoot 'models'
$LogsDir = Join-Path $AppDataRoot 'logs'
$ConfigDir = Join-Path $AppDataRoot 'config'
$DataDir = Join-Path $AppDataRoot 'data'
$LlamaModel = Join-Path $ModelsDir 'llama/default/model.gguf'
$SherpaDir = Join-Path $ModelsDir 'sherpa/default'
$SherpaRuntime = Join-Path $AppDataRoot 'runtime/sherpa-onnx-vit'
$SqlitePath = Join-Path $DataDir 'local_ai.sqlite'

if ($Apply) {
  New-Item -ItemType Directory -Force -Path $AppDataRoot, $ModelsDir, $LogsDir, $ConfigDir, $DataDir, $SherpaRuntime | Out-Null
}

Write-Output 'AI Engine Launcher Machine Setup'
Write-Output ''
Write-Output ("App data root: {0}" -f $AppDataRoot)
Write-Output ("Language model: {0}" -f $(if (Test-Path $LlamaModel) { 'OK' } else { 'MISSING' }))
Write-Output ("Speech model: {0}" -f $(if (Test-Path $SherpaDir) { 'OK' } else { 'MISSING' }))
Write-Output 'llama binary: Bundled'
Write-Output ("sherpa runtime: {0}" -f $(if ((Test-Path (Join-Path $SherpaRuntime 'python3')) -or (Test-Path (Join-Path $SherpaRuntime 'python.exe'))) { 'Packaged Python runtime' } else { 'Missing packaged runtime' }))
Write-Output ("SQLite: {0}" -f $(if (Test-Path $SqlitePath) { 'Initialized' } else { 'Pending' }))
Write-Output ''
Write-Output ("Result: {0}" -f $(if ((Test-Path $LlamaModel) -and (Test-Path $SherpaDir)) { 'Machine needs database/runtime verification.' } else { 'Machine is not ready.' }))
