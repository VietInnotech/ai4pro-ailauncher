# sherpa-onnx-vit Update Assessment and Plan

Date assessed: 2026-05-08

## Summary

`VietInnotech/sherpa-onnx-vit` has a new upstream `main` commit:

- Current repo pin: `6a7fe63ded85cd089bff73c12c54e1bda3bd7cf3`
- Upstream `main`: `2ce802dc045dbb306d38085423de5327d45f1d26`
- Upstream repo: <https://github.com/VietInnotech/sherpa-onnx-vit>

This is not a simple dependency bump. The upstream launcher contract changed from
single-model CLI arguments to a required JSON model registry. This repo should
update the Sherpa adapter, default engine profile, validation, setup scripts,
packaging docs, and release verification before changing the pin.

## Evidence Checked

Local repo files checked:

- `src-tauri/src/adapters/sherpa_onnx.rs`
- `src-tauri/src/db.rs`
- `src-tauri/src/validation.rs`
- `scripts/build-sherpa-onnx.sh`
- `scripts/build-sherpa-onnx.ps1`
- `scripts/initialize-machine-config.sh`
- `scripts/initialize-machine-config.ps1`
- `scripts/validate-bundle-artifacts.mjs`
- `src-tauri/binaries/expected-layout.md`
- `docs/RELEASE.md`

Upstream checks:

- `git ls-remote https://github.com/VietInnotech/sherpa-onnx-vit.git HEAD 'refs/heads/*' 'refs/tags/*'`
- Local diff of `6a7fe63ded85cd089bff73c12c54e1bda3bd7cf3..2ce802dc045dbb306d38085423de5327d45f1d26`
- Base `k2-fsa/sherpa-onnx` release context from:
  - <https://github.com/k2-fsa/sherpa-onnx/releases>
  - <https://sourceforge.net/projects/sherpa-onnx.mirror/files/v1.13.0/>

## Upstream Change Assessment

Commits after this repo's current pin:

```text
2ce802d 2026-05-08 Switch to JSON model registry; remove submodule
9ec3fd8 2026-05-07 Unify English and Vietnamese VAD defaults
4cde0f3 2026-05-07 Update plan
```

Important upstream file changes:

- Added `models.example.json`
- Added `src/sherpa_onnx_vit/services/registry.py`
- Removed `src/sherpa_onnx_vit/services/streaming.py`
- Removed streaming tests
- Removed `sherpa-onnx` git submodule
- Changed CLI and settings to require a models config file
- Updated model configuration to support Vietnamese and English models in one process
- Changed VAD default `vad_min_silence` from `0.25` to `0.5`

The new example registry is:

```json
{
  "models": [
    {
      "language": "vi",
      "model_dir": "models/stt/gipformer-65M-rnnt",
      "postprocess_mode": "capu",
      "vad_min_silence": 0.5
    },
    {
      "language": "en",
      "model_dir": "models/stt/sherpa-onnx-zipformer-en-libriheavy-20230830-medium-punct-case",
      "postprocess_mode": "none",
      "vad_min_silence": 0.5
    }
  ]
}
```

New CLI shape:

```bash
python -m sherpa_onnx_vit \
  --host 127.0.0.1 \
  --port 6006 \
  --provider cpu \
  --models-config /path/to/models.local.json \
  --num-threads 2
```

Old CLI shape used by this repo:

```bash
python -m sherpa_onnx_vit \
  --host 127.0.0.1 \
  --port 6006 \
  --provider cpu \
  --stt-model-family offline_int8 \
  --model-dir /path/to/model-dir \
  --postprocess-mode clean \
  --alias default-speech
```

The old shape is now incompatible with upstream `main`.

## Local Impact

### Adapter

`src-tauri/src/adapters/sherpa_onnx.rs` currently builds the old argument list
and has fallback defaults for `--model-dir`, `--stt-model-family`,
`--postprocess-mode`, and `--alias`.

Required change:

- Add a `{modelsConfigPath}` replacement.
- Prefer `runtime.modelsConfigPath`.
- Keep `model_dir` for validation and for generating the registry, not for CLI launch.
- Change default args template to `--models-config`.
- Keep process spawning as binary plus argument vector.

### Default Database Records

`src-tauri/src/db.rs` currently embeds:

- `upstreamCommit: 6a7fe63ded85cd089bff73c12c54e1bda3bd7cf3`
- old args template
- one model directory at `models/sherpa/default`
- alias `default-speech`

Required change:

- Update the upstream commit in default records.
- Add a default `modelsConfigPath`, probably `config/sherpa/models.local.json`.
- Update the default args template.
- Add a schema/data migration for existing local databases.

### Validation

`src-tauri/src/validation.rs` currently validates a single Sherpa model directory.
That remains useful, but it is no longer sufficient.

Required change:

- Validate that the models config file exists.
- Parse the JSON with `serde_json`.
- Validate `models` is a non-empty array.
- For each entry, validate:
  - `language` is currently one of `vi` or `en`
  - `model_dir` exists
  - `encoder*.onnx` exists
  - `decoder*.onnx` exists
  - `joiner*.onnx` exists
  - `tokens.txt` or `config.json` exists
  - `postprocess_mode` is compatible with language
- Preserve Simple Mode safe messaging. Detailed registry errors stay in Developer Mode.

### Machine Setup

The app currently expects:

- Llama model: `<app-root>/models/llama/default/model.gguf`
- Sherpa model: `<app-root>/models/sherpa/default`
- Sherpa runtime: `<app-root>/runtime/sherpa-onnx-vit`

Required change:

- Create `<app-root>/config/sherpa/`.
- Scaffold or validate `<app-root>/config/sherpa/models.local.json`.
- The scaffold should use absolute or app-root-relative paths consistently.
- Keep model placement developer-managed. Do not add user-facing model download or model selection.

### Runtime Packaging

`scripts/build-sherpa-onnx.*` and `src-tauri/binaries/expected-layout.md` pin
the old commit and package a Python runtime containing `sherpa_onnx_vit`,
`sherpa_onnx`, FastAPI, uvicorn, and `silero_vad.onnx`.

Required change:

- Pin `2ce802dc045dbb306d38085423de5327d45f1d26`.
- Package a runtime with the new Python package code.
- Confirm compatible `sherpa-onnx` version. Upstream README says macOS has been
  verified with source-built `sherpa-onnx 1.13.0`.
- Keep `silero_vad.onnx` as the only allowed model-like runtime asset in
  `src-tauri/bundle/runtime/sherpa-onnx-vit`.
- Do not bundle product STT model directories.

### Product Behavior

Simple Mode should not change:

- Still show aggregate `Local AI`.
- Do not expose engine names, model paths, config paths, ports, logs, PIDs, raw errors, or CLI args.
- Do not add model marketplace, model download, or model selection flows.

Developer Mode should change:

- Show the generated args with `--models-config`.
- Show the registry config path.
- Show per-model validation details only in Developer Mode.
- If multi-model registry is enabled, make clear that multiple internal STT models are still part of the single user-facing `Local AI` service.

## Recommended Implementation Plan

### 1. Capture the New Runtime Contract

- Update docs to state that `sherpa-onnx-vit` now requires a JSON model registry.
- Replace old single-model CLI examples with `--models-config`.
- Update all old upstream commit references.
- Add a short note that streaming has been removed or disabled in upstream multi-model mode.

### 2. Add Registry File Generation

Add a small backend helper that can generate the default Sherpa registry at:

```text
<app-root>/config/sherpa/models.local.json
```

Suggested single-model MVP output:

```json
{
  "models": [
    {
      "id": "default-speech",
      "language": "vi",
      "model_dir": "<app-root>/models/sherpa/default",
      "postprocess_mode": "clean_lower",
      "vad_min_silence": 0.5
    }
  ]
}
```

Use `clean_lower` instead of the old `clean`, because upstream normalizes
`clean` to `clean_lower` but the explicit new value is clearer.

### 3. Update Sherpa Adapter

Update `src-tauri/src/adapters/sherpa_onnx.rs`:

- Add `models_config_path`.
- Add replacement token `{modelsConfigPath}`.
- Add replacement token `{numThreads}` if not already covered by direct JSON use.
- Change default args to:

```json
[
  "-m",
  "sherpa_onnx_vit",
  "--host",
  "{host}",
  "--port",
  "{port}",
  "--provider",
  "{provider}",
  "--models-config",
  "{modelsConfigPath}",
  "--num-threads",
  "{numThreads}"
]
```

Keep support for `argsTemplate` so field deployments can override the contract
if upstream changes again.

### 4. Update Defaults and Migrations

- Bump schema version.
- Update default model package manifest with the new upstream commit.
- Update default Sherpa engine runtime JSON.
- Add a migration that updates old default `speech_engine` runtime only when it
  still points at the old upstream commit or old default args template.
- Avoid overwriting custom developer-edited profile paths, ports, providers, or extra args.

### 5. Update Validation

- Validate the registry config file in Developer Mode.
- Continue validating the default `model_dir` for old or partially migrated setups.
- Add clear Developer Mode issue codes:
  - `MISSING_MODELS_CONFIG`
  - `INVALID_MODELS_CONFIG`
  - `EMPTY_MODELS_CONFIG`
  - `MISSING_REGISTRY_MODEL_DIR`
  - `INVALID_REGISTRY_MODEL_FILES`
  - `INCOMPATIBLE_POSTPROCESS_MODE`
- Keep Simple Mode mapped to generic `Local AI needs attention`.

### 6. Update Setup Scripts

Update both shell and PowerShell scripts:

- Create `config/sherpa`.
- Check or scaffold `models.local.json`.
- Report readiness without leaking these paths to Simple Mode.
- Keep `--apply` behavior explicit.

### 7. Update Bundle Validation

`scripts/validate-bundle-artifacts.mjs` probably does not need a major change,
but it should smoke-test the new CLI import path if possible:

```bash
python3 -c 'import fastapi, uvicorn, sherpa_onnx, sherpa_onnx_vit'
python3 -m sherpa_onnx_vit --help
```

Do not require bundled product model files.

### 8. Verify End to End

Minimum verification target:

1. Generate or place `models.local.json`.
2. Run the packaged runtime:

   ```bash
   python -m sherpa_onnx_vit \
     --host 127.0.0.1 \
     --port 6006 \
     --provider cpu \
     --models-config /absolute/path/to/models.local.json \
     --num-threads 2
   ```

3. Verify:

   ```bash
   curl http://127.0.0.1:6006/health
   curl http://127.0.0.1:6006/v1/models
   ```

4. Verify `POST /v1/audio/transcriptions` with a known audio file.
5. Run Rust tests or at least `cargo check` under `src-tauri`.
6. Run frontend type/build checks using the repo's real package manager command.
7. Run bundle validation for the release target.
8. Start the Tauri app and confirm Simple Mode still hides all internals.

## Risks

- Existing developer machines with old DB records will fail to launch Sherpa
  until migrated.
- If `models.local.json` uses relative paths, upstream resolves them relative to
  its package repo root, not necessarily this app's app-root. Prefer absolute
  paths in generated registry files.
- CAPU is the new upstream Vietnamese default, but this product previously used
  safe `clean` behavior. Enabling CAPU may introduce extra model/runtime
  dependencies and possible downloads. MVP should keep `clean_lower` unless
  explicitly approved.
- Upstream removed streaming support in the multi-model path. Do not promise or
  expose streaming diagnostics until the contract is re-established.
- Python runtime packaging on macOS still needs relocation checks so it does not
  link to Homebrew, `/usr/local`, or a developer home directory.

## Questions Before Implementation

1. Should the launcher keep a single Vietnamese STT model for MVP, or should we
   adopt upstream's Vietnamese plus English two-model registry now?
2. Should default Vietnamese postprocess remain safe and local as `clean_lower`,
   or should we enable upstream's `capu` default and accept the extra CAPU
   dependency surface?
3. Should `models.local.json` be generated automatically by the app on startup
   from the existing engine profile, or only created by machine setup scripts?
4. Should existing local developer DBs be migrated automatically, or should this
   update reset/reseed Sherpa defaults only when explicitly requested?
5. Are Windows CUDA and Linux or Jetson targets in scope for this repo update, or
   should the immediate implementation remain macOS Apple Silicon plus Windows
   packaging scaffolding only?

## Recommended Answers If We Want the Smallest Safe Update

1. Keep one Vietnamese model for MVP.
2. Use `clean_lower`, not `capu`, by default.
3. Generate `models.local.json` automatically if missing, and let setup scripts
   scaffold it for field preparation.
4. Migrate only profiles that still match the old default Sherpa args template.
5. Validate macOS Apple Silicon first, then update Windows scripts/docs without
   claiming Windows runtime verification until tested on a target machine.
