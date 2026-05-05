# AGENTS.md

## Current repo state
- The repo is still greenfield: there are no manifests, lockfiles, source directories, CI workflows, or repo-local tool configs yet.
- `plan.md` is now the main source of truth for intended product scope and architecture, but it is still a plan, not proof that code or commands exist.

## Working rules for this repo
- There are no verified setup/dev/build/test/lint/typecheck commands yet; do not guess a stack, package manager, or framework.
- Treat the project as greenfield until real project files are added.
- On non-trivial tasks, bias toward caution over speed.

## Default agent behavior
- Think before coding: state assumptions, surface ambiguity and tradeoffs, and ask instead of guessing.
- Simplicity first: implement the smallest thing that solves the request; avoid speculative abstractions or configurability.
- Surgical changes: touch only what the request requires; do not refactor or clean unrelated code, comments, or formatting.
- Goal-driven execution: define a concrete verification target before implementing and verify it when possible.

## Planned stack and architecture from `plan.md`
- Intended stack: Svelte + TypeScript + Tailwind on the frontend, Tauri 2 + Rust on the backend, and SQLite for persistence.
- Planned boundary: `src/` for Svelte UI, `src-tauri/` for Rust/Tauri code, `src-tauri/binaries/` for sidecars, and `scripts/` for machine setup/build helpers.
- The product is a cross-platform desktop launcher for local AI that internally manages `llama.cpp` (`llama-server`) and `sherpa-onnx`, but presents them as a single service named `Local AI`.

## Product constraints to preserve
- Default UX is Simple Mode: one aggregate `Local AI` status with Start/Stop/Restart and safe generic messaging only.
- Simple Mode must not expose engine names, model paths, binary paths, ports, logs, PIDs, CLI args, raw errors, or database paths.
- Developer Mode is intentionally hidden; `plan.md` specifies activation via 7 logo clicks within 5 seconds and session-only by default.
- Do not build user-facing model download, model marketplace, or model selection flows. Model files are manually placed by developers or operators per machine.
- Treat machine setup as developer-managed, not self-serve end-user onboarding.

## Backend and security constraints from the plan
- Keep Simple Mode and Developer Mode commands separate. Simple commands return safe aggregate DTOs only; developer commands may expose internals only after Developer Mode is enabled.
- Do not rely on frontend-only hiding for sensitive information.
- When spawning processes in Rust, do not build shell command strings; pass the binary and args separately (no `sh -c`, `cmd /c`, `eval`, or interpolated command strings).
- Planned defaults are localhost-only bindings (`127.0.0.1`), with ports hidden from Simple Mode.

## Planned deployment and storage conventions
- Production should prefer bundled sidecar binaries; custom binary paths are for developer or field setup.
- Models, config, and logs are expected under one app data root managed per machine.
- Large logs should stay in files, not SQLite.
- Machine preparation is expected to happen through `scripts/initialize-machine-config.sh` and `scripts/initialize-machine-config.ps1` once those scripts exist.

## Delivery priorities from `plan.md`
- Follow the milestone order in the plan unless the repo later establishes a different executable workflow: app skeleton → SQLite/app paths → machine setup scripts → llama adapter → sherpa adapter → aggregate control → developer diagnostics → packaging.
- Non-MVP features include model download, marketplace, chat UI, cloud sync, remote server management, plugin system, auto-updater, and advanced GPU tuning.

## When the project is initialized
- Rebuild this file from executable sources of truth first: root manifests, lockfiles, task runner config, CI, and any repo-local instruction files.
