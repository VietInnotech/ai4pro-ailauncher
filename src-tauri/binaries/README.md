# Sidecar runtime notes

This directory documents native sidecar expectations.
Release-builder supplied runtime artifacts now belong under `src-tauri/bundle/`, not here.

Required release-builder input files:

- `src-tauri/bundle/binaries/llama-server-aarch64-apple-darwin`
- `src-tauri/bundle/binaries/llama-server-x86_64-pc-windows-msvc.exe`
- `src-tauri/bundle/runtime/sherpa-onnx-vit/python3`
- `src-tauri/bundle/runtime/sherpa-onnx-vit/python.exe`

`sherpa-onnx-vit` is **not** a native sidecar in this repo.
Treat it as a Python program packaged under `src-tauri/bundle/runtime/sherpa-onnx-vit/`.

Use the scripts in `scripts/` as manual-build scaffolding only.
