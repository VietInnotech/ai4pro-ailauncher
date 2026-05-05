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
bun run check
bun run build
cd src-tauri && cargo check
```

`cargo check` still emits warnings.

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
- `src-tauri/binaries/` expected layout documentation (updated for sherpa Python runtime)
- machine setup scaffolding scripts (`initialize-machine-config.sh`, `initialize-machine-config.ps1`)
- sidecar preparation/build scaffolding scripts (`prepare-sidecars.sh`, `prepare-sidecars.ps1`)
- sherpa-onnx build scripts (`build-sherpa-onnx.sh`, `build-sherpa-onnx.ps1`)
- `tauri.conf.json` with bundling enabled

## Partially implemented

### Engine integration

`llama.cpp` and `sherpa-onnx` are represented in the codebase, but the launcher is not yet validated against real llama binaries, real sherpa Python runtimes, and real model folders end-to-end.

Current state:

- adapters exist
- default engine profiles exist
- argument generation exists
- validation exists
- real engine artifacts are not present in `src-tauri/binaries/`
- start/stop/restart are not verified against real engines

### Sherpa runtime packaging still incomplete

The repo now treats `sherpa-onnx-vit` as a Python program, not a native sherpa binary.

Current state:

- adapter defaults launch `python -m sherpa_onnx_vit`
- default engine profile seed data points at a Python runtime
- layout/scripts document a packaged sherpa runtime instead of native sherpa sidecars

Still missing:

- a real packaged Python runtime under `src-tauri/runtime/sherpa-onnx-vit/`
- a validated developer-managed Python setup path
- end-to-end launch validation against a real sherpa runtime and model directory

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

### Packaging

Tauri packaging is not yet finished.

Current state:

- `src-tauri/tauri.conf.json` exists
- bundling is disabled
- icons are empty
- sidecar inclusion/signing/notarization are incomplete

## Current limitations

The following should be treated as unresolved before release:

- no real bundled engine binaries
- sherpa packaging/launch contract is not resolved
- no proven machine setup flow
- no real packaging/distribution process
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
