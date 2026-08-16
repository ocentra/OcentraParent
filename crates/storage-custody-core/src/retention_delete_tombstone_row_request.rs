use ocentra_schema::retention_delete_tombstone as contracts;

use super::{
    retention_delete_tombstone_row_state::{
        retention_delete_requirements, signal_state, state_retention_class,
        validate_retention_delete_state_requirements,
    },
    RetentionDeleteDerivationError, RetentionDeleteDerivationInput,
};

pub(super) fn derive_retention_delete_tombstone_row(
    request: &contracts::RetentionDeleteRequest,
    input: RetentionDeleteDerivationInput,
) -> Result<contracts::RetentionDeleteRow, RetentionDeleteDerivationError> {
    validate_request(request, input.request_expired)?;

    let state = signal_state(input.signal);
    let requirements = retention_delete_requirements(state);
    validate_retention_delete_state_requirements(state, &requirements, &input)?;

    Ok(contracts::RetentionDeleteRow {
        row_id: input.row_id,
        request_id: request.request_id.clone(),
        data_class: input.data_class,
        state,
        retention_class: state_retention_class(state),
        tombstone_ref: input.tombstone_ref,
        replay_ref: input.replay_ref,
        proof_ref: input.proof_ref,
        request_expired: false,
        parent_authorized: request.parent_authorized,
        tombstone_written: requirements.tombstone_written,
        local_payload_present: !requirements.requires_redaction,
        local_payload_redacted: requirements.requires_redaction,
        propagation_pending: state == contracts::RetentionDeleteState::PropagationPending,
        propagated: matches!(
            state,
            contracts::RetentionDeleteState::Propagated
                | contracts::RetentionDeleteState::ReplayProtected
                | contracts::RetentionDeleteState::AuditRetained
                | contracts::RetentionDeleteState::HardDeleted
        ),
        replay_blocked: requirements.requires_replay_protection,
        restore_revival_blocked: requirements.requires_replay_protection,
        minimal_audit_ref_retained: requirements.requires_minimal_audit,
        audit_payload_redacted: requirements.requires_minimal_audit,
        report_export_leak_blocked: request.derived_outputs_must_redact,
        assistant_leak_blocked: request.derived_outputs_must_redact,
        notification_leak_blocked: request.derived_outputs_must_redact,
        hard_deleted: state == contracts::RetentionDeleteState::HardDeleted,
        claim_safe: true,
    })
}

fn validate_request(
    request: &contracts::RetentionDeleteRequest,
    request_expired: bool,
) -> Result<(), RetentionDeleteDerivationError> {
    if !request.parent_authorized {
        return Err(RetentionDeleteDerivationError::ActorNotAuthorized);
    }
    if request_expired {
        return Err(RetentionDeleteDerivationError::DeleteRequestExpired);
    }
    if !request.raw_payload_delete_requested {
        return Err(RetentionDeleteDerivationError::DeleteRequestMustTargetRawPayload);
    }
    if !request.derived_outputs_must_redact {
        return Err(RetentionDeleteDerivationError::DerivedOutputsMustRedact);
    }
    Ok(())
}
