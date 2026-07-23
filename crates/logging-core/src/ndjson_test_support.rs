use std::{
    fs::{create_dir_all, OpenOptions},
    io,
    path::Path,
};

use crate::ndjson_operation::{append_operation_with_fault, FaultPoint};
use crate::ndjson_operation_fault::{
    append_operation_with_marker_fault, OperationMarkerFault as InternalOperationMarkerFault,
};

#[derive(Clone, Copy)]
pub enum ArtifactFallbackFault {
    Copy,
    Sync,
}

#[derive(Clone, Copy)]
pub enum AppendFault {
    Write,
    Sync,
}

#[derive(Clone, Copy)]
pub enum OperationMarkerFault {
    IntentWrite,
    IntentSync,
    CommitWrite,
    CommitSync,
}

pub fn publish_artifact_with_forced_fallback(path: &Path, content: &[u8]) -> io::Result<()> {
    crate::artifact_publish::publish_immutable_with_fallback(path, content, true)
}

pub fn publish_artifact_with_forced_fallback_fault(
    path: &Path,
    content: &[u8],
    fault: ArtifactFallbackFault,
) -> io::Result<()> {
    let fault = match fault {
        ArtifactFallbackFault::Copy => crate::artifact_publish_copy::FallbackPublishFault::Copy,
        ArtifactFallbackFault::Sync => crate::artifact_publish_copy::FallbackPublishFault::Sync,
    };
    crate::artifact_publish::publish_immutable_with_fallback_fault(path, content, fault)
}

pub fn publish_artifact_with_parent_sync_fault(path: &Path, content: &[u8]) -> io::Result<()> {
    crate::artifact_publish::publish_immutable_with_parent_sync_fault(path, content)
}

pub fn append_record_with_fault(
    path: &Path,
    operation_id: &str,
    record: &[u8],
    fault: AppendFault,
) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(path)?;
    file.lock()?;
    let result = append_operation_with_fault(&mut file, path, operation_id, record, |point| {
        if fault_matches(fault, point) {
            return Err(io::Error::other("injected NDJSON append failure"));
        }
        Ok(())
    });
    let unlock_result = file.unlock();
    result.and(unlock_result)
}

pub fn append_record_with_marker_fault(
    path: &Path,
    operation_id: &str,
    record: &[u8],
    fault: OperationMarkerFault,
) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        create_dir_all(parent)?;
    }
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(path)?;
    file.lock()?;
    let fault = match fault {
        OperationMarkerFault::IntentWrite => InternalOperationMarkerFault::IntentWrite,
        OperationMarkerFault::IntentSync => InternalOperationMarkerFault::IntentSync,
        OperationMarkerFault::CommitWrite => InternalOperationMarkerFault::CommitWrite,
        OperationMarkerFault::CommitSync => InternalOperationMarkerFault::CommitSync,
    };
    let result = append_operation_with_marker_fault(&mut file, path, operation_id, record, fault);
    let unlock_result = file.unlock();
    result.and(unlock_result)
}

fn fault_matches(fault: AppendFault, point: FaultPoint) -> bool {
    matches!(
        (fault, point),
        (AppendFault::Write, FaultPoint::Write) | (AppendFault::Sync, FaultPoint::Sync)
    )
}
