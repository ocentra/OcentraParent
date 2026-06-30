use std::collections::HashSet;

use ocentra_schema::retention_delete_tombstone as contracts;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetentionDeleteSignal {
    DeleteRequested,
    DeleteValidated,
    TombstoneWritten,
    LocalRedacted,
    PropagationPending,
    Propagated,
    ReplayProtected,
    AuditRetained,
    HardDeleted,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetentionDeleteDerivationInput {
    pub row_id: contracts::RetentionDeleteRowId,
    pub data_class: contracts::RetentionDeleteDataClass,
    pub signal: RetentionDeleteSignal,
    pub proof_ref: contracts::RetentionDeleteProofRef,
    pub tombstone_ref: Option<contracts::RetentionDeleteTombstoneRef>,
    pub replay_ref: Option<contracts::RetentionDeleteReplayRef>,
    pub request_expired: bool,
    pub local_payload_redacted: bool,
    pub propagation_complete: bool,
    pub replay_blocked: bool,
    pub audit_payload_redacted: bool,
    pub hard_delete_eligible: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RetentionDeleteDerivationError {
    ActorNotAuthorized,
    DeleteRequestExpired,
    DeleteRequestMustTargetRawPayload,
    DerivedOutputsMustRedact,
    MissingTombstoneRef,
    LocalPayloadMustBeRedacted,
    PropagationStillPending,
    MissingReplayRef,
    ReplayProtectionRequired,
    AuditMustBeMinimal,
    HardDeleteNotEligible,
    DuplicateState(contracts::RetentionDeleteState),
    MissingRequiredState(contracts::RetentionDeleteState),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct RetentionDeleteStateRequirements {
    tombstone_written: bool,
    requires_redaction: bool,
    requires_propagation: bool,
    requires_replay_protection: bool,
    requires_minimal_audit: bool,
}

fn option_or_unreachable<T>(value: Option<T>, context: &str) -> T {
    match value {
        Some(value) => value,
        None => unreachable!("{context}"),
    }
}

pub fn derive_retention_delete_tombstone_row(
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

fn retention_delete_requirements(
    state: contracts::RetentionDeleteState,
) -> RetentionDeleteStateRequirements {
    RetentionDeleteStateRequirements {
        tombstone_written: matches!(
            state,
            contracts::RetentionDeleteState::TombstoneWritten
                | contracts::RetentionDeleteState::LocalRedacted
                | contracts::RetentionDeleteState::PropagationPending
                | contracts::RetentionDeleteState::Propagated
                | contracts::RetentionDeleteState::ReplayProtected
                | contracts::RetentionDeleteState::AuditRetained
                | contracts::RetentionDeleteState::HardDeleted
        ),
        requires_redaction: matches!(
            state,
            contracts::RetentionDeleteState::LocalRedacted
                | contracts::RetentionDeleteState::PropagationPending
                | contracts::RetentionDeleteState::Propagated
                | contracts::RetentionDeleteState::ReplayProtected
                | contracts::RetentionDeleteState::AuditRetained
                | contracts::RetentionDeleteState::HardDeleted
        ),
        requires_propagation: matches!(
            state,
            contracts::RetentionDeleteState::Propagated
                | contracts::RetentionDeleteState::ReplayProtected
                | contracts::RetentionDeleteState::AuditRetained
                | contracts::RetentionDeleteState::HardDeleted
        ),
        requires_replay_protection: matches!(
            state,
            contracts::RetentionDeleteState::ReplayProtected
                | contracts::RetentionDeleteState::AuditRetained
                | contracts::RetentionDeleteState::HardDeleted
        ),
        requires_minimal_audit: matches!(
            state,
            contracts::RetentionDeleteState::AuditRetained
                | contracts::RetentionDeleteState::HardDeleted
        ),
    }
}

fn validate_retention_delete_state_requirements(
    state: contracts::RetentionDeleteState,
    requirements: &RetentionDeleteStateRequirements,
    input: &RetentionDeleteDerivationInput,
) -> Result<(), RetentionDeleteDerivationError> {
    if requirements.tombstone_written && input.tombstone_ref.is_none() {
        return Err(RetentionDeleteDerivationError::MissingTombstoneRef);
    }
    if requirements.requires_redaction && !input.local_payload_redacted {
        return Err(RetentionDeleteDerivationError::LocalPayloadMustBeRedacted);
    }
    if matches!(state, contracts::RetentionDeleteState::PropagationPending)
        && input.propagation_complete
    {
        return Err(RetentionDeleteDerivationError::PropagationStillPending);
    }
    if requirements.requires_propagation && !input.propagation_complete {
        return Err(RetentionDeleteDerivationError::PropagationStillPending);
    }
    if requirements.requires_replay_protection && input.replay_ref.is_none() {
        return Err(RetentionDeleteDerivationError::MissingReplayRef);
    }
    if requirements.requires_replay_protection && !input.replay_blocked {
        return Err(RetentionDeleteDerivationError::ReplayProtectionRequired);
    }
    if requirements.requires_minimal_audit && !input.audit_payload_redacted {
        return Err(RetentionDeleteDerivationError::AuditMustBeMinimal);
    }
    if state == contracts::RetentionDeleteState::HardDeleted && !input.hard_delete_eligible {
        return Err(RetentionDeleteDerivationError::HardDeleteNotEligible);
    }
    Ok(())
}

pub fn build_retention_delete_tombstone_proof(
    request: &contracts::RetentionDeleteRequest,
    inputs: Vec<RetentionDeleteDerivationInput>,
    updated_at: contracts::RetentionDeleteTimestamp,
) -> Result<contracts::RetentionDeleteTombstoneContractProof, RetentionDeleteDerivationError> {
    let mut rows = Vec::with_capacity(inputs.len());
    let mut seen_states = HashSet::with_capacity(inputs.len());

    for input in inputs {
        let state = signal_state(input.signal);
        if !seen_states.insert(state.as_str()) {
            return Err(RetentionDeleteDerivationError::DuplicateState(state));
        }
        rows.push(derive_retention_delete_tombstone_row(request, input)?);
    }

    rows.sort_by_key(|row| state_rank(row.state));

    for required_state in contracts::required_retention_delete_states() {
        if !rows.iter().any(|row| row.state == required_state) {
            return Err(RetentionDeleteDerivationError::MissingRequiredState(
                required_state,
            ));
        }
    }

    Ok(contracts::RetentionDeleteTombstoneContractProof {
        schema_version: contracts::RETENTION_DELETE_TOMBSTONE_SCHEMA_VERSION.to_string(),
        contract_version: option_or_unreachable(
            contracts::RetentionDeleteContractVersion::parse("v0.4"),
            "contract version",
        ),
        request: request.clone(),
        retention_matrix: contracts::retention_delete_policy_matrix(),
        rows,
        non_claims: contracts::required_retention_delete_non_claims(),
        report_runtime_claimed: false,
        notification_runtime_claimed: false,
        restore_runtime_claimed: false,
        ts_business_owner_claimed: false,
        updated_at,
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

fn signal_state(signal: RetentionDeleteSignal) -> contracts::RetentionDeleteState {
    match signal {
        RetentionDeleteSignal::DeleteRequested => contracts::RetentionDeleteState::DeleteRequested,
        RetentionDeleteSignal::DeleteValidated => contracts::RetentionDeleteState::DeleteValidated,
        RetentionDeleteSignal::TombstoneWritten => {
            contracts::RetentionDeleteState::TombstoneWritten
        }
        RetentionDeleteSignal::LocalRedacted => contracts::RetentionDeleteState::LocalRedacted,
        RetentionDeleteSignal::PropagationPending => {
            contracts::RetentionDeleteState::PropagationPending
        }
        RetentionDeleteSignal::Propagated => contracts::RetentionDeleteState::Propagated,
        RetentionDeleteSignal::ReplayProtected => contracts::RetentionDeleteState::ReplayProtected,
        RetentionDeleteSignal::AuditRetained => contracts::RetentionDeleteState::AuditRetained,
        RetentionDeleteSignal::HardDeleted => contracts::RetentionDeleteState::HardDeleted,
    }
}

fn state_retention_class(
    state: contracts::RetentionDeleteState,
) -> contracts::RetentionDeleteRetentionClass {
    match state {
        contracts::RetentionDeleteState::DeleteRequested
        | contracts::RetentionDeleteState::DeleteValidated => {
            contracts::RetentionDeleteRetentionClass::DeleteRequested
        }
        contracts::RetentionDeleteState::TombstoneWritten
        | contracts::RetentionDeleteState::LocalRedacted
        | contracts::RetentionDeleteState::PropagationPending
        | contracts::RetentionDeleteState::Propagated
        | contracts::RetentionDeleteState::ReplayProtected => {
            contracts::RetentionDeleteRetentionClass::DeleteConfirmed
        }
        contracts::RetentionDeleteState::AuditRetained => {
            contracts::RetentionDeleteRetentionClass::AuditMinimal
        }
        contracts::RetentionDeleteState::HardDeleted => {
            contracts::RetentionDeleteRetentionClass::HardDeleted
        }
    }
}

fn state_rank(state: contracts::RetentionDeleteState) -> usize {
    match state {
        contracts::RetentionDeleteState::DeleteRequested => 0,
        contracts::RetentionDeleteState::DeleteValidated => 1,
        contracts::RetentionDeleteState::TombstoneWritten => 2,
        contracts::RetentionDeleteState::LocalRedacted => 3,
        contracts::RetentionDeleteState::PropagationPending => 4,
        contracts::RetentionDeleteState::Propagated => 5,
        contracts::RetentionDeleteState::ReplayProtected => 6,
        contracts::RetentionDeleteState::AuditRetained => 7,
        contracts::RetentionDeleteState::HardDeleted => 8,
    }
}
