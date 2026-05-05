#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'EOF'
build-sherpa-onnx.sh — packaged runtime scaffold for sherpa-onnx-vit

Usage:
  scripts/build-sherpa-onnx.sh [--repo-root PATH] [--source-dir PATH] [--runtime-dir PATH] [--check-only]

This script is intentionally non-building scaffolding. It documents the packaged Python runtime contract and can only check for already-produced runtime artifacts.
EOF
}

repo_root=""
source_dir=""
runtime_dir=""
check_only=0

while (($#)); do
  case "$1" in
    --repo-root) repo_root="${2:-}"; shift 2 ;;
    --source-dir) source_dir="${2:-}"; shift 2 ;;
    --runtime-dir) runtime_dir="${2:-}"; shift 2 ;;
    --check-only) check_only=1; shift ;;
    -h|--help) usage; exit 0 ;;
    *) printf 'Unknown argument: %s\n' "$1" >&2; usage >&2; exit 2 ;;
  esac
done

script_dir="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
: "${repo_root:=$(CDPATH= cd -- "$script_dir/.." && pwd)}"
: "${runtime_dir:=$repo_root/src-tauri/runtime/sherpa-onnx-vit}"

printf 'sherpa-onnx-vit packaged runtime scaffold\n'
printf 'Repo root: %s\n' "$repo_root"
printf 'Source dir: %s\n' "${source_dir:-<gh repo clone path for VietInnotech/sherpa-onnx-vit>}"
printf 'Runtime dir: %s\n' "$runtime_dir"
printf 'Pinned upstream commit: %s\n' '6a7fe63ded85cd089bff73c12c54e1bda3bd7cf3'

if ((check_only)); then
  for name in python3 python.exe; do
    if [[ -e "$runtime_dir/$name" ]]; then
      printf 'present: %s\n' "$runtime_dir/$name"
    else
      printf 'missing: %s/%s\n' "$runtime_dir" "$name"
    fi
  done
  exit 0
fi

printf 'No build is performed here.\n'
printf 'Package a Python runtime that can launch `python -m sherpa_onnx_vit` and place it under src-tauri/runtime/sherpa-onnx-vit/.\n'
printf 'The runtime must include FastAPI, uvicorn, and the sherpa_onnx_vit package from VietInnotech/sherpa-onnx-vit at commit 6a7fe63ded85cd089bff73c12c54e1bda3bd7cf3.\n'
