# Unsigned Preview Deployment

This guide covers simple install-and-launch deployment for preview builds.
It is for technical users and operators who understand the app is unsigned and still requires operator-supplied model files.

## Current release

Release page:

```text
https://github.com/VietInnotech/ai4pro-ailauncher/releases/tag/v0.1.0
```

Preview assets:

```text
macOS Apple Silicon:
https://github.com/VietInnotech/ai4pro-ailauncher/releases/download/v0.1.0/Local-AI_0.1.0_aarch64.dmg

Windows x64:
https://github.com/VietInnotech/ai4pro-ailauncher/releases/download/v0.1.0/Local-AI_0.1.0_x86_64-pc-windows-msvc.exe
```

Known `v0.1.0` SHA-256 checksums:

```text
95fc814925761ce8ac682b664f9e0430d45c971763a68884763f0307b2442b88  Local-AI_0.1.0_aarch64.dmg
c974f2ec8c65e3a7c8ff29ea17165ea683fa7c9fb96a487372be881916f854de  Local-AI_0.1.0_x86_64-pc-windows-msvc.exe
```

Verify a downloaded file on macOS:

```bash
shasum -a 256 ~/Downloads/Local-AI_0.1.0_aarch64.dmg
```

Verify a downloaded file on Windows PowerShell:

```powershell
Get-FileHash "$env:USERPROFILE\Downloads\Local-AI_0.1.0_x86_64-pc-windows-msvc.exe" -Algorithm SHA256
```

## Deployment status

This deployment is intended to verify:

- the app can be installed or copied to another machine
- the app opens without crashing
- the app creates its local app data root
- bundled runtime artifacts are copied into the app data root when the release was built with `src-tauri/bundle/` artifacts
- SQLite initialization creates `local_ai.sqlite`

This deployment does not verify:

- `llama.cpp` startup until an external GGUF model is configured
- `sherpa-onnx` startup until an external speech model directory is configured
- real model loading
- production health checks
- unattended machine setup

## macOS Apple Silicon install

Download `Local-AI_0.1.0_aarch64.dmg` from the release page.

Install the app:

```bash
hdiutil attach ~/Downloads/Local-AI_0.1.0_aarch64.dmg
cp -R "/Volumes/Local AI/Local AI.app" /Applications/
hdiutil detach "/Volumes/Local AI"
xattr -dr com.apple.quarantine "/Applications/Local AI.app"
open "/Applications/Local AI.app"
```

If macOS blocks the DMG itself before install, clear quarantine on the downloaded DMG:

```bash
xattr -dr com.apple.quarantine ~/Downloads/Local-AI_0.1.0_aarch64.dmg
```

If `/Applications` permissions block quarantine removal, use:

```bash
sudo xattr -dr com.apple.quarantine "/Applications/Local AI.app"
```

### macOS smoke test

After launch, verify the app data root:

```bash
ls -la "$HOME/Library/Application Support/AI4Pro/AILauncher"
ls -la "$HOME/Library/Application Support/AI4Pro/AILauncher/binaries"
ls -la "$HOME/Library/Application Support/AI4Pro/AILauncher/runtime"
ls -la "$HOME/Library/Application Support/AI4Pro/AILauncher/data/local_ai.sqlite"
```

Expected result:

- `Local AI.app` opens
- no startup crash
- bundled runtimes exist under `binaries/` and `runtime/`
- `~/Library/Application Support/AI4Pro/AILauncher/data/local_ai.sqlite` exists

If the app does not open, try launching from Terminal to see stderr:

```bash
"/Applications/Local AI.app/Contents/MacOS/ai-launcher-tauri"
```

## Windows x64 install

Download `Local-AI_0.1.0_x86_64-pc-windows-msvc.exe` from the release page.

Copy the executable into a stable local app folder and launch it:

```powershell
New-Item -ItemType Directory -Force "$env:LOCALAPPDATA\AI4Pro\LocalAI"
Copy-Item "$env:USERPROFILE\Downloads\Local-AI_0.1.0_x86_64-pc-windows-msvc.exe" "$env:LOCALAPPDATA\AI4Pro\LocalAI\Local AI.exe"
Start-Process "$env:LOCALAPPDATA\AI4Pro\LocalAI\Local AI.exe"
```

If Windows SmartScreen appears, choose **More info** and then **Run anyway**.
This is expected for the unsigned preview executable.

### Windows smoke test

After launch, verify the app data root:

```powershell
Test-Path "$env:LOCALAPPDATA\AI4Pro\AILauncher\data\local_ai.sqlite"
Test-Path "$env:LOCALAPPDATA\AI4Pro\AILauncher\binaries"
Test-Path "$env:LOCALAPPDATA\AI4Pro\AILauncher\runtime"
Get-ChildItem "$env:LOCALAPPDATA\AI4Pro\AILauncher" -Force
```

Expected result:

- the app window opens
- no startup crash
- bundled runtimes exist under `binaries\` and `runtime\`
- `%LOCALAPPDATA%\AI4Pro\AILauncher\data\local_ai.sqlite` exists

## Operational caveats

- macOS builds are unsigned and not notarized.
- Windows builds are unsigned and may trigger SmartScreen.
- Windows deployment is a copied executable, not an installer.
- Runtime artifacts are bundled only when the release builder supplies `src-tauri/bundle/` before packaging.
- Model files are not bundled. Operators configure one Llama GGUF file and one Sherpa model directory.
- The default convention is `models/llama/default/model.gguf` and `models/sherpa/default/`, but Developer Mode may point to absolute paths during field setup.
- `tokensPath` should normally stay empty; Sherpa resolves `tokens.txt` from the configured model directory.
- Local AI Start/Stop/Restart should not be considered production-ready until model folders are prepared and tested on the target machine.
- Machine setup scripts are still scaffolds and should not be treated as unattended provisioning.
