# Implementation Status

This document describes what is verified in the repository today.
It is intentionally conservative: if something is not verified from repo files or runnable commands, it is listed as unfinished.

## Summary

The repo is no longer greenfield.

It now contains a working prototype scaffold for:

- Svelte frontend
- Tauri 2 desktop host
- Rust backend
- SQLite persistence
- hidden Developer Mode with backend-gated developer commands
- adapter-based engine launching

It is **not** yet a finished production launcher.

## Verified now

These commands currently succeed in this repository:

```bash
bun install --frozen-lockfile
bun run check
bun run build
cd src-tauri
cargo check
cargo test
```

Additional release commands verified for `v0.1.0`:

```bash
bun run tauri build
bun run tauri build --runner cargo-xwin --target x86_64-pc-windows-msvc --no-bundle
```

`cargo check`, `cargo test`, and release builds still emit dead-code warnings.
There are no Rust unit tests yet.

Current Tauri packaging now also runs:

```bash
bun run validate:bundle-artifacts
```

That validator is expected to fail until release builders provide runtime artifacts under `src-tauri/bundle/`.
For Apple Silicon macOS it now checks the actual runtime dependency set: `llama-server`, adjacent `llama.cpp`/`ggml` dylibs, packaged `sherpa_onnx_vit`, packaged `sherpa_onnx`, and the Sherpa runtime VAD asset.

## Implemented

### Frontend

- project initialized with Svelte 5 + TypeScript + Tailwind + Vite
- Simple Mode screen with aggregate status card
- Start/Stop/Restart wiring in Simple Mode
- hidden Developer Mode activation via 7 logo clicks within 5 seconds
- frontend stores for simple status, developer mode, and app settings
- Tauri command wrappers for simple and developer APIs
- developer view navigation and data loading in `src/App.svelte`
- developer settings save flow for stop-on-exit and auto-start toggles

### Backend

- Tauri bootstrap and command registration
- app path resolution and local directory creation
- SQLite initialization and migrations
- default app settings + default model packages + default engine profiles
- safe Simple Mode DTOs
- richer developer DTOs
- backend developer-mode gating via `require_enabled()`
- engine adapter pattern with sherpa packaged-Python runtime contract
- process spawn/stop scaffolding with log redirection
- diagnostics bundle DTO generation
- validation layer for engine profiles and model package presence
- developer log tailing and open-logs-folder command
- HTTP health checks for sherpa (`/health`, `/v1/models`)
- runtime persistence and reconciliation from `engine_runtime_state`
- stop-on-exit cleanup hook implemented
- developer engine start/stop/restart commands working

### Scripts and repo support

- Bun workflow with `bun.lock`
- Cargo workflow with `Cargo.lock`
- repo `justfile` for install/check/build/version/tag helper workflows
- `src-tauri/binaries/` expected layout documentation (updated for sherpa Python runtime)
- machine setup scaffolding scripts (`initialize-machine-config.sh`, `initialize-machine-config.ps1`)
- sidecar preparation/build scaffolding scripts (`prepare-sidecars.sh`, `prepare-sidecars.ps1`)
- sherpa-onnx build scripts (`build-sherpa-onnx.sh`, `build-sherpa-onnx.ps1`)
- `tauri.conf.json` with bundling enabled
- usable Tauri icon assets under `src-tauri/icons/`
- generated Windows capability schema under `src-tauri/gen/schemas/windows-schema.json`
- `src-tauri/bundle/` input layout for release-builder supplied runtime artifacts
- bundle artifact validator that rejects missing runtime dependencies and product model files
- startup sync that copies bundled `binaries/` and `runtime/` resources into the app data root

## Partially implemented

### Engine integration

`llama.cpp` and `sherpa-onnx` are represented in the codebase and have been validated on Apple Silicon macOS against local real assets under `local-assets/`.

Current state:

- adapters exist
- default engine profiles exist
- argument generation exists
- validation exists
- runtime artifacts are expected under `src-tauri/bundle/` for release builds and copied to the app data root on startup
- real runtime artifacts are not committed
- packaged Apple Silicon app auto-start was verified with:
  - one Llama GGUF file
  - one Sherpa model directory
  - one WAV transcription request

Still missing:

- repeated lifecycle/stress testing
- Windows real-engine validation
- production readiness around crash recovery and health transitions

### Sherpa runtime packaging

The repo now treats `sherpa-onnx-vit` as a Python program, not a native sherpa binary.

Current state:

- adapter defaults launch `python -m sherpa_onnx_vit`
- default engine profile seed data points at a Python runtime
- release packaging expects a prepared Python runtime under `src-tauri/bundle/runtime/sherpa-onnx-vit/`
- packaged app startup sync copies that runtime into `<app-root>/runtime/sherpa-onnx-vit/`
- local release-builder workspace has been tested with a packaged Python runtime containing `sherpa_onnx_vit`, `sherpa_onnx`, and `silero_vad.onnx`
- end-to-end transcription was verified against `local-assets/audio/government-meeting-20s.wav`

Still missing:

- committed runtime artifacts; release builders still supply them locally
- Windows packaged runtime validation

### Process lifecycle

Current process supervision supports:

- spawn
- immediate stop via `kill()`
- PID tracking
- stdout/stderr log file routing

Still missing or incomplete:

- graceful shutdown with timeout escalation
- persistent runtime state updates and reconciliation from `engine_runtime_state`
- crash recovery strategy
- child cleanup on Tauri exit hook
- actual stop-on-exit execution
- actual auto-start execution

### Health checks

Current health behavior is minimal.

Current state:

- `src-tauri/src/health.rs` only does a TCP connect probe
- real engine tests additionally exercised HTTP `/health` and `/v1/models` manually

Still needed:

- real llama HTTP health probing
- sherpa-specific HTTP health strategy
- startup grace periods
- unhealthy transition rules
- runtime use of those checks

### Developer tools

Developer UI exists, but it is still mixed between functional pieces and display-first scaffolds.

Currently interactive:

- Simple Mode controls
- Developer Mode unlock flow
- developer view switching
- developer settings save flow

Still scaffold-first or incomplete:

- Developer dashboard
- Engines page actions and selection flow
- model validation actions
- richer log browsing and selection
- diagnostics export button wiring

### Setup scripts

The current scripts are conservative scaffolds.
They mostly create or preview directory layout and print guidance.

They do not yet:

- validate all required model files for the pinned contracts
- initialize or update SQLite content comprehensively
- validate ports and runtime availability
- print a full machine readiness report from real inspection

### Packaging and release

Tauri packaging works for scaffold preview builds, but production distribution is not finished.

Current state:

- macOS Apple Silicon app bundle builds
- macOS Apple Silicon DMG builds
- packaged macOS binary smoke launch works with an isolated app data root
- Windows x64 executable cross-build works through `cargo-xwin`
- release artifacts can be published through GitHub CLI
- Tauri resources include locally supplied runtime artifacts from `src-tauri/bundle/`

Still missing:

- macOS signing, notarization, and stapling
- Windows installer generation
- Windows code signing
- smoke testing on a real Windows machine
- real runtime artifacts in the release-builder workspace

## Current limitations

The following should be treated as unresolved before release:

- no committed engine runtime artifacts
- sherpa runtime artifact packaging still needs a real supplied runtime
- no proven machine setup flow
- no signed/notarized production packaging process
- no release-grade health supervision
- no runtime reconciliation or cleanup-on-exit flow
- warnings remain in `cargo check`

## Release readiness

Current readiness: **prototype scaffold**

Suitable for:

- iterating on app structure
- verifying frontend/backend wiring
- continuing implementation against the plan

Not yet suitable for:

- end-user deployment
- packaged distribution
- unattended machine provisioning
- support/debug workflow handoff on prepared machines

Next step: use [FINISHING_IMPLEMENTATION.md](FINISHING_IMPLEMENTATION.md).
