use super::{
    NetworkCaptureIngestProof, NetworkEndToEndPipelineError, NetworkEndToEndPipelineRefs,
    NetworkRetentionDeleteExportProof,
};

use super::validation::required_ref;

pub(super) fn capture_ingest_proof(
    refs: &NetworkEndToEndPipelineRefs,
    summary_refs: &[String],
    evidence_refs: &[String],
) -> Result<NetworkCaptureIngestProof, NetworkEndToEndPipelineError> {
    Ok(NetworkCaptureIngestProof {
        trigger_ref: required_ref(
            &refs.trigger_ref,
            NetworkEndToEndPipelineError::EmptyTriggerRef,
        )?,
        capture_ref: required_ref(
            &refs.capture_ref,
            NetworkEndToEndPipelineError::EmptyCaptureRef,
        )?,
        ingest_ref: required_ref(
            &refs.ingest_ref,
            NetworkEndToEndPipelineError::EmptyIngestRef,
        )?,
        typed_event_ref: required_ref(
            &refs.typed_event_ref,
            NetworkEndToEndPipelineError::EmptyTypedEventRef,
        )?,
        summary_refs: summary_refs.to_vec(),
        evidence_refs: evidence_refs.to_vec(),
        audit_event_ref: required_ref(
            &refs.audit_event_ref,
            NetworkEndToEndPipelineError::EmptyAuditEventRef,
        )?,
        same_product_path: true,
    })
}

pub(super) fn retention_delete_export(
    refs: &NetworkEndToEndPipelineRefs,
    evidence_refs: &[String],
) -> Result<NetworkRetentionDeleteExportProof, NetworkEndToEndPipelineError> {
    Ok(NetworkRetentionDeleteExportProof {
        retention_ref: required_ref(
            &refs.retention_ref,
            NetworkEndToEndPipelineError::EmptyRetentionRef,
        )?,
        deletion_ref: required_ref(
            &refs.deletion_ref,
            NetworkEndToEndPipelineError::EmptyDeletionRef,
        )?,
        export_ref: required_ref(
            &refs.export_ref,
            NetworkEndToEndPipelineError::EmptyExportRef,
        )?,
        tombstone_ref: required_ref(
            &refs.tombstone_ref,
            NetworkEndToEndPipelineError::EmptyTombstoneRef,
        )?,
        audit_event_ref: required_ref(
            &refs.audit_event_ref,
            NetworkEndToEndPipelineError::EmptyAuditEventRef,
        )?,
        portal_read_model_ref: required_ref(
            &refs.portal_read_model_ref,
            NetworkEndToEndPipelineError::EmptyPortalReadModelRef,
        )?,
        evidence_refs: evidence_refs.to_vec(),
        same_product_path: true,
    })
}
