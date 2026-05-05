use crate::errors::{AppError, AppResult};
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone)]
pub struct EngineLogPaths {
    pub stdout_path: PathBuf,
    pub stderr_path: PathBuf,
}

pub fn engine_log_paths(log_root: &Path, id: &str) -> AppResult<EngineLogPaths> {
    let dir = log_root.join("engines").join(id);
    fs::create_dir_all(&dir)?;
    Ok(EngineLogPaths {
        stdout_path: dir.join("stdout.log"),
        stderr_path: dir.join("stderr.log"),
    })
}

pub fn open_log_files(paths: &EngineLogPaths) -> AppResult<(File, File)> {
    let stdout = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&paths.stdout_path)?;
    let stderr = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&paths.stderr_path)?;
    Ok((stdout, stderr))
}

pub fn tail_file(path: &Path, lines: usize) -> AppResult<String> {
    if !path.exists() {
        return Ok(String::new());
    }

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut all = Vec::new();
    for line in reader.lines() {
        all.push(line?);
    }

    let start = all.len().saturating_sub(lines);
    Ok(all[start..].join("\n"))
}

pub fn rotate_if_large(path: &Path, max_bytes: u64, max_files: usize) -> AppResult<()> {
    if !path.exists() || path.metadata()?.len() < max_bytes {
        return Ok(());
    }

    for index in (1..=max_files).rev() {
        let source = if index == 1 {
            path.to_path_buf()
        } else {
            rotated_path(path, index - 1)
        };
        let destination = rotated_path(path, index);
        if source.exists() {
            let _ = fs::rename(&source, &destination);
        }
    }

    File::create(path)?;
    Ok(())
}

pub fn write_log_line(path: &Path, line: &str) -> AppResult<()> {
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    writeln!(file, "{line}")?;
    Ok(())
}

pub fn open_in_file_manager(path: &Path) -> AppResult<()> {
    let status = if cfg!(target_os = "macos") {
        Command::new("open").arg(path).status()?
    } else if cfg!(target_os = "windows") {
        Command::new("explorer").arg(path).status()?
    } else {
        Command::new("xdg-open").arg(path).status()?
    };

    if status.success() {
        Ok(())
    } else {
        Err(AppError::new(
            "UNKNOWN_ERROR",
            "Unable to open the logs folder.",
        ))
    }
}

fn rotated_path(path: &Path, index: usize) -> PathBuf {
    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("log");
    let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("log");
    path.with_file_name(format!("{stem}.{index}.{ext}"))
}
