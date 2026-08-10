use ocentra_parent_agent_protocol::activity::{
    ActivityEvent, ActivityEventKind, ActivityObserver, ActivitySource, ActivitySubject,
    ActivitySubjectKind,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, PolicyRequestParentResolutionRequest, PolicyRequestParentResolutionResult,
};
use ocentra_parent_agent_protocol::ACTIVITY_SCHEMA_VERSION;
use ocentra_policy_control_core::policy_request::PolicyRequestResolution;
use ocentra_policy_control_core::policy_source::PolicyTargetKind;

const RESOLUTION_AUDIT_SOURCE_ID: &str = "policy-request-parent-resolution";

struct SerializedAuditFields {
    result_json: String,
    request_json: String,
    override_json: Option<String>,
}

pub(crate) fn build_event(
    command: &AgentCommandEnvelope,
    request: &PolicyRequestParentResolutionRequest,
    resolution: &PolicyRequestResolution,
    result: &PolicyRequestParentResolutionResult,
) -> Option<ActivityEvent> {
    let result_json = serde_json::to_string(result).ok()?;
    let request_json = serde_json::to_string(&resolution.request).ok()?;
    let override_json = resolution
        .temporary_override
        .as_ref()
        .and_then(|value| serde_json::to_string(value).ok());
    let fields = build_fields(
        request,
        resolution,
        SerializedAuditFields {
            result_json,
            request_json,
            override_json,
        },
    );

    Some(ActivityEvent {
        schema_version: ACTIVITY_SCHEMA_VERSION,
        event_id: request.approval_audit_reference_id.clone(),
        observed_at: request.decided_at.clone(),
        source: ActivitySource {
            device_id: command.target.device_id.clone(),
            platform: command.target.platform.clone(),
            observer: ActivityObserver::AgentService,
            source_id: RESOLUTION_AUDIT_SOURCE_ID.to_string(),
        },
        kind: ActivityEventKind::EnforcementAuditRecorded,
        subject: ActivitySubject {
            kind: target_subject_kind(resolution.request.scope.target.kind),
            subject_id: resolution
                .request
                .scope
                .target
                .reference_id
                .as_str()
                .to_string(),
            display_name: Some(
                resolution
                    .request
                    .scope
                    .target
                    .reference_id
                    .as_str()
                    .to_string(),
            ),
        },
        fields,
        evidence: Vec::new(),
    })
}

fn build_fields(
    request: &PolicyRequestParentResolutionRequest,
    resolution: &PolicyRequestResolution,
    serialized: SerializedAuditFields,
) -> LogFields {
    let mut fields = LogFields::new();
    fields.insert(
        constants::field::POLICY_REQUEST_STATUS.to_string(),
        LogFieldValue::String(resolution.request.status.as_protocol_str().to_string()),
    );
    fields.insert(
        constants::field::POLICY_AUDIT_REFERENCE_ID.to_string(),
        LogFieldValue::String(request.approval_audit_reference_id.clone()),
    );
    fields.insert(
        constants::field::POLICY_REVIEWED_BY_ACTOR_ID.to_string(),
        LogFieldValue::String(request.parent_actor_id.clone()),
    );
    fields.insert(
        constants::field::POLICY_REVIEWED_AT.to_string(),
        LogFieldValue::String(request.decided_at.clone()),
    );
    fields.insert(
        constants::field::POLICY_REQUEST_PARENT_RESOLUTION_RESULT.to_string(),
        LogFieldValue::String(serialized.result_json),
    );
    fields.insert(
        constants::policy_control::request::FIELD_CANONICAL_RESOLVED_REQUEST_JSON.to_string(),
        LogFieldValue::String(serialized.request_json),
    );
    if let Some(override_json) = serialized.override_json {
        fields.insert(
            constants::policy_control::request::FIELD_CANONICAL_TEMPORARY_OVERRIDE_JSON.to_string(),
            LogFieldValue::String(override_json),
        );
    }
    fields
}

fn target_subject_kind(target_kind: PolicyTargetKind) -> ActivitySubjectKind {
    match target_kind {
        PolicyTargetKind::App => ActivitySubjectKind::Process,
        PolicyTargetKind::Site => ActivitySubjectKind::Url,
        _ => ActivitySubjectKind::Device,
    }
}
