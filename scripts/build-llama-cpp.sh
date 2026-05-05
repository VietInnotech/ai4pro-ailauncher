#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'EOF'
build-llama-cpp.sh — manual build scaffold for llama.cpp

Usage:
  scripts/build-llama-cpp.sh [--repo-root PATH] [--source-dir PATH] [--output-dir PATH] [--check-only]

This script is intentionally non-building scaffolding. It describes the manual
build step and can only check for already-produced sidecars.
EOF
}

repo_root=""
source_dir=""
output_dir=""
check_only=0

while (($#)); do
  case "$1" in
    --repo-root) repo_root="${2:-}"; shift 2 ;;
    --source-dir) source_dir="${2:-}"; shift 2 ;;
    --output-dir) output_dir="${2:-}"; shift 2 ;;
    --check-only) check_only=1; shift ;;
    -h|--help) usage; exit 0 ;;
    *) printf 'Unknown argument: %s\n' "$1" >&2; usage >&2; exit 2 ;;
  esac
done

script_dir="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
: "${repo_root:=$(CDPATH= cd -- "$script_dir/.." && pwd)}"
: "${output_dir:=$repo_root/src-tauri/binaries}"

printf 'llama.cpp build scaffold\n'
printf 'Repo root: %s\n' "$repo_root"
printf 'Source dir: %s\n' "${source_dir:-<manual checkout path>}"
printf 'Output dir: %s\n' "$output_dir"

if ((check_only)); then
  for name in \
    llama-server-aarch64-apple-darwin \
    llama-server-x86_64-apple-darwin \
    llama-server-x86_64-pc-windows-msvc.exe; do
    if [[ -e "$output_dir/$name" ]]; then
      printf 'present: %s\n' "$name"
    else
      printf 'missing: %s\n' "$name"
    fi
  done
  exit 0
fi

printf 'No build is performed here.\n'
printf 'Manually build llama.cpp for the target platform, then place the resulting llama-server binary into src-tauri/binaries/.\n'
