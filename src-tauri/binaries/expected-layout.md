# Expected runtime layout

Release builders place runtime artifacts under `src-tauri/bundle/`.
Tauri packages those artifacts as resources, and the app copies them into the app-managed runtime and binary roots on startup.

## Llama.cpp

Native llama runtime inputs:

- `src-tauri/bundle/binaries/llama-server-aarch64-apple-darwin`
- adjacent macOS dylibs required by that binary, including `libllama*.dylib`, `libggml*.dylib`, and `libmtmd*.dylib`

Copied runtime outputs:

- `<app-root>/binaries/llama-server-aarch64-apple-darwin`
- `<app-root>/binaries/*.dylib`

Intel macOS is not supported. Windows runtime artifacts are separate release-builder inputs when building Windows packages.

## Sherpa-ONNX-VIT

Sherpa Option A is **not** a native sidecar in this repo.

Expected packaged runtime shape:

- `src-tauri/bundle/runtime/sherpa-onnx-vit/python3` on macOS
- packaged Python site-packages containing `sherpa_onnx_vit`
- packaged Python site-packages containing `sherpa_onnx`
- `src-tauri/bundle/runtime/sherpa-onnx-vit/lib/python3.14/models/vad/silero_vad.onnx`

Copied runtime output:

- `<app-root>/runtime/sherpa-onnx-vit/`

Model files remain external and operator-managed:

- one Llama GGUF file, by convention `<app-root>/models/llama/default/model.gguf`
- one Sherpa model directory, by convention `<app-root>/models/sherpa/default/`

Developer Mode may point to absolute model paths during machine setup. `tokensPath` should normally remain empty because Sherpa resolves `tokens.txt` from the configured model directory.

Upstream contract pinned from:

- repo: `https://github.com/VietInnotech/sherpa-onnx-vit`
- branch: `main`
- commit: `6a7fe63ded85cd089bff73c12c54e1bda3bd7cf3`

Preferred launch shape in this repo:

- `python -m sherpa_onnx_vit`

Upstream may also expose:

- `sherpa-onnx-vit-server` (console-script wrapper around the same Python program)

`scripts/validate-bundle-artifacts.mjs` rejects product model files under `src-tauri/bundle/`. The packaged `silero_vad.onnx` path is allowed because it is a Sherpa runtime dependency, not an operator-selected speech model.
