use super::constants;
use crate::app_game::*;
use ocentra_eventing::expect_value::ExpectValue;

#[test]
fn app_game_identity_serializes_deterministic_identity_contract_shape() {
    let identity = deterministic_game_identity();

    let serialized =
        serde_json::to_value(identity).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(serialized["schemaVersion"], APP_GAME_SCHEMA_VERSION);
    assert_eq!(serialized["identityId"], APP_GAME_TEST_IDENTITY_ID);
    assert_eq!(serialized["productKind"], APP_GAME_PRODUCT_NATIVE_GAME);
    assert_eq!(serialized["displayLabel"], APP_GAME_TEST_DISPLAY_LABEL);
    assert_eq!(serialized["parentLabel"], APP_GAME_TEST_PARENT_LABEL);
    assert_eq!(
        serialized["confidence"],
        APP_GAME_IDENTITY_CONFIDENCE_DETERMINISTIC
    );
    assert_eq!(
        serialized["classificationState"],
        APP_GAME_CLASSIFICATION_KNOWN_GAME
    );
    assert_eq!(serialized["packageId"], APP_GAME_TEST_STORE_GAME_PACKAGE_ID);
    assert_eq!(
        serialized["appUserModelId"],
        APP_GAME_TEST_STORE_GAME_USER_MODEL_ID
    );
    assert_eq!(serialized["launcherRef"], APP_GAME_TEST_LAUNCHER_REF);
    assert_eq!(
        serialized["childGameEvidenceClaimId"],
        APP_GAME_TEST_LAUNCHER_CHILD_GAME_CLAIM_ID
    );
}

#[test]
fn app_game_identity_merge_proof_serializes_sources_and_shared_refs() {
    let merge = AppGameIdentityMergeProof {
        schema_version: APP_GAME_SCHEMA_VERSION,
        merge_id: APP_GAME_TEST_MERGE_ID.to_string(),
        target_identity: deterministic_game_identity(),
        source_identity_ids: vec![
            APP_GAME_TEST_IDENTITY_ID.to_string(),
            APP_GAME_TEST_SECOND_IDENTITY_ID.to_string(),
        ],
        merge_confidence: 0.91,
        display_label_matched: true,
        parent_label_changed: false,
        conflicting_file_hash_refs: false,
        shared_deterministic_refs: vec![
            APP_GAME_IDENTITY_DETERMINISTIC_REF_STORE_ID.to_string(),
            APP_GAME_IDENTITY_DETERMINISTIC_REF_FILE_HASH_REF.to_string(),
        ],
        evidence: Vec::new(),
    };

    let serialized =
        serde_json::to_value(merge).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(serialized["schemaVersion"], APP_GAME_SCHEMA_VERSION);
    assert_eq!(serialized["mergeId"], APP_GAME_TEST_MERGE_ID);
    assert_eq!(
        serialized["targetIdentity"]["identityId"],
        APP_GAME_TEST_IDENTITY_ID
    );
    assert_eq!(
        serialized["sourceIdentityIds"][0],
        APP_GAME_TEST_IDENTITY_ID
    );
    assert_eq!(
        serialized["sourceIdentityIds"][1],
        APP_GAME_TEST_SECOND_IDENTITY_ID
    );
    assert_eq!(
        serialized["sharedDeterministicRefs"][0],
        APP_GAME_IDENTITY_DETERMINISTIC_REF_STORE_ID
    );
    assert_eq!(
        serialized["sharedDeterministicRefs"][1],
        APP_GAME_IDENTITY_DETERMINISTIC_REF_FILE_HASH_REF
    );
    assert_eq!(serialized["conflictingFileHashRefs"], false);
}

#[test]
fn app_game_evidence_claim_serializes_inventory_without_use_claims() {
    let claim = AppGameEvidenceClaim {
        schema_version: APP_GAME_SCHEMA_VERSION,
        claim_id: APP_GAME_TEST_EVIDENCE_CLAIM_ID.to_string(),
        observed_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        claim_kind: APP_GAME_EVIDENCE_CLAIM_KIND_INVENTORY.to_string(),
        observation_mode: APP_GAME_OBSERVATION_MODE_INVENTORY_SCAN.to_string(),
        display_name: APP_GAME_TEST_DISPLAY_LABEL.to_string(),
        identity_strength: APP_GAME_IDENTITY_STRENGTH_CATALOG_MATCHED.to_string(),
        classification_state: APP_GAME_CLASSIFICATION_KNOWN_GAME.to_string(),
        catalog_ready_state: APP_GAME_CATALOG_READY.to_string(),
        runtime_state: APP_GAME_RUNTIME_NOT_CLAIMED.to_string(),
        foreground_state: APP_GAME_FOREGROUND_NOT_CLAIMED.to_string(),
        inventory_entry_id: Some(APP_GAME_TEST_LAUNCHER_SOURCE_REF.to_string()),
        process_identity: None,
        launcher_ref: Some(APP_GAME_TEST_LAUNCHER_REF.to_string()),
        catalog_ref: Some(APP_GAME_TEST_CATALOG_REF.to_string()),
        confidence: 0.88,
        evidence: Vec::new(),
    };

    let serialized =
        serde_json::to_value(claim).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(serialized["schemaVersion"], APP_GAME_SCHEMA_VERSION);
    assert_eq!(serialized["claimId"], APP_GAME_TEST_EVIDENCE_CLAIM_ID);
    assert_eq!(
        serialized["claimKind"],
        APP_GAME_EVIDENCE_CLAIM_KIND_INVENTORY
    );
    assert_eq!(
        serialized["observationMode"],
        APP_GAME_OBSERVATION_MODE_INVENTORY_SCAN
    );
    assert_eq!(
        serialized["identityStrength"],
        APP_GAME_IDENTITY_STRENGTH_CATALOG_MATCHED
    );
    assert_eq!(serialized["runtimeState"], APP_GAME_RUNTIME_NOT_CLAIMED);
    assert_eq!(
        serialized["foregroundState"],
        APP_GAME_FOREGROUND_NOT_CLAIMED
    );
    assert_eq!(
        serialized["inventoryEntryId"],
        APP_GAME_TEST_LAUNCHER_SOURCE_REF
    );
    assert!(serialized["processIdentity"].is_null());
}

#[test]
fn app_game_ai_digest_reference_serializes_stored_evidence_sources() {
    let digest = AppGameAiDigestReference {
        schema_version: APP_GAME_SCHEMA_VERSION,
        digest_ref: APP_GAME_TEST_AI_DIGEST_REF.to_string(),
        digest: Some(APP_GAME_TEST_AI_DIGEST.to_string()),
        generated_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        confidence: 0.67,
        source_evidence_ids: vec![APP_GAME_TEST_RUNTIME_EVIDENCE_ID.to_string()],
        source_session_ids: vec![constants::activity_store::TEST_APP_GAME_SESSION_ID.to_string()],
        unavailable_reason: None,
    };

    let serialized =
        serde_json::to_value(digest).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(serialized["schemaVersion"], APP_GAME_SCHEMA_VERSION);
    assert_eq!(serialized["digestRef"], APP_GAME_TEST_AI_DIGEST_REF);
    assert_eq!(serialized["digest"], APP_GAME_TEST_AI_DIGEST);
    assert_eq!(
        serialized["sourceEvidenceIds"][0],
        APP_GAME_TEST_RUNTIME_EVIDENCE_ID
    );
    assert_eq!(
        serialized["sourceSessionIds"][0],
        constants::activity_store::TEST_APP_GAME_SESSION_ID
    );
    assert!(serialized["unavailableReason"].is_null());
}

#[test]
fn app_game_ai_classification_digest_serializes_classify_only_handoff() {
    let digest = AppGameAiClassificationDigest {
        schema_version: APP_GAME_SCHEMA_VERSION,
        digest_ref: APP_GAME_TEST_AI_DIGEST_REF.to_string(),
        digest: Some(APP_GAME_TEST_AI_DIGEST.to_string()),
        generated_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        classification_state: APP_GAME_CLASSIFICATION_POSSIBLY_GAME.to_string(),
        confidence: 0.52,
        action_hints: vec![
            APP_GAME_AI_ACTION_HINT_CLASSIFY_ONLY.to_string(),
            APP_GAME_AI_ACTION_HINT_PARENT_REVIEW.to_string(),
        ],
        source_evidence_ids: vec![APP_GAME_TEST_RUNTIME_EVIDENCE_ID.to_string()],
        source_session_ids: vec![constants::activity_store::TEST_APP_GAME_SESSION_ID.to_string()],
        unavailable_reason: None,
    };

    let serialized =
        serde_json::to_value(digest).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(serialized["schemaVersion"], APP_GAME_SCHEMA_VERSION);
    assert_eq!(serialized["digestRef"], APP_GAME_TEST_AI_DIGEST_REF);
    assert_eq!(
        serialized["classificationState"],
        APP_GAME_CLASSIFICATION_POSSIBLY_GAME
    );
    assert_eq!(
        serialized["actionHints"][0],
        APP_GAME_AI_ACTION_HINT_CLASSIFY_ONLY
    );
    assert_eq!(
        serialized["actionHints"][1],
        APP_GAME_AI_ACTION_HINT_PARENT_REVIEW
    );
    assert_eq!(
        serialized["sourceEvidenceIds"][0],
        APP_GAME_TEST_RUNTIME_EVIDENCE_ID
    );
    assert!(serialized["unavailableReason"].is_null());
}

fn deterministic_game_identity() -> AppGameIdentity {
    AppGameIdentity {
        schema_version: APP_GAME_SCHEMA_VERSION,
        identity_id: APP_GAME_TEST_IDENTITY_ID.to_string(),
        product_kind: APP_GAME_PRODUCT_NATIVE_GAME.to_string(),
        display_label: APP_GAME_TEST_DISPLAY_LABEL.to_string(),
        parent_label: Some(APP_GAME_TEST_PARENT_LABEL.to_string()),
        confidence: APP_GAME_IDENTITY_CONFIDENCE_DETERMINISTIC.to_string(),
        classification_state: APP_GAME_CLASSIFICATION_KNOWN_GAME.to_string(),
        package_id: Some(APP_GAME_TEST_STORE_GAME_PACKAGE_ID.to_string()),
        bundle_id: Some(APP_GAME_TEST_STORE_GAME_BUNDLE_ID.to_string()),
        app_user_model_id: Some(APP_GAME_TEST_STORE_GAME_USER_MODEL_ID.to_string()),
        desktop_entry_id: None,
        application_token_ref: None,
        executable_path_ref: Some(APP_GAME_TEST_EXECUTABLE_PATH_REF.to_string()),
        publisher_signature_ref: Some(APP_GAME_TEST_PUBLISHER_SIGNATURE_REF.to_string()),
        file_hash_ref: Some(APP_GAME_TEST_FILE_HASH_REF.to_string()),
        launcher_ref: Some(APP_GAME_TEST_LAUNCHER_REF.to_string()),
        launcher_app_id: Some(APP_GAME_TEST_LAUNCHER_APP_ID.to_string()),
        launcher_manifest_id: Some(APP_GAME_TEST_LAUNCHER_MANIFEST_ID.to_string()),
        store_id: Some(APP_GAME_TEST_STORE_GAME_STORE_ID.to_string()),
        catalog_ref: Some(APP_GAME_TEST_CATALOG_REF.to_string()),
        child_game_evidence_claim_id: Some(APP_GAME_TEST_LAUNCHER_CHILD_GAME_CLAIM_ID.to_string()),
        evidence: Vec::new(),
    }
}
