use crate::errors::{AppError, AppResult};
use crate::logs;
use crate::models::{ProcessSnapshotDto, ProcessStatus};
use crate::process_registry::ProcessRegistry;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Clone)]
pub struct LaunchSpec {
    pub id: String,
    pub binary_path: std::path::PathBuf,
    pub args: Vec<String>,
    pub env: Vec<(String, String)>,
    pub log_root: std::path::PathBuf,
}

#[derive(Clone)]
pub struct ProcessSupervisor {
    registry: Arc<Mutex<ProcessRegistry>>,
}

impl ProcessSupervisor {
    pub fn new() -> Self {
        Self {
            registry: Arc::new(Mutex::new(ProcessRegistry::default())),
        }
    }

    pub fn spawn(&self, spec: LaunchSpec) -> AppResult<ProcessSnapshotDto> {
        let log_paths = logs::engine_log_paths(&spec.log_root, &spec.id)?;
        logs::rotate_if_large(&log_paths.stdout_path, 10 * 1024 * 1024, 5)?;
        logs::rotate_if_large(&log_paths.stderr_path, 10 * 1024 * 1024, 5)?;

        let stdout = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_paths.stdout_path)?;
        let stderr = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_paths.stderr_path)?;

        let mut command = Command::new(&spec.binary_path);
        command
            .args(spec.args)
            .envs(spec.env)
            .stdin(Stdio::null())
            .stdout(Stdio::from(stdout))
            .stderr(Stdio::from(stderr));

        let child = command.spawn().map_err(|error| {
            AppError::with_details(
                "PROCESS_START_FAILED",
                error.to_string(),
                serde_json::json!({"id": spec.id, "binaryPath": spec.binary_path}),
            )
        })?;

        let snapshot = ProcessSnapshotDto {
            id: spec.id.clone(),
            status: ProcessStatus::Running,
            pid: Some(child.id()),
            last_error: None,
            last_exit_code: None,
        };

        self.registry.lock().unwrap().upsert_running(spec.id, child);
        Ok(snapshot)
    }

    pub fn stop(&self, id: &str) -> AppResult<Option<ProcessSnapshotDto>> {
        let maybe_snapshot = self.registry.lock().unwrap().with_child_mut(id, |child, record| {
            record.status = ProcessStatus::Stopping;
            let terminate_attempt = child.kill();
            let start = Instant::now();
            loop {
                match child.try_wait() {
                    Ok(Some(status)) => {
                        record.last_exit_code = status.code();
                        record.last_error = terminate_attempt.as_ref().err().map(|error| error.to_string());
                        record.status = ProcessStatus::Stopped;
                        record.pid = None;
                        record.child = None;
                        return record.snapshot();
                    }
                    Ok(None) if start.elapsed() < Duration::from_secs(2) => {
                        thread::sleep(Duration::from_millis(100));
                    }
                    Ok(None) => {
                        let _ = child.kill();
                        let status = child.wait().ok();
                        record.last_exit_code = status.and_then(|exit| exit.code());
                        record.status = ProcessStatus::Stopped;
                        record.pid = None;
                        record.child = None;
                        return record.snapshot();
                    }
                    Err(error) => {
                        record.status = ProcessStatus::Crashed;
                        record.last_error = Some(error.to_string());
                        record.pid = None;
                        record.child = None;
                        return record.snapshot();
                    }
                }
            }
        });

        Ok(maybe_snapshot)
    }

    pub fn stop_all(&self) -> AppResult<Vec<ProcessSnapshotDto>> {
        let ids = self.snapshots().into_iter().map(|snapshot| snapshot.id).collect::<Vec<_>>();
        let mut stopped = Vec::new();
        for id in ids {
            if let Some(snapshot) = self.stop(&id)? {
                stopped.push(snapshot);
            }
        }
        Ok(stopped)
    }

    pub fn snapshot(&self, id: &str) -> Option<ProcessSnapshotDto> {
        self.registry.lock().unwrap().snapshot(id)
    }

    pub fn snapshots(&self) -> Vec<ProcessSnapshotDto> {
        self.registry.lock().unwrap().snapshots()
    }

    pub fn refresh(&self) {
        self.registry.lock().unwrap().refresh();
    }
}
