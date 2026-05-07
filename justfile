# Local AI launcher task runner.
# Keep recipes aligned with verified commands and docs in this repo.

prepare_sidecars_cmd := if os_family() == "windows" { "powershell -NoProfile -ExecutionPolicy Bypass -File scripts/prepare-sidecars.ps1" } else { "bash scripts/prepare-sidecars.sh" }

initialize_machine_cmd := if os_family() == "windows" { "powershell -NoProfile -ExecutionPolicy Bypass -File scripts/initialize-machine-config.ps1" } else { "bash scripts/initialize-machine-config.sh" }

default:
    @just --list --unsorted

help:
    @just --list --unsorted

# Toolchain and status
install:
    bun install --frozen-lockfile

version:
    @node --input-type=module -e "import fs from 'node:fs'; const pkg = JSON.parse(fs.readFileSync('package.json', 'utf8')); const tauri = JSON.parse(fs.readFileSync('src-tauri/tauri.conf.json', 'utf8')); const cargoMatch = fs.readFileSync('src-tauri/Cargo.toml', 'utf8').match(/^version\s*=\s*\"([^\"]+)\"/m); const cargo = cargoMatch ? cargoMatch[1] : '<missing>'; console.log('package.json: ' + pkg.version); console.log('src-tauri/Cargo.toml: ' + cargo); console.log('src-tauri/tauri.conf.json: ' + tauri.version);"

version-check:
    @node --input-type=module -e "import fs from 'node:fs'; const pkg = JSON.parse(fs.readFileSync('package.json', 'utf8')); const tauri = JSON.parse(fs.readFileSync('src-tauri/tauri.conf.json', 'utf8')); const cargoMatch = fs.readFileSync('src-tauri/Cargo.toml', 'utf8').match(/^version\s*=\s*\"([^\"]+)\"/m); if (!cargoMatch) { console.error('Could not read version from src-tauri/Cargo.toml'); process.exit(1); } const versions = [['package.json', pkg.version], ['src-tauri/Cargo.toml', cargoMatch[1]], ['src-tauri/tauri.conf.json', tauri.version]]; for (const [name, value] of versions) console.log(name + ': ' + value); if (new Set(versions.map(([, value]) => value)).size !== 1) { console.error('\nVersion mismatch detected.'); process.exit(1); } console.log('\nVersions are in sync.');"

set-version version:
    node --input-type=module -e "import fs from 'node:fs'; const version = '{{ version }}'; if (!/^\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?(?:\+[0-9A-Za-z.-]+)?$/.test(version)) { console.error('Expected semver version like 0.1.1 or 0.2.0-beta.1'); process.exit(1); } const pkgPath = 'package.json'; const cargoPath = 'src-tauri/Cargo.toml'; const tauriPath = 'src-tauri/tauri.conf.json'; const pkg = JSON.parse(fs.readFileSync(pkgPath, 'utf8')); pkg.version = version; fs.writeFileSync(pkgPath, JSON.stringify(pkg, null, 2) + '\n'); const cargo = fs.readFileSync(cargoPath, 'utf8'); const nextCargo = cargo.replace(/^version\s*=\s*\"[^\"]+\"/m, 'version = \"' + version + '\"'); if (nextCargo === cargo) { console.error('Could not update version in src-tauri/Cargo.toml'); process.exit(1); } fs.writeFileSync(cargoPath, nextCargo); const tauri = JSON.parse(fs.readFileSync(tauriPath, 'utf8')); tauri.version = version; fs.writeFileSync(tauriPath, JSON.stringify(tauri, null, 2) + '\n'); console.log('Updated package.json, src-tauri/Cargo.toml, and src-tauri/tauri.conf.json to version ' + version);"

tag-check version:
    @node --input-type=module -e "import fs from 'node:fs'; const version = '{{ version }}'; const pkg = JSON.parse(fs.readFileSync('package.json', 'utf8')); if (pkg.version !== version) { console.error('package.json version (' + pkg.version + ') does not match requested tag version (' + version + '). Run just set-version ' + version + ' first if needed.'); process.exit(1); }"
    just version-check
    @if [ "$(git rev-parse --abbrev-ref HEAD)" = "HEAD" ]; then printf 'Cannot tag from a detached HEAD.\n' >&2; exit 1; fi
    @if [ -n "$(git status --porcelain)" ]; then printf 'Working tree is not clean.\n' >&2; git status --short >&2; exit 1; fi
    @if git rev-parse --verify --quiet "refs/tags/v{{ version }}" >/dev/null; then printf 'Tag v{{ version }} already exists locally.\n' >&2; exit 1; fi
    @if git remote get-url origin >/dev/null 2>&1 && git ls-remote --exit-code --tags origin "refs/tags/v{{ version }}" >/dev/null 2>&1; then printf 'Tag v{{ version }} already exists on origin.\n' >&2; exit 1; fi
    @printf 'Tag checks passed for v{{ version }}.\n'

tag-release version:
    just tag-check "{{ version }}"
    git tag -a "v{{ version }}" -m "Local AI v{{ version }}"
    @printf 'Created local tag v{{ version }}.\nReview it with: git show v{{ version }}\nPush when ready:\n  git push origin HEAD\n  git push origin v{{ version }}\n'

# Development
dev:
    bun run dev

tauri-dev:
    bun run tauri dev

# Validation and tests
check-frontend:
    bun run check

check-backend:
    cargo check --manifest-path src-tauri/Cargo.toml

check: check-frontend check-backend

test-backend:
    cargo test --manifest-path src-tauri/Cargo.toml

test: test-backend

validate-bundle:
    bun run validate:bundle-artifacts

preflight: install version-check check build-frontend test-backend

release-check: install version-check check build-frontend test-backend validate-bundle

# Builds
build-frontend:
    bun run build

build-desktop:
    bun run tauri build

build-windows:
    bun run tauri build --runner cargo-xwin --target x86_64-pc-windows-msvc --no-bundle

# Repo helper scripts
prepare-sidecars:
    {{ prepare_sidecars_cmd }} --dry-run

prepare-sidecars-apply:
    {{ prepare_sidecars_cmd }} --apply

init-machine:
    {{ initialize_machine_cmd }} --dry-run

init-machine-apply:
    {{ initialize_machine_cmd }} --apply

# Cleanup
clean-frontend:
    node --input-type=module -e "import fs from 'node:fs'; fs.rmSync('dist', { recursive: true, force: true }); console.log('Removed dist/');"

clean-backend:
    cargo clean --manifest-path src-tauri/Cargo.toml

clean: clean-frontend clean-backend
