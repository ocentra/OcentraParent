use std::io::ErrorKind;

use ocentra_parent_agent_core::activity_store::ActivityStore;
use ocentra_parent_agent_protocol::logging::LogFields;

use super::{recovery_store_error, EnforcementRetryRecoveryError};
use crate::enforcement_api::enforcement_command_execution::EnforcementJournalPaths;
use crate::enforcement_payload::EnforcementText;

pub(super) struct StoredAudit {
    pub(super) fields: LogFields,
}

pub(super) async fn read_stored_audit(
    paths: &EnforcementJournalPaths,
    audit_event_id: &EnforcementText,
) -> Result<Option<StoredAudit>, EnforcementRetryRecoveryError> {
    match tokio::fs::metadata(&paths.store_path).await {
        Ok(metadata) if metadata.is_file() => {}
        Ok(_) => return Err(EnforcementRetryRecoveryError::Store),
        Err(error) if error.kind() == ErrorKind::NotFound => return Ok(None),
        Err(_) => return Err(EnforcementRetryRecoveryError::Store),
    }
    let store_path = paths.store_path.clone();
    let audit_event_id = audit_event_id.0.clone();
    tokio::task::spawn_blocking(move || {
        let store = ActivityStore::open(store_path).map_err(recovery_store_error)?;
        Ok(store
            .enforcement_audit_fields_by_event_id(&audit_event_id)
            .map_err(recovery_store_error)?
            .map(|fields| StoredAudit { fields }))
    })
    .await
    .map_err(recovery_store_error)?
}
