# Expected sidecar layout

The launcher plans for release artifacts to live under the app-managed runtime and binary roots.
This directory only documents the native sidecar portion of that layout.

## Llama.cpp

Native llama sidecars still belong in this directory:

- `llama-server-aarch64-apple-darwin`
- `llama-server-x86_64-apple-darwin`
- `llama-server-x86_64-pc-windows-msvc.exe`

## Sherpa-ONNX-VIT

Sherpa Option A is **not** a native sidecar in this repo.

Expected packaged runtime shape:

- `runtime/sherpa-onnx-vit/python3` on macOS/Linux
- `runtime/sherpa-onnx-vit/python.exe` on Windows
- packaged Python site-packages containing `sherpa_onnx_vit`
- model files under `models/sherpa/default/`

Upstream contract pinned from:

- repo: `https://github.com/VietInnotech/sherpa-onnx-vit`
- branch: `main`
- commit: `6a7fe63ded85cd089bff73c12c54e1bda3bd7cf3`

Preferred launch shape in this repo:

- `python -m sherpa_onnx_vit`

Upstream may also expose:

- `sherpa-onnx-vit-server` (console-script wrapper around the same Python program)

These files are placeholders for layout documentation only; no binaries or runtimes are included.
