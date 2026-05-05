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
data_dir="$app_data_root/data"
llama_model="$models_dir/llama/default/model.gguf"
sherpa_dir="$models_dir/sherpa/default"
sherpa_runtime="$app_data_root/runtime/sherpa-onnx-vit"
sqlite_path="$data_dir/local_ai.sqlite"

if ((apply)); then
  mkdir -p "$app_data_root" "$models_dir" "$logs_dir" "$config_dir" "$data_dir" "$sherpa_runtime"
fi

printf 'AI Engine Launcher Machine Setup\n\n'
printf 'App data root: %s\n' "$app_data_root"
printf 'Language model: %s\n' "$( [[ -f "$llama_model" ]] && printf OK || printf MISSING )"
printf 'Speech model: %s\n' "$( [[ -d "$sherpa_dir" ]] && printf OK || printf MISSING )"
printf 'llama binary: %s\n' "Bundled"
printf 'sherpa runtime: %s\n' "$( [[ -x "$sherpa_runtime/python3" || -f "$sherpa_runtime/python.exe" ]] && printf 'Packaged Python runtime' || printf 'Missing packaged runtime' )"
printf 'SQLite: %s\n' "$( [[ -f "$sqlite_path" ]] && printf Initialized || printf Pending )"
printf '\nResult: %s\n' "$( [[ -f "$llama_model" && -d "$sherpa_dir" ]] && printf 'Machine needs database/runtime verification.' || printf 'Machine is not ready.' )"
