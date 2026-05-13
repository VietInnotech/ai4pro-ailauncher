# Release Guide

This document describes the current manual release process for the scaffold app.
It is based on the commands verified for `v0.1.0`.

## Release status

The repo can currently produce:

- an ad-hoc signed macOS Apple Silicon `.app` bundle
- an ad-hoc signed macOS Apple Silicon `.dmg`
- a Windows x64 executable cross-built from macOS through `cargo-xwin`
- packages that include locally supplied runtime artifacts from `src-tauri/bundle/`

The repo does not yet produce:

- a Developer ID signed or notarized macOS app
- a Windows installer
- signed Windows binaries
- model-inclusive release artifacts

Treat releases as developer/operator preview builds until those gaps are closed.

Deployment instructions for published preview builds live in [DEPLOYMENT.md](DEPLOYMENT.md).

## Prerequisites

Install the local toolchain:

```bash
just --version
bun --version
cargo --version
gh --version
```

`just` is optional but recommended. The repo `justfile` wraps the verified install, check, build, versioning, and tagging flows.

The expected package manager is declared in `package.json`:

```json
"packageManager": "bun@1.3.13"
```

For Windows cross-builds from macOS, install `cargo-xwin` and the Windows MSVC target:

```bash
rustup target add x86_64-pc-windows-msvc
cargo install cargo-xwin
cargo xwin --version
```

For GitHub releases, confirm authentication:

```bash
gh auth status
```

## Runtime artifact input

Release packaging requires prepared runtime artifacts under:

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

Validate the layout before packaging:

```bash
bun run validate:bundle-artifacts
```

The validator intentionally rejects model-like files under `src-tauri/bundle/`.
The allowed `silero_vad.onnx` path is a Sherpa runtime dependency. Product model files are never bundled.

The runtime model contract is intentionally small:

- Llama uses one GGUF file.
- Sherpa uses one model directory containing `encoder*.onnx`, `decoder*.onnx`, `joiner*.onnx`, and `tokens.txt` or `config.json`.
- `tokensPath` should normally stay empty; Sherpa infers tokens from the model directory.

## Pre-release checks

Start from a clean working tree unless the release requires a committed fix:

```bash
git status --short --branch
git fetch origin --tags --prune
```

Install dependencies exactly from the lockfile:

```bash
bun install --frozen-lockfile
```

Or run the full verified release baseline with:

```bash
just release-check
```

Run frontend and Rust checks:

```bash
bun run check
bun run build
cd src-tauri
cargo check
cargo test
cd ..
```

Current expected caveat: Rust emits dead-code warnings because the project is still a scaffold. Warnings should be reviewed before release, but they do not currently block `v0.1.x` preview releases.

## macOS build

Build the macOS app and DMG:

```bash
bun run tauri build
```

Expected outputs:

```text
src-tauri/target/release/bundle/macos/Local AI.app
src-tauri/target/release/bundle/dmg/Local AI_0.1.0_aarch64.dmg
```

The current macOS build uses an ad-hoc signature through `bundle.macOS.signingIdentity = "-"` in `src-tauri/tauri.conf.json`. This creates a structurally valid signed app bundle for preview testing, but it is not Developer ID signed or notarized.

Before publishing a DMG, verify both the app bundle and the image:

```bash
codesign --verify --deep --strict --verbose=4 \
  "src-tauri/target/release/bundle/macos/Local AI.app"
hdiutil verify "src-tauri/target/release/bundle/dmg/Local AI_0.1.0_aarch64.dmg"
```

Mount the DMG and verify the app that users will copy:

```bash
MOUNT_DIR="$(mktemp -d /tmp/local-ai-dmg.XXXXXX)"
hdiutil attach -readonly -nobrowse -mountpoint "$MOUNT_DIR" \
  "src-tauri/target/release/bundle/dmg/Local AI_0.1.0_aarch64.dmg"
codesign --verify --deep --strict --verbose=4 "$MOUNT_DIR/Local AI.app"
hdiutil detach "$MOUNT_DIR"
```

Gatekeeper assessment is still expected to reject preview artifacts because there is no Developer ID notarization:

```bash
spctl --assess --type execute --verbose=4 \
  "src-tauri/target/release/bundle/macos/Local AI.app"
spctl --assess --type open --context context:primary-signature --verbose=4 \
  "src-tauri/target/release/bundle/dmg/Local AI_0.1.0_aarch64.dmg"
```

If `codesign --verify` reports `code has no resources but signature indicates they must be present`, do not publish that DMG. It means the outer `.app` bundle was not signed after resources were added, and macOS can show the misleading damaged-app dialog on another machine.

## macOS smoke test

Use an isolated app data root so the test does not touch real local data. Run the binary inside the packaged `.app`:

```bash
rm -rf /tmp/ai4pro-ailauncher-smoke
mkdir -p /tmp/ai4pro-ailauncher-smoke
LOCAL_AI_APP_DATA_ROOT=/tmp/ai4pro-ailauncher-smoke \
  "./src-tauri/target/release/bundle/macos/Local AI.app/Contents/MacOS/ai-launcher-tauri"
```

Expected result:

- the process starts and stays running
- stderr is empty during basic startup
- bundled runtimes are copied into `/tmp/ai4pro-ailauncher-smoke/binaries` and `/tmp/ai4pro-ailauncher-smoke/runtime`
- `/tmp/ai4pro-ailauncher-smoke/data/local_ai.sqlite` exists

This verifies app startup, runtime sync, and SQLite initialization only. It does not verify real engine startup until model files are supplied.

Do not use the loose `src-tauri/target/release/ai-launcher-tauri` binary for packaged-resource testing. It can use stale loose resources under `src-tauri/target/release/`; the `.app` binary reflects the deployable bundle.

## Real-asset development test

Use this workflow before treating a macOS build as deployable with real engines.

Expected local test assets:

```text
models/llama/default/model.gguf
models/stt/gipformer-65M-rnnt
models/audio/government-meeting-20s.wav
```

First test the engines directly on non-default ports:

```bash
./src-tauri/bundle/binaries/llama-server-aarch64-apple-darwin \
  --host 127.0.0.1 \
  --port 18081 \
  --model "$PWD/models/llama/default/model.gguf"
```

```bash
curl -fsS http://127.0.0.1:18081/health
curl -fsS http://127.0.0.1:18081/v1/models
```

```bash
PYTHONPATH="$PWD/src-tauri/bundle/runtime/sherpa-onnx-vit" \
  ./src-tauri/bundle/runtime/sherpa-onnx-vit/python3 \
  -m sherpa_onnx_vit \
  --host 127.0.0.1 \
  --port 18082 \
  --provider cpu \
  --stt-model-family offline_int8 \
  --model-dir "$PWD/models/stt/gipformer-65M-rnnt" \
  --postprocess-mode clean \
  --alias default-speech
```

```bash
curl -fsS http://127.0.0.1:18082/health
curl -fsS http://127.0.0.1:18082/v1/models
curl -fsS -X POST http://127.0.0.1:18082/v1/audio/transcriptions \
  -F file=@"$PWD/models/audio/government-meeting-20s.wav" \
  -F model=default-speech
```

Then test the packaged app with an isolated app data root. Bootstrap once so SQLite defaults exist:

```bash
APP_ROOT=/tmp/ai4pro-app-real-assets
rm -rf "$APP_ROOT"
mkdir -p "$APP_ROOT"
LOCAL_AI_APP_DATA_ROOT="$APP_ROOT" \
  "./src-tauri/target/release/bundle/macos/Local AI.app/Contents/MacOS/ai-launcher-tauri" &
APP_PID=$!
sleep 4
kill "$APP_PID" 2>/dev/null || true
wait "$APP_PID" 2>/dev/null || true
```

Configure the temporary database to use the real assets and auto-start:

```bash
python3 - <<'PY'
import sqlite3
from pathlib import Path

root = Path('/tmp/ai4pro-app-real-assets')
conn = sqlite3.connect(root / 'data/local_ai.sqlite')
now = '2026-05-06T00:00:00Z'

conn.execute("INSERT OR REPLACE INTO app_settings (key, value) VALUES (?, ?)", ('app_data_root', str(root)))
conn.execute("INSERT OR REPLACE INTO app_settings (key, value) VALUES (?, ?)", ('auto_start_local_ai', 'true'))
conn.execute("INSERT OR REPLACE INTO app_settings (key, value) VALUES (?, ?)", ('machine_configured', 'true'))
conn.execute(
    "UPDATE engine_profiles SET model_path=?, model_dir=NULL, tokens_path=NULL, status='stopped', pid=NULL, last_error=NULL, last_exit_code=NULL, updated_at=? WHERE id='language_engine'",
    ('/Users/leakless/code/ai4pro-ailauncher/models/llama/default/model.gguf', now),
)
conn.execute(
    "UPDATE engine_profiles SET model_path=NULL, model_dir=?, tokens_path=NULL, status='stopped', pid=NULL, last_error=NULL, last_exit_code=NULL, updated_at=? WHERE id='speech_engine'",
    ('/Users/leakless/code/ai4pro-ailauncher/models/stt/gipformer-65M-rnnt', now),
)
conn.execute("UPDATE engine_runtime_state SET status='stopped', pid=NULL, last_error=NULL, last_exit_code=NULL, updated_at=?", (now,))
conn.commit()
conn.close()
PY
```

Launch the packaged app and verify both default ports:

```bash
LOCAL_AI_APP_DATA_ROOT=/tmp/ai4pro-app-real-assets \
  "./src-tauri/target/release/bundle/macos/Local AI.app/Contents/MacOS/ai-launcher-tauri"
```

In another terminal:

```bash
curl -fsS http://127.0.0.1:8080/health
curl -fsS http://127.0.0.1:8080/v1/models
curl -fsS http://127.0.0.1:6006/health
curl -fsS http://127.0.0.1:6006/v1/models
curl -fsS -X POST http://127.0.0.1:6006/v1/audio/transcriptions \
  -F file=@"$PWD/models/audio/government-meeting-20s.wav" \
  -F model=default-speech
```

Expected result:

- Llama `/health` returns `{"status":"ok"}`.
- Llama `/v1/models` lists the GGUF model.
- Sherpa `/health` returns `{"status":"ok"}`.
- Sherpa `/v1/models` lists `default-speech`.
- Sherpa transcription returns Vietnamese text.

## Windows cross-build

Build the Windows x64 executable from macOS:

```bash
bun run tauri build --runner cargo-xwin --target x86_64-pc-windows-msvc --no-bundle
```

Expected output:

```text
src-tauri/target/x86_64-pc-windows-msvc/release/ai-launcher-tauri.exe
```

The `--no-bundle` flag is intentional for the current scaffold. It validates the Windows executable build without producing a Windows installer. A real Windows release should still be tested on Windows before it is treated as production-ready.

## Tagging

Use the version from `package.json`, `src-tauri/Cargo.toml`, and `src-tauri/tauri.conf.json`.
For `0.1.0`, the tag is `v0.1.0`.

Inspect or update the synced version with:

```bash
just version
just version-check
just set-version 0.1.1
```

Create the local release tag with the safer helper flow:

```bash
just tag-check 0.1.0
just tag-release 0.1.0
git show v0.1.0
git push origin HEAD
git push origin v0.1.0
```

`just tag-check <version>` fails if versions are out of sync, the working tree is dirty, `HEAD` is detached, or the tag already exists locally or on `origin`.

`just tag-release <version>` creates the local annotated tag only. It does not push anything.

If a tag already exists, inspect it before changing anything:

```bash
git show v0.1.0
gh release view v0.1.0 --repo VietInnotech/ai4pro-ailauncher
```

Do not delete or retag published releases unless the team explicitly agrees.

## Publishing to GitHub Releases

Prepare release-friendly asset names:

```bash
mkdir -p /tmp/ai4pro-release-assets
cp "src-tauri/target/release/bundle/dmg/Local AI_0.1.0_aarch64.dmg" \
  /tmp/ai4pro-release-assets/Local-AI_0.1.0_aarch64.dmg
cp src-tauri/target/x86_64-pc-windows-msvc/release/ai-launcher-tauri.exe \
  /tmp/ai4pro-release-assets/Local-AI_0.1.0_x86_64-pc-windows-msvc.exe
```

Generate SHA-256 checksums and include them in the release notes:

```bash
shasum -a 256 /tmp/ai4pro-release-assets/Local-AI_0.1.0_aarch64.dmg
shasum -a 256 /tmp/ai4pro-release-assets/Local-AI_0.1.0_x86_64-pc-windows-msvc.exe
```

Create the release:

```bash
gh release create v0.1.0 \
  /tmp/ai4pro-release-assets/Local-AI_0.1.0_aarch64.dmg \
  /tmp/ai4pro-release-assets/Local-AI_0.1.0_x86_64-pc-windows-msvc.exe \
  --repo VietInnotech/ai4pro-ailauncher \
  --title "Local AI v0.1.0" \
  --notes "Initial scaffold release.

Validation performed before publishing:
- bun install --frozen-lockfile
- bun run check
- bun run build
- cargo check
- cargo test
- macOS Tauri build
- macOS smoke launch with isolated app data root
- Windows x86_64 MSVC cross-build via cargo-xwin

Notes:
- macOS artifact is an unsigned aarch64 DMG.
- Windows artifact is a cross-compiled executable, not an installer.
- Local AI runtime artifacts are bundled from src-tauri/bundle/.
- Model files are not bundled.
- Install-and-launch deployment instructions: docs/DEPLOYMENT.md

SHA-256:
- Local-AI_0.1.0_aarch64.dmg: 95fc814925761ce8ac682b664f9e0430d45c971763a68884763f0307b2442b88
- Local-AI_0.1.0_x86_64-pc-windows-msvc.exe: c974f2ec8c65e3a7c8ff29ea17165ea683fa7c9fb96a487372be881916f854de"
```

Verify the published release:

```bash
gh release view v0.1.0 \
  --repo VietInnotech/ai4pro-ailauncher \
  --json tagName,name,url,assets,isDraft,isPrerelease,publishedAt
```

## Current release artifacts

`v0.1.0` was published at:

```text
https://github.com/VietInnotech/ai4pro-ailauncher/releases/tag/v0.1.0
```

Published assets:

```text
Local-AI_0.1.0_aarch64.dmg
Local-AI_0.1.0_x86_64-pc-windows-msvc.exe
```

Published SHA-256 checksums:

```text
95fc814925761ce8ac682b664f9e0430d45c971763a68884763f0307b2442b88  Local-AI_0.1.0_aarch64.dmg
c974f2ec8c65e3a7c8ff29ea17165ea683fa7c9fb96a487372be881916f854de  Local-AI_0.1.0_x86_64-pc-windows-msvc.exe
```

Preview deployment instructions:

```text
docs/DEPLOYMENT.md
```

## Production release blockers

Before treating this as an end-user production release, finish:

- macOS signing, notarization, and stapling
- Windows installer and code signing
- real model preparation flow
- end-to-end engine launch validation on prepared macOS and Windows machines
- release-grade health checks and process supervision
- machine setup scripts that produce a clear readiness report
