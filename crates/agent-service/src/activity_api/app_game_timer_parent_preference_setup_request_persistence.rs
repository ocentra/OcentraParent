use std::path::PathBuf;

use ocentra_parent_agent_core::ActivityStore;
use ocentra_parent_agent_protocol::{
    constants, ActivityEvent, ActivityEventKind, ActivityEvidenceKind, ActivityEvidenceRef,
    ActivityObserver, ActivitySource, ActivitySubject, ActivitySubjectKind, AgentCommandEnvelope,
    AppGameControlActionResult, AppGameControlApprovalDecision, AppGameControlApprovalRequest,
    AppGameEnforcementCapabilityStatus, AppGameParentActionReference, AppGameParentActorReference,
    AppGameParentDeviceReference, AppGameParentEvidenceReference, AppGamePolicyTarget,
    AppGameTimerParentPreferenceSetupRequestResult, LogFieldValue, LogFields,
    ACTIVITY_SCHEMA_VERSION, APP_GAME_CONTROL_ACTION_STATUS_MANUAL_REQUIRED,
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

pub(crate) async fn persist_setup_handoff(
    command: &AgentCommandEnvelope,
    result: &AppGameTimerParentPreferenceSetupRequestResult,
    store_path: PathBuf,
) -> bool {
    let persisted_result = persisted_result(result);
    let action_result_event = action_result_activity_event(command, &persisted_result);
    let mutation_receipt_event = mutation_receipt_activity_event(command, &persisted_result);
    let child_runtime_delivery_handoff_event =
        child_runtime_delivery_handoff_activity_event(command, &persisted_result);
    let child_runtime_delivery_queue_event =
        child_runtime_delivery_queue_activity_event(command, &persisted_result);
    let child_runtime_delivery_dispatch_event =
        child_runtime_delivery_dispatch_activity_event(command, &persisted_result);
    let child_runtime_delivery_receipt_requirement_event =
        child_runtime_delivery_receipt_requirement_activity_event(command, &persisted_result);
    tokio::task::spawn_blocking(move || {
        let store = ActivityStore::open(store_path).map_err(|_| ())?;
        store
            .ingest_events(&[
                action_result_event,
                mutation_receipt_event,
                child_runtime_delivery_handoff_event,
                child_runtime_delivery_queue_event,
                child_runtime_delivery_dispatch_event,
                child_runtime_delivery_receipt_requirement_event,
            ])
            .map_err(|_| ())?;
        Ok::<(), ()>(())
    })
    .await
    .ok()
    .and_then(Result::ok)
    .is_some()
}

fn persisted_result(
    result: &AppGameTimerParentPreferenceSetupRequestResult,
) -> AppGameTimerParentPreferenceSetupRequestResult {
    let mut persisted = result.clone();
    persisted.action_result_persistence_status =
        constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_ACTION_RESULT_PERSISTED.to_string();
    persisted.action_result_persistence_claimed = true;
    persisted.parent_preference_mutation_receipt_status =
        constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_MUTATION_RECEIPT_PERSISTED.to_string();
    persisted.parent_preference_mutation_receipt_claimed = true;
    persisted.child_runtime_delivery_handoff_status =
        constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_HANDOFF_READY
            .to_string();
    persisted.child_runtime_delivery_handoff_claimed = true;
    persisted.child_runtime_delivery_queue_status =
        constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_QUEUE_QUEUED
            .to_string();
    persisted.child_runtime_delivery_queue_claimed = true;
    persisted.child_runtime_delivery_dispatch_status =
        constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_DISPATCH_READY
            .to_string();
    persisted.child_runtime_delivery_dispatch_claimed = true;
    persisted.child_runtime_delivery_receipt_requirement_status =
        constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_RECEIPT_REQUIRED
            .to_string();
    persisted.child_runtime_delivery_receipt_requirement_claimed = true;
    persisted
}

fn child_runtime_delivery_receipt_requirement_activity_event(
    command: &AgentCommandEnvelope,
    result: &AppGameTimerParentPreferenceSetupRequestResult,
) -> ActivityEvent {
    let mut fields = LogFields::new();
    fields.insert(
        constants::field::APP_GAME_TIMER_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_RECEIPT_REQUIREMENT
            .to_string(),
        LogFieldValue::String(
            serde_json::to_string(result).expect(constants::error::AGENT_EVENT_SERIALIZES),
        ),
    );
    fields.insert(
        constants::field::APP_GAME_TIMER_PARENT_PREFERENCE_SETUP_REQUEST.to_string(),
        LogFieldValue::String(result.request_id.clone()),
    );
    fields.insert(
        constants::field::CAPABILITY_STATUS.to_string(),
        LogFieldValue::String(
            constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_RECEIPT_REQUIRED
                .to_string(),
        ),
    );
    fields.insert(
        constants::field::CHILD_DELIVERY_STATE.to_string(),
        LogFieldValue::String(
            constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_RECEIPT_REQUIRED
                .to_string(),
        ),
    );

    ActivityEvent {
        schema_version: ACTIVITY_SCHEMA_VERSION,
        event_id: result
            .child_runtime_delivery_receipt_requirement_id
            .clone(),
        observed_at: result.accepted_at.clone(),
        source: ActivitySource {
            device_id: command.target.device_id.clone(),
            platform: command.target.platform.clone(),
            observer: ActivityObserver::AgentService,
            source_id: APP_GAME_JOURNAL_SOURCE_ID.to_string(),
        },
        kind: ActivityEventKind::EnforcementAuditRecorded,
        subject: ActivitySubject {
            kind: ActivitySubjectKind::Device,
            subject_id: result.parent_preference_setup_reference_id.clone(),
            display_name: Some(
                constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_RECEIPT_REQUIRED
                    .to_string(),
            ),
        },
        fields,
        evidence: evidence_references(result)
            .into_iter()
            .map(|reference| ActivityEvidenceRef {
                evidence_id: reference.evidence_reference_id,
                kind: ActivityEvidenceKind::LocalDbRow,
                digest: None,
                uri: None,
            })
            .collect(),
    }
}

fn child_runtime_delivery_dispatch_activity_event(
    command: &AgentCommandEnvelope,
    result: &AppGameTimerParentPreferenceSetupRequestResult,
) -> ActivityEvent {
    let mut fields = LogFields::new();
    fields.insert(
        constants::field::APP_GAME_TIMER_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_DISPATCH
            .to_string(),
        LogFieldValue::String(
            serde_json::to_string(result).expect(constants::error::AGENT_EVENT_SERIALIZES),
        ),
    );
    fields.insert(
        constants::field::APP_GAME_TIMER_PARENT_PREFERENCE_SETUP_REQUEST.to_string(),
        LogFieldValue::String(result.request_id.clone()),
    );
    fields.insert(
        constants::field::CAPABILITY_STATUS.to_string(),
        LogFieldValue::String(
            constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_DISPATCH_READY
                .to_string(),
        ),
    );
    fields.insert(
        constants::field::CHILD_DELIVERY_STATE.to_string(),
        LogFieldValue::String(
            constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_DISPATCH_READY
                .to_string(),
        ),
    );

    ActivityEvent {
        schema_version: ACTIVITY_SCHEMA_VERSION,
        event_id: result.child_runtime_delivery_dispatch_id.clone(),
        observed_at: result.accepted_at.clone(),
        source: ActivitySource {
            device_id: command.target.device_id.clone(),
            platform: command.target.platform.clone(),
            observer: ActivityObserver::AgentService,
            source_id: APP_GAME_JOURNAL_SOURCE_ID.to_string(),
        },
        kind: ActivityEventKind::EnforcementAuditRecorded,
        subject: ActivitySubject {
            kind: ActivitySubjectKind::Device,
            subject_id: result.parent_preference_setup_reference_id.clone(),
            display_name: Some(
                constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_DISPATCH_READY
                    .to_string(),
            ),
        },
        fields,
        evidence: evidence_references(result)
            .into_iter()
            .map(|reference| ActivityEvidenceRef {
                evidence_id: reference.evidence_reference_id,
                kind: ActivityEvidenceKind::LocalDbRow,
                digest: None,
                uri: None,
            })
            .collect(),
    }
}

fn child_runtime_delivery_queue_activity_event(
    command: &AgentCommandEnvelope,
    result: &AppGameTimerParentPreferenceSetupRequestResult,
) -> ActivityEvent {
    let mut fields = LogFields::new();
    fields.insert(
        constants::field::APP_GAME_TIMER_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_QUEUE
            .to_string(),
        LogFieldValue::String(
            serde_json::to_string(result).expect(constants::error::AGENT_EVENT_SERIALIZES),
        ),
    );
    fields.insert(
        constants::field::APP_GAME_TIMER_PARENT_PREFERENCE_SETUP_REQUEST.to_string(),
        LogFieldValue::String(result.request_id.clone()),
    );
    fields.insert(
        constants::field::CAPABILITY_STATUS.to_string(),
        LogFieldValue::String(
            constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_QUEUE_QUEUED
                .to_string(),
        ),
    );
    fields.insert(
        constants::field::CHILD_DELIVERY_STATE.to_string(),
        LogFieldValue::String(
            constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_QUEUE_QUEUED
                .to_string(),
        ),
    );

    ActivityEvent {
        schema_version: ACTIVITY_SCHEMA_VERSION,
        event_id: result.child_runtime_delivery_queue_id.clone(),
        observed_at: result.accepted_at.clone(),
        source: ActivitySource {
            device_id: command.target.device_id.clone(),
            platform: command.target.platform.clone(),
            observer: ActivityObserver::AgentService,
            source_id: APP_GAME_JOURNAL_SOURCE_ID.to_string(),
        },
        kind: ActivityEventKind::EnforcementAuditRecorded,
        subject: ActivitySubject {
            kind: ActivitySubjectKind::Device,
            subject_id: result.parent_preference_setup_reference_id.clone(),
            display_name: Some(
                constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_QUEUE_QUEUED
                    .to_string(),
            ),
        },
        fields,
        evidence: evidence_references(result)
            .into_iter()
            .map(|reference| ActivityEvidenceRef {
                evidence_id: reference.evidence_reference_id,
                kind: ActivityEvidenceKind::LocalDbRow,
                digest: None,
                uri: None,
            })
            .collect(),
    }
}

fn child_runtime_delivery_handoff_activity_event(
    command: &AgentCommandEnvelope,
    result: &AppGameTimerParentPreferenceSetupRequestResult,
) -> ActivityEvent {
    let mut fields = LogFields::new();
    fields.insert(
        constants::field::APP_GAME_TIMER_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_HANDOFF
            .to_string(),
        LogFieldValue::String(
            serde_json::to_string(result).expect(constants::error::AGENT_EVENT_SERIALIZES),
        ),
    );
    fields.insert(
        constants::field::APP_GAME_TIMER_PARENT_PREFERENCE_SETUP_REQUEST.to_string(),
        LogFieldValue::String(result.request_id.clone()),
    );
    fields.insert(
        constants::field::CAPABILITY_STATUS.to_string(),
        LogFieldValue::String(
            constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_HANDOFF_READY
                .to_string(),
        ),
    );
    fields.insert(
        constants::field::CHILD_DELIVERY_STATE.to_string(),
        LogFieldValue::String(
            constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_HANDOFF_READY
                .to_string(),
        ),
    );

    ActivityEvent {
        schema_version: ACTIVITY_SCHEMA_VERSION,
        event_id: result.child_runtime_delivery_handoff_id.clone(),
        observed_at: result.accepted_at.clone(),
        source: ActivitySource {
            device_id: command.target.device_id.clone(),
            platform: command.target.platform.clone(),
            observer: ActivityObserver::AgentService,
            source_id: APP_GAME_JOURNAL_SOURCE_ID.to_string(),
        },
        kind: ActivityEventKind::EnforcementAuditRecorded,
        subject: ActivitySubject {
            kind: ActivitySubjectKind::Device,
            subject_id: result.parent_preference_setup_reference_id.clone(),
            display_name: Some(
                constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_CHILD_RUNTIME_DELIVERY_HANDOFF_READY
                    .to_string(),
            ),
        },
        fields,
        evidence: evidence_references(result)
            .into_iter()
            .map(|reference| ActivityEvidenceRef {
                evidence_id: reference.evidence_reference_id,
                kind: ActivityEvidenceKind::LocalDbRow,
                digest: None,
                uri: None,
            })
            .collect(),
    }
}

fn mutation_receipt_activity_event(
    command: &AgentCommandEnvelope,
    result: &AppGameTimerParentPreferenceSetupRequestResult,
) -> ActivityEvent {
    let mut fields = LogFields::new();
    fields.insert(
        constants::field::APP_GAME_TIMER_PARENT_PREFERENCE_SETUP_MUTATION_RECEIPT.to_string(),
        LogFieldValue::String(
            serde_json::to_string(result).expect(constants::error::AGENT_EVENT_SERIALIZES),
        ),
    );
    fields.insert(
        constants::field::APP_GAME_TIMER_PARENT_PREFERENCE_SETUP_REQUEST.to_string(),
        LogFieldValue::String(result.request_id.clone()),
    );
    fields.insert(
        constants::field::CAPABILITY_STATUS.to_string(),
        LogFieldValue::String(
            constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_MUTATION_RECEIPT_PERSISTED
                .to_string(),
        ),
    );

    ActivityEvent {
        schema_version: ACTIVITY_SCHEMA_VERSION,
        event_id: result.parent_preference_mutation_receipt_id.clone(),
        observed_at: result.accepted_at.clone(),
        source: ActivitySource {
            device_id: command.target.device_id.clone(),
            platform: command.target.platform.clone(),
            observer: ActivityObserver::AgentService,
            source_id: APP_GAME_JOURNAL_SOURCE_ID.to_string(),
        },
        kind: ActivityEventKind::EnforcementAuditRecorded,
        subject: ActivitySubject {
            kind: ActivitySubjectKind::Device,
            subject_id: result.parent_preference_setup_reference_id.clone(),
            display_name: Some(
                constants::value::APP_GAME_PARENT_PREFERENCE_SETUP_MUTATION_RECEIPT_PERSISTED
                    .to_string(),
            ),
        },
        fields,
        evidence: evidence_references(result)
            .into_iter()
            .map(|reference| ActivityEvidenceRef {
                evidence_id: reference.evidence_reference_id,
                kind: ActivityEvidenceKind::LocalDbRow,
                digest: None,
                uri: None,
            })
            .collect(),
    }
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
