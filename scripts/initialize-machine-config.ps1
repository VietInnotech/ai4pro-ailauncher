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
$SherpaConfigDir = Join-Path $ConfigDir 'sherpa'
$DataDir = Join-Path $AppDataRoot 'data'
$LlamaModel = Join-Path $ModelsDir 'llama/default/model.gguf'
$SherpaViDir = Join-Path $ModelsDir 'stt/gipformer-65M-rnnt'
$SherpaEnDir = Join-Path $ModelsDir 'stt/sherpa-onnx-zipformer-en-libriheavy-20230830-medium-punct-case'
$SherpaModelsConfig = Join-Path $SherpaConfigDir 'models.local.json'
$SherpaRuntime = Join-Path $AppDataRoot 'runtime/sherpa-onnx-vit'
$SqlitePath = Join-Path $DataDir 'local_ai.sqlite'

if ($Apply) {
  New-Item -ItemType Directory -Force -Path $AppDataRoot, $ModelsDir, $LogsDir, $ConfigDir, $SherpaConfigDir, $DataDir, $SherpaRuntime | Out-Null
  if (-not (Test-Path $SherpaModelsConfig)) {
@'
{
  "models": [
    {
      "id": "stt-vi",
      "language": "vi",
      "model_dir": "models/stt/gipformer-65M-rnnt",
      "postprocess_mode": "capu",
      "vad_min_silence": 0.5
    },
    {
      "id": "stt-en",
      "language": "en",
      "model_dir": "models/stt/sherpa-onnx-zipformer-en-libriheavy-20230830-medium-punct-case",
      "postprocess_mode": "none",
      "vad_min_silence": 0.5
    }
  ]
}
'@ | Set-Content -Encoding UTF8 $SherpaModelsConfig
  }
}

Write-Output 'AI Engine Launcher Machine Setup'
Write-Output ''
Write-Output ("App data root: {0}" -f $AppDataRoot)
Write-Output ("Language model: {0}" -f $(if (Test-Path $LlamaModel) { 'OK' } else { 'MISSING' }))
Write-Output ("Speech model (vi): {0}" -f $(if (Test-Path $SherpaViDir) { 'OK' } else { 'MISSING' }))
Write-Output ("Speech model (en): {0}" -f $(if (Test-Path $SherpaEnDir) { 'OK' } else { 'MISSING' }))
Write-Output ("Sherpa registry: {0}" -f $(if (Test-Path $SherpaModelsConfig) { 'OK' } else { 'MISSING' }))
Write-Output 'llama binary: Bundled'
Write-Output ("sherpa runtime: {0}" -f $(if ((Test-Path (Join-Path $SherpaRuntime 'python3')) -or (Test-Path (Join-Path $SherpaRuntime 'python.exe'))) { 'Packaged Python runtime' } else { 'Missing packaged runtime' }))
Write-Output ("SQLite: {0}" -f $(if (Test-Path $SqlitePath) { 'Initialized' } else { 'Pending' }))
Write-Output ''
Write-Output ("Result: {0}" -f $(if ((Test-Path $LlamaModel) -and (Test-Path $SherpaViDir) -and (Test-Path $SherpaEnDir) -and (Test-Path $SherpaModelsConfig)) { 'Machine needs database/runtime verification.' } else { 'Machine is not ready.' }))
