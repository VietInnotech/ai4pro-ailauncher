use crate::models::{ProcessSnapshotDto, ProcessStatus};
use std::collections::HashMap;
use std::process::Child;

pub struct ProcessRecord {
    pub id: String,
    pub child: Option<Child>,
    pub status: ProcessStatus,
    pub pid: Option<u32>,
    pub last_error: Option<String>,
    pub last_exit_code: Option<i32>,
}

#[derive(Default)]
pub struct ProcessRegistry {
    records: HashMap<String, ProcessRecord>,
}

impl ProcessRegistry {
    pub fn upsert_running(&mut self, id: String, child: Child) {
        let pid = Some(child.id());
        self.records.insert(
            id.clone(),
            ProcessRecord {
                id,
                child: Some(child),
                status: ProcessStatus::Running,
                pid,
                last_error: None,
                last_exit_code: None,
            },
        );
    }

    pub fn set_status(&mut self, id: &str, status: ProcessStatus) {
        if let Some(record) = self.records.get_mut(id) {
            record.status = status.clone();
            if !matches!(status, ProcessStatus::Running) {
                record.pid = record.child.as_ref().map(|child| child.id());
            }
        }
    }

    pub fn set_error(&mut self, id: &str, error: String) {
        self.records.insert(
            id.to_string(),
            ProcessRecord {
                id: id.to_string(),
                child: None,
                status: ProcessStatus::Crashed,
                pid: None,
                last_error: Some(error),
                last_exit_code: None,
            },
        );
    }

    pub fn with_child_mut<T>(
        &mut self,
        id: &str,
        f: impl FnOnce(&mut Child, &mut ProcessRecord) -> T,
    ) -> Option<T> {
        let record = self.records.get_mut(id)?;
        let mut child = record.child.take()?;
        let result = f(&mut child, record);
        if record.child.is_none()
            && matches!(
                record.status,
                ProcessStatus::Running | ProcessStatus::Starting | ProcessStatus::Stopping
            )
        {
            record.pid = Some(child.id());
            record.child = Some(child);
        }
        Some(result)
    }

    pub fn snapshot(&self, id: &str) -> Option<ProcessSnapshotDto> {
        self.records.get(id).map(ProcessRecord::snapshot)
    }

    pub fn snapshots(&self) -> Vec<ProcessSnapshotDto> {
        self.records.values().map(ProcessRecord::snapshot).collect()
    }

    pub fn refresh(&mut self) {
        for record in self.records.values_mut() {
            if let Some(child) = record.child.as_mut() {
                match child.try_wait() {
                    Ok(Some(status)) => {
                        record.last_exit_code = status.code();
                        record.status = if status.success() {
                            ProcessStatus::Stopped
                        } else {
                            ProcessStatus::Crashed
                        };
                        record.pid = None;
                        record.child = None;
                    }
                    Ok(None) => {
                        record.status = ProcessStatus::Running;
                        record.pid = Some(child.id());
                    }
                    Err(error) => {
                        record.status = ProcessStatus::Crashed;
                        record.last_error = Some(error.to_string());
                        record.pid = None;
                        record.child = None;
                    }
                }
            }
        }
    }
}

impl ProcessRecord {
    pub fn snapshot(&self) -> ProcessSnapshotDto {
        ProcessSnapshotDto {
            id: self.id.clone(),
            status: self.status.clone(),
            pid: self.pid,
            last_error: self.last_error.clone(),
            last_exit_code: self.last_exit_code,
        }
    }
}
