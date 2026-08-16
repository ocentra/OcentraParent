use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_core::activity_store::ActivityStore;
use ocentra_parent_agent_protocol::activity::policy_preview::PolicySourceStatus;
use ocentra_parent_agent_protocol::activity::policy_preview::PolicySourceSurface;
use ocentra_parent_agent_protocol::activity::ActivityEvent;
use ocentra_parent_agent_protocol::activity::ActivityEventKind;
use ocentra_parent_agent_protocol::activity::ActivityObserver;
use ocentra_parent_agent_protocol::activity::ActivitySource;
use ocentra_parent_agent_protocol::activity::ActivitySubject;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::logging::LogLevel;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventName;
use ocentra_parent_agent_protocol::transport::PolicyRequestAssistantPreviewConfirmClaimState;
use ocentra_parent_agent_protocol::transport::PolicyRequestAssistantPreviewConfirmRequest;
use ocentra_parent_agent_protocol::transport::PolicyRequestAssistantPreviewConfirmResult;
use ocentra_parent_agent_protocol::transport::PolicyRequestAssistantPreviewConfirmResultState;
use ocentra_parent_agent_protocol::ACTIVITY_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;
use ocentra_policy_control_core::policy_request::{
    confirm_assistant_policy_request_preview, policy_request_schema_version,
    AssistantPolicyRequestConfirmation, ChildPolicyRequest, PolicyDurationMinutes, PolicyRequestId,
    PolicyRequestScope, PolicyRequestSubmissionKey, PolicyRequestTarget, PolicyRequestTimestamp,
};
use ocentra_policy_control_core::policy_source::{
    ParentPolicyDocumentId, PolicyActorId, PolicyAuditReferenceId, PolicyChildProfileId,
    PolicyDeviceId, PolicyHouseholdId, PolicyRuleId, PolicyTargetReferenceId, PolicyVersion,
};

use crate::{
    activity_store_path::activity_db_path, event_builder::build_event, fields::fields_from_pairs,
    json_contract::serialize_json_string,
};

#[path = "policy_request_confirm_mapping.rs"]
mod mapping;
#[path = "policy_request_confirm/parse.rs"]
mod parse;
#[path = "policy_request_confirm/target.rs"]
mod target;

use self::mapping::{
    actor_role_protocol, map_actor_role, map_actor_state, map_confirmation_state,
    map_protocol_confirmation_state, map_protocol_request_status, map_request_kind,
    map_request_origin, map_request_status, map_requested_action, map_target_kind,
};
use self::parse::{
    parse_policy_request_assistant_preview_confirm_request,
    PolicyRequestAssistantPreviewConfirmParseState,
};
use self::target::{supported_policy_preview_target, SupportedPolicyPreviewTarget};

const POLICY_REQUEST_CONFIRM_SOURCE_ID: &str = "policy-request-assistant-preview-confirm";
const POLICY_REQUEST_CONFIRM_REJECTION_REASON_INVALID_REQUEST: &str = "invalid-request";

struct PolicyRequestAssistantPreviewConfirmStoreOutcome {
    activity_store_mutation_claim_state: PolicyRequestAssistantPreviewConfirmClaimState,
    upstream_writer_claim_state: PolicyRequestAssistantPreviewConfirmClaimState,
    read_model_projection_claim_state: PolicyRequestAssistantPreviewConfirmClaimState,
}

struct PolicyRequestAssistantPreviewConfirmRejectionReason(String);

impl PolicyRequestAssistantPreviewConfirmStoreOutcome {
    fn claimed() -> Self {
        Self {
            activity_store_mutation_claim_state:
                PolicyRequestAssistantPreviewConfirmClaimState::Claimed,
            upstream_writer_claim_state: PolicyRequestAssistantPreviewConfirmClaimState::Claimed,
            read_model_projection_claim_state:
                PolicyRequestAssistantPreviewConfirmClaimState::Claimed,
        }
    }

    fn unclaimed() -> Self {
        Self {
            activity_store_mutation_claim_state:
                PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed,
            upstream_writer_claim_state: PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed,
            read_model_projection_claim_state:
                PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed,
        }
    }
}

pub(crate) async fn build_policy_request_assistant_preview_confirm_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let (request, parse_state) = parse_policy_request_assistant_preview_confirm_request(&command);
    let result =
        execute_policy_request_assistant_preview_confirm(&command, &request, parse_state).await;
    let result_text = serialize_json_string(&result).0;

    build_event(
        constants::event_id::POLICY_REQUEST_ASSISTANT_PREVIEW_CONFIRM_REPORTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentPolicyRequestAssistantPreviewConfirmReported,
        LogLevel::Info,
        fields_from_pairs(vec![(
            constants::field::POLICY_REQUEST_ASSISTANT_PREVIEW_CONFIRM_RESULT,
            LogFieldValue::String(result_text),
        )]),
        None,
    )
}

async fn execute_policy_request_assistant_preview_confirm(
    command: &AgentCommandEnvelope,
    request: &PolicyRequestAssistantPreviewConfirmRequest,
    parse_state: PolicyRequestAssistantPreviewConfirmParseState,
) -> PolicyRequestAssistantPreviewConfirmResult {
    if parse_state == PolicyRequestAssistantPreviewConfirmParseState::Rejected {
        return rejected_result(
            request,
            PolicyRequestAssistantPreviewConfirmRejectionReason(
                POLICY_REQUEST_CONFIRM_REJECTION_REASON_INVALID_REQUEST.to_string(),
            ),
        );
    }

    match build_core_child_policy_request(request).and_then(|core_request| {
        let confirmation = build_core_confirmation(request)?;
        confirm_assistant_policy_request_preview(&core_request, confirmation)
    }) {
        Ok(confirmed_request) => {
            let store_outcome =
                persist_supported_policy_preview_confirmation(command, request, &confirmed_request)
                    .await;
            confirmed_result(request, &confirmed_request, &store_outcome)
        }
        Err(error) => rejected_result(
            request,
            PolicyRequestAssistantPreviewConfirmRejectionReason(error.to_string()),
        ),
    }
}

fn build_core_child_policy_request(
    request: &PolicyRequestAssistantPreviewConfirmRequest,
) -> Result<ChildPolicyRequest, EventingError> {
    if request.schema_version != AGENT_PROTOCOL_SCHEMA_VERSION {
        return Err(EventingError::InvalidValue {
            field: constants::field::SCHEMA_VERSION,
            value: request.schema_version.to_string(),
        });
    }

    let rule_id = request
        .rule_id
        .clone()
        .map(PolicyRuleId::parse)
        .transpose()?;
    let requested_bonus_minutes = request
        .requested_bonus_minutes
        .map(PolicyDurationMinutes::new)
        .transpose()?;

    Ok(ChildPolicyRequest {
        schema_version: policy_request_schema_version()?,
        request_id: PolicyRequestId::parse(request.request_id.clone())?,
        submission_key: PolicyRequestSubmissionKey::parse(request.submission_key.clone())?,
        household_id: PolicyHouseholdId::parse(request.household_id.clone())?,
        child_profile_id: PolicyChildProfileId::parse(request.child_profile_id.clone())?,
        device_id: request
            .device_id
            .clone()
            .map(PolicyDeviceId::parse)
            .transpose()?,
        source_document_id: ParentPolicyDocumentId::parse(request.source_document_id.clone())?,
        policy_version: PolicyVersion::new(request.policy_version)?,
        origin: map_request_origin(request.origin),
        assistant_preview_id: Some(
            ocentra_policy_control_core::policy_request::PolicyAssistantPreviewId::parse(
                request.assistant_preview_id.clone(),
            )?,
        ),
        assistant_confirmation_state: map_confirmation_state(request.assistant_confirmation_state),
        status: map_request_status(request.request_status),
        scope: PolicyRequestScope {
            request_kind: map_request_kind(request.request_kind),
            target: PolicyRequestTarget {
                kind: map_target_kind(request.target_kind),
                reference_id: PolicyTargetReferenceId::parse(request.target_reference_id.clone())?,
            },
            requested_action: map_requested_action(request.requested_action),
            rule_id,
            requested_bonus_minutes,
        },
        requested_at: PolicyRequestTimestamp::parse(request.requested_at.clone())?,
        expires_at: PolicyRequestTimestamp::parse(request.expires_at.clone())?,
        audit_reference_ids: request
            .audit_reference_ids
            .iter()
            .cloned()
            .map(PolicyAuditReferenceId::parse)
            .collect::<Result<Vec<_>, _>>()?,
        resolved_approval_id: None,
        resolved_at: None,
    })
}

fn build_core_confirmation(
    request: &PolicyRequestAssistantPreviewConfirmRequest,
) -> Result<AssistantPolicyRequestConfirmation, EventingError> {
    Ok(AssistantPolicyRequestConfirmation {
        actor_id: PolicyActorId::parse(request.confirmation_actor_id.clone())?,
        actor_role: map_actor_role(request.confirmation_actor_role),
        actor_state: map_actor_state(request.confirmation_actor_state),
        confirmed_at: PolicyRequestTimestamp::parse(request.confirmed_at.clone())?,
        audit_reference_id: PolicyAuditReferenceId::parse(
            request.confirmation_audit_reference_id.clone(),
        )?,
    })
}

fn confirmed_result(
    request: &PolicyRequestAssistantPreviewConfirmRequest,
    confirmed_request: &ChildPolicyRequest,
    store_outcome: &PolicyRequestAssistantPreviewConfirmStoreOutcome,
) -> PolicyRequestAssistantPreviewConfirmResult {
    PolicyRequestAssistantPreviewConfirmResult {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        command_id: request.command_id.clone(),
        request_id: request.request_id.clone(),
        assistant_preview_id: Some(request.assistant_preview_id.clone()),
        result_state: PolicyRequestAssistantPreviewConfirmResultState::Confirmed,
        policy_request_status: map_protocol_request_status(confirmed_request.status),
        policy_assistant_confirmation_state: map_protocol_confirmation_state(
            confirmed_request.assistant_confirmation_state,
        ),
        policy_audit_reference_id: confirmed_request
            .audit_reference_ids
            .last()
            .map(|reference| reference.as_str().to_string()),
        confirmed_at: Some(request.confirmed_at.clone()),
        rejection_reason: None,
        command_transport_claim_state: PolicyRequestAssistantPreviewConfirmClaimState::Claimed,
        service_validation_claim_state: PolicyRequestAssistantPreviewConfirmClaimState::Claimed,
        activity_store_mutation_claim_state: store_outcome.activity_store_mutation_claim_state,
        upstream_writer_claim_state: store_outcome.upstream_writer_claim_state,
        read_model_projection_claim_state: store_outcome.read_model_projection_claim_state,
        portal_writable_ui_claim_state: PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed,
        child_device_delivery_claim_state:
            PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed,
        provider_delivery_claim_state: PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed,
        platform_enforcement_claim_state: PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed,
        product_claim_state: PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed,
    }
}

fn rejected_result(
    request: &PolicyRequestAssistantPreviewConfirmRequest,
    rejection_reason: PolicyRequestAssistantPreviewConfirmRejectionReason,
) -> PolicyRequestAssistantPreviewConfirmResult {
    PolicyRequestAssistantPreviewConfirmResult {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        command_id: request.command_id.clone(),
        request_id: request.request_id.clone(),
        assistant_preview_id: Some(request.assistant_preview_id.clone()),
        result_state: PolicyRequestAssistantPreviewConfirmResultState::Rejected,
        policy_request_status: request.request_status,
        policy_assistant_confirmation_state: request.assistant_confirmation_state,
        policy_audit_reference_id: None,
        confirmed_at: None,
        rejection_reason: Some(rejection_reason.0),
        command_transport_claim_state: PolicyRequestAssistantPreviewConfirmClaimState::Claimed,
        service_validation_claim_state: PolicyRequestAssistantPreviewConfirmClaimState::Claimed,
        activity_store_mutation_claim_state:
            PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed,
        upstream_writer_claim_state: PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed,
        read_model_projection_claim_state:
            PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed,
        portal_writable_ui_claim_state: PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed,
        child_device_delivery_claim_state:
            PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed,
        provider_delivery_claim_state: PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed,
        platform_enforcement_claim_state: PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed,
        product_claim_state: PolicyRequestAssistantPreviewConfirmClaimState::Unclaimed,
    }
}

async fn persist_supported_policy_preview_confirmation(
    command: &AgentCommandEnvelope,
    request: &PolicyRequestAssistantPreviewConfirmRequest,
    confirmed_request: &ChildPolicyRequest,
) -> PolicyRequestAssistantPreviewConfirmStoreOutcome {
    let Some(event) = confirmed_policy_preview_activity_event(command, request, confirmed_request)
    else {
        return PolicyRequestAssistantPreviewConfirmStoreOutcome::unclaimed();
    };

    let path = activity_db_path();
    tokio::task::spawn_blocking(move || {
        let store = ActivityStore::open(&path).map_err(|_error| ())?;
        store.ingest_events(&[event]).map_err(|_error| ())?;
        Ok::<(), ()>(())
    })
    .await
    .ok()
    .and_then(Result::ok)
    .map(|_| PolicyRequestAssistantPreviewConfirmStoreOutcome::claimed())
    .unwrap_or_else(PolicyRequestAssistantPreviewConfirmStoreOutcome::unclaimed)
}

fn confirmed_policy_preview_activity_event(
    command: &AgentCommandEnvelope,
    request: &PolicyRequestAssistantPreviewConfirmRequest,
    confirmed_request: &ChildPolicyRequest,
) -> Option<ActivityEvent> {
    let target = supported_policy_preview_target(request)?;
    let canonical_confirmed_request = serde_json::to_string(confirmed_request).ok()?;
    let mut fields = build_confirmed_policy_preview_fields(request, confirmed_request, &target);
    fields.insert(
        constants::policy_control::request::FIELD_CANONICAL_CONFIRMED_REQUEST_JSON.to_string(),
        LogFieldValue::String(canonical_confirmed_request),
    );

    Some(ActivityEvent {
        schema_version: ACTIVITY_SCHEMA_VERSION,
        event_id: request.confirmation_audit_reference_id.clone(),
        observed_at: request.confirmed_at.clone(),
        source: ActivitySource {
            device_id: command.target.device_id.clone(),
            platform: command.target.platform.clone(),
            observer: ActivityObserver::AgentService,
            source_id: POLICY_REQUEST_CONFIRM_SOURCE_ID.to_string(),
        },
        kind: ActivityEventKind::EnforcementAuditRecorded,
        subject: ActivitySubject {
            kind: target.subject_kind,
            subject_id: request.target_reference_id.clone(),
            display_name: Some(target.subject_display_name),
        },
        fields,
        evidence: Vec::new(),
    })
}

fn build_confirmed_policy_preview_fields(
    request: &PolicyRequestAssistantPreviewConfirmRequest,
    confirmed_request: &ChildPolicyRequest,
    target: &SupportedPolicyPreviewTarget,
) -> LogFields {
    let mut fields = LogFields::new();
    insert_target_fields(&mut fields, target);
    insert_policy_metadata_fields(&mut fields, request, confirmed_request);
    insert_review_fields(&mut fields, request);
    insert_optional_subject_fields(&mut fields, confirmed_request, target);
    fields
}

fn insert_target_fields(fields: &mut LogFields, target: &SupportedPolicyPreviewTarget) {
    fields.insert(
        constants::field::POLICY_TARGET_TYPE.to_string(),
        LogFieldValue::String(target.target_type.as_protocol_str().to_string()),
    );
    fields.insert(
        constants::field::POLICY_TARGET_VALUE.to_string(),
        LogFieldValue::String(target.target_value.clone()),
    );
    if let Some((field, value)) = target.subject_field.clone() {
        fields.insert(field.to_string(), LogFieldValue::String(value));
    }
}

fn insert_policy_metadata_fields(
    fields: &mut LogFields,
    request: &PolicyRequestAssistantPreviewConfirmRequest,
    confirmed_request: &ChildPolicyRequest,
) {
    fields.insert(
        constants::field::POLICY_SOURCE_STATUS.to_string(),
        LogFieldValue::String(PolicySourceStatus::Confirmed.as_protocol_str().to_string()),
    );
    fields.insert(
        constants::field::POLICY_SOURCE_SURFACE.to_string(),
        LogFieldValue::String(PolicySourceSurface::AiPreview.as_protocol_str().to_string()),
    );
    fields.insert(
        constants::field::POLICY_REQUEST_ORIGIN.to_string(),
        LogFieldValue::String(request.origin.as_protocol_str().to_string()),
    );
    fields.insert(
        constants::field::POLICY_ASSISTANT_CONFIRMATION_STATE.to_string(),
        LogFieldValue::String(
            map_protocol_confirmation_state(confirmed_request.assistant_confirmation_state)
                .as_protocol_str()
                .to_string(),
        ),
    );
    fields.insert(
        constants::field::POLICY_REQUEST_STATUS.to_string(),
        LogFieldValue::String(
            map_protocol_request_status(confirmed_request.status)
                .as_protocol_str()
                .to_string(),
        ),
    );
}

fn insert_review_fields(
    fields: &mut LogFields,
    request: &PolicyRequestAssistantPreviewConfirmRequest,
) {
    fields.insert(
        constants::field::POLICY_REVIEWED_BY_ACTOR_ID.to_string(),
        LogFieldValue::String(request.confirmation_actor_id.clone()),
    );
    fields.insert(
        constants::field::POLICY_REVIEWED_BY_ACTOR_ROLE.to_string(),
        LogFieldValue::String(
            actor_role_protocol(request.confirmation_actor_role)
                .0
                .to_string(),
        ),
    );
    fields.insert(
        constants::field::POLICY_REVIEWED_AT.to_string(),
        LogFieldValue::String(request.confirmed_at.clone()),
    );
}

fn insert_optional_subject_fields(
    fields: &mut LogFields,
    confirmed_request: &ChildPolicyRequest,
    _target: &SupportedPolicyPreviewTarget,
) {
    if let Some(audit_reference_id) = confirmed_request.audit_reference_ids.last() {
        fields.insert(
            constants::field::POLICY_AUDIT_REFERENCE_ID.to_string(),
            LogFieldValue::String(audit_reference_id.as_str().to_string()),
        );
    }
}

pub(crate) fn default_policy_request_assistant_preview_confirm_request(
) -> PolicyRequestAssistantPreviewConfirmRequest {
    parse::default_policy_request_assistant_preview_confirm_request()
}
