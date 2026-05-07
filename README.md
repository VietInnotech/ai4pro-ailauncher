# AI4Pro Local AI Launcher

Desktop launcher for local AI inference services built with:

- Svelte 5
- TypeScript
- Tailwind CSS
- Tauri 2
- Rust
- SQLite

The product goal is intentionally narrow:

- **Simple Mode** gives normal users one aggregate service called **Local AI**
- **Developer Mode** reveals engines, paths, ports, logs, diagnostics, and runtime details
- model files are **manually placed by developers/operators**
- the app does **not** download models, expose model selection to normal users, or act as a marketplace

## Current state

This repository is now a **working scaffold** rather than an empty plan.

What exists today:

- Svelte/Tauri project structure
- Simple Mode UI
- hidden Developer Mode activation
- frontend API wrappers and stores
- Rust backend command surface
- SQLite initialization and default records
- engine adapter pattern for `llama.cpp` and `sherpa-onnx`
- process supervisor/log scaffolding
- developer-facing setup/build helper scripts
- local bundle input layout and validation for runtime artifacts
- startup sync that copies bundled runtimes into the app data root
- Tauri macOS packaging
- Windows executable cross-build validation through `cargo-xwin`
- GitHub release artifact publishing for scaffold builds

What does **not** exist yet:

- committed engine artifacts/runtimes; release builders must provide them under `src-tauri/bundle/`
- production-complete health checks
- production-complete machine initialization
- complete runtime reconciliation / crash recovery / graceful shutdown behavior
- finished signing/notarization/installer flow

See [docs/IMPLEMENTATION_STATUS.md](docs/IMPLEMENTATION_STATUS.md), [docs/FINISHING_IMPLEMENTATION.md](docs/FINISHING_IMPLEMENTATION.md), [docs/RELEASE.md](docs/RELEASE.md), and [docs/DEPLOYMENT.md](docs/DEPLOYMENT.md).

## Requirements

Install these before building:

- Bun 1.3.x
- Rust and Cargo
- `just` (recommended for the repo task runner)
- Tauri platform prerequisites for macOS builds
- GitHub CLI (`gh`) if publishing releases
- `cargo-xwin` and the Rust Windows MSVC target if cross-building Windows from macOS
- prepared runtime artifacts under `src-tauri/bundle/` before Tauri packaging

The repo currently declares:

```json
"packageManager": "bun@1.3.13"
```

Useful setup commands:

```bash
bun install --frozen-lockfile
rustup target add x86_64-pc-windows-msvc
cargo install cargo-xwin
```

Required runtime bundle input layout:

```text
src-tauri/bundle/
  binaries/
    llama-server-aarch64-apple-darwin
    libllama*.dylib / libggml*.dylib / libmtmd*.dylib for macOS
  runtime/
    sherpa-onnx-vit/
      python3
      lib/python3.14/site-packages/sherpa_onnx/
      lib/python3.14/site-packages/sherpa_onnx_vit/
      lib/python3.14/models/vad/silero_vad.onnx
      ...packaged Python runtime and dependencies...
```

Model files must not be placed under `src-tauri/bundle/`.
They remain operator-managed. The app needs only:

- one Llama GGUF file
- one Sherpa model directory containing `encoder*.onnx`, `decoder*.onnx`, `joiner*.onnx`, and `tokens.txt` or `config.json`

## Development

Frontend uses **Bun** and Vite.

Install dependencies:

```bash
bun install --frozen-lockfile
```

Run the frontend dev server:

```bash
bun run dev
```

Run the Tauri app in development:

```bash
bun run tauri dev
```

If you use `just`, the repo also exposes the common workflows as recipes:

```bash
just install
just check
just test
just build-frontend
just build-desktop
just build-windows
just release-check
```

Versioning and release-tag helpers:

```bash
just version
just version-check
just set-version 0.1.1
just tag-check 0.1.1
just tag-release 0.1.1
```

`just tag-release <version>` creates the local annotated tag only. Review and push it separately.

### Local real-asset testing

The repo-local test asset convention is:

```text
local-assets/
  models/
    llama/*.gguf
    sherpa/<model-dir>/
  audio/*.wav
```

The currently verified local assets are:

```text
local-assets/models/llama/gemma-4-E4B-it-UD-Q5_K_XL.gguf
local-assets/models/sherpa/gipformer-65M-rnnt
local-assets/audio/government-meeting-20s.wav
```

For Developer Mode configuration, set only these model fields:

- Llama: `modelPath` = absolute path to the GGUF file
- Sherpa: `modelDir` = absolute path to the Sherpa model directory
- Leave `tokensPath` empty; the Sherpa runtime resolves `tokens.txt` from the model directory.

To test engines directly during development, use ports that do not conflict with the app defaults:

```bash
./src-tauri/bundle/binaries/llama-server-aarch64-apple-darwin \
  --host 127.0.0.1 \
  --port 18081 \
  --model "$PWD/local-assets/models/llama/gemma-4-E4B-it-UD-Q5_K_XL.gguf"
```

Then in another terminal:

```bash
curl -fsS http://127.0.0.1:18081/health
curl -fsS http://127.0.0.1:18081/v1/models
```

Test Sherpa directly:

```bash
PYTHONPATH="$PWD/src-tauri/bundle/runtime/sherpa-onnx-vit" \
  ./src-tauri/bundle/runtime/sherpa-onnx-vit/python3 \
  -m sherpa_onnx_vit \
  --host 127.0.0.1 \
  --port 18082 \
  --provider cpu \
  --stt-model-family offline_int8 \
  --model-dir "$PWD/local-assets/models/sherpa/gipformer-65M-rnnt" \
  --postprocess-mode clean \
  --alias default-speech
```

Then in another terminal:

```bash
curl -fsS http://127.0.0.1:18082/health
curl -fsS http://127.0.0.1:18082/v1/models
curl -fsS -X POST http://127.0.0.1:18082/v1/audio/transcriptions \
  -F file=@"$PWD/local-assets/audio/government-meeting-20s.wav" \
  -F model=default-speech
```

## Verification

Use this baseline before pushing or releasing:

```bash
bun run check
bun run build
cd src-tauri
cargo check
cargo test
```

Or, with the repo task runner:

```bash
just release-check
```

Verified during the `v0.1.0` release:

- `bun install --frozen-lockfile`
- `bun run check`
- `bun run build`
- `cargo check`
- `cargo test`
- `bun run tauri build`
- packaged macOS smoke launch with an isolated app data root
- `bun run tauri build --runner cargo-xwin --target x86_64-pc-windows-msvc --no-bundle`

`cargo check`, `cargo test`, and release builds currently pass with dead-code warnings. There are no Rust unit tests yet, so `cargo test` verifies compilation of the test target rather than behavior.

## Packaging

Build a macOS Apple Silicon app bundle and DMG:

```bash
bun run tauri build
```

Expected outputs:

```text
src-tauri/target/release/bundle/macos/Local AI.app
src-tauri/target/release/bundle/dmg/Local AI_0.1.0_aarch64.dmg
```

Cross-build the Windows x64 executable from macOS:

```bash
bun run tauri build --runner cargo-xwin --target x86_64-pc-windows-msvc --no-bundle
```

Expected output:

```text
src-tauri/target/x86_64-pc-windows-msvc/release/ai-launcher-tauri.exe
```

Current packaging caveats:

- macOS artifacts are unsigned and not notarized.
- Windows output is an executable, not an installer.
- Runtime artifacts are bundled only when supplied under `src-tauri/bundle/`; model files are never bundled.
- Windows cross-build proves the Rust/Tauri executable builds for Windows; it does not replace smoke testing on a real Windows machine.

Detailed release instructions live in [docs/RELEASE.md](docs/RELEASE.md).

## Install Preview Builds

Unsigned preview builds can be installed on other machines for install-and-launch testing.

Use [docs/DEPLOYMENT.md](docs/DEPLOYMENT.md) for:

- macOS DMG install commands
- `xattr` quarantine removal for unsigned macOS builds
- Windows executable copy/install commands
- SmartScreen expectations for unsigned Windows builds
- smoke-test steps for local SQLite initialization

Preview deployments include runtimes only when the release builder supplies `src-tauri/bundle/` artifacts. Model files remain external, so install-and-launch success is not proof that Local AI engine startup works end-to-end.

## Smoke testing

Run the packaged macOS app binary with an isolated app data root:

```bash
rm -rf /tmp/ai4pro-ailauncher-smoke
mkdir -p /tmp/ai4pro-ailauncher-smoke
LOCAL_AI_APP_DATA_ROOT=/tmp/ai4pro-ailauncher-smoke \
  "./src-tauri/target/release/bundle/macos/Local AI.app/Contents/MacOS/ai-launcher-tauri"
```

Expected result:

- app process starts and stays running
- no stderr output during basic startup
- bundled runtime files are copied into `/tmp/ai4pro-ailauncher-smoke/binaries` and `/tmp/ai4pro-ailauncher-smoke/runtime` when release artifacts were built with bundled runtimes
- SQLite database is created at `/tmp/ai4pro-ailauncher-smoke/data/local_ai.sqlite`

Stop the process after confirming startup. This smoke test only verifies packaged startup and database initialization; it does not validate real `llama.cpp` or `sherpa-onnx` engine launch behavior.

Use the binary inside `Local AI.app` for packaged-resource testing. The loose `src-tauri/target/release/ai-launcher-tauri` binary can use stale loose resources under `src-tauri/target/release/`.

## Repository layout

```text
.
├── src/                    Svelte frontend
│   ├── lib/
│   │   ├── api/            Tauri invoke wrappers
│   │   ├── components/     Simple and Developer mode UI pieces
│   │   ├── stores/         Svelte state
│   │   └── types/          Frontend DTOs
│   └── routes/             Main screen sections
├── src-tauri/              Tauri/Rust backend
│   ├── src/
│   │   ├── adapters/       Engine adapters
│   │   ├── developer/      Developer mode and diagnostics
│   │   └── *.rs            Commands, DB, paths, supervisor, validation
│   ├── binaries/           Sidecar layout docs
│   └── bundle/             Release-builder supplied runtime artifacts; no models
├── scripts/                Developer-facing scaffolding scripts
├── plan.md                 Product source-of-truth plan
└── docs/                   Implementation docs
```

## Product behavior

### Simple Mode

Default user-facing mode.

It should only communicate safe aggregate states such as:

- `Local AI is ready`
- `Local AI is starting`
- `Local AI is running`
- `Local AI needs attention`

Simple Mode must not expose:

- engine names
- model paths
- binary paths
- ports
- logs
- raw errors
- runtime args
- database paths

### Developer Mode

Developer Mode is intentionally hidden.

Activation in the current scaffold:

- click the app logo **7 times within 5 seconds**

Developer Mode can reveal:

- engine profiles
- resolved model/binary paths
- generated args
- logs
- settings
- diagnostics

## Backend command split

The backend already follows the core separation rule from the plan:

- **Simple commands** return safe aggregate DTOs only
- **Developer commands** require Developer Mode first

Implemented command groups:

### Simple commands

- `get_simple_local_ai_status`
- `start_local_ai`
- `stop_local_ai`
- `restart_local_ai`

### Developer commands

- `enable_developer_mode_for_session`
- `disable_developer_mode_for_session`
- `get_app_settings`
- `update_app_settings`
- `dev_list_engine_profiles`
- `dev_get_engine_profile`
- `dev_update_engine_profile`
- `dev_validate_engine_profile`
- `dev_list_model_packages`
- `dev_validate_model_package`
- `dev_read_engine_log`
- `dev_get_diagnostics_bundle`
- `dev_open_logs_folder`

## Expected local data layout

The scaffold resolves an app data root and creates:

```text
<app-root>/
  config/
  data/
    local_ai.sqlite
  logs/
  models/
  binaries/                 copied bundled runtime binaries
  runtime/                  copied bundled packaged runtimes
```

Current default roots:

- macOS: `~/Library/Application Support/AI4Pro/AILauncher`
- Linux: `~/.local/share/AI4Pro/AILauncher`
- Windows: `%LOCALAPPDATA%\AI4Pro\AILauncher`

Override with:

```bash
LOCAL_AI_APP_DATA_ROOT=/custom/path
```

## Bundled engine artifacts

Production intent is split by engine type:

- native `llama.cpp` sidecars supplied under `src-tauri/bundle/binaries/`
- packaged `sherpa-onnx-vit` Python runtime supplied under `src-tauri/bundle/runtime/sherpa-onnx-vit/`
- model files supplied per machine under the app data root, never under `src-tauri/bundle/`

Expected layouts are documented in:

- `src-tauri/binaries/README.md`
- `src-tauri/binaries/expected-layout.md`

No real binaries or runtimes are committed yet. Release builders provide them under `src-tauri/bundle/`, and packaged apps copy them into the app data root on startup. Model files are still expected under `<app-root>/models/`.

## Scripts

Current scripts are **developer-facing scaffolds**, not full automation.

### Sidecar/layout helpers

- `scripts/prepare-sidecars.sh`
- `scripts/prepare-sidecars.ps1`
- `scripts/build-llama-cpp.sh`
- `scripts/build-llama-cpp.ps1`
- `scripts/build-sherpa-onnx.sh`
- `scripts/build-sherpa-onnx.ps1`

### Machine setup helpers

- `scripts/initialize-machine-config.sh`
- `scripts/initialize-machine-config.ps1`

They currently help with:

- previewing/applying directory layouts
- checking expected llama sidecar names and sherpa runtime layout
- documenting manual build expectations

They do **not** yet fully implement machine provisioning from the plan.

## Important caveats

1. The app is still **MVP scaffold quality**, not release quality.
2. `sherpa-onnx` integration is still partially speculative because the exact referenced Python program/runtime contract in `plan.md` is not fully confirmed.
3. Health checks and process lifecycle handling are not yet production-complete.
4. Simple Mode UX is present, but the real end-to-end experience still depends on adding actual engine artifacts and validating runtime behavior.

## Recommended next read

- [docs/IMPLEMENTATION_STATUS.md](docs/IMPLEMENTATION_STATUS.md)
- [docs/FINISHING_IMPLEMENTATION.md](docs/FINISHING_IMPLEMENTATION.md)
- [docs/RELEASE.md](docs/RELEASE.md)
- [docs/DEPLOYMENT.md](docs/DEPLOYMENT.md)
