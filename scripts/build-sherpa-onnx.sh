#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'EOF'
build-sherpa-onnx.sh — packaged runtime scaffold for sherpa-onnx-vit

Usage:
  scripts/build-sherpa-onnx.sh [--repo-root PATH] [--source-dir PATH] [--runtime-dir PATH] [--check-only]
  scripts/build-sherpa-onnx.sh --repair-macos-runtime [--python-framework PATH] [--runtime-dir PATH]

This script is intentionally non-building scaffolding. It documents the packaged Python runtime contract and can only check for already-produced runtime artifacts.
EOF
}

repo_root=""
source_dir=""
runtime_dir=""
check_only=0
repair_macos_runtime=0
python_framework=""

while (($#)); do
  case "$1" in
    --repo-root) repo_root="${2:-}"; shift 2 ;;
    --source-dir) source_dir="${2:-}"; shift 2 ;;
    --runtime-dir) runtime_dir="${2:-}"; shift 2 ;;
    --check-only) check_only=1; shift ;;
    --repair-macos-runtime) repair_macos_runtime=1; shift ;;
    --python-framework) python_framework="${2:-}"; shift 2 ;;
    -h|--help) usage; exit 0 ;;
    *) printf 'Unknown argument: %s\n' "$1" >&2; usage >&2; exit 2 ;;
  esac
done

script_dir="$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)"
: "${repo_root:=$(CDPATH= cd -- "$script_dir/.." && pwd)}"
: "${runtime_dir:=$repo_root/src-tauri/bundle/runtime/sherpa-onnx-vit}"

printf 'sherpa-onnx-vit packaged runtime scaffold\n'
printf 'Repo root: %s\n' "$repo_root"
printf 'Source dir: %s\n' "${source_dir:-<gh repo clone path for VietInnotech/sherpa-onnx-vit>}"
printf 'Runtime dir: %s\n' "$runtime_dir"
printf 'Pinned upstream commit: %s\n' '2ce802dc045dbb306d38085423de5327d45f1d26'

if ((repair_macos_runtime)); then
  if [[ "$(uname -s)" != "Darwin" ]]; then
    printf 'macOS runtime repair requires Darwin.\n' >&2
    exit 1
  fi

  if ! command -v install_name_tool >/dev/null 2>&1; then
    printf 'missing required tool: install_name_tool\n' >&2
    exit 1
  fi

  : "${python_framework:=/opt/homebrew/opt/python@3.14/Frameworks/Python.framework}"

  framework_python="$python_framework/Versions/3.14/Python"
  framework_bin="$python_framework/Versions/3.14/bin/python3.14"
  if [[ ! -f "$framework_python" || ! -f "$framework_bin" ]]; then
    printf 'invalid Python.framework path: %s\n' "$python_framework" >&2
    printf 'expected both Versions/3.14/Python and Versions/3.14/bin/python3.14\n' >&2
    exit 1
  fi

  mkdir -p "$runtime_dir/Frameworks" "$runtime_dir/bin"
  rm -rf "$runtime_dir/Frameworks/Python.framework"
  cp -R "$python_framework" "$runtime_dir/Frameworks/Python.framework"

  rm -f "$runtime_dir/python3" "$runtime_dir/bin/python" "$runtime_dir/bin/python3" "$runtime_dir/bin/python3.14"
  cp "$framework_bin" "$runtime_dir/python3"
  cp "$framework_bin" "$runtime_dir/bin/python3.14"
  ln -s python3.14 "$runtime_dir/bin/python"
  ln -s python3.14 "$runtime_dir/bin/python3"
  chmod 755 "$runtime_dir/python3" "$runtime_dir/bin/python3.14"

  old_install_name="$(otool -L "$framework_bin" | awk 'NR == 2 { print $1 }')"
  install_name_tool -id '@rpath/Python.framework/Versions/3.14/Python' "$runtime_dir/Frameworks/Python.framework/Versions/3.14/Python"
  install_name_tool -change "$old_install_name" '@executable_path/Frameworks/Python.framework/Versions/3.14/Python' "$runtime_dir/python3"
  install_name_tool -change "$old_install_name" '@executable_path/../Frameworks/Python.framework/Versions/3.14/Python' "$runtime_dir/bin/python3.14"
  install_name_tool -change "$old_install_name" '@executable_path/../Python' "$runtime_dir/Frameworks/Python.framework/Versions/3.14/bin/python3.14"
  install_name_tool -change "$old_install_name" '@executable_path/../../../../Python' "$runtime_dir/Frameworks/Python.framework/Versions/3.14/Resources/Python.app/Contents/MacOS/Python"

  dynload_dir="$runtime_dir/Frameworks/Python.framework/Versions/3.14/lib/python3.14/lib-dynload"
  for extension in "$dynload_dir"/*.so; do
    while IFS= read -r linked; do
      [[ -n "$linked" ]] || continue
      linked_real="$(realpath "$linked")"
      linked_name="$(basename "$linked_real")"
      cp "$linked_real" "$dynload_dir/$linked_name"
      chmod 755 "$dynload_dir/$linked_name"
      install_name_tool -change "$linked" "@loader_path/$linked_name" "$extension"
    done < <(otool -L "$extension" | awk 'NR > 1 { print $1 }' | grep -E '^/(opt/homebrew|usr/local)/' || true)
  done

  for dylib in "$dynload_dir"/*.dylib; do
    [[ -f "$dylib" ]] || continue
    install_name_tool -id "@rpath/$(basename "$dylib")" "$dylib"
    while IFS= read -r linked; do
      [[ -n "$linked" ]] || continue
      linked_real="$(realpath "$linked")"
      linked_name="$(basename "$linked_real")"
      if [[ ! -f "$dynload_dir/$linked_name" ]]; then
        cp "$linked_real" "$dynload_dir/$linked_name"
        chmod 755 "$dynload_dir/$linked_name"
      fi
      install_name_tool -change "$linked" "@loader_path/$linked_name" "$dylib"
    done < <(otool -L "$dylib" | awk 'NR > 1 { print $1 }' | grep -E '^/(opt/homebrew|usr/local)/' || true)
  done

  if command -v codesign >/dev/null 2>&1; then
    codesign --force --sign - \
      "$runtime_dir/Frameworks/Python.framework/Versions/3.14/Python" \
      "$runtime_dir/Frameworks/Python.framework/Versions/3.14/bin/python3.14" \
      "$runtime_dir/Frameworks/Python.framework/Versions/3.14/Resources/Python.app/Contents/MacOS/Python" \
      "$runtime_dir/python3" \
      "$runtime_dir/bin/python3.14" \
      "$dynload_dir"/*.so \
      "$dynload_dir"/*.dylib
  fi

  python_home="$runtime_dir/Frameworks/Python.framework/Versions/3.14"
  PYTHONHOME="$python_home" "$runtime_dir/python3" -c 'import fastapi, uvicorn, sherpa_onnx, sherpa_onnx_vit'
  printf 'macOS sherpa runtime repaired and smoke-tested.\n'
  exit 0
fi

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
printf 'Package a Python runtime that can launch `python -m sherpa_onnx_vit` and place it under src-tauri/bundle/runtime/sherpa-onnx-vit/.\n'
printf 'The runtime must include FastAPI, uvicorn, and the sherpa_onnx_vit package from VietInnotech/sherpa-onnx-vit at commit 2ce802dc045dbb306d38085423de5327d45f1d26.\n'
printf 'On macOS, run this script with --repair-macos-runtime after creating the venv so the runtime vendors Python.framework and does not depend on Homebrew.\n'
