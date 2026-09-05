use ocentra_parent_logging_core::local_artifact_mutation::{
    LocalArtifactDirectoryDurability, LocalArtifactMutationOutcome, LocalArtifactMutationSession,
};
use serde_json::json;

use super::super::{
    FailureDisposition, LeaseRequirement, LeaseState, OperationExecution, ProviderError,
};
use crate::protocol::{self, ProviderRelativePath, ReadMaximum, ValidatedRequest};

pub(super) fn recover(
    session: &mut LocalArtifactMutationSession<'_>,
    lease: &LeaseState,
    request: &ValidatedRequest,
) -> Result<OperationExecution, ProviderError> {
    super::super::lease::authorize_lease(lease, request.lease_id(), LeaseRequirement::Required)?;
    let receipts = session
        .recover()
        .map_err(|error| super::super::map_owner_error(&error))?;
    if receipts.len() > 65_536 {
        return Err(ProviderError::new(
            protocol::text::RECOVERED_RECEIPT_LIMIT,
            FailureDisposition::Terminate,
        ));
    }
    for receipt in &receipts {
        if matches!(
            receipt.outcome(),
            LocalArtifactMutationOutcome::Unsupported { .. }
        ) {
            return Err(super::results::unsupported_failure());
        }
    }
    Ok(super::super::success(protocol::text::object(vec![(
        protocol::text::TextId::RecoveredKey,
        json!(receipts.len()),
    )])))
}

pub(super) fn ensure_directory(
    session: &LocalArtifactMutationSession<'_>,
    lease: &LeaseState,
    request: &ValidatedRequest,
    relative_path: &ProviderRelativePath,
) -> Result<OperationExecution, ProviderError> {
    super::super::lease::authorize_lease(lease, request.lease_id(), LeaseRequirement::Optional)?;
    let path = relative_path.text();
    let durability = session
        .ensure_directory(&path)
        .map_err(|error| super::super::map_owner_error(&error))?;
    require_synced(durability)?;
    Ok(super::super::success(protocol::text::object(vec![(
        protocol::text::TextId::ReadyKey,
        json!(true),
    )])))
}

pub(super) fn sync_directory(
    session: &LocalArtifactMutationSession<'_>,
    lease: &LeaseState,
    request: &ValidatedRequest,
    relative_path: &ProviderRelativePath,
) -> Result<OperationExecution, ProviderError> {
    super::super::lease::authorize_lease(lease, request.lease_id(), LeaseRequirement::Optional)?;
    let path = relative_path.text();
    let durability = session
        .sync_directory(&path)
        .map_err(|error| super::super::map_owner_error(&error))?;
    require_synced(durability)?;
    Ok(super::super::success(protocol::text::object(vec![(
        protocol::text::TextId::SyncedKey,
        json!(true),
    )])))
}

pub(super) fn stat(
    session: &LocalArtifactMutationSession<'_>,
    lease: &LeaseState,
    request: &ValidatedRequest,
    relative_path: &ProviderRelativePath,
) -> Result<OperationExecution, ProviderError> {
    super::super::lease::authorize_lease(lease, request.lease_id(), LeaseRequirement::Optional)?;
    let path = relative_path.text();
    let stat = session
        .stat(&path)
        .map_err(|error| super::super::map_owner_error(&error))?;
    let result = stat
        .map(super::encoding::stat_value)
        .transpose()?
        .unwrap_or_else(crate::protocol::text::null);
    Ok(super::super::success(result))
}

pub(super) fn read_snapshot(
    session: &LocalArtifactMutationSession<'_>,
    lease: &LeaseState,
    request: &ValidatedRequest,
    relative_path: &ProviderRelativePath,
    maximum_bytes: ReadMaximum,
) -> Result<OperationExecution, ProviderError> {
    super::super::lease::authorize_lease(lease, request.lease_id(), LeaseRequirement::Optional)?;
    let path = relative_path.text();
    let snapshot = session
        .read_snapshot(&path, maximum_bytes.value())
        .map_err(|error| super::super::map_owner_error(&error))?;
    let result = snapshot
        .as_ref()
        .map(|value| super::encoding::snapshot_value(value, maximum_bytes))
        .transpose()?
        .unwrap_or_else(crate::protocol::text::null);
    Ok(super::super::success(result))
}

pub(super) fn list(
    session: &LocalArtifactMutationSession<'_>,
    lease: &LeaseState,
    request: &ValidatedRequest,
    relative_path: &ProviderRelativePath,
) -> Result<OperationExecution, ProviderError> {
    super::super::lease::authorize_lease(lease, request.lease_id(), LeaseRequirement::Optional)?;
    let path = relative_path.text();
    let entries = session
        .list(&path)
        .map_err(|error| super::super::map_owner_error(&error))?;
    if entries.len() > 65_536 {
        return Err(ProviderError::new(
            protocol::text::DIRECTORY_ENTRY_LIMIT,
            FailureDisposition::Continue,
        ));
    }
    let values = entries
        .iter()
        .map(super::encoding::entry_value)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(super::super::success(protocol::text::array(values)))
}

fn require_synced(durability: LocalArtifactDirectoryDurability) -> Result<(), ProviderError> {
    match durability {
        LocalArtifactDirectoryDurability::Synced => Ok(()),
    }
}
