# Finishing Implementation

This is the conservative path from the current scaffold to the finished launcher described by `plan.md`.
It is intentionally grounded in what is actually present in this repository today.

## Goal

Ship a boring, reliable desktop launcher where:

- normal users only see **Local AI**
- developers can unlock diagnostics when needed
- engine details never leak through Simple Mode
- machine setup is repeatable
- packaged builds work on supported targets

---

## Verified starting point

The repo already has a working scaffold for:

- Bun + Svelte 5 + TypeScript + Tailwind frontend
- Tauri 2 + Rust host
- SQLite schema and seeded defaults
- hidden Developer Mode activation and backend-gated developer commands
- basic engine adapters, process spawning, and log file routing

These commands currently succeed:

```bash
bun install --frozen-lockfile
bun run check
bun run build
cd src-tauri
cargo check
cargo test
```

Preview packaging commands verified for `v0.1.0`:

```bash
bun run tauri build
bun run tauri build --runner cargo-xwin --target x86_64-pc-windows-msvc --no-bundle
```

`cargo check`, `cargo test`, and release builds still emit dead-code warnings.
There are no Rust unit tests yet.

Tauri packaging now validates local runtime bundle inputs first:

```bash
bun run validate:bundle-artifacts
```

That command fails until required runtime artifacts are supplied under `src-tauri/bundle/`.

---

## Current blockers

These are the real blockers in the current repo state:

1. **Real runtime artifacts still need to be supplied**
   - release builds expect `llama-server` binaries and a packaged `sherpa-onnx-vit` Python runtime under `src-tauri/bundle/`
   - model files remain outside the bundle under the app data root
2. **End-to-end engine launch is not verified on target machines**
3. **Health checks are minimal**
   - `src-tauri/src/health.rs` only does a TCP connect probe
4. **Process supervision is incomplete**
   - stop is currently immediate `kill()`
   - no graceful shutdown timeout escalation
   - no runtime-state reconciliation from SQLite
   - no cleanup hook on app exit
5. **Setup scripts are scaffolds only**
6. **Production distribution is incomplete**
   - macOS builds are unsigned and not notarized
   - Windows cross-build produces an executable only, not an installer
   - release builders must provide runtime artifacts locally before packaging

---

## Recommended order

Do the work in this order:

1. confirm real engine contracts
2. supply real bundled runtime artifacts
3. add real model folders on prepared machines
4. validate end-to-end launch locally
5. finish runtime supervision and health checks
6. finish setup scripts
7. finish developer diagnostics workflows
8. package and test on real machines

---

## Phase 1 — Confirm the real engine contracts

This is the highest-value step.
Do not treat the launcher as finished until the adapter contracts match real upstream behavior.

### 1. Confirm the llama contract

Validate against real `llama-server` binaries for:

- macOS Apple Silicon
- macOS Intel
- Windows x64

Confirm at minimum:

- binary names
- actual `--help` output
- supported CLI flags
- working health endpoints
- startup timing
- shutdown behavior

After verification, update the repo assumptions in:

- `src-tauri/src/adapters/llama_cpp.rs`
- `src-tauri/src/db.rs`
- `src-tauri/binaries/expected-layout.md`
- `scripts/prepare-sidecars.sh`
- `scripts/prepare-sidecars.ps1`

### 2. Confirm the sherpa contract

For this repo, keep **Option A** as the intended target until proven otherwise:

- upstream repo: `https://github.com/VietInnotech/sherpa-onnx-vit.git`
- preferred launch shape: `python -m sherpa_onnx_vit`
- optional upstream wrapper: `sherpa-onnx-vit-server` (console-script wrapper)

Validate against the real upstream project, not a placeholder assumption.
Pin one verified upstream Git commit for implementation and packaging notes.

Use the upstream repo itself as the source of truth:

- `README.md`
- `STARTUP.md`
- `.env.example`
- the actual server entrypoint

Required contract decisions:

- exact Python launch shape
- exact argv contract
- HTTP endpoints actually present
- whether `/v1/audio/streaming` requires explicit streaming mode
- graceful shutdown behavior
- startup readiness timing

Working contract to preserve unless real verification disproves it:

- always pass `--host 127.0.0.1`
- always pass `--port {port}`
- use `serverType: "http"`
- prefer model-directory-first launching
- pin one known-good model family for MVP
- recommended MVP family: `offline_int8`

For the pinned offline transducer family, the model directory should contain at least:

- `encoder*.onnx`
- `decoder*.onnx`
- `joiner*.onnx`
- `tokens.txt` or `config.json`
- optional `bpe.model`

The launcher contract should support these sherpa arguments after verification:

- `--host`
- `--port`
- `--provider`
- `--stt-model-family`
- `--model-dir` or `-m`
- optional `--postprocess-mode`
- optional `--alias`
- optional explicit overrides for encoder/decoder/joiner/tokens/bpe paths

Keep raw encoder/decoder/joiner/tokens fields only as advanced overrides.
The default path should stay model-dir-first.

### Required caveat

Option A is **not** a native sidecar binary contract.
It is a Python/FastAPI server program with runtime dependencies.

Treat that as a release blocker unless one of these is true:

- the product remains developer-managed and the machine setup scripts install and validate the required Python environment
- you define a reproducible packaged Python runtime/artifact strategy for every supported OS
- you later move to a controlled wrapper/fork with a simpler bundled artifact story

Also preserve these explicit constraints:

- do not rely on default host values; always pass `--host 127.0.0.1`
- treat `/v1/audio/streaming` as conditional, not universally available
- do not document a fake upstream GitHub release artifact if none exists
- current `src-tauri/binaries/sherpa-onnx-server-*` assumptions are wrong for Option A as written

### Files that must change after sherpa contract verification

- `src-tauri/src/adapters/sherpa_onnx.rs`
- `src-tauri/src/engine_manager.rs`
- `src-tauri/src/validation.rs`
- `src-tauri/src/db.rs`
- `src-tauri/binaries/expected-layout.md`
- `scripts/prepare-sidecars.sh`
- `scripts/prepare-sidecars.ps1`
- `scripts/build-sherpa-onnx.sh`
- `scripts/build-sherpa-onnx.ps1`

### Definition of done for Phase 1

- [x] one verified llama contract is documented from real binaries
- [x] one verified sherpa upstream contract is documented from a pinned commit
- [x] sherpa assumptions are changed from native `sherpa-onnx-server*` sidecars to the real launch shape
- [x] adapter args are verified against real `--help` output and at least one real launch

### Phase 1 verification completed

- Verified sherpa upstream repo: `https://github.com/VietInnotech/sherpa-onnx-vit`
- Pinned commit: `6a7fe63ded85cd089bff73c12c54e1bda3bd7cf3`
- Verified sherpa launch: `python -m sherpa_onnx_vit --host 127.0.0.1 --port 18080 --provider cpu --postprocess-mode clean --stt-model-family offline_int8 --model-dir <model-dir>`
- Verified sherpa endpoints: `GET /health`, `GET /v1/models`, `POST /v1/audio/transcriptions`
- Verified llama-cli with real GGUF: `gemma-4-E4B-it-UD-Q5_K_XL.gguf`
- Verified llama output: `Four.` for prompt `what is 2+2?`
- Verified sherpa transcription with real audio: `government-meeting-20s.wav` → Vietnamese transcript obtained
- Identified missing VAD dependency: `silero_vad.onnx` now documented as required runtime asset

---

## Phase 2 — Add real artifacts and validate manual launching

### 1. Populate `src-tauri/bundle/`

For Apple Silicon macOS llama, provide the native server binary and its adjacent dylibs:

```text
src-tauri/bundle/binaries/llama-server-aarch64-apple-darwin
src-tauri/bundle/binaries/libllama*.dylib
src-tauri/bundle/binaries/libggml*.dylib
src-tauri/bundle/binaries/libmtmd*.dylib
```

For sherpa Option A on macOS, provide a packaged Python runtime:

```text
src-tauri/bundle/runtime/sherpa-onnx-vit/python3
src-tauri/bundle/runtime/sherpa-onnx-vit/lib/python3.14/site-packages/sherpa_onnx_vit/
src-tauri/bundle/runtime/sherpa-onnx-vit/lib/python3.14/site-packages/sherpa_onnx/
src-tauri/bundle/runtime/sherpa-onnx-vit/lib/python3.14/models/vad/silero_vad.onnx
src-tauri/bundle/runtime/sherpa-onnx-vit/<other Python runtime files>
```

Do not place model files under `src-tauri/bundle/`.

### 2. Prepare real model folders

Prepare model inputs under the resolved app data root, or configure absolute paths in Developer Mode, and verify:

- one llama GGUF path exists
- one sherpa model directory exists
- sherpa model directory matches the pinned family contract
- the required sherpa files are actually present

### 3. Validate launch flows through the app

Confirm all of these with real binaries and real models:

- start works
- stop works
- restart works
- logs are written
- missing runtime/model conditions become safe Simple Mode messages

### Definition of done for Phase 2

- both engines can be launched manually through the app on a prepared machine
- both engines can stop cleanly enough for local testing
- missing runtime/model conditions are detected and surfaced safely

---

## Phase 3 — Finish backend runtime behavior

This phase is mainly in:

- `src-tauri/src/process_supervisor.rs`
- `src-tauri/src/process_registry.rs`
- `src-tauri/src/engine_manager.rs`
- `src-tauri/src/health.rs`
- `src-tauri/src/main.rs`

### 1. Process supervision

Add:

- graceful shutdown first
- timeout-based force kill
- runtime-state persistence updates
- crash detection persistence
- cleanup on Tauri exit

### 2. Health checks

#### Llama

Probe supported endpoints such as:

- `/health`
- `/v1/models`
- `/props`
- `/`

#### Sherpa

Sherpa Option A is HTTP-first.
Health should proceed in this order:

- process alive
- `GET /health`
- `GET /v1/models`
- optional WebSocket probe for `/v1/audio/streaming` only when streaming mode is enabled
- startup grace period before declaring failure

If a deeper readiness flow still does not exist, use `GET /health` plus a bounded grace period first.

### 3. Runtime-state reconciliation

Use `engine_runtime_state` for real recovery behavior.
On app startup:

- load persisted runtime state
- clear stale records
- confirm whether old PIDs still exist
- restore safe aggregate UI state

### Definition of done for Phase 3

- runtime state survives restart safely
- health checks drive status transitions correctly
- crashes move Local AI into `needs_attention`
- app exit does not leave orphaned children when stop-on-exit is enabled

---

## Phase 4 — Finish machine setup scripts

Current scripts are scaffolds only.
They must become real provisioning tools.

Primary files:

- `scripts/initialize-machine-config.sh`
- `scripts/initialize-machine-config.ps1`

Supporting files that may also need updates:

- `scripts/prepare-sidecars.sh`
- `scripts/prepare-sidecars.ps1`
- `scripts/build-llama-cpp.sh`
- `scripts/build-llama-cpp.ps1`
- `scripts/build-sherpa-onnx.sh`
- `scripts/build-sherpa-onnx.ps1`

### Required script behavior

Implement all of these in both shell and PowerShell versions:

- create the app data root
- create `models/`, `logs/`, `config/`, and `data/`
- validate llama model presence
- validate the pinned sherpa model directory and required files
- create or update the SQLite database
- insert default app settings
- insert default model package records
- insert default engine profiles
- validate configured ports
- validate llama artifact availability
- validate sherpa runtime availability for the chosen deployment mode
- print a developer-facing readiness report

### Recommended output

```text
AI Engine Launcher Machine Setup

App data root: ...
Language model: OK
Speech model: OK
llama binary: Bundled
sherpa runtime: Developer-managed Python
Ports: OK
SQLite: Initialized

Result: Machine is ready.
```

### Definition of done for Phase 4

- a developer can prepare a machine without editing SQLite by hand
- setup failures identify exactly what is missing

---

## Phase 5 — Finish Developer Mode workflows

Current UI is mixed:

- Simple Mode controls work
- Developer settings save flow works
- most other developer pages are still display-first scaffolds

Primary frontend files:

- `src/App.svelte`
- `src/routes/DeveloperDashboard.svelte`
- `src/routes/DeveloperEngines.svelte`
- `src/routes/DeveloperModels.svelte`
- `src/routes/DeveloperLogs.svelte`
- `src/routes/DeveloperDiagnostics.svelte`
- `src/lib/components/DeveloperEngineTable.svelte`
- `src/lib/components/DeveloperEngineDetail.svelte`
- `src/lib/components/DeveloperModelTable.svelte`
- `src/lib/components/DeveloperLogViewer.svelte`
- `src/lib/components/DeveloperDiagnostics.svelte`

### Engines view

Complete:

- per-engine selection
- validate action
- edit/save flow
- start/stop/restart actions
- generated CLI display from the real adapter output

### Models view

Complete:

- model validation action
- resolved-path display
- optional checksum support
- open-containing-folder action

### Logs view

Complete:

- engine selector
- stdout/stderr switching
- launcher log access
- copy/export actions

### Diagnostics view

Complete:

- export diagnostics bundle action
- sanitized config
- runtime state
- recent logs
- validation results

### Definition of done for Phase 5

- developers can diagnose a broken prepared machine entirely from the app

---

## Phase 6 — Tighten security and information hiding

Before release, explicitly re-audit the Simple Mode / Developer Mode boundary.

Check these rules:

- Simple Mode commands never return paths
- Simple Mode commands never return ports
- Simple Mode commands never return engine names
- Simple Mode commands never return raw errors
- Developer Mode checks are enforced in backend commands
- no shell-string process construction is introduced anywhere
- sherpa Option A network exposure is explicitly mitigated or treated as a release blocker

Also verify:

- logs are only reachable through developer flows
- model paths only appear in developer DTOs
- normal-user settings remain safe
- Simple Mode hiding is not being mistaken for real network protection

### Definition of done for Phase 6

- sensitive fields are impossible to fetch through Simple Mode APIs

---

## Phase 7 — Packaging and deployment

Primary file:

- `src-tauri/tauri.conf.json`

### 1. Maintain current preview packaging

Already working:

- Tauri bundling is enabled
- macOS Apple Silicon `.app` and `.dmg` build
- Tauri icon assets exist
- Windows x64 executable cross-build works through `cargo-xwin`
- Tauri resources include locally supplied runtime artifacts
- app startup copies bundled `binaries/` and `runtime/` resources into the app data root

Still finish:

- version bump process across `package.json`, `Cargo.toml`, and `tauri.conf.json`
- installer output for Windows
- real release notes and changelog discipline

For sherpa Option A, release bundling means an internal packaged Python runtime per supported OS. Models are still excluded from the bundle.

### 2. Package for targets

Planned targets:

- Windows x64
- macOS Apple Silicon
- macOS Intel

### 3. Finish signing/distribution

#### Windows

- installer strategy
- optional MSI
- code signing if available
- smoke test on a real Windows machine

#### macOS

- app signing
- sidecar signing
- notarization
- stapling

### Definition of done for Phase 7

- packaged app launches on a prepared machine
- packaged app finds llama sidecars and sherpa runtime artifacts correctly

---

## Short checklist

Use this as the conservative release checklist.

### Contracts

- [ ] real llama binary contract verified
- [ ] sherpa upstream commit pinned
- [ ] sherpa launch shape chosen: developer-managed Python or packaged runtime
- [ ] llama adapter updated to match real CLI
- [ ] sherpa adapter updated to match real launch shape

### Models

- [ ] llama GGUF validation works against a real model file
- [ ] sherpa model-dir validation works for the pinned family

### Runtime

- [ ] start works end-to-end
- [ ] stop works end-to-end
- [ ] restart works end-to-end
- [ ] llama HTTP health checks work
- [ ] sherpa HTTP health checks work
- [ ] sherpa streaming probe works when enabled
- [ ] crash detection works
- [ ] stop-on-exit works
- [ ] auto-start works

### Security

- [ ] sherpa always launches with `--host 127.0.0.1`
- [ ] simple APIs cannot fetch developer-sensitive details

### Developer mode

- [ ] engines page complete
- [ ] models page complete
- [ ] logs page complete
- [ ] diagnostics export complete

### Setup and packaging

- [ ] machine init scripts are real
- [x] Tauri bundling enabled
- [x] bundled runtime resource sync implemented
- [x] build validation rejects missing runtime artifacts and bundled model files
- [x] preview packaging works on macOS Apple Silicon
- [x] Windows executable cross-build works through cargo-xwin
- [ ] Windows installer packaging works
- [ ] production signing/notarization works

---

## If continuing from the current repo

The next concrete steps should be:

1. pin one real `VietInnotech/sherpa-onnx-vit` commit and keep the launcher on `python -m sherpa_onnx_vit` unless a wrapper is intentionally introduced
2. replace the current native `sherpa-onnx-server*` assumptions in code, defaults, and scripts
3. place real `llama-server` binaries and a packaged sherpa runtime under `src-tauri/bundle/`
4. update health checks from TCP-only probes to real HTTP readiness checks
5. run the app and fix launch/health issues until Simple Mode can reliably show:

```text
Local AI is running
```

That is the point where this repo stops being only a compilable scaffold and becomes a working local launcher.
