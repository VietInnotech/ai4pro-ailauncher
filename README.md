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

What does **not** exist yet:

- real bundled engine binaries
- production-complete health checks
- production-complete machine initialization
- complete runtime reconciliation / crash recovery / graceful shutdown behavior
- finished packaging/signing flow

See [docs/IMPLEMENTATION_STATUS.md](docs/IMPLEMENTATION_STATUS.md) and [docs/FINISHING_IMPLEMENTATION.md](docs/FINISHING_IMPLEMENTATION.md).

## Verified commands

Frontend uses **bun**.

```bash
bun install
bun run dev
bun run check
bun run build
```

Backend:

```bash
cd src-tauri
cargo check
```

Verified in this repo:

- `bun run check` ✅
- `bun run build` ✅
- `cargo check` ✅

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
│   └── binaries/           Planned sidecar location (currently placeholders only)
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
  binaries/
```

Current default roots:

- macOS: `~/Library/Application Support/AI4Pro/AILauncher`
- Linux: `~/.local/share/AI4Pro/AILauncher`
- Windows: `%LOCALAPPDATA%\AI4Pro\AILauncher`

Override with:

```bash
LOCAL_AI_APP_DATA_ROOT=/custom/path
```

## Sidecar binaries

Production intent is to bundle sidecars under:

```text
src-tauri/binaries/
```

Expected filenames are documented in:

- `src-tauri/binaries/README.md`
- `src-tauri/binaries/expected-layout.md`

No real binaries are committed yet.

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
- checking expected sidecar names
- documenting manual build expectations

They do **not** yet fully implement machine provisioning from the plan.

## Important caveats

1. The app is still **MVP scaffold quality**, not release quality.
2. `sherpa-onnx` integration is still partially speculative because the exact referenced server repo/binary contract in `plan.md` is not fully confirmed.
3. Health checks and process lifecycle handling are not yet production-complete.
4. Simple Mode UX is present, but the real end-to-end experience still depends on adding actual engine binaries and validating runtime behavior.

## Recommended next read

- [docs/IMPLEMENTATION_STATUS.md](docs/IMPLEMENTATION_STATUS.md)
- [docs/FINISHING_IMPLEMENTATION.md](docs/FINISHING_IMPLEMENTATION.md)
