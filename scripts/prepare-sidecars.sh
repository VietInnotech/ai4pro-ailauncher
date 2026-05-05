#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'EOF'
prepare-sidecars.sh — scaffold the planned runtime layout

Usage:
  scripts/prepare-sidecars.sh [--repo-root PATH] [--binaries-dir PATH] [--runtime-dir PATH] [--apply] [--dry-run]

This script is developer-facing scaffolding only. It does not build binaries.
It prepares or previews the llama sidecar directory plus the packaged sherpa runtime root.
EOF
}

repo_root=""
binaries_dir=""
runtime_dir=""
apply=0

while (($#)); do
  case "$1" in
    --repo-root)
      repo_root="${2:-}"; shift 2 ;;
    --binaries-dir)
      binaries_dir="${2:-}"; shift 2 ;;
    --runtime-dir)
      runtime_dir="${2:-}"; shift 2 ;;
    --apply)
      apply=1; shift ;;
    --dry-run)
      apply=0; shift ;;
    -h|--help)
      usage; exit 0 ;;
    *)
      printf 'Unknown argument: %s\n' "$1" >&2
      usage >&2
      exit 2 ;;
  esac
done

script_dir="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
: "${repo_root:=$(CDPATH= cd -- "$script_dir/.." && pwd)}"
: "${binaries_dir:=$repo_root/src-tauri/binaries}"
: "${runtime_dir:=$repo_root/src-tauri/runtime/sherpa-onnx-vit}"

expected_llama=(
  "llama-server-aarch64-apple-darwin"
  "llama-server-x86_64-apple-darwin"
  "llama-server-x86_64-pc-windows-msvc.exe"
)

mkdir_preview() {
  if ((apply)); then
    mkdir -p "$1"
  else
    printf 'mkdir -p %s\n' "$1"
  fi
}

printf 'Llama sidecar root: %s\n' "$binaries_dir"
printf 'Sherpa runtime root: %s\n' "$runtime_dir"
printf 'Mode: %s\n' "$( ((apply)) && printf apply || printf dry-run )"
mkdir_preview "$binaries_dir"
mkdir_preview "$runtime_dir"

for name in "${expected_llama[@]}"; do
  if [[ -e "$binaries_dir/$name" ]]; then
    printf 'present: %s\n' "$name"
  else
    printf 'missing: %s (manual build still required)\n' "$name"
  fi
done

for name in python3 python.exe; do
  if [[ -e "$runtime_dir/$name" ]]; then
    printf 'present: %s\n' "$runtime_dir/$name"
  else
    printf 'missing: %s/%s (expected for packaged sherpa runtime)\n' "$runtime_dir" "$name"
  fi
done

printf 'See %s for the explicit placeholder layout.\n' "$repo_root/src-tauri/binaries/expected-layout.md"
