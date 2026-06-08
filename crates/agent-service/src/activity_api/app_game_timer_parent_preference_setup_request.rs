use std::path::PathBuf;

use ocentra_parent_agent_core::ActivityStore;
use ocentra_parent_agent_protocol::{
    constants, ActivityEvent, ActivityEventKind, ActivityObserver, ActivitySource, ActivitySubject,
    ActivitySubjectKind, AgentCommandEnvelope, AgentEventEnvelope, AgentEventName,
    AppGameControlActionResult, AppGameControlApprovalDecision, AppGameControlApprovalRequest,
    AppGameEnforcementCapabilityStatus, AppGameParentActionReference, AppGameParentActorReference,
    AppGameParentDeviceReference, AppGameParentEvidenceReference, AppGamePolicyTarget,
    AppGameTimerParentPreferenceSetupRequest, AppGameTimerParentPreferenceSetupRequestResult,
    LogFieldValue, LogFields, LogLevel, ACTIVITY_SCHEMA_VERSION,
    APP_GAME_CONTROL_ACTION_STATUS_MANUAL_REQUIRED,
    APP_GAME_CONTROL_APPROVAL_STATE_MANUAL_REQUIRED, APP_GAME_CONTROL_CHILD_REASON_NOT_REQUESTED,
    APP_GAME_CONTROL_DECISION_APPROVED, APP_GAME_CONTROL_EVIDENCE_PROOF_APP_IDENTITY,
    APP_GAME_CONTROL_PARENT_RESPONSE_ALLOW_ONCE, APP_GAME_CONTROL_PERSISTENCE_REPLAYABLE,
    APP_GAME_CONTROL_POLICY_KIND_APP, APP_GAME_CONTROL_UNANSWERED_FALLBACK_DENY,
    APP_GAME_ENFORCEMENT_ADAPTER_PROCESS_CONTROL, APP_GAME_ENFORCEMENT_CAPABILITY_MANUAL_REQUIRED,
    APP_GAME_JOURNAL_AUTHORITY_SUBJECT_ID, APP_GAME_JOURNAL_CUSTODY_LOCAL_JOURNAL,
    APP_GAME_JOURNAL_FIELD_CUSTODY_LABEL, APP_GAME_JOURNAL_FIELD_REPLAY_STATE,
    APP_GAME_JOURNAL_FIELD_ROW_JSON, APP_GAME_JOURNAL_FIELD_ROW_KIND,
    APP_GAME_JOURNAL_REPLAY_STATE_STORED, APP_GAME_JOURNAL_ROW_KIND_APPROVAL_ACTION_RESULT,
    APP_GAME_JOURNAL_SOURCE_ID, APP_GAME_PARENT_ACTOR_ROLE_PARENT,
    APP_GAME_PARENT_CONTRACT_SCHEMA_VERSION, APP_GAME_PARENT_EVIDENCE_KIND_ACTIVITY_EVENT,
    APP_GAME_PLATFORM_ACTION_BLOCK_LAUNCH, APP_GAME_POLICY_TARGET_TYPE_APP,
};

use crate::{
    activity_store_path::activity_db_path, event_builder::build_event, fields::fields_from_pairs,
    time::timestamp_now,
};

type FieldPair = (&'static str, LogFieldValue);

pub async fn build_activity_app_game_timer_parent_preference_setup_request_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    build_activity_app_game_timer_parent_preference_setup_request_report_for_store_path(
        command,
        activity_db_path(),
    )
    .await
}

pub(crate) async fn build_activity_app_game_timer_parent_preference_setup_request_report_for_store_path(
    command: AgentCommandEnvelope,
    store_path: PathBuf,
) -> AgentEventEnvelope {
    let mut result = app_game_timer_parent_preference_setup_request_from_command(&command);
    if persist_action_result(&command, &result, store_path).await {
        result.action_result_persistence_status =
            constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_ACTION_RESULT_PERSISTED.to_string();
        result.action_result_persistence_claimed = true;
    }
    build_event(
        constants::event_id::ACTIVITY_APP_GAME_TIMER_PARENT_PREFERENCE_SETUP_REQUESTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentActivityAppGameTimerParentPreferenceSetupRequested,
        LogLevel::Info,
        app_game_timer_parent_preference_setup_request_payload(&result),
        None,
    )
}

pub fn app_game_timer_parent_preference_setup_request_from_command(
    command: &AgentCommandEnvelope,
) -> AppGameTimerParentPreferenceSetupRequestResult {
    let request = request_from_command(command);
    let action_result_reference_ids = action_result_reference_ids(&request);
    AppGameTimerParentPreferenceSetupRequestResult {
        schema_version: constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_REQUEST_SCHEMA_VERSION
            .to_string(),
        request_id: request.request_id,
        requested_at: request.requested_at,
        accepted_at: timestamp_now(),
        request_status: constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_REQUEST_ACCEPTED
            .to_string(),
        parent_surface_intent_reference_id: request.parent_surface_intent_reference_id,
        parent_preference_setup_reference_id: request.parent_preference_setup_reference_id.clone(),
        request_reference_ids: request.request_reference_ids,
        action_result_reference_id: request.parent_preference_setup_reference_id.clone(),
        action_result_reference_ids,
        action_result_persistence_status:
            constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_ACTION_RESULT_UNAVAILABLE.to_string(),
        command_boundary_claimed: true,
        action_result_handoff_claimed: true,
        action_result_persistence_claimed: false,
        parent_preference_mutation_claimed: false,
        notification_rule_mutation_claimed: false,
        provider_delivery_claimed: false,
        durable_outbox_claimed: false,
        adapter_dispatch_claimed: false,
        platform_enforcement_claimed: false,
    }
}

async fn persist_action_result(
    command: &AgentCommandEnvelope,
    result: &AppGameTimerParentPreferenceSetupRequestResult,
    store_path: PathBuf,
) -> bool {
    let event = action_result_activity_event(command, result);
    tokio::task::spawn_blocking(move || {
        let store = ActivityStore::open(store_path).map_err(|_| ())?;
        store.ingest_events(&[event]).map_err(|_| ())?;
        Ok::<(), ()>(())
    })
    .await
    .ok()
    .and_then(Result::ok)
    .is_some()
}

fn action_result_activity_event(
    command: &AgentCommandEnvelope,
    result: &AppGameTimerParentPreferenceSetupRequestResult,
) -> ActivityEvent {
    let row = action_result_row(command, result);
    let mut fields = LogFields::new();
    fields.insert(
        APP_GAME_JOURNAL_FIELD_ROW_KIND.to_string(),
        LogFieldValue::String(APP_GAME_JOURNAL_ROW_KIND_APPROVAL_ACTION_RESULT.to_string()),
    );
    fields.insert(
        APP_GAME_JOURNAL_FIELD_CUSTODY_LABEL.to_string(),
        LogFieldValue::String(APP_GAME_JOURNAL_CUSTODY_LOCAL_JOURNAL.to_string()),
    );
    fields.insert(
        APP_GAME_JOURNAL_FIELD_REPLAY_STATE.to_string(),
        LogFieldValue::String(APP_GAME_JOURNAL_REPLAY_STATE_STORED.to_string()),
    );
    fields.insert(
        APP_GAME_JOURNAL_FIELD_ROW_JSON.to_string(),
        LogFieldValue::String(
            serde_json::to_string(&row).expect(constants::error::AGENT_EVENT_SERIALIZES),
        ),
    );

    ActivityEvent {
        schema_version: ACTIVITY_SCHEMA_VERSION,
        event_id: row.result_id.clone(),
        observed_at: row.recorded_at.clone(),
        source: ActivitySource {
            device_id: command.target.device_id.clone(),
            platform: command.target.platform.clone(),
            observer: ActivityObserver::AgentService,
            source_id: APP_GAME_JOURNAL_SOURCE_ID.to_string(),
        },
        kind: ActivityEventKind::EnforcementAuditRecorded,
        subject: ActivitySubject {
            kind: ActivitySubjectKind::Device,
            subject_id: APP_GAME_JOURNAL_AUTHORITY_SUBJECT_ID.to_string(),
            display_name: Some(row.result_status.clone()),
        },
        fields,
        evidence: Vec::new(),
    }
}

fn action_result_row(
    command: &AgentCommandEnvelope,
    result: &AppGameTimerParentPreferenceSetupRequestResult,
) -> AppGameControlActionResult {
    let capability = manual_required_capability(command, result);
    AppGameControlActionResult {
        schema_version: APP_GAME_PARENT_CONTRACT_SCHEMA_VERSION.to_string(),
        result_id: result.action_result_reference_id.clone(),
        request: approval_request(command, result),
        decision: approval_decision(command, result),
        approval_state: APP_GAME_CONTROL_APPROVAL_STATE_MANUAL_REQUIRED.to_string(),
        capability_state: APP_GAME_ENFORCEMENT_CAPABILITY_MANUAL_REQUIRED.to_string(),
        capability,
        evidence_proof_kind: APP_GAME_CONTROL_EVIDENCE_PROOF_APP_IDENTITY.to_string(),
        result_status: APP_GAME_CONTROL_ACTION_STATUS_MANUAL_REQUIRED.to_string(),
        enforcement_result: None,
        recorded_at: result.accepted_at.clone(),
    }
}

fn approval_request(
    command: &AgentCommandEnvelope,
    result: &AppGameTimerParentPreferenceSetupRequestResult,
) -> AppGameControlApprovalRequest {
    AppGameControlApprovalRequest {
        schema_version: APP_GAME_PARENT_CONTRACT_SCHEMA_VERSION.to_string(),
        request_id: result.request_id.clone(),
        policy_kind: APP_GAME_CONTROL_POLICY_KIND_APP.to_string(),
        device: parent_device(command),
        target: AppGamePolicyTarget {
            target_id: result.parent_preference_setup_reference_id.clone(),
            target_type: APP_GAME_POLICY_TARGET_TYPE_APP.to_string(),
            target_value: result.parent_preference_setup_reference_id.clone(),
        },
        requested_action: APP_GAME_PLATFORM_ACTION_BLOCK_LAUNCH.to_string(),
        requested_mode: None,
        requested_setting_refs: Vec::new(),
        evidence_references: evidence_references(result),
        candidate: None,
        child_reason_state: APP_GAME_CONTROL_CHILD_REASON_NOT_REQUESTED.to_string(),
        child_reason_references: result.request_reference_ids.clone(),
        child_status_references: result.action_result_reference_ids.clone(),
        expires_at: result.accepted_at.clone(),
        unanswered_fallback: APP_GAME_CONTROL_UNANSWERED_FALLBACK_DENY.to_string(),
    }
}

fn approval_decision(
    command: &AgentCommandEnvelope,
    result: &AppGameTimerParentPreferenceSetupRequestResult,
) -> AppGameControlApprovalDecision {
    AppGameControlApprovalDecision {
        schema_version: APP_GAME_PARENT_CONTRACT_SCHEMA_VERSION.to_string(),
        decision_id: result.action_result_reference_id.clone(),
        request_id: result.request_id.clone(),
        policy_kind: APP_GAME_CONTROL_POLICY_KIND_APP.to_string(),
        decision_state: APP_GAME_CONTROL_DECISION_APPROVED.to_string(),
        parent_action: Some(AppGameParentActionReference {
            action_reference_id: result.parent_preference_setup_reference_id.clone(),
            actor: parent_actor(command),
            policy_version: result.schema_version.clone(),
            created_at: result.accepted_at.clone(),
        }),
        reason_codes: result.request_reference_ids.clone(),
        policy_version: result.schema_version.clone(),
        evidence_references: evidence_references(result),
        response_scope: Some(APP_GAME_CONTROL_PARENT_RESPONSE_ALLOW_ONCE.to_string()),
        decision_expires_at: None,
        audit_references: result.action_result_reference_ids.clone(),
        persistence_state: APP_GAME_CONTROL_PERSISTENCE_REPLAYABLE.to_string(),
        decided_at: result.accepted_at.clone(),
    }
}

fn manual_required_capability(
    command: &AgentCommandEnvelope,
    result: &AppGameTimerParentPreferenceSetupRequestResult,
) -> AppGameEnforcementCapabilityStatus {
    AppGameEnforcementCapabilityStatus {
        schema_version: APP_GAME_PARENT_CONTRACT_SCHEMA_VERSION.to_string(),
        platform: command.target.platform.clone(),
        adapter_kind: APP_GAME_ENFORCEMENT_ADAPTER_PROCESS_CONTROL.to_string(),
        capability_state: APP_GAME_ENFORCEMENT_CAPABILITY_MANUAL_REQUIRED.to_string(),
        permission_state: APP_GAME_ENFORCEMENT_CAPABILITY_MANUAL_REQUIRED.to_string(),
        dependency_state: APP_GAME_ENFORCEMENT_CAPABILITY_MANUAL_REQUIRED.to_string(),
        supported_actions: vec![APP_GAME_PLATFORM_ACTION_BLOCK_LAUNCH.to_string()],
        degraded_reason: Some(result.parent_preference_setup_reference_id.clone()),
        last_checked_at: result.accepted_at.clone(),
    }
}

fn parent_device(command: &AgentCommandEnvelope) -> AppGameParentDeviceReference {
    AppGameParentDeviceReference {
        device_id: command.target.device_id.clone(),
        child_profile_id: None,
        label: command.target.device_id.clone(),
        platform: command.target.platform.clone(),
    }
}

fn parent_actor(command: &AgentCommandEnvelope) -> AppGameParentActorReference {
    AppGameParentActorReference {
        actor_id: command.source.peer_id.clone(),
        role: APP_GAME_PARENT_ACTOR_ROLE_PARENT.to_string(),
    }
}

fn evidence_references(
    result: &AppGameTimerParentPreferenceSetupRequestResult,
) -> Vec<AppGameParentEvidenceReference> {
    result
        .request_reference_ids
        .iter()
        .map(|reference_id| AppGameParentEvidenceReference {
            evidence_reference_id: reference_id.clone(),
            kind: APP_GAME_PARENT_EVIDENCE_KIND_ACTIVITY_EVENT.to_string(),
            observed_at: result.requested_at.clone(),
        })
        .collect()
}

fn action_result_reference_ids(request: &AppGameTimerParentPreferenceSetupRequest) -> Vec<String> {
    let mut refs = vec![request.parent_preference_setup_reference_id.clone()];
    refs.extend(request.request_reference_ids.clone());
    unique_refs(refs)
}

fn unique_refs(reference_ids: Vec<String>) -> Vec<String> {
    let mut unique = Vec::new();
    for reference_id in reference_ids {
        if reference_id.is_empty() || unique.iter().any(|existing| existing == &reference_id) {
            continue;
        }
        unique.push(reference_id);
    }
    unique
}

pub fn app_game_timer_parent_preference_setup_request_payload(
    result: &AppGameTimerParentPreferenceSetupRequestResult,
) -> LogFields {
    fields_from_pairs(result_pairs(result))
}

fn result_pairs(result: &AppGameTimerParentPreferenceSetupRequestResult) -> Vec<FieldPair> {
    vec![
        (
            constants::field::GENERATED_AT,
            LogFieldValue::String(result.accepted_at.clone()),
        ),
        (
            constants::field::CAPABILITY_STATUS,
            LogFieldValue::String(result.request_status.clone()),
        ),
        (
            constants::field::APP_GAME_TIMER_PARENT_PREFERENCE_SETUP_REQUEST,
            LogFieldValue::String(
                serde_json::to_string(result).expect(constants::error::AGENT_EVENT_SERIALIZES),
            ),
        ),
    ]
}

fn request_from_command(
    command: &AgentCommandEnvelope,
) -> AppGameTimerParentPreferenceSetupRequest {
    match command
        .payload
        .get(constants::field::APP_GAME_TIMER_PARENT_PREFERENCE_SETUP_REQUEST)
    {
        Some(LogFieldValue::String(value)) if !value.is_empty() => {
            serde_json::from_str(value).expect(constants::error::AGENT_EVENT_SERIALIZES)
        }
        _ => AppGameTimerParentPreferenceSetupRequest {
            request_id: command.message_id.clone(),
            requested_at: command.sent_at.clone(),
            parent_surface_intent_reference_id: command.message_id.clone(),
            parent_preference_setup_reference_id: command.message_id.clone(),
            request_reference_ids: vec![command.message_id.clone()],
        },
    }
}
