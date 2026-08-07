use ocentra_eventing::{ids::CorrelationId, journal::policy::JournalDispatchPhase};
use ocentra_parent_agent_core::enforcement_boundary::EnforcementBoundaryRejection;
use ocentra_parent_agent_protocol::{
    activity::{
        ActivityEvent, ActivityEventKind, ActivityObserver, ActivitySource, ActivitySubject,
        ActivitySubjectKind, ACTIVITY_SCHEMA_VERSION,
    },
    activity_query::ActivityIngestStatus,
    constants,
    enforcement::{
        EnforcementAdapterResultCode, EnforcementAuditEventKind, EnforcementAuditJournalEvent,
        EnforcementResultStatus,
    },
    logging::{LogFieldValue, LogFields},
};

use crate::{
    enforcement_payload::{EnforcementCommandPayload, EnforcementText},
    fields::fields_from_pairs,
};

use super::super::enforcement_pre_action_journal::eventing_journal::{
    append_enforcement_audit_journal_event_phase, EnforcementEventingJournalPath,
};
use super::{
    record_enforcement_activity_event, EnforcementJournalBuildError, EnforcementJournalPaths,
};

pub(super) async fn record_rejected_enforcement_audit(
    command_correlation_id: &EnforcementText,
    command_sent_at: &EnforcementText,
    request: &EnforcementCommandPayload,
    rejection: EnforcementBoundaryRejection,
    observed_at: &EnforcementText,
    paths: &EnforcementJournalPaths,
) -> Result<ActivityIngestStatus, EnforcementJournalBuildError> {
    record_rejected_eventing_enforcement_audit(
        command_correlation_id,
        command_sent_at,
        request,
        paths,
    )
    .await?;
    record_enforcement_activity_event(
        rejected_enforcement_activity_event(request, rejection, observed_at),
        paths,
    )
    .await
}

async fn record_rejected_eventing_enforcement_audit(
    command_correlation_id: &EnforcementText,
    command_sent_at: &EnforcementText,
    request: &EnforcementCommandPayload,
    paths: &EnforcementJournalPaths,
) -> Result<(), EnforcementJournalBuildError> {
    let mut eventing_journal_path = paths.journal_path.clone();
    eventing_journal_path.set_extension(constants::enforcement::EVENTING_JOURNAL_EXTENSION);
    append_enforcement_audit_journal_event_phase(
        EnforcementEventingJournalPath {
            path: eventing_journal_path,
        },
        rejected_eventing_audit_event(request, command_sent_at),
        CorrelationId::parse(command_correlation_id.0.clone()).map_err(eventing_journal_error)?,
        JournalDispatchPhase::AfterDispatch,
    )
    .await
    .map_err(eventing_journal_error)?;
    Ok(())
}

fn rejected_eventing_audit_event(
    request: &EnforcementCommandPayload,
    command_sent_at: &EnforcementText,
) -> EnforcementAuditJournalEvent {
    EnforcementAuditJournalEvent {
        audit_event_id: rejected_enforcement_event_id(request).0,
        action_id: request.input.action_id.clone(),
        result_id: request.input.result_id.clone(),
        audit_event_kind: EnforcementAuditEventKind::Failed,
        result_status: EnforcementResultStatus::Failed,
        adapter_result_code: EnforcementAdapterResultCode::NoOp,
        capability_state: request.input.capability.capability_state,
        observed_at: command_sent_at.0.clone(),
    }
}

fn rejected_enforcement_activity_event(
    request: &EnforcementCommandPayload,
    rejection: EnforcementBoundaryRejection,
    observed_at: &EnforcementText,
) -> ActivityEvent {
    ActivityEvent {
        schema_version: ACTIVITY_SCHEMA_VERSION,
        event_id: rejected_enforcement_event_id(request).0,
        observed_at: observed_at.0.clone(),
        source: ActivitySource {
            device_id: request.device_id.clone().0,
            platform: request.platform.clone(),
            observer: ActivityObserver::AgentService,
            source_id: constants::enforcement::SOURCE_ID_AGENT_SERVICE.to_string(),
        },
        kind: ActivityEventKind::EnforcementAuditRecorded,
        subject: ActivitySubject {
            kind: ActivitySubjectKind::Intervention,
            subject_id: request.input.action_id.clone(),
            display_name: Some(constants::enforcement::RESULT_FAILED.to_string()),
        },
        fields: rejected_enforcement_audit_fields(request, rejection),
        evidence: Vec::new(),
    }
}

fn rejected_enforcement_event_id(request: &EnforcementCommandPayload) -> EnforcementText {
    let mut event_id = constants::enforcement::JOURNAL_REJECTED_ID_PREFIX.to_string();
    event_id.push_str(&request.input.audit_event_id);
    EnforcementText(event_id)
}

fn rejected_enforcement_audit_fields(
    request: &EnforcementCommandPayload,
    rejection: EnforcementBoundaryRejection,
) -> LogFields {
    rejected_enforcement_identity_fields(request)
        .into_inner()
        .into_iter()
        .chain(rejected_enforcement_outcome_fields(request, rejection).into_inner())
        .collect()
}

fn rejected_enforcement_identity_fields(request: &EnforcementCommandPayload) -> LogFields {
    let intent = &request.input.intent;
    let decision = &request.input.decision;
    fields_from_pairs(vec![
        (
            constants::field::POLICY_DECISION_ID,
            LogFieldValue::String(decision.decision_id.clone()),
        ),
        (
            constants::field::POLICY_VERSION,
            LogFieldValue::String(request.input.policy_version.clone()),
        ),
        (
            constants::field::POLICY_ACTION,
            LogFieldValue::String(decision.action.as_protocol_str().to_string()),
        ),
        (
            constants::field::POLICY_TARGET_TYPE,
            LogFieldValue::String(intent.target.target_type.as_protocol_str().to_string()),
        ),
        (
            constants::field::TARGET_ID,
            LogFieldValue::String(intent.target.target_id.clone()),
        ),
        (
            constants::field::COMMAND_SOURCE_PEER_ID,
            LogFieldValue::String(request.source_peer_id.0.clone()),
        ),
        (
            constants::field::COMMAND_TARGET_ROUTE,
            LogFieldValue::String(request.target_route.0.clone()),
        ),
    ])
}

fn rejected_enforcement_outcome_fields(
    request: &EnforcementCommandPayload,
    rejection: EnforcementBoundaryRejection,
) -> LogFields {
    fields_from_pairs(vec![
        (
            constants::field::ENFORCEMENT_ACTION_ID,
            LogFieldValue::String(request.input.action_id.clone()),
        ),
        (
            constants::field::ENFORCEMENT_RESULT_ID,
            LogFieldValue::String(request.input.result_id.clone()),
        ),
        (
            constants::field::ENFORCEMENT_AUDIT_EVENT_ID,
            LogFieldValue::String(request.input.audit_event_id.clone()),
        ),
        (
            constants::field::ENFORCEMENT_STATUS,
            LogFieldValue::String(constants::enforcement::RESULT_FAILED.to_string()),
        ),
        (
            constants::field::ENFORCEMENT_ADAPTER_RESULT_CODE,
            LogFieldValue::String(constants::enforcement::ADAPTER_NO_OP.to_string()),
        ),
        (
            constants::field::ENFORCEMENT_ROLLBACK_STATE,
            LogFieldValue::String(constants::enforcement::ROLLBACK_NOT_REQUIRED.to_string()),
        ),
        (
            constants::field::ENFORCEMENT_CAPABILITY_STATE,
            LogFieldValue::String(
                request
                    .input
                    .capability
                    .capability_state
                    .as_protocol_str()
                    .to_string(),
            ),
        ),
        (
            constants::field::EVIDENCE_REFERENCE_IDS,
            LogFieldValue::String(rejected_evidence_reference_ids(request).0),
        ),
        (
            constants::field::REASON,
            LogFieldValue::String(rejection.as_protocol_str().to_string()),
        ),
    ])
}

fn rejected_evidence_reference_ids(request: &EnforcementCommandPayload) -> EnforcementText {
    let mut separator = [0; 4];
    EnforcementText(
        request
            .input
            .intent
            .evidence_references
            .iter()
            .map(|reference| reference.evidence_reference_id.as_str())
            .collect::<Vec<_>>()
            .join(constants::delimiter::LIST.encode_utf8(&mut separator)),
    )
}

fn eventing_journal_error(_: impl std::fmt::Debug) -> EnforcementJournalBuildError {
    EnforcementJournalBuildError::Store
}
