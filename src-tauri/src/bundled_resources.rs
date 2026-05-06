use crate::app_paths::AppPaths;
use crate::errors::{AppError, AppResult};
use std::fs;
use std::path::{Path, PathBuf};

const BUNDLED_DIRS: &[&str] = &["binaries", "runtime"];

pub fn sync_bundled_resources(paths: &AppPaths, resource_dir: &Path) -> AppResult<()> {
    for dir in BUNDLED_DIRS {
        let source = resource_dir.join(dir);
        if source.exists() {
            copy_dir_contents(&source, &paths.app_root.join(dir))?;
        }
    }

    Ok(())
}

fn copy_dir_contents(source: &Path, destination: &Path) -> AppResult<()> {
    fs::create_dir_all(destination)?;

    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());
        let metadata = entry.metadata()?;

        if metadata.is_dir() {
            copy_dir_contents(&source_path, &destination_path)?;
        } else if metadata.is_file() {
            if should_copy_file(&source_path, &destination_path)? {
                if let Some(parent) = destination_path.parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::copy(&source_path, &destination_path)?;
            }
            set_runtime_permissions(&destination_path)?;
        }
    }

    Ok(())
}

fn should_copy_file(source: &Path, destination: &Path) -> AppResult<bool> {
    if !destination.exists() {
        return Ok(true);
    }

    let source_metadata = fs::metadata(source)?;
    let destination_metadata = fs::metadata(destination)?;

    if source_metadata.len() != destination_metadata.len() {
        return Ok(true);
    }

    let source_modified = source_metadata.modified().ok();
    let destination_modified = destination_metadata.modified().ok();

    Ok(source_modified
        .zip(destination_modified)
        .is_some_and(|(source, destination)| source > destination))
}

#[cfg(unix)]
fn set_runtime_permissions(path: &Path) -> AppResult<()> {
    use std::os::unix::fs::PermissionsExt;

    if is_runtime_executable(path) {
        let mut permissions = fs::metadata(path)?.permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(path, permissions)?;
    }

    Ok(())
}

#[cfg(not(unix))]
fn set_runtime_permissions(_path: &Path) -> AppResult<()> {
    Ok(())
}

fn is_runtime_executable(path: &Path) -> bool {
    let Some(name) = path.file_name().and_then(|value| value.to_str()) else {
        return false;
    };

    name == "python3" || name == "python.exe" || name.starts_with("llama-server")
}

pub fn resource_dir_error(error: tauri::Error) -> AppError {
    AppError::with_details(
        "BUNDLED_RESOURCE_ERROR",
        error.to_string(),
        serde_json::json!({"kind": "tauri_resource_dir"}),
    )
}

pub fn resource_dir_for_dev() -> Option<PathBuf> {
    let candidate = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("bundle");
    candidate.exists().then_some(candidate)
}
