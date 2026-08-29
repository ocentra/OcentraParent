use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::app_game::*;
use ocentra_parent_agent_protocol::app_game_authority_classifier::*;
use std::fs::remove_file;

use ocentra_parent_agent_protocol::activity::{
    ActivityEvent, ActivityEvidenceKind, ActivityEvidenceRef,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::journal::ActivityJournalLine;
use ocentra_parent_agent_protocol::logging::LogFieldValue;

use crate::{
    activity_store::ActivityStore,
    journal::ActivityJournal,
    journal_crypto::{JournalKey, JOURNAL_KEY_BYTES},
};
use ocentra_parent_agent_core::activity_store_error::ActivityStoreError;

use super::app_game_journal_sqlite_ingest::{
    protocol_rows::{
        app_game_ai_classifier_result_journal_event, app_game_approval_action_result_journal_event,
        app_game_approval_authority_journal_event, app_game_evidence_claim_journal_event,
        app_game_identity_journal_event, app_game_platform_authority_matrix_journal_event,
    },
    read_model::app_game_journal_sqlite_read_model,
    AppGameJournalSqliteIngestError,
};

#[test]
fn journal_replay_projects_new_protocol_boundary_rows() {
    let (store, lines) = append_and_replay(&protocol_boundary_events());
    let model = app_game_journal_sqlite_read_model(
        store.connection_for_test(),
        constants::activity_store::DEFAULT_RECENT_LIMIT,
        APP_GAME_TEST_TIMESTAMP,
    )
    .expect_value(constants::error::ACTIVITY_STORE_QUERIES);

    assert_eq!(lines.len(), 6);
    assert_eq!(model.evidence_claim_returned, 1);
    assert_eq!(model.identity_returned, 1);
    assert_eq!(model.approval_authority_returned, 1);
    assert_eq!(model.approval_action_result_returned, 1);
    assert_eq!(model.platform_authority_matrix_returned, 1);
    assert_eq!(model.ai_classifier_result_returned, 1);
    assert_eq!(
        model.evidence_claim_rows[0].claim_id,
        APP_GAME_TEST_EVIDENCE_CLAIM_ID
    );
    assert_eq!(
        model.identity_rows[0].identity_id,
        APP_GAME_TEST_IDENTITY_ID
    );
    assert_eq!(
        model.approval_authority_rows[0].authority_id,
        APP_GAME_TEST_AUTHORITY_ID
    );
    assert_eq!(
        model.approval_action_result_rows[0].result_status,
        APP_GAME_CONTROL_ACTION_STATUS_MANUAL_REQUIRED
    );
    assert!(!model.platform_authority_matrices[0].rows[0].can_execute_adapter);
    assert!(!model.ai_classifier_result_rows[0].direct_action_requested);
}

#[test]
fn invalid_protocol_boundary_rows_are_rejected_before_sqlite_ingest() {
    let mut claim = evidence_claim();
    claim.runtime_state = APP_GAME_RUNTIME_RUNNING.to_string();
    let mut authority = approval_authority();
    authority.authority_state = APP_GAME_CONTROL_AUTHORITY_OBSERVE_ONLY.to_string();
    let mut action = manual_action_result();
    action.enforcement_result = Some(AppGameEnforcementResult {
        schema_version: APP_GAME_PARENT_CONTRACT_SCHEMA_VERSION.to_string(),
        result_id: APP_GAME_TEST_ENFORCEMENT_RESULT_ID.to_string(),
        action_id: APP_GAME_TEST_ENFORCEMENT_ACTION_ID.to_string(),
        status: APP_GAME_ENFORCEMENT_RESULT_ACTUALLY_ENFORCED.to_string(),
        adapter_result_code: APP_GAME_ENFORCEMENT_ADAPTER_RESULT_PROCESS_TERMINATED.to_string(),
        started_at: APP_GAME_TEST_TIMESTAMP.to_string(),
        completed_at: Some(APP_GAME_TEST_TIMESTAMP.to_string()),
        rollback_token: None,
        rollback_state: APP_GAME_ENFORCEMENT_ROLLBACK_NOT_REQUIRED.to_string(),
        unavailable_reason: None,
        unavailable_status: None,
        failed_reason: None,
        next_check_at: None,
        capability: capability(),
    });
    let mut matrix = platform_matrix();
    matrix.rows[0].can_execute_adapter = true;
    let mut classifier = classifier_result();
    classifier.direct_action_requested = true;

    assert_eq!(
        app_game_evidence_claim_journal_event(
            constants::peer::LOCAL_DEV_AGENT,
            std::env::consts::OS,
            &claim
        ),
        Err(AppGameJournalSqliteIngestError::EvidenceClaimInventoryClaimsUse)
    );
    assert_eq!(
        app_game_approval_authority_journal_event(
            constants::peer::LOCAL_DEV_AGENT,
            std::env::consts::OS,
            &authority
        ),
        Err(AppGameJournalSqliteIngestError::AuthorityInactiveGrants)
    );
    assert_eq!(
        app_game_approval_action_result_journal_event(
            constants::peer::LOCAL_DEV_AGENT,
            std::env::consts::OS,
            &action
        ),
        Err(AppGameJournalSqliteIngestError::ActionResultManualExecution)
    );
    assert_eq!(
        app_game_platform_authority_matrix_journal_event(
            constants::peer::LOCAL_DEV_AGENT,
            std::env::consts::OS,
            &matrix
        ),
        Err(AppGameJournalSqliteIngestError::PlatformAuthorityManualExecution)
    );
    assert_eq!(
        app_game_ai_classifier_result_journal_event(
            constants::peer::LOCAL_DEV_AGENT,
            std::env::consts::OS,
            &classifier
        ),
        Err(AppGameJournalSqliteIngestError::ClassifierRequestsAction)
    );
}

#[test]
fn invalid_identity_rows_are_rejected_before_sqlite_ingest() {
    let mut stale = identity();
    stale.schema_version = APP_GAME_SCHEMA_VERSION.saturating_sub(1);
    assert_eq!(
        app_game_identity_journal_event(
            constants::peer::LOCAL_DEV_AGENT,
            std::env::consts::OS,
            APP_GAME_TEST_TIMESTAMP,
            &stale,
        ),
        Err(AppGameJournalSqliteIngestError::SchemaVersionUnsupported)
    );

    let mut missing_evidence = identity();
    missing_evidence.evidence.clear();
    assert_eq!(
        app_game_identity_journal_event(
            constants::peer::LOCAL_DEV_AGENT,
            std::env::consts::OS,
            APP_GAME_TEST_TIMESTAMP,
            &missing_evidence,
        ),
        Err(AppGameJournalSqliteIngestError::IdentityInvalid)
    );

}

#[test]
fn invalid_ai_classifier_shape_is_rejected_before_sqlite_ingest() {
    let mut invalid_confidence = classifier_result();
    invalid_confidence.confidence = 1.01;
    assert_eq!(
        app_game_ai_classifier_result_journal_event(
            constants::peer::LOCAL_DEV_AGENT,
            std::env::consts::OS,
            &invalid_confidence,
        ),
        Err(AppGameJournalSqliteIngestError::ClassifierInputInvalid)
    );

    let mut missing_evidence = classifier_result();
    missing_evidence.source_evidence_refs.clear();
    assert_eq!(
        app_game_ai_classifier_result_journal_event(
            constants::peer::LOCAL_DEV_AGENT,
            std::env::consts::OS,
            &missing_evidence,
        ),
        Err(AppGameJournalSqliteIngestError::ClassifierInputInvalid)
    );

    let mut blank_evidence = classifier_result();
    blank_evidence.source_evidence_refs = vec![" ".to_string()];
    assert_eq!(
        app_game_ai_classifier_result_journal_event(
            constants::peer::LOCAL_DEV_AGENT,
            std::env::consts::OS,
            &blank_evidence,
        ),
        Err(AppGameJournalSqliteIngestError::ClassifierInputInvalid)
    );

    let mut duplicate_evidence = classifier_result();
    duplicate_evidence
        .source_evidence_refs
        .push(APP_GAME_TEST_CLASSIFIER_EVIDENCE_REF.to_string());
    assert_eq!(
        app_game_ai_classifier_result_journal_event(
            constants::peer::LOCAL_DEV_AGENT,
            std::env::consts::OS,
            &duplicate_evidence,
        ),
        Err(AppGameJournalSqliteIngestError::ClassifierInputInvalid)
    );

    let mut mismatched_candidate = classifier_result();
    mismatched_candidate.candidate_kind = APP_GAME_AI_CLASSIFIER_CANDIDATE_GAME_CONTEXT.to_string();
    assert_eq!(
        app_game_ai_classifier_result_journal_event(
            constants::peer::LOCAL_DEV_AGENT,
            std::env::consts::OS,
            &mismatched_candidate,
        ),
        Err(AppGameJournalSqliteIngestError::ClassifierInputInvalid)
    );

    let mut mismatched_fallback = classifier_result();
    mismatched_fallback.classifier_state = APP_GAME_AI_CLASSIFIER_STATE_CANDIDATE.to_string();
    assert_eq!(
        app_game_ai_classifier_result_journal_event(
            constants::peer::LOCAL_DEV_AGENT,
            std::env::consts::OS,
            &mismatched_fallback,
        ),
        Err(AppGameJournalSqliteIngestError::ClassifierInputInvalid)
    );

    let mut raw_scan = classifier_result();
    raw_scan.raw_scan_included = true;
    assert_eq!(
        app_game_ai_classifier_result_journal_event(
            constants::peer::LOCAL_DEV_AGENT,
            std::env::consts::OS,
            &raw_scan,
        ),
        Err(AppGameJournalSqliteIngestError::ClassifierRequestsAction)
    );

    let mut content_claim = classifier_result();
    content_claim.content_claim_included = true;
    assert_eq!(
        app_game_ai_classifier_result_journal_event(
            constants::peer::LOCAL_DEV_AGENT,
            std::env::consts::OS,
            &content_claim,
        ),
        Err(AppGameJournalSqliteIngestError::ClassifierRequestsAction)
    );
}

#[test]
fn replay_rejects_action_shaped_ai_classifier_json() {
    let mut event = protocol_boundary_events()
        .pop()
        .expect("classifier event fixture is present");
    let row_json = match event
        .fields
        .get(constants::field::APP_GAME_JOURNAL_FIELD_ROW_JSON)
    {
        Some(LogFieldValue::String(row_json)) => row_json,
        _ => panic!("classifier event fixture contains row json"),
    };
    let mut encoded: serde_json::Value =
        serde_json::from_str(row_json).expect("classifier event fixture is valid json");
    encoded["block"] = serde_json::Value::Bool(true);
    event.fields.insert(
        constants::field::APP_GAME_JOURNAL_FIELD_ROW_JSON.to_string(),
        LogFieldValue::String(encoded.to_string()),
    );

    let store =
        ActivityStore::open_in_memory().expect_value(constants::error::ACTIVITY_STORE_OPENS);
    store
        .ingest_events(std::slice::from_ref(&event))
        .expect_value(constants::error::ACTIVITY_STORE_INGESTS);

    match app_game_journal_sqlite_read_model(
        store.connection_for_test(),
        constants::activity_store::DEFAULT_RECENT_LIMIT,
        APP_GAME_TEST_TIMESTAMP,
    ) {
        Err(ActivityStoreError::InvalidAppGameJournalRow { reason }) => {
            assert_eq!(reason, "invalid-ai-classifier-result");
        }
        _ => panic!("expected invalid AI classifier row"),
    }
}

#[test]
fn durable_protocol_replay_preserves_manual_required_state_after_restart() {
    let journal_path = protocol_journal_path("protocol-durable-replay");
    let store_path = protocol_store_path("protocol-durable-replay");
    cleanup_journal_files(&journal_path);
    cleanup_store_files(&store_path);
    let events = protocol_boundary_events();
    let key = JournalKey::from_bytes([7; JOURNAL_KEY_BYTES]);
    let mut journal = ActivityJournal::open(journal_path.clone(), key.clone())
        .expect_value(constants::error::JOURNAL_OPENS);
    for event in &events {
        journal
            .append(event)
            .expect_value(constants::error::JOURNAL_APPENDS);
    }
    let reader = ActivityJournal::open(journal_path.clone(), key)
        .expect_value(constants::error::JOURNAL_OPENS);

    {
        let store =
            ActivityStore::open(&store_path).expect_value(constants::error::ACTIVITY_STORE_OPENS);
        let status = store
            .ingest_journal(&reader)
            .expect_value(constants::error::ACTIVITY_STORE_INGESTS);
        assert_eq!(status.events_ingested, events.len() as u64);
        assert_eq!(status.events_stored, events.len() as u64);
    }

    let restarted =
        ActivityStore::open(&store_path).expect_value(constants::error::ACTIVITY_STORE_OPENS);
    let replay_status = restarted
        .ingest_journal(&reader)
        .expect_value(constants::error::ACTIVITY_STORE_INGESTS);
    let model = app_game_journal_sqlite_read_model(
        restarted.connection_for_test(),
        constants::activity_store::DEFAULT_RECENT_LIMIT,
        APP_GAME_TEST_TIMESTAMP,
    )
    .expect_value(constants::error::ACTIVITY_STORE_QUERIES);
    drop(restarted);
    cleanup_journal_files(&journal_path);
    cleanup_store_files(&store_path);

    assert_eq!(replay_status.events_ingested, 0);
    assert_eq!(replay_status.duplicate_events, events.len() as u64);
    assert_eq!(replay_status.events_stored, events.len() as u64);
    assert_eq!(model.approval_authority_rows.len(), 1);
    assert_eq!(model.approval_action_result_rows.len(), 1);
    assert_eq!(
        model.approval_action_result_rows[0].result_status,
        APP_GAME_CONTROL_ACTION_STATUS_MANUAL_REQUIRED
    );
    assert!(model.approval_action_result_rows[0]
        .enforcement_result
        .is_none());
}

#[test]
fn replay_rejects_semantically_invalid_protocol_rows_from_sqlite() {
    let mut invalid_claim = evidence_claim();
    invalid_claim.runtime_state = APP_GAME_RUNTIME_RUNNING.to_string();
    let invalid_claim_json =
        serde_json::to_string(&invalid_claim).expect("serialize invalid evidence claim fixture");
    let mut event = protocol_boundary_events().remove(0);
    event.fields.insert(
        APP_GAME_JOURNAL_FIELD_ROW_JSON.to_string(),
        LogFieldValue::String(invalid_claim_json),
    );
    let store =
        ActivityStore::open_in_memory().expect_value(constants::error::ACTIVITY_STORE_OPENS);
    store
        .ingest_events(std::slice::from_ref(&event))
        .expect_value(constants::error::ACTIVITY_STORE_INGESTS);

    let result = app_game_journal_sqlite_read_model(
        store.connection_for_test(),
        constants::activity_store::DEFAULT_RECENT_LIMIT,
        APP_GAME_TEST_TIMESTAMP,
    );

    match result {
        Err(ActivityStoreError::InvalidAppGameJournalRow { reason }) => {
            assert_eq!(reason, "invalid-evidence-claim");
        }
        _ => panic!("expected invalid protocol row"),
    }
}

fn protocol_boundary_events() -> Vec<ActivityEvent> {
    vec![
        app_game_evidence_claim_journal_event(
            constants::peer::LOCAL_DEV_AGENT,
            std::env::consts::OS,
            &evidence_claim(),
        )
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES),
        app_game_identity_journal_event(
            constants::peer::LOCAL_DEV_AGENT,
            std::env::consts::OS,
            APP_GAME_TEST_TIMESTAMP,
            &identity(),
        )
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES),
        app_game_approval_authority_journal_event(
            constants::peer::LOCAL_DEV_AGENT,
            std::env::consts::OS,
            &approval_authority(),
        )
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES),
        app_game_approval_action_result_journal_event(
            constants::peer::LOCAL_DEV_AGENT,
            std::env::consts::OS,
            &manual_action_result(),
        )
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES),
        app_game_platform_authority_matrix_journal_event(
            constants::peer::LOCAL_DEV_AGENT,
            std::env::consts::OS,
            &platform_matrix(),
        )
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES),
        app_game_ai_classifier_result_journal_event(
            constants::peer::LOCAL_DEV_AGENT,
            std::env::consts::OS,
            &classifier_result(),
        )
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES),
    ]
}

fn evidence_claim() -> AppGameEvidenceClaim {
    AppGameEvidenceClaim {
        schema_version: APP_GAME_SCHEMA_VERSION,
        claim_id: APP_GAME_TEST_EVIDENCE_CLAIM_ID.to_string(),
        observed_at: APP_GAME_TEST_TIMESTAMP.to_string(),
        claim_kind: APP_GAME_EVIDENCE_CLAIM_KIND_INVENTORY.to_string(),
        observation_mode: APP_GAME_OBSERVATION_MODE_INVENTORY_SCAN.to_string(),
        display_name: APP_GAME_TEST_DISPLAY_LABEL.to_string(),
        identity_strength: APP_GAME_IDENTITY_STRENGTH_CATALOG_MATCHED.to_string(),
        classification_state: APP_GAME_CLASSIFICATION_KNOWN_GAME.to_string(),
        catalog_ready_state: APP_GAME_CATALOG_READY.to_string(),
        runtime_state: APP_GAME_RUNTIME_NOT_CLAIMED.to_string(),
        foreground_state: APP_GAME_FOREGROUND_NOT_CLAIMED.to_string(),
        inventory_entry_id: Some(APP_GAME_TEST_EVIDENCE_REF_ID.to_string()),
        process_identity: None,
        launcher_ref: None,
        catalog_ref: Some(APP_GAME_TEST_CATALOG_REF.to_string()),
        confidence: 0.82,
        evidence: vec![source_evidence(APP_GAME_TEST_EVIDENCE_REF_ID)],
    }
}

fn identity() -> AppGameIdentity {
    AppGameIdentity {
        schema_version: APP_GAME_SCHEMA_VERSION,
        identity_id: APP_GAME_TEST_IDENTITY_ID.to_string(),
        product_kind: APP_GAME_PRODUCT_NATIVE_GAME.to_string(),
        display_label: APP_GAME_TEST_DISPLAY_LABEL.to_string(),
        parent_label: None,
        confidence: APP_GAME_IDENTITY_CONFIDENCE_DETERMINISTIC.to_string(),
        classification_state: APP_GAME_CLASSIFICATION_KNOWN_GAME.to_string(),
        package_id: None,
        bundle_id: None,
        app_user_model_id: None,
        desktop_entry_id: None,
        application_token_ref: None,
        executable_path_ref: None,
        publisher_signature_ref: None,
        file_hash_ref: None,
        launcher_ref: None,
        launcher_app_id: None,
        launcher_manifest_id: None,
        store_id: None,
        catalog_ref: Some(APP_GAME_TEST_CATALOG_REF.to_string()),
        child_game_evidence_claim_id: None,
        evidence: vec![source_evidence(APP_GAME_TEST_IDENTITY_ID)],
    }
}

fn approval_authority() -> AppGameControlApprovalAuthority {
    AppGameControlApprovalAuthority {
        schema_version: APP_GAME_PARENT_CONTRACT_SCHEMA_VERSION.to_string(),
        authority_id: APP_GAME_TEST_AUTHORITY_ID.to_string(),
        actor: parent_actor(),
        device: child_device(),
        policy_version: APP_GAME_TEST_POLICY_VERSION.to_string(),
        authority_state: APP_GAME_CONTROL_AUTHORITY_ACTIVE.to_string(),
        allowed_policy_kinds: vec![APP_GAME_CONTROL_POLICY_KIND_APP.to_string()],
        can_approve: true,
        can_deny: true,
        can_extend: false,
        can_override: false,
        can_observe_only: true,
        checked_at: APP_GAME_TEST_TIMESTAMP.to_string(),
    }
}

fn manual_action_result() -> AppGameControlActionResult {
    let request = AppGameControlApprovalRequest {
        schema_version: APP_GAME_PARENT_CONTRACT_SCHEMA_VERSION.to_string(),
        request_id: APP_GAME_TEST_REQUEST_ID.to_string(),
        policy_kind: APP_GAME_CONTROL_POLICY_KIND_APP.to_string(),
        device: child_device(),
        target: AppGamePolicyTarget {
            target_id: APP_GAME_TEST_TARGET_ID.to_string(),
            target_type: APP_GAME_POLICY_TARGET_TYPE_APP.to_string(),
            target_value: APP_GAME_TEST_TARGET_VALUE.to_string(),
        },
        requested_action: APP_GAME_POLICY_ACTION_BLOCK.to_string(),
        requested_mode: None,
        requested_setting_refs: vec![AppGameControlSettingReference {
            setting_id: APP_GAME_TEST_SETTING_ID.to_string(),
            writes_to: APP_GAME_TEST_SETTING_PATH.to_string(),
        }],
        evidence_references: vec![parent_evidence()],
        candidate: None,
        child_reason_state: APP_GAME_CONTROL_CHILD_REASON_NOT_REQUESTED.to_string(),
        child_reason_references: Vec::new(),
        child_status_references: Vec::new(),
        expires_at: APP_GAME_TEST_TIMESTAMP.to_string(),
        unanswered_fallback: APP_GAME_CONTROL_UNANSWERED_FALLBACK_DENY.to_string(),
    };
    let decision = AppGameControlApprovalDecision {
        schema_version: APP_GAME_PARENT_CONTRACT_SCHEMA_VERSION.to_string(),
        decision_id: APP_GAME_TEST_DECISION_ID.to_string(),
        request_id: APP_GAME_TEST_REQUEST_ID.to_string(),
        policy_kind: APP_GAME_CONTROL_POLICY_KIND_APP.to_string(),
        decision_state: APP_GAME_CONTROL_DECISION_DENIED.to_string(),
        parent_action: Some(AppGameParentActionReference {
            action_reference_id: APP_GAME_TEST_ACTION_REFERENCE_ID.to_string(),
            actor: parent_actor(),
            policy_version: APP_GAME_TEST_POLICY_VERSION.to_string(),
            created_at: APP_GAME_TEST_TIMESTAMP.to_string(),
        }),
        reason_codes: Vec::new(),
        policy_version: APP_GAME_TEST_POLICY_VERSION.to_string(),
        evidence_references: vec![parent_evidence()],
        response_scope: None,
        decision_expires_at: None,
        audit_references: Vec::new(),
        persistence_state: APP_GAME_CONTROL_PERSISTENCE_NOT_PERSISTED.to_string(),
        decided_at: APP_GAME_TEST_TIMESTAMP.to_string(),
    };
    AppGameControlActionResult {
        schema_version: APP_GAME_PARENT_CONTRACT_SCHEMA_VERSION.to_string(),
        result_id: APP_GAME_TEST_ACTION_RESULT_ID.to_string(),
        request,
        decision,
        approval_state: APP_GAME_CONTROL_APPROVAL_STATE_MANUAL_REQUIRED.to_string(),
        capability_state: APP_GAME_ENFORCEMENT_CAPABILITY_MANUAL_REQUIRED.to_string(),
        capability: capability(),
        evidence_proof_kind: APP_GAME_CONTROL_EVIDENCE_PROOF_LAUNCHER_ONLY.to_string(),
        result_status: APP_GAME_CONTROL_ACTION_STATUS_MANUAL_REQUIRED.to_string(),
        enforcement_result: None,
        recorded_at: APP_GAME_TEST_TIMESTAMP.to_string(),
    }
}

fn platform_matrix() -> AppGamePlatformAuthorityMatrix {
    AppGamePlatformAuthorityMatrix {
        schema_version: APP_GAME_PARENT_CONTRACT_SCHEMA_VERSION.to_string(),
        matrix_id: APP_GAME_TEST_PLATFORM_MATRIX_ID.to_string(),
        rows: vec![AppGamePlatformAuthorityRow {
            schema_version: APP_GAME_PARENT_CONTRACT_SCHEMA_VERSION.to_string(),
            row_id: APP_GAME_TEST_WINDOWS_ROW_ID.to_string(),
            platform: APP_GAME_PARENT_PLATFORM_WINDOWS.to_string(),
            action: APP_GAME_PLATFORM_ACTION_BLOCK_LAUNCH.to_string(),
            authority_tier: APP_GAME_PLATFORM_TIER_MANUAL_REQUIRED.to_string(),
            setup_state: APP_GAME_PLATFORM_SETUP_MANUAL_REQUIRED.to_string(),
            proof_state: APP_GAME_PLATFORM_PROOF_STATE_MANUAL_REQUIRED.to_string(),
            capability_state: APP_GAME_ENFORCEMENT_CAPABILITY_MANUAL_REQUIRED.to_string(),
            parent_visible_state: APP_GAME_PLATFORM_PARENT_VISIBLE_MANUAL_REQUIRED.to_string(),
            parent_visible_limitation: APP_GAME_TEST_WINDOWS_LIMITATION.to_string(),
            can_execute_adapter: false,
            supported_modes: Vec::new(),
            proof_references: Vec::new(),
            proof_needed_to_claim: vec![
                APP_GAME_PLATFORM_PROOF_KIND_WINDOWS_APPLOCKER.to_string(),
                APP_GAME_PLATFORM_PROOF_KIND_ROLLBACK.to_string(),
            ],
            linux_mechanism: None,
            linux_distro: None,
            linux_session: None,
            last_checked_at: APP_GAME_TEST_TIMESTAMP.to_string(),
        }],
        generated_at: APP_GAME_TEST_TIMESTAMP.to_string(),
    }
}

fn classifier_result() -> AppGameAiClassifierResult {
    AppGameAiClassifierResult {
        schema_version: APP_GAME_SCHEMA_VERSION,
        classifier_run_id: APP_GAME_TEST_CLASSIFIER_RUN_ID.to_string(),
        product_kind: APP_GAME_AI_CLASSIFIER_PRODUCT_UNKNOWN_APP.to_string(),
        digest_ref: APP_GAME_TEST_CLASSIFIER_DIGEST_REF.to_string(),
        source_digest_kind: APP_GAME_AI_CLASSIFIER_DIGEST_INVENTORY.to_string(),
        source_evidence_refs: vec![APP_GAME_TEST_CLASSIFIER_EVIDENCE_REF.to_string()],
        source_session_refs: Vec::new(),
        candidate_kind: APP_GAME_AI_CLASSIFIER_CANDIDATE_UNKNOWN_IDENTITY.to_string(),
        candidate_label: APP_GAME_TEST_CLASSIFIER_LABEL.to_string(),
        classifier_state: APP_GAME_AI_CLASSIFIER_STATE_PROVIDER_UNAVAILABLE.to_string(),
        confidence: 0.0,
        uncertainty_reason_codes: vec![APP_GAME_TEST_CLASSIFIER_REASON_CODE.to_string()],
        model_runtime_ref: APP_GAME_TEST_CLASSIFIER_RUNTIME_REF.to_string(),
        prompt_template_ref: APP_GAME_TEST_CLASSIFIER_PROMPT_REF.to_string(),
        prompt_version: APP_GAME_TEST_CLASSIFIER_PROMPT_REF.to_string(),
        fallback_state: APP_GAME_AI_CLASSIFIER_FALLBACK_LOCAL_MODEL_UNAVAILABLE.to_string(),
        policy_handoff: APP_GAME_AI_CLASSIFIER_HANDOFF_MANUAL_REVIEW.to_string(),
        generated_at: APP_GAME_TEST_TIMESTAMP.to_string(),
        direct_action_requested: false,
        raw_scan_included: false,
        content_claim_included: false,
    }
}

fn capability() -> AppGameEnforcementCapabilityStatus {
    AppGameEnforcementCapabilityStatus {
        schema_version: APP_GAME_PARENT_CONTRACT_SCHEMA_VERSION.to_string(),
        platform: APP_GAME_PARENT_PLATFORM_WINDOWS.to_string(),
        adapter_kind: APP_GAME_ENFORCEMENT_ADAPTER_PROCESS_CONTROL.to_string(),
        capability_state: APP_GAME_ENFORCEMENT_CAPABILITY_MANUAL_REQUIRED.to_string(),
        permission_state: APP_GAME_ENFORCEMENT_PERMISSION_ALLOWED.to_string(),
        dependency_state: APP_GAME_ENFORCEMENT_DEPENDENCY_INSTALLED.to_string(),
        supported_actions: Vec::new(),
        degraded_reason: Some(APP_GAME_ENFORCEMENT_CAPABILITY_MANUAL_REQUIRED.to_string()),
        last_checked_at: APP_GAME_TEST_TIMESTAMP.to_string(),
    }
}

fn parent_actor() -> AppGameParentActorReference {
    AppGameParentActorReference {
        actor_id: APP_GAME_TEST_PARENT_ACTOR_ID.to_string(),
        role: APP_GAME_PARENT_ACTOR_ROLE_PARENT.to_string(),
    }
}

fn child_device() -> AppGameParentDeviceReference {
    AppGameParentDeviceReference {
        device_id: APP_GAME_TEST_DEVICE_ID.to_string(),
        child_profile_id: Some(APP_GAME_TEST_CHILD_PROFILE_ID.to_string()),
        label: APP_GAME_TEST_DEVICE_LABEL.to_string(),
        platform: APP_GAME_PARENT_PLATFORM_WINDOWS.to_string(),
    }
}

fn parent_evidence() -> AppGameParentEvidenceReference {
    AppGameParentEvidenceReference {
        evidence_reference_id: APP_GAME_TEST_EVIDENCE_REF_ID.to_string(),
        kind: APP_GAME_PARENT_EVIDENCE_KIND_ACTIVITY_EVENT.to_string(),
        observed_at: APP_GAME_TEST_TIMESTAMP.to_string(),
    }
}

fn source_evidence(evidence_id: impl std::fmt::Display) -> ActivityEvidenceRef {
    ActivityEvidenceRef {
        evidence_id: evidence_id.to_string(),
        kind: ActivityEvidenceKind::StorageObject,
        digest: None,
        uri: None,
    }
}

fn append_and_replay(events: &[ActivityEvent]) -> (ActivityStore, Vec<ActivityJournalLine>) {
    let path = protocol_journal_path(constants::journal::TEST_PROTOCOL_ROWS_SUFFIX);
    cleanup_journal_files(&path);
    let key = JournalKey::from_bytes([7; JOURNAL_KEY_BYTES]);
    let mut journal = ActivityJournal::open(path.clone(), key.clone())
        .expect_value(constants::error::JOURNAL_OPENS);
    let lines = events
        .iter()
        .map(|event| {
            journal
                .append(event)
                .expect_value(constants::error::JOURNAL_APPENDS)
        })
        .collect::<Vec<_>>();
    let reader =
        ActivityJournal::open(path.clone(), key).expect_value(constants::error::JOURNAL_OPENS);
    let store =
        ActivityStore::open_in_memory().expect_value(constants::error::ACTIVITY_STORE_OPENS);
    let status = store
        .ingest_journal(&reader)
        .expect_value(constants::error::ACTIVITY_STORE_INGESTS);
    cleanup_journal_files(&path);

    assert_eq!(status.events_ingested, events.len() as u64);
    assert_eq!(status.events_stored, events.len() as u64);
    (store, lines)
}

fn protocol_journal_path(suffix: impl std::fmt::Display) -> std::path::PathBuf {
    let mut path = std::env::temp_dir();
    let mut file_name = constants::journal::TEST_FILE_PREFIX.to_string();
    file_name.push_str(&std::process::id().to_string());
    file_name.push(constants::delimiter::HYPHEN);
    file_name.push_str(&suffix.to_string());
    path.push(file_name);
    path.set_extension(constants::journal::FILE_EXTENSION);
    path
}

fn protocol_store_path(suffix: impl std::fmt::Display) -> std::path::PathBuf {
    let mut path = std::env::temp_dir();
    let mut file_name = constants::activity_store::TEST_FILE_PREFIX.to_string();
    file_name.push_str(&std::process::id().to_string());
    file_name.push(constants::delimiter::HYPHEN);
    file_name.push_str(&suffix.to_string());
    path.push(file_name);
    path.set_extension(constants::activity_store::FILE_EXTENSION);
    path
}

fn cleanup_journal_files(path: impl AsRef<std::path::Path>) {
    let path = path.as_ref();
    let _ = remove_file(path);
    for index in 1..=3 {
        let mut rotated_path = path.to_path_buf();
        let mut extension = index.to_string();
        extension.push_str(constants::journal::ROTATED_EXTENSION_SEPARATOR);
        extension.push_str(constants::journal::FILE_EXTENSION);
        rotated_path.set_extension(extension);
        let _ = remove_file(rotated_path);
    }
}

fn cleanup_store_files(path: impl AsRef<std::path::Path>) {
    let path = path.as_ref();
    let _ = remove_file(path);
    for extension in [
        constants::activity_store::WAL_FILE_EXTENSION,
        constants::activity_store::SHM_FILE_EXTENSION,
    ] {
        let mut sidecar = path.to_path_buf();
        sidecar.set_extension(extension);
        let _ = remove_file(sidecar);
    }
}
