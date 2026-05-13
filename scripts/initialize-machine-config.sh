#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'EOF'
initialize-machine-config.sh — prepare machine directories and report readiness

Usage:
  scripts/initialize-machine-config.sh [--app-data-root PATH] [--apply] [--dry-run]
EOF
}

app_data_root=""
apply=0

while (($#)); do
  case "$1" in
    --app-data-root) app_data_root="${2:-}"; shift 2 ;;
    --apply) apply=1; shift ;;
    --dry-run) apply=0; shift ;;
    -h|--help) usage; exit 0 ;;
    *) printf 'Unknown argument: %s\n' "$1" >&2; usage >&2; exit 2 ;;
  esac
done

: "${app_data_root:=${LOCAL_AI_APP_DATA_ROOT:-$HOME/.local/share/AI4Pro/AILauncher}}"
models_dir="$app_data_root/models"
logs_dir="$app_data_root/logs"
config_dir="$app_data_root/config"
sherpa_config_dir="$config_dir/sherpa"
data_dir="$app_data_root/data"
llama_model="$models_dir/llama/default/model.gguf"
sherpa_vi_dir="$models_dir/stt/gipformer-65M-rnnt"
sherpa_en_dir="$models_dir/stt/sherpa-onnx-zipformer-en-libriheavy-20230830-medium-punct-case"
sherpa_models_config="$sherpa_config_dir/models.local.json"
sherpa_runtime="$app_data_root/runtime/sherpa-onnx-vit"
sqlite_path="$data_dir/local_ai.sqlite"

if ((apply)); then
  mkdir -p "$app_data_root" "$models_dir" "$logs_dir" "$config_dir" "$sherpa_config_dir" "$data_dir" "$sherpa_runtime"
  if [[ ! -f "$sherpa_models_config" ]]; then
    cat >"$sherpa_models_config" <<EOF
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
EOF
  fi
fi

printf 'AI Engine Launcher Machine Setup\n\n'
printf 'App data root: %s\n' "$app_data_root"
printf 'Language model: %s\n' "$( [[ -f "$llama_model" ]] && printf OK || printf MISSING )"
printf 'Speech model (vi): %s\n' "$( [[ -d "$sherpa_vi_dir" ]] && printf OK || printf MISSING )"
printf 'Speech model (en): %s\n' "$( [[ -d "$sherpa_en_dir" ]] && printf OK || printf MISSING )"
printf 'Sherpa registry: %s\n' "$( [[ -f "$sherpa_models_config" ]] && printf OK || printf MISSING )"
printf 'llama binary: %s\n' "Bundled"
printf 'sherpa runtime: %s\n' "$( [[ -x "$sherpa_runtime/python3" || -f "$sherpa_runtime/python.exe" ]] && printf 'Packaged Python runtime' || printf 'Missing packaged runtime' )"
printf 'SQLite: %s\n' "$( [[ -f "$sqlite_path" ]] && printf Initialized || printf Pending )"
printf '\nResult: %s\n' "$( [[ -f "$llama_model" && -d "$sherpa_vi_dir" && -d "$sherpa_en_dir" && -f "$sherpa_models_config" ]] && printf 'Machine needs database/runtime verification.' || printf 'Machine is not ready.' )"
