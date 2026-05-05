
Here’s the rewritten full `.md` plan for an AI coding agent, updated for your real deployment model:

* **User-facing UI must stay simple**
* **Advanced info hidden behind Developer Mode**
* **Model files are manually placed**
* **Developers configure each machine individually**
* **No model marketplace**
* **No model download**
* **No user-facing model path selection**
* **No normal-user access to model directories**
* **Svelte + Tailwind + SQLite + Tauri**

Humanity survives another configuration file.

````md
# AI Inference Engine Launcher Plan

## 1. Objective

Build a cross-platform desktop launcher for local AI inference engines on:

- Windows
- macOS Intel
- macOS Apple Silicon

The launcher manages local AI engine processes for:

1. `llama.cpp` via `llama-server`
2. `sherpa-onnx server`, using the repository:
   - https://github.com/VietInnotech/sherpa-onnx-vit.git

The app must be built with:

- Svelte
- Tailwind CSS
- SQLite
- Tauri 2

The app is intended for deployment on machines that are individually prepared by developers or technical operators.

Model files are not downloaded by the app.

Model files are not selected by normal users.

Model files are manually placed on each machine by developers before or during setup.

The launcher must provide a very simple user-facing UI while hiding advanced configuration, model paths, engine details, logs, ports, and internal diagnostics.

---

## 2. Product Summary

The application is a simple local AI launcher.

To a normal user, the app should look like this:

```text
Local AI is ready

[ Start ]
````

or:

```text
Local AI is running

[ Stop ]
```

or:

```text
Local AI needs attention

[ Restart ]
```

Internally, the app manages multiple local inference engines:

* A language model engine using `llama.cpp`
* A speech or audio inference engine using `sherpa-onnx`

The normal user should not know or care that these engines exist separately.

The app must hide:

* Model directories
* Model filenames
* Binary paths
* Engine process names
* Ports
* Host bindings
* SQLite location
* Logs
* Runtime arguments
* PID/process information
* Tokens files
* ONNX file names
* GGUF file names
* Advanced hardware settings

Developer Mode should reveal these details only when intentionally activated.

---

## 3. Deployment Model

This app is not a self-service model installer.

Each target machine will be prepared manually by developers, support engineers, or technical operators.

The setup process is:

```text
1. Install the app.
2. Place required model files in predefined local directories.
3. Place or verify engine binaries if not bundled.
4. Configure machine-specific paths and runtime settings.
5. Verify the app can start both engines.
6. Leave the app in Simple Mode for the end user.
```

Normal users should not be responsible for:

* Choosing model files
* Choosing model directories
* Editing engine settings
* Selecting ports
* Viewing logs
* Fixing missing model files
* Changing binaries
* Configuring runtime arguments

If something is missing or misconfigured, Simple Mode should show a safe generic message.

Developer Mode should show the real diagnostic information.

---

## 4. High-Level Architecture

Use the following architecture:

```text
Svelte UI
  ↓
Tauri command bridge
  ↓
Rust backend
  ↓
SQLite database
  ↓
Engine manager
  ↓
Process supervisor
  ↓
Sidecar or configured native binaries
  ↓
llama.cpp / sherpa-onnx servers
```

The app should treat each inference server as an internal component of a single user-facing service called:

```text
Local AI
```

Internally:

```text
Local AI
  ├── Language Engine: llama.cpp
  └── Speech Engine: sherpa-onnx
```

Simple Mode shows only the aggregate `Local AI` state.

Developer Mode shows the individual component states.

---

## 5. Core Stack

## 5.1 Frontend

Use:

* Svelte
* TypeScript
* Tailwind CSS

Recommended UI structure:

```text
src/
  app.css
  main.ts
  lib/
    api/
    components/
    stores/
    types/
  routes/
```

## 5.2 Desktop Runtime

Use:

* Tauri 2
* Rust backend
* Tauri commands
* Tauri sidecars where appropriate

The app should support both:

1. Bundled engine binaries
2. Manually configured engine binary paths

Bundled binaries are preferred for release builds.

Manually configured binary paths are useful for developer setup and field deployment.

## 5.3 Database

Use:

* SQLite

SQLite stores:

* App settings
* Engine profiles
* Runtime state
* Model package records
* Developer-mode settings
* Machine-specific configuration
* Log metadata

Do not store large logs inside SQLite.

Logs should be stored as files.

---

## 6. Project Structure

Use this structure:

```text
ai-engine-launcher/
  package.json
  pnpm-lock.yaml
  vite.config.ts
  svelte.config.js
  tailwind.config.js
  postcss.config.js

  src/
    app.css
    main.ts

    lib/
      api/
        local-ai.ts
        developer.ts
        settings.ts

      components/
        SimpleStatusCard.svelte
        LocalAiControl.svelte
        SimpleErrorMessage.svelte
        DeveloperModeGate.svelte
        DeveloperLayout.svelte
        DeveloperEngineTable.svelte
        DeveloperEngineDetail.svelte
        DeveloperModelTable.svelte
        DeveloperLogViewer.svelte
        DeveloperDiagnostics.svelte
        StatusBadge.svelte

      stores/
        local-ai.ts
        developer-mode.ts
        app-settings.ts

      types/
        local-ai.ts
        engine.ts
        developer.ts
        settings.ts

    routes/
      SimpleHome.svelte
      DeveloperDashboard.svelte
      DeveloperEngines.svelte
      DeveloperModels.svelte
      DeveloperLogs.svelte
      DeveloperSettings.svelte
      DeveloperDiagnostics.svelte

  src-tauri/
    Cargo.toml
    tauri.conf.json

    capabilities/
      default.json

    src/
      main.rs
      commands.rs
      db.rs
      migrations.rs
      models.rs
      app_paths.rs
      app_settings.rs
      local_ai.rs
      engine_manager.rs
      process_supervisor.rs
      process_registry.rs
      health.rs
      ports.rs
      logs.rs
      validation.rs
      errors.rs

      adapters/
        mod.rs
        llama_cpp.rs
        sherpa_onnx.rs

      developer/
        mod.rs
        diagnostics.rs
        developer_mode.rs

      binaries/
      llama-server-aarch64-apple-darwin
      llama-server-x86_64-apple-darwin
      llama-server-x86_64-pc-windows-msvc.exe

      # sherpa-onnx-vit: not a native sidecar
      # packaged as internal Python runtime/artifact
      # see scripts/prepare-sherpa-runtime.sh


  scripts/
    prepare-sidecars.sh
    prepare-sidecars.ps1
    build-llama-cpp.sh
    build-llama-cpp.ps1
    build-sherpa-onnx.sh
    build-sherpa-onnx.ps1
    initialize-machine-config.sh
    initialize-machine-config.ps1
```

---

## 7. UX Modes

The app must have two UI modes:

```text
Simple Mode
Developer Mode
```

---

# 7.1 Simple Mode

Simple Mode is the default.

It is the only mode normal users should see.

Simple Mode must hide all implementation details.

The main screen should show:

```text
Local AI status
Start button
Stop button
Restart button
Short safe error message if needed
```

Simple Mode must not show:

```text
llama.cpp
sherpa-onnx
model path
model directory
GGUF
ONNX
tokens.txt
binary path
port
host
PID
stdout
stderr
CLI args
database path
logs path
runtime JSON
developer settings
```

## Simple Mode Screens

The Simple Mode UI should be one main screen.

### State: Ready

```text
Local AI is ready

[ Start ]
```

### State: Starting

```text
Local AI is starting

Please wait...
```

### State: Running

```text
Local AI is running

[ Stop ]
```

### State: Needs Attention

```text
Local AI needs attention

[ Restart ]
```

Optional secondary action:

```text
Contact support
```

Do not expose detailed errors in Simple Mode.

---

# 7.2 Developer Mode

Developer Mode is hidden.

It can be activated by clicking the app logo multiple times.

Recommended activation:

```text
Click the app logo 7 times within 5 seconds.
```

After activation, show:

```text
Developer Mode enabled.
Advanced settings and diagnostics are now visible.
```

Developer Mode should be session-only by default.

Persistent Developer Mode may be supported, but only as a developer setting.

Developer Mode may show:

```text
Engine names
Model paths
Binary paths
Ports
Host bindings
PIDs
Health URLs
Logs
SQLite records
Runtime arguments
Validation details
Diagnostics
```

Developer Mode is a UX barrier, not a security boundary.

Do not rely on hidden UI alone for real security.

The backend must not send sensitive details to Simple Mode commands.

---

## 8. User-Facing Status Model

Simple Mode should use a simplified status model.

```ts
export type SimpleLocalAiStatus =
  | "not_running"
  | "starting"
  | "ready"
  | "stopping"
  | "needs_attention";
```

Simple DTO:

```ts
export type SimpleLocalAiStatusDto = {
  status: SimpleLocalAiStatus;
  title: string;
  message: string;
  canStart: boolean;
  canStop: boolean;
  canRestart: boolean;
};
```

Example:

```json
{
  "status": "ready",
  "title": "Local AI is running",
  "message": "The local AI service is available.",
  "canStart": false,
  "canStop": true,
  "canRestart": true
}
```

This DTO must not include:

```text
model paths
ports
engine names
binary paths
process IDs
logs
raw errors
```

---

## 9. Internal Engine Status Model

Internally, each engine can have a more detailed status.

```ts
export type EngineStatus =
  | "stopped"
  | "starting"
  | "running"
  | "unhealthy"
  | "stopping"
  | "crashed"
  | "missing_binary"
  | "missing_model"
  | "invalid_config"
  | "port_conflict";
```

Status mapping:

```text
Internal State       Simple Mode State
--------------------------------------
stopped              not_running
starting             starting
running              ready
stopping             stopping
unhealthy            needs_attention
crashed              needs_attention
missing_binary       needs_attention
missing_model        needs_attention
invalid_config       needs_attention
port_conflict        needs_attention
```

Simple Mode should never display raw internal status names.

---

## 10. Model File Strategy

Model files are manually placed on each machine.

The app must not require normal users to select model files.

The app must not expose model paths in Simple Mode.

The app should expect models in predefined directories.

Developers configure those directories per machine.

---

# 10.1 Recommended Model Directory Layout

Use app-managed machine-specific directories.

## Windows, machine-wide

Preferred:

```text
C:\ProgramData\CompanyName\AppName\
  models\
    llama\
      default\
        model.gguf
    sherpa\
      default\
        tokens.txt
        *.onnx
  config\
  logs\
```

## Windows, per-user fallback

```text
%APPDATA%\CompanyName\AppName\
  models\
    llama\
      default\
        model.gguf
    sherpa\
      default\
        tokens.txt
        *.onnx
  config\
  logs\
```

## macOS, machine-wide

Preferred:

```text
/Library/Application Support/CompanyName/AppName/
  models/
    llama/
      default/
        model.gguf
    sherpa/
      default/
        tokens.txt
        *.onnx
  config/
  logs/
```

## macOS, per-user fallback

```text
~/Library/Application Support/CompanyName/AppName/
  models/
    llama/
      default/
        model.gguf
    sherpa/
      default/
        tokens.txt
        *.onnx
  config/
  logs/
```

Use one configurable app data root.

Store the resolved root in SQLite or an app config file.

---

# 10.2 Model Manifest

Use a model manifest to describe expected files.

The manifest should be created or edited by developers during setup.

Example:

```json
{
  "schemaVersion": 1,
  "models": [
    {
      "id": "default_llm",
      "kind": "llama_cpp",
      "displayName": "Language Model",
      "internalName": "default_llama_model",
      "relativePath": "models/llama/default/model.gguf",
      "required": true,
      "sha256": null
    },
    {
      "id": "default_speech",
      "kind": "sherpa_onnx",
      "displayName": "Speech Model",
      "internalName": "default_sherpa_model",
      "relativePath": "models/sherpa/default",
      "requiredFiles": [
        "tokens.txt",
        "*.onnx"
      ],
      "required": true,
      "sha256": null
    }
  ]
}
```

The app should support checksums but not require them for MVP.

If checksums are provided, validate them in Developer Mode diagnostics.

---

# 10.3 Simple Mode Model Messages

Simple Mode may show:

```text
AI files are ready.
```

or:

```text
Required AI files are missing. Please contact support.
```

Simple Mode must not show:

```text
C:\ProgramData\CompanyName\AppName\models\llama\default\model.gguf is missing
```

Developer Mode may show the full path.

---

## 11. Machine Setup Strategy

Each machine must be configured by a developer or support engineer.

Provide setup scripts:

```text
scripts/initialize-machine-config.sh
scripts/initialize-machine-config.ps1
```

These scripts should:

```text
1. Create app data directories.
2. Verify model files are present.
3. Verify engine binaries are present or bundled.
4. Create default SQLite database if needed.
5. Insert default engine profiles.
6. Insert model package records.
7. Validate ports.
8. Print a developer-facing setup report.
```

The scripts may accept arguments:

```text
--app-data-root
--llama-model-path
--sherpa-model-dir
--llama-port
--sherpa-port
--use-bundled-binaries
--llama-binary-path
--sherpa-binary-path
```

Example Windows setup:

```powershell
.\scripts\initialize-machine-config.ps1 `
  -AppDataRoot "C:\ProgramData\CompanyName\AppName" `
  -LlamaModelPath "C:\ProgramData\CompanyName\AppName\models\llama\default\model.gguf" `
  -SherpaModelDir "C:\ProgramData\CompanyName\AppName\models\sherpa\default" `
  -LlamaPort 8080 `
  -SherpaPort 6006 `
  -UseBundledBinaries
```

Example macOS setup:

```bash
./scripts/initialize-machine-config.sh \
  --app-data-root "/Library/Application Support/CompanyName/AppName" \
  --llama-model-path "/Library/Application Support/CompanyName/AppName/models/llama/default/model.gguf" \
  --sherpa-model-dir "/Library/Application Support/CompanyName/AppName/models/sherpa/default" \
  --llama-port 8080 \
  --sherpa-port 6006 \
  --use-bundled-binaries
```

---

## 12. SQLite Database Design

SQLite must persist:

```text
machine configuration
engine profiles
model package references
runtime state
developer settings
logs metadata
```

---

# 12.1 Table: app_settings

```sql
CREATE TABLE IF NOT EXISTS app_settings (
  key TEXT PRIMARY KEY,
  value_json TEXT NOT NULL,
  updated_at TEXT NOT NULL
);
```

Example keys:

```text
app_data_root
developer_mode_persisted
stop_engines_on_exit
simple_mode_only
machine_configured
setup_version
```

---

# 12.2 Table: model_packages

```sql
CREATE TABLE IF NOT EXISTS model_packages (
  id TEXT PRIMARY KEY,
  kind TEXT NOT NULL,
  display_name TEXT NOT NULL,
  internal_name TEXT NOT NULL,
  relative_path TEXT NOT NULL,
  manifest_json TEXT NOT NULL,
  installed INTEGER NOT NULL DEFAULT 0,
  verified INTEGER NOT NULL DEFAULT 0,
  last_verified_at TEXT,
  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL
);
```

Notes:

* `relative_path` is relative to app data root.
* Full resolved path should be calculated internally.
* Simple Mode must not receive full resolved paths.

---

# 12.3 Table: engine_profiles

```sql
CREATE TABLE IF NOT EXISTS engine_profiles (
  id TEXT PRIMARY KEY,
  kind TEXT NOT NULL,
  name TEXT NOT NULL,
  enabled INTEGER NOT NULL DEFAULT 1,

  binary_mode TEXT NOT NULL DEFAULT 'bundled',
  binary_name TEXT NOT NULL,
  binary_path TEXT,

  model_package_id TEXT,

  host TEXT NOT NULL DEFAULT '127.0.0.1',
  port INTEGER NOT NULL,

  runtime_json TEXT NOT NULL DEFAULT '{}',
  extra_args_json TEXT NOT NULL DEFAULT '[]',

  auto_start INTEGER NOT NULL DEFAULT 0,

  created_at TEXT NOT NULL,
  updated_at TEXT NOT NULL,

  FOREIGN KEY(model_package_id) REFERENCES model_packages(id)
);
```

Important:

* `binary_path` may be null when using bundled sidecars.
* `model_package_id` links to `model_packages`.
* The engine adapter resolves actual model paths internally.

---

# 12.4 Table: engine_runtime_state

```sql
CREATE TABLE IF NOT EXISTS engine_runtime_state (
  engine_id TEXT PRIMARY KEY,
  status TEXT NOT NULL,
  pid INTEGER,
  health_url TEXT,
  started_at TEXT,
  stopped_at TEXT,
  last_error TEXT,
  last_exit_code INTEGER,
  updated_at TEXT NOT NULL,

  FOREIGN KEY(engine_id) REFERENCES engine_profiles(id)
);
```

---

# 12.5 Table: log_files

```sql
CREATE TABLE IF NOT EXISTS log_files (
  id TEXT PRIMARY KEY,
  engine_id TEXT NOT NULL,
  log_type TEXT NOT NULL,
  path TEXT NOT NULL,
  created_at TEXT NOT NULL,

  FOREIGN KEY(engine_id) REFERENCES engine_profiles(id)
);
```

Do not expose `path` in Simple Mode.

---

## 13. Engine Profiles

The app should create two default internal engine profiles during machine setup:

```text
language_engine
speech_engine
```

---

# 13.1 llama.cpp Engine Profile

Example:

```json
{
  "id": "language_engine",
  "kind": "llama_cpp",
  "name": "Language Engine",
  "enabled": true,
  "binaryMode": "bundled",
  "binaryName": "llama-server",
  "binaryPath": null,
  "modelPackageId": "default_llm",
  "host": "127.0.0.1",
  "port": 8080,
  "runtime": {
    "ctxSize": 4096,
    "gpuLayers": 99,
    "threads": 8,
    "parallel": 1,
    "metrics": false,
    "apiKey": null
  },
  "extraArgs": [],
  "autoStart": false
}
```

---

# 13.2 sherpa-onnx Engine Profile

Example:

```json
{
  "id": "speech_engine",
  "kind": "sherpa_onnx",
  "name": "Speech Engine",
  "enabled": true,
  "binaryMode": "bundled",
  "binaryName": "sherpa-onnx-vit-server",
  "binaryPath": null,
  "pythonEntrypoint": "sherpa_onnx_vit",
  "pythonLaunchMode": "module",
  "modelPackageId": "default_speech",
  "host": "127.0.0.1",
  "port": 6006,
  "runtime": {
    "serverType": "http",
    "numThreads": 4,
    "provider": "cpu",
    "sampleRate": 16000,
    "argsTemplate": [
      "--host",
      "{host}",
      "--port",
      "{port}",
      "--model-dir",
      "{modelDir}",
      "--tokens",
      "{tokensPath}",
      "--num-threads",
      "{numThreads}"
    ]
  },
  "extraArgs": [],
  "autoStart": false
}
```

The exact sherpa args must be confirmed from the built `sherpa-onnx-vit` server binary.

The implementation must support configurable args templates for sherpa.

---

## 14. Engine Adapter Pattern

Each engine must have its own adapter.

The engine manager must not hardcode CLI flags.

Use this Rust trait:

```rust
pub trait EngineAdapter {
    fn kind(&self) -> EngineKind;

    fn validate_profile(
        &self,
        profile: &EngineProfile,
        model: &ModelPackage,
        app_paths: &AppPaths,
    ) -> Result<(), EngineError>;

    fn build_args(
        &self,
        profile: &EngineProfile,
        model: &ModelPackage,
        app_paths: &AppPaths,
    ) -> Result<Vec<String>, EngineError>;

    fn health_url(
        &self,
        profile: &EngineProfile,
    ) -> String;

    fn display_name(&self) -> &'static str;
}
```

Adapters:

```text
adapters/llama_cpp.rs
adapters/sherpa_onnx.rs
```

---

## 15. llama.cpp Adapter

The llama adapter manages `llama-server`.

Required model:

```text
GGUF model file
```

The model file path is resolved from:

```text
app_data_root + model_packages.relative_path
```

Example internal resolved path:

```text
C:\ProgramData\CompanyName\AppName\models\llama\default\model.gguf
```

Simple Mode must never display that path.

---

# 15.1 llama Runtime Settings

Use runtime JSON:

```json
{
  "ctxSize": 4096,
  "gpuLayers": 99,
  "threads": 8,
  "parallel": 1,
  "metrics": false,
  "apiKey": null
}
```

---

# 15.2 llama Args Builder

Build args:

```text
-m {resolvedModelPath}
--host {host}
--port {port}
-c {ctxSize}
-ngl {gpuLayers}
-t {threads}
-np {parallel}
```

If metrics are enabled:

```text
--metrics
```

If API key is configured:

```text
--api-key {apiKey}
```

Do not expose these args in Simple Mode.

---

# 15.3 llama Validation

Validate:

```text
model file exists
model file extension is .gguf
binary exists or bundled sidecar exists
host is valid
port is valid
port is available
runtime settings are valid
```

Simple Mode error:

```text
Local AI could not start. Please contact support.
```

Developer Mode error:

```text
Missing GGUF model file:
{resolved path}
```

---

## 16. sherpa-onnx Adapter

The sherpa adapter manages the **Python/FastAPI server** from:

```text
https://github.com/VietInnotech/sherpa-onnx-vit.git
```

This is **not a native sidecar binary**. It is a Python package launched via:
- `sherpa-onnx-vit-server` (console script)
- or `python -m sherpa_onnx_vit`

The entrypoint is a FastAPI app using uvicorn, with HTTP APIs and optional WebSocket streaming.

The adapter must use a command argv contract, not a single native binary assumption.

The implementation must not assume too much.

Use a configurable args template.

---

# 16.1 sherpa Model Files

Expected model directory:

```text
models/sherpa/default/
```

The server uses a **model-directory-first** contract.

For the recommended offline_int8 family, the model directory should contain at least:

```text
encoder*.onnx
decoder*.onnx
joiner*.onnx
tokens.txt or config.json
optional: bpe.model
```

The required files must be described in the model manifest.

Example manifest:

```json
{
  "id": "default_speech",
  "kind": "sherpa_onnx",
  "displayName": "Speech Model",
  "internalName": "default_sherpa_model",
  "relativePath": "models/sherpa/default",
  "requiredFiles": [
    "encoder*.onnx",
    "decoder*.onnx",
    "joiner*.onnx",
    "tokens.txt"
  ],
  "required": true
}
```

---

# 16.2 sherpa Runtime Settings

Example runtime JSON:

```json
{
  "serverType": "http",
  "provider": "cpu",
  "postprocessMode": "capu",
  "sttModelFamily": "offline_int8",
  "numThreads": 4,
  "argsTemplate": [
    "--host",
    "{host}",
    "--port",
    "{port}",
    "--provider",
    "{provider}",
    "--stt-model-family",
    "{sttModelFamily}",
    "--model-dir",
    "{modelDir}",
    "--postprocess-mode",
    "{postprocessMode}"
  ]
}
```

---

# 16.3 sherpa Args Template Replacement

The adapter must support placeholders:

```text
{host}
{port}
{modelDir}
{provider}
{sttModelFamily}
{postprocessMode}
{numThreads}
```

Example output:

```text
--host 127.0.0.1
--port 6006
--provider cpu
--stt-model-family offline_int8
--model-dir /Library/Application Support/CompanyName/AppName/models/sherpa/default
--postprocess-mode capu
--num-threads 4
```

Optional advanced overrides for explicit file paths:
```text
{modelEncoder}
{modelDecoder}
{modelJoiner}
{modelTokens}
{modelBpeVocab}
```

Developer Mode may show this.

Simple Mode must not show this.

---

# 16.4 sherpa Validation

Validate:

```text
model directory exists
required files exist
at least one ONNX file exists if required
tokens file exists if required
binary exists or bundled sidecar exists
host is valid
port is valid
port is available
runtime args template is valid
```

Simple Mode error:

```text
Local AI could not start. Please contact support.
```

Developer Mode error:

```text
Missing tokens file:
{resolved path}
```

---

## 17. Process Supervisor

Implement a Rust process supervisor.

Responsibilities:

```text
Start engine process
Stop engine process
Restart engine process
Track PID
Track status
Capture stdout
Capture stderr
Write logs to files
Update SQLite runtime state
Run health checks
Detect crashes
Clean up child processes when app exits
```

---

# 17.1 Startup Flow

```text
1. User clicks Start in Simple Mode.
2. Frontend calls start_local_ai().
3. Backend loads enabled engine profiles.
4. Backend validates all required models and binaries.
5. Backend checks ports.
6. Backend starts each required engine.
7. Backend writes logs to files.
8. Backend polls health checks.
9. Backend updates runtime state.
10. Simple Mode shows Local AI is running if all required engines are healthy.
```

If one required engine fails, Simple Mode shows:

```text
Local AI needs attention.
```

Developer Mode shows which engine failed and why.

---

# 17.2 Stop Flow

```text
1. User clicks Stop.
2. Frontend calls stop_local_ai().
3. Backend stops all managed engines.
4. Backend waits for graceful shutdown.
5. Backend force-kills remaining processes after timeout.
6. Backend updates runtime state.
7. Simple Mode shows Local AI is not running.
```

---

# 17.3 Restart Flow

```text
1. Stop all running engines.
2. Revalidate configuration.
3. Start all required engines.
4. Poll health checks.
5. Return simple aggregate status.
```

---

# 17.4 Crash Detection

If a child process exits unexpectedly:

```text
1. Capture exit code.
2. Store last_exit_code.
3. Store last_error if available.
4. Mark engine as crashed.
5. Mark aggregate Local AI as needs_attention.
6. Keep logs available for Developer Mode.
```

---

## 18. Health Checks

Each engine should have its own health check strategy.

---

# 18.1 llama.cpp Health Check

Try:

```text
GET /health
GET /v1/models
GET /props
GET /
```

If any supported endpoint responds successfully, mark as healthy.

---

# 18.2 sherpa-onnx Health Check

sherpa Option A is **HTTP-first** when using `VietInnotech/sherpa-onnx-vit`.

```text
1. Check process is alive.
2. GET /health
3. GET /v1/models for additional readiness validation.
4. If streaming mode is enabled, optional WebSocket probe for /v1/audio/streaming.
5. If endpoints are not yet ready, use startup grace period before declaring failure.
```

The health strategy should be configurable in runtime JSON.

Example:

```json
{
  "healthCheck": {
    "type": "http",
    "endpoint": "/health",
    "startupGraceMs": 3000,
    "timeoutMs": 10000
  }
}
```

---

## 19. Backend Command Design

Expose separate Simple Mode and Developer Mode commands.

This prevents accidental leakage of sensitive details.

---

# 19.1 Simple Mode Commands

```rust
#[tauri::command]
async fn get_simple_local_ai_status() -> Result<SimpleLocalAiStatusDto, AppError>;

#[tauri::command]
async fn start_local_ai() -> Result<SimpleLocalAiStatusDto, AppError>;

#[tauri::command]
async fn stop_local_ai() -> Result<SimpleLocalAiStatusDto, AppError>;

#[tauri::command]
async fn restart_local_ai() -> Result<SimpleLocalAiStatusDto, AppError>;
```

These commands must not return:

```text
paths
ports
PIDs
raw errors
logs
engine args
binary names
```

---

# 19.2 Developer Mode Commands

```rust
#[tauri::command]
async fn enable_developer_mode_for_session() -> Result<(), AppError>;

#[tauri::command]
async fn disable_developer_mode_for_session() -> Result<(), AppError>;

#[tauri::command]
async fn dev_list_engine_profiles() -> Result<Vec<DeveloperEngineProfileDto>, AppError>;

#[tauri::command]
async fn dev_get_engine_profile(id: String) -> Result<DeveloperEngineProfileDto, AppError>;

#[tauri::command]
async fn dev_update_engine_profile(
    id: String,
    input: UpdateEngineProfileDto,
) -> Result<DeveloperEngineProfileDto, AppError>;

#[tauri::command]
async fn dev_validate_engine_profile(id: String) -> Result<ValidationResultDto, AppError>;

#[tauri::command]
async fn dev_read_engine_log(
    id: String,
    log_type: String,
    tail_lines: Option<u32>,
) -> Result<String, AppError>;

#[tauri::command]
async fn dev_get_diagnostics_bundle() -> Result<DiagnosticsBundleDto, AppError>;

#[tauri::command]
async fn dev_open_logs_folder(id: String) -> Result<(), AppError>;
```

Developer commands must check that Developer Mode is active.

---

## 20. Developer Mode DTOs

Developer DTOs may expose internal details.

```ts
export type DeveloperEngineProfileDto = {
  id: string;
  kind: "llama_cpp" | "sherpa_onnx";
  name: string;
  enabled: boolean;

  binaryMode: "bundled" | "custom";
  binaryName: string;
  binaryPath?: string;
  resolvedBinaryPath?: string;

  modelPackageId?: string;
  resolvedModelPath?: string;
  resolvedModelDir?: string;
  resolvedTokensPath?: string;

  host: string;
  port: number;
  healthUrl?: string;

  runtime: Record<string, unknown>;
  extraArgs: string[];

  status: string;
  pid?: number;
  lastError?: string;
  lastExitCode?: number;
};
```

Simple Mode must not use this DTO.

---

## 21. Error Handling

Use structured errors internally.

```ts
export type AppError = {
  code: string;
  message: string;
  details?: unknown;
};
```

Internal error codes:

```text
ENGINE_NOT_FOUND
INVALID_ENGINE_KIND
MISSING_BINARY
MISSING_MODEL
INVALID_MODEL_PATH
INVALID_MODEL_DIR
INVALID_TOKENS_PATH
PORT_IN_USE
PROCESS_START_FAILED
PROCESS_STOP_FAILED
HEALTH_CHECK_FAILED
DATABASE_ERROR
CONFIG_PARSE_ERROR
DEVELOPER_MODE_REQUIRED
UNKNOWN_ERROR
```

---

# 21.1 Simple Error Mapping

Map technical errors to safe messages.

## Missing Model

Internal:

```text
Missing model file:
C:\ProgramData\CompanyName\AppName\models\llama\default\model.gguf
```

Simple Mode:

```text
Required AI files are missing. Please contact support.
```

## Missing Binary

Internal:

```text
Missing sidecar binary:
llama-server-x86_64-pc-windows-msvc.exe
```

Simple Mode:

```text
The AI engine installation is incomplete. Please contact support.
```

## Port Conflict

Internal:

```text
Port 8080 is already in use on 127.0.0.1.
```

Simple Mode:

```text
Local AI could not start because another service is already running.
```

## Invalid Config

Internal:

```text
Invalid sherpa args template: missing {modelDir}
```

Simple Mode:

```text
Local AI is not configured correctly. Please contact support.
```

---

## 22. Simple UI Design

The Simple UI should be minimal.

Use one centered card.

Example:

```text
------------------------------------------------
|                                              |
|                 App Logo                     |
|                                              |
|            Local AI is ready                 |
|                                              |
|        The local AI service is available.    |
|                                              |
|                 [ Start ]                    |
|                                              |
------------------------------------------------
```

When running:

```text
------------------------------------------------
|                                              |
|                 App Logo                     |
|                                              |
|           Local AI is running                |
|                                              |
|        The local AI service is available.    |
|                                              |
|                  [ Stop ]                    |
|                                              |
------------------------------------------------
```

When error:

```text
------------------------------------------------
|                                              |
|                 App Logo                     |
|                                              |
|          Local AI needs attention            |
|                                              |
|       Local AI could not start.              |
|       Please contact support.                |
|                                              |
|                [ Restart ]                   |
|                                              |
------------------------------------------------
```

No tabs.

No sidebar.

No model selector.

No settings gear in Simple Mode.

No engine names.

No log viewer.

No port display.

No path display.

Keep the whole thing boring and hard to misuse. A tragically rare design philosophy.

---

## 23. Developer Mode UI

Once Developer Mode is enabled, reveal a developer navigation area.

Developer pages:

```text
Developer Dashboard
Engines
Models
Logs
Settings
Diagnostics
```

---

# 23.1 Developer Dashboard

Show:

```text
Aggregate Local AI status
Language Engine status
Speech Engine status
Engine health
Last startup result
Last crash
Machine configuration status
```

---

# 23.2 Developer Engines Page

Show table:

```text
Engine ID
Kind
Name
Enabled
Status
PID
Host
Port
Binary Mode
Model Package
Actions
```

Actions:

```text
Start
Stop
Restart
Validate
View logs
Edit profile
```

---

# 23.3 Developer Engine Detail Page

Show:

```text
Engine ID
Kind
Name
Binary name
Binary mode
Binary path
Resolved binary path
Model package ID
Resolved model path
Resolved model directory
Resolved tokens path
Host
Port
Runtime JSON
Extra args
Generated CLI args
Health URL
PID
Last error
Last exit code
```

Allow editing:

```text
enabled
binary mode
binary path
host
port
runtime JSON
extra args
auto start
```

Editing model paths should be done carefully.

Preferred pattern:

* Edit model package records instead of editing raw paths inside engine profiles.

---

# 23.4 Developer Models Page

Show:

```text
Model package ID
Kind
Display name
Internal name
Relative path
Resolved path
Installed
Verified
Last verified
Required files
```

Actions:

```text
Validate model
Recalculate checksum
Open containing folder
```

Do not allow normal users here.

---

# 23.5 Developer Logs Page

Show:

```text
Engine selector
stdout
stderr
launcher logs
health check logs
copy logs
export logs
open logs folder
```

Support tailing latest logs.

---

# 23.6 Developer Diagnostics Page

Show:

```text
App version
OS
Architecture
App data root
SQLite path
Engine binaries
Binary checksums
Model packages
Model validation
Port status
Runtime states
Recent crashes
```

Add button:

```text
Export diagnostics bundle
```

Diagnostics bundle should include:

```text
sanitized config
runtime state
recent logs
validation report
OS and architecture
engine versions if available
```

Avoid including large model files.

---

## 24. Developer Mode Activation

Implement logo click activation.

Svelte logic:

```ts
import { writable } from "svelte/store";
import { invoke } from "@tauri-apps/api/core";

export const developerMode = writable(false);

let logoClickCount = 0;
let firstClickAt = 0;

export async function handleLogoClick() {
  const now = Date.now();

  if (!firstClickAt || now - firstClickAt > 5000) {
    firstClickAt = now;
    logoClickCount = 1;
    return;
  }

  logoClickCount += 1;

  if (logoClickCount >= 7) {
    await invoke("enable_developer_mode_for_session");
    developerMode.set(true);

    logoClickCount = 0;
    firstClickAt = 0;
  }
}
```

Developer Mode should be disabled again when the app restarts unless persistence is explicitly enabled by a developer.

---

## 25. Logs

Logs should be stored in files.

Recommended structure:

```text
logs/
  launcher.log
  engines/
    language_engine/
      stdout.log
      stderr.log
    speech_engine/
      stdout.log
      stderr.log
```

Simple Mode must not show logs.

Developer Mode may show logs.

---

# 25.1 Log Rotation

Implement simple log rotation:

```text
Max file size: 10 MB
Keep last: 5 files per log type
```

Example:

```text
stdout.log
stdout.1.log
stdout.2.log
stderr.log
stderr.1.log
stderr.2.log
```

---

# 25.2 Log Privacy

Avoid logging sensitive user data if possible.

Do not log full prompts or audio contents.

Paths may appear in Developer Mode logs, but they must not be shown in Simple Mode.

---

## 26. Port Management

Ports are configured by developers.

Default ports:

```text
llama.cpp: 8080
sherpa-onnx: 6006
```

Use localhost only:

```text
127.0.0.1
```

Simple Mode must not expose ports.

Developer Mode may expose and edit ports.

The backend should check if a port is available before starting an engine.

If a port is busy:

Simple Mode:

```text
Local AI could not start because another service is already running.
```

Developer Mode:

```text
Port 8080 is already in use on 127.0.0.1.
```

Do not bind to:

```text
0.0.0.0
```

unless explicitly configured in Developer Mode.

---

## 27. Security and Information Hiding

The primary security rule:

```text
Normal users must not see model locations or engine internals.
```

Important:

Developer Mode hiding is not a complete security boundary.

A local administrator can still inspect files.

The goal is to prevent accidental exposure, casual tampering, and unnecessary user confusion.

---

# 27.1 Backend Data Separation

Do not rely on frontend-only hiding.

Simple Mode commands must not return sensitive fields.

Bad:

```json
{
  "status": "needs_attention",
  "modelPath": "C:\\ProgramData\\CompanyName\\AppName\\models\\llama\\default\\model.gguf"
}
```

Good:

```json
{
  "status": "needs_attention",
  "message": "Required AI files are missing. Please contact support."
}
```

---

# 27.2 No Raw Shell Commands

Never construct shell strings.

Bad:

```rust
Command::new("sh")
  .arg("-c")
  .arg(format!("{} {}", binary, args.join(" ")));
```

Good:

```rust
Command::new(binary)
  .args(args);
```

Arguments must be passed as arrays.

No shell interpolation.

No `eval`.

No `cmd /c`.

---

# 27.3 File Permissions

For machine-wide installs, use restricted directories where possible.

Recommended:

Windows:

```text
C:\ProgramData\CompanyName\AppName\
```

macOS:

```text
/Library/Application Support/CompanyName/AppName/
```

Developers should set file permissions so normal users cannot casually modify model files or engine binaries.

---

## 28. Packaging

Support:

```text
Windows x64
macOS Apple Silicon
macOS Intel
```

---

# 28.1 Windows

Artifacts:

```text
.exe installer
.msi optional
portable zip optional for developer use
```

Requirements:

```text
Sign app if possible
Sign bundled sidecar binaries if possible
Install app data directory
Do not expose model directory in UI
```

Machine setup may be separate from app install.

---

# 28.2 macOS

Artifacts:

```text
.app
.dmg
```

Requirements:

```text
Sign app
Sign sidecar binaries
Hardened runtime
Notarization
Stapling
```

Machine setup may be separate from app install.

---

## 29. Sidecar Binary Strategy

### llama.cpp

Preferred production strategy:

```text
Bundle engine binaries with the app.
```

### sherpa-onnx-vit

**Not a native sidecar**. Requires internal Python runtime packaging.

Choose one of these strategies for sherpa:

```text
1. Developer-managed Python install (for now)
   - Machine setup scripts install Python + dependencies
   - Launcher calls sherpa-onnx-vit-server or python -m sherpa_onnx_vit

2. Packaged internal Python runtime/artifact per OS
   - Bundle a pinned Python environment/artifact
   - Launcher calls the bundled runtime directly
   - See scripts/prepare-sherpa-runtime.sh
```

---

### Shared strategy

Sidecar binaries (for llama) should live in:

```text
src-tauri/binaries/
```

Tauri target triple examples for llama:

```text
llama-server-aarch64-apple-darwin
llama-server-x86_64-apple-darwin
llama-server-x86_64-pc-windows-msvc.exe
```

The app should support:

```text
binary_mode = bundled
binary_mode = custom
```

In `custom` mode, `binary_path` must be set by a developer.

Simple Mode must not expose either option.

---

## 30. Internal Python Runtime Packaging for sherpa-onnx-vit

Since sherpa-onnx-vit is a Python/FastAPI package, not a native binary, the app must either:

### Option A1: Developer-Managed Python (MVP)

```text
Machine setup scripts:
- Install Python 3.10-3.14
- Install uv (or use venv)
- Run: uv pip install -e '.[dev]' huggingface_hub
- Or: pip install sherpa-onnx-vit and dependencies
```

Launcher calls:
```bash
uv run sherpa-onnx-vit-server --host 127.0.0.1 --port {port} ...
# or
python -m sherpa_onnx_vit --host 127.0.0.1 --port {port} ...
```

### Option A2: Packaged Internal Python Runtime (Release)

Create per-target artifacts:

```text
scripts/prepare-sherpa-runtime.sh
scripts/prepare-sherpa-runtime.ps1
```

Each script should:
1. Pin Python version (e.g., 3.11)
2. Pin sherpa-onnx-vit commit
3. Create isolated Python environment
4. Install dependencies from pyproject.toml
5. Produce artifact: `runtimes/sherpa-onnx-vit/<target>/bin/sherpa-onnx-vit-server`

Tauri bundling:
```json
{
  "tauri": {
    "bundle": {
      "externalBin": [
        "runtimes/sherpa-onnx-vit/aarch64-apple-darwin/bin/sherpa-onnx-vit-server",
        "runtimes/sherpa-onnx-vit/x86_64-apple-darwin/bin/sherpa-onnx-vit-server",
        "runtimes/sherpa-onnx-vit/x86_64-pc-windows-msvc/bin/sherpa-onnx-vit-server.exe"
      ]
    }
  }
}
```

Launcher calls the bundled runtime directly.

### Runtime Validation in Machine Setup

```text
AI Engine Launcher Machine Setup

sherpa runtime:
  Developer-managed Python (uv/pip)
  - or -
  Packaged runtime: OK (aarch64-apple-darwin)
```

---

## 31. Machine Initialization Scripts

Create:

```text
scripts/initialize-machine-config.sh
scripts/initialize-machine-config.ps1
```

These scripts should prepare a machine.

---

# 30.1 Script Responsibilities

```text
Create app data root
Create models directory
Create logs directory
Create config directory
Verify llama model file exists
Verify sherpa model directory exists
Verify sherpa required files exist
Create or update model manifest
Create or update SQLite database
Insert default model packages
Insert default engine profiles
Validate ports
Print setup report
```

---

# 30.2 Script Output Example

```text
AI Engine Launcher Machine Setup

App data root:
C:\ProgramData\CompanyName\AppName

Language model:
OK

Speech model:
OK

llama.cpp binary:
Bundled

sherpa runtime:
Developer-managed Python
(or "Packaged Python artifact" when using internal runtime)

Ports:
8080 available
6006 available

SQLite:
Initialized

Result:
Machine is ready.
```

If failed:

```text
Result:
Machine setup incomplete.

Missing:
C:\ProgramData\CompanyName\AppName\models\llama\default\model.gguf
```

This output is developer-facing only.

---

## 31. App Startup Behavior

On startup:

```text
1. Initialize app paths.
2. Open SQLite database.
3. Load app settings.
4. Check if machine_configured is true.
5. Reconcile stale runtime states.
6. Check whether previously running PIDs still exist.
7. Set Simple Mode status.
8. Render Simple UI.
```

If machine is not configured:

Simple Mode:

```text
Local AI is not ready. Please contact support.
```

Developer Mode:

```text
Machine configuration is incomplete.
Missing model package records.
```

---

## 32. Auto-Start Behavior

Support optional auto-start.

Developer Mode setting:

```text
Start Local AI when app opens
```

Stored in SQLite:

```text
auto_start_local_ai
```

Default:

```text
false
```

If enabled:

```text
1. App starts.
2. Backend validates configuration.
3. Backend starts required engines.
4. Simple Mode shows starting/running.
```

If auto-start fails, Simple Mode shows:

```text
Local AI needs attention.
```

Developer Mode shows details.

---

## 33. Stop-on-Exit Behavior

Default:

```text
Stop engines when app exits = true
```

This avoids orphaned local servers.

Developer Mode may allow disabling this for debugging.

Stored setting:

```text
stop_engines_on_exit
```

---

## 34. Acceptance Criteria

The MVP is complete when:

```text
[ ] App launches on Windows
[ ] App launches on macOS
[ ] App opens in Simple Mode by default
[ ] Simple Mode shows one Local AI status
[ ] Simple Mode does not show engine names
[ ] Simple Mode does not show model paths
[ ] Simple Mode does not show binary paths
[ ] Simple Mode does not show ports
[ ] Simple Mode does not show logs
[ ] Simple Mode can start Local AI
[ ] Simple Mode can stop Local AI
[ ] Simple Mode can restart Local AI
[ ] Developer Mode activates by clicking logo 7 times
[ ] Developer Mode shows engine profiles
[ ] Developer Mode shows model package records
[ ] Developer Mode shows resolved paths
[ ] Developer Mode shows logs
[ ] Developer Mode shows diagnostics
[ ] Machine setup scripts create required config
[ ] SQLite stores engine profiles
[ ] SQLite stores model package references
[ ] llama.cpp starts from configured model path
[ ] sherpa-onnx starts from configured model directory
[ ] App detects missing model files
[ ] App detects missing binaries
[ ] App detects port conflicts
[ ] App maps technical errors to safe Simple Mode messages
[ ] App cleans up child processes on exit
```

---

## 35. Non-MVP Features

Do not build these in MVP:

```text
Model download
Model marketplace
User model selection
Cloud sync
Remote server management
Multi-user auth
Benchmark dashboard
Plugin system
Auto-updater
Prompt history
Chat interface
Audio recording UI
Advanced GPU tuner
```

The MVP is only a local engine launcher.

---

## 36. Implementation Milestones

---

# Milestone 1: Svelte + Tauri Skeleton

Tasks:

```text
Create Svelte project
Add Tailwind
Add Tauri 2
Create Simple Mode home screen
Add logo click detection
Add Developer Mode store
```

Acceptance:

```text
App launches
Simple screen displays
Logo click enables Developer Mode
```

---

# Milestone 2: SQLite and App Paths

Tasks:

```text
Implement app path resolver
Initialize SQLite
Create migrations
Create app_settings table
Create model_packages table
Create engine_profiles table
Create runtime state table
```

Acceptance:

```text
Database is created
Migrations run
App can load settings
```

---

# Milestone 3: Machine Setup Script

Tasks:

```text
Create Windows setup script
Create macOS setup script
Create app data folders
Insert default model packages
Insert default engine profiles
Validate manually placed model files
```

Acceptance:

```text
Developer can prepare a machine
Database has default profiles
Models are validated
```

---

# Milestone 4: llama.cpp Launch

Tasks:

```text
Implement llama adapter
Resolve model path from model package
Build llama-server args
Spawn process
Write stdout/stderr logs
Run health check
Update runtime state
```

Acceptance:

```text
llama-server starts
llama-server stops
Developer Mode shows logs
Simple Mode shows Local AI running
```

---

# Milestone 5: sherpa-onnx Launch

Tasks:

```text
Build or configure sherpa-onnx-vit server binary
Implement sherpa adapter
Resolve model directory
Resolve tokens file
Apply args template
Spawn process
Run health check
Update runtime state
```

Acceptance:

```text
sherpa server starts
sherpa server stops
Developer Mode shows logs
Simple Mode shows aggregate status
```

---

# Milestone 6: Aggregate Local AI Control

Tasks:

```text
Implement start_local_ai
Implement stop_local_ai
Implement restart_local_ai
Aggregate engine statuses
Map internal errors to Simple Mode messages
```

Acceptance:

```text
Simple Mode controls both engines
Simple Mode never leaks internal details
```

---

# Milestone 7: Developer Diagnostics

Tasks:

```text
Developer dashboard
Engine table
Model table
Log viewer
Diagnostics report
Validation commands
```

Acceptance:

```text
Developer can debug failed machines
Normal user cannot see developer info
```

---

# Milestone 8: Packaging

Tasks:

```text
Configure Tauri sidecars
Package Windows app
Package macOS app
Test installed app
Verify app data directory
Verify manually placed models
Verify engine startup
```

Acceptance:

```text
Packaged app works on configured machines
```

---

## 37. Final Guidance for the AI Agent

Build this as a simple user-facing launcher with a hidden developer console.

The normal user should experience exactly one thing:

```text
Local AI is ready.
```

The developer should be able to reveal the machinery:

```text
language_engine:
  llama-server
  model: C:\ProgramData\CompanyName\AppName\models\llama\default\model.gguf
  port: 8080
  status: running

speech_engine:
  sherpa-onnx-vit-server (Python/FastAPI)
  model dir: C:\ProgramData\CompanyName\AppName\models\sherpa\default
  port: 6006
  status: running
```

Do not build user-facing model selection.

Do not build model downloading.

Do not expose model directories in Simple Mode.

Do not leak paths through Simple Mode DTOs.

Do not rely on CSS hiding for sensitive information.

Use backend command separation:

```text
Simple commands return safe aggregate status.
Developer commands return internal details only after Developer Mode is enabled.
```

The app must be boring, reliable, and difficult for normal users to break.

That is the product.

```
```
