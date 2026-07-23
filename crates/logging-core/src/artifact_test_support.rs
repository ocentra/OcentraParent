use std::{io, path::Path};

#[derive(Clone, Copy)]
pub enum ArtifactFallbackFault {
    Copy,
    Sync,
    Crash,
}

#[derive(Clone, Copy)]
pub enum HardLinkFault {
    PermissionDenied,
    AlreadyExists,
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
        ArtifactFallbackFault::Crash => crate::artifact_publish_copy::FallbackPublishFault::Crash,
    };
    crate::artifact_publish::publish_immutable_with_fallback_fault(path, content, fault)
}

pub fn publish_artifact_with_hard_link_fault(
    path: &Path,
    content: &[u8],
    fault: HardLinkFault,
) -> io::Result<()> {
    let kind = match fault {
        HardLinkFault::PermissionDenied => io::ErrorKind::PermissionDenied,
        HardLinkFault::AlreadyExists => io::ErrorKind::AlreadyExists,
    };
    crate::artifact_publish::publish_immutable_with_link_error(path, content, kind)
}

pub fn publish_artifact_with_parent_sync_fault(path: &Path, content: &[u8]) -> io::Result<()> {
    crate::artifact_publish::publish_immutable_with_parent_sync_fault(path, content)
}
