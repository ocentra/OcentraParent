use super::constants;
use crate::activity::{ActivityEvidenceKind, ActivityEvidenceRef};
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
fn app_game_identity_requires_evidence_and_rejects_display_only_promotion() {
    let mut missing_evidence = serde_json::to_value(deterministic_game_identity())
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES);
    missing_evidence["evidence"] = serde_json::json!([]);

    assert!(serde_json::from_value::<AppGameIdentity>(missing_evidence).is_err());

    let display_only = display_only_identity();
    let accepted = serde_json::to_value(display_only.clone())
        .and_then(serde_json::from_value::<AppGameIdentity>);
    assert!(accepted.is_ok());

    let mut promoted = display_only;
    promoted.confidence = APP_GAME_IDENTITY_CONFIDENCE_DETERMINISTIC.to_string();
    promoted.classification_state = APP_GAME_CLASSIFICATION_KNOWN_GAME.to_string();
    let promoted =
        serde_json::to_value(promoted).and_then(serde_json::from_value::<AppGameIdentity>);
    assert!(promoted.is_err());
}

#[test]
fn app_game_identity_keeps_launcher_claims_separate_from_child_games() {
    let mut launcher = deterministic_game_identity();
    launcher.identity_id = "identity-ocentra-launcher".to_string();
    launcher.product_kind = APP_GAME_PRODUCT_LAUNCHER.to_string();
    launcher.classification_state = APP_GAME_CLASSIFICATION_KNOWN_LAUNCHER.to_string();
    launcher.package_id = None;
    launcher.bundle_id = None;
    launcher.app_user_model_id = None;
    launcher.desktop_entry_id = None;
    launcher.application_token_ref = None;
    launcher.executable_path_ref = None;
    launcher.publisher_signature_ref = None;
    launcher.file_hash_ref = None;
    launcher.store_id = None;
    launcher.catalog_ref = None;
    launcher.child_game_evidence_claim_id = None;

    let accepted =
        serde_json::to_value(launcher.clone()).and_then(serde_json::from_value::<AppGameIdentity>);
    assert!(accepted.is_ok());

    let mut launcher_as_game = launcher.clone();
    launcher_as_game.product_kind = APP_GAME_PRODUCT_NATIVE_GAME.to_string();
    launcher_as_game.classification_state = APP_GAME_CLASSIFICATION_KNOWN_GAME.to_string();
    let rejected =
        serde_json::to_value(launcher_as_game).and_then(serde_json::from_value::<AppGameIdentity>);
    assert!(rejected.is_err());

    let mut child_game = launcher;
    child_game.product_kind = APP_GAME_PRODUCT_NATIVE_GAME.to_string();
    child_game.classification_state = APP_GAME_CLASSIFICATION_KNOWN_GAME.to_string();
    child_game.executable_path_ref = Some("path-ref-child-game".to_string());
    child_game.child_game_evidence_claim_id = Some("claim-child-game-proof-2".to_string());
    let accepted =
        serde_json::to_value(child_game).and_then(serde_json::from_value::<AppGameIdentity>);
    assert!(accepted.is_ok());
}

#[test]
fn app_game_identity_merge_proof_serializes_sources_and_shared_refs() {
    let merge = AppGameIdentityMergeProof {
        schema_version: APP_GAME_SCHEMA_VERSION,
        merge_id: APP_GAME_TEST_MERGE_ID.to_string(),
        target_identity: deterministic_game_identity(),
        source_identity_ids: vec![
            "identity-ocentra-game-source-store".to_string(),
            "identity-ocentra-game-source-process".to_string(),
        ],
        merge_confidence: 0.91,
        display_label_matched: true,
        parent_label_changed: false,
        conflicting_file_hash_refs: false,
        shared_deterministic_refs: vec![
            APP_GAME_IDENTITY_DETERMINISTIC_REF_STORE_ID.to_string(),
            APP_GAME_IDENTITY_DETERMINISTIC_REF_FILE_HASH_REF.to_string(),
        ],
        evidence: vec![identity_evidence()],
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
        "identity-ocentra-game-source-store"
    );
    assert_eq!(
        serialized["sourceIdentityIds"][1],
        "identity-ocentra-game-source-process"
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
fn app_game_identity_merge_rejects_weak_or_conflicting_identity_proofs() {
    let mut merge = AppGameIdentityMergeProof {
        schema_version: APP_GAME_SCHEMA_VERSION,
        merge_id: APP_GAME_TEST_MERGE_ID.to_string(),
        target_identity: deterministic_game_identity(),
        source_identity_ids: vec![
            "identity-ocentra-game-source-store".to_string(),
            "identity-ocentra-game-source-process".to_string(),
        ],
        merge_confidence: 0.8,
        display_label_matched: true,
        parent_label_changed: false,
        conflicting_file_hash_refs: false,
        shared_deterministic_refs: Vec::new(),
        evidence: vec![identity_evidence()],
    };

    let rejected = serde_json::to_value(merge.clone())
        .and_then(serde_json::from_value::<AppGameIdentityMergeProof>);
    assert!(rejected.is_err());

    merge.shared_deterministic_refs =
        vec![APP_GAME_IDENTITY_DETERMINISTIC_REF_FILE_HASH_REF.to_string()];
    merge.conflicting_file_hash_refs = true;
    let rejected =
        serde_json::to_value(merge).and_then(serde_json::from_value::<AppGameIdentityMergeProof>);
    assert!(rejected.is_err());
}

#[test]
fn app_game_identity_merge_parent_label_preserves_raw_identity() {
    let mut target = deterministic_game_identity();
    target.parent_label = Some("Weekend RPG".to_string());
    target.display_label = "Weekend RPG".to_string();
    target.confidence = APP_GAME_IDENTITY_CONFIDENCE_PARENT_LABELED.to_string();
    let expected_identity_id = target.identity_id.clone();
    let expected_file_hash_ref = target.file_hash_ref.clone();

    let merge = AppGameIdentityMergeProof {
        schema_version: APP_GAME_SCHEMA_VERSION,
        merge_id: "identity-merge-parent-label".to_string(),
        target_identity: target,
        source_identity_ids: vec![
            "identity-before-parent-label".to_string(),
            "identity-after-parent-label".to_string(),
        ],
        merge_confidence: 0.9,
        display_label_matched: false,
        parent_label_changed: true,
        conflicting_file_hash_refs: false,
        shared_deterministic_refs: vec![
            APP_GAME_IDENTITY_DETERMINISTIC_REF_FILE_HASH_REF.to_string()
        ],
        evidence: vec![identity_evidence()],
    };

    let parsed = serde_json::to_value(merge)
        .and_then(serde_json::from_value::<AppGameIdentityMergeProof>)
        .expect("app game identity merge deserializes");
    assert_eq!(parsed.target_identity.identity_id, expected_identity_id);
    assert_eq!(parsed.target_identity.file_hash_ref, expected_file_hash_ref);
    assert_eq!(
        parsed.target_identity.parent_label.as_deref(),
        Some("Weekend RPG")
    );
}

#[test]
fn app_game_identity_rejects_unknown_or_empty_contract_fields() {
    let mut unknown_kind = serde_json::to_value(deterministic_game_identity())
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES);
    unknown_kind["productKind"] = serde_json::json!("caller-defined-app");
    assert!(serde_json::from_value::<AppGameIdentity>(unknown_kind).is_err());

    let mut empty_reference = serde_json::to_value(deterministic_game_identity())
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES);
    empty_reference["fileHashRef"] = serde_json::json!("");
    assert!(serde_json::from_value::<AppGameIdentity>(empty_reference).is_err());

    let mut unknown_field = serde_json::to_value(deterministic_game_identity())
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES);
    unknown_field["callerAuthority"] = serde_json::json!("accepted");
    assert!(serde_json::from_value::<AppGameIdentity>(unknown_field).is_err());
}

#[test]
fn app_game_identity_rejects_whitespace_only_required_fields_and_evidence_ids() {
    for (field, value) in [
        ("identityId", serde_json::json!(" \t")),
        ("displayLabel", serde_json::json!("\n")),
        ("parentLabel", serde_json::json!("  ")),
        ("packageId", serde_json::json!("\t")),
        ("bundleId", serde_json::json!(" \n")),
        ("appUserModelId", serde_json::json!("\r\n")),
        ("desktopEntryId", serde_json::json!(" ")),
        ("applicationTokenRef", serde_json::json!("\t\t")),
        ("executablePathRef", serde_json::json!("\n")),
        ("publisherSignatureRef", serde_json::json!(" \r")),
        ("fileHashRef", serde_json::json!("\t")),
        ("launcherRef", serde_json::json!("  ")),
        ("launcherAppId", serde_json::json!("\n")),
        ("launcherManifestId", serde_json::json!("\r")),
        ("storeId", serde_json::json!(" \t")),
        ("catalogRef", serde_json::json!("\n\n")),
        ("childGameEvidenceClaimId", serde_json::json!("\t")),
    ] {
        let mut payload = serde_json::to_value(deterministic_game_identity())
            .expect_value(constants::error::AGENT_EVENT_SERIALIZES);
        payload[field] = value;
        assert!(
            serde_json::from_value::<AppGameIdentity>(payload).is_err(),
            "whitespace-only {field} must be rejected"
        );
    }

    let mut evidence = serde_json::to_value(deterministic_game_identity())
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES);
    evidence["evidence"][0]["evidenceId"] = serde_json::json!(" \t");
    assert!(serde_json::from_value::<AppGameIdentity>(evidence).is_err());
}

#[test]
fn app_game_identity_rejects_unknown_nested_evidence_fields() {
    let mut identity = serde_json::to_value(deterministic_game_identity())
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES);
    identity["evidence"][0]["callerAuthority"] = serde_json::json!("accepted");
    assert!(serde_json::from_value::<AppGameIdentity>(identity).is_err());

    let mut merge = serde_json::to_value(valid_merge_proof())
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES);
    merge["evidence"][0]["callerAuthority"] = serde_json::json!("accepted");
    assert!(serde_json::from_value::<AppGameIdentityMergeProof>(merge).is_err());
}

#[test]
fn app_game_identity_merge_rejects_whitespace_only_ids_and_references() {
    for (field, value) in [
        ("mergeId", serde_json::json!(" \t")),
        ("sharedDeterministicRefs", serde_json::json!([" \n"])),
    ] {
        let mut payload = serde_json::to_value(valid_merge_proof())
            .expect_value(constants::error::AGENT_EVENT_SERIALIZES);
        payload[field] = value;
        assert!(
            serde_json::from_value::<AppGameIdentityMergeProof>(payload).is_err(),
            "whitespace-only {field} must be rejected"
        );
    }

    let mut source = serde_json::to_value(valid_merge_proof())
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES);
    source["sourceIdentityIds"][0] = serde_json::json!(" \t");
    assert!(serde_json::from_value::<AppGameIdentityMergeProof>(source).is_err());

    let mut evidence = serde_json::to_value(valid_merge_proof())
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES);
    evidence["evidence"][0]["evidenceId"] = serde_json::json!("\n");
    assert!(serde_json::from_value::<AppGameIdentityMergeProof>(evidence).is_err());
}

#[test]
fn app_game_identity_merge_rejects_duplicate_or_target_sources() {
    let mut duplicate = valid_merge_proof();
    duplicate.source_identity_ids = vec![
        "identity-ocentra-game-source-store".to_string(),
        "identity-ocentra-game-source-store".to_string(),
    ];
    let rejected = serde_json::to_value(duplicate)
        .and_then(serde_json::from_value::<AppGameIdentityMergeProof>);
    assert!(rejected.is_err());

    let mut target = valid_merge_proof();
    target.source_identity_ids = vec![
        "identity-ocentra-game-source-store".to_string(),
        APP_GAME_TEST_IDENTITY_ID.to_string(),
    ];
    let rejected =
        serde_json::to_value(target).and_then(serde_json::from_value::<AppGameIdentityMergeProof>);
    assert!(rejected.is_err());
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
        evidence: vec![identity_evidence()],
    }
}

fn display_only_identity() -> AppGameIdentity {
    let mut identity = deterministic_game_identity();
    identity.identity_id = "identity-display-only".to_string();
    identity.product_kind = APP_GAME_PRODUCT_UNKNOWN_EXECUTABLE.to_string();
    identity.confidence = APP_GAME_IDENTITY_CONFIDENCE_WEAK.to_string();
    identity.classification_state = APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS.to_string();
    identity.package_id = None;
    identity.bundle_id = None;
    identity.app_user_model_id = None;
    identity.desktop_entry_id = None;
    identity.application_token_ref = None;
    identity.executable_path_ref = None;
    identity.publisher_signature_ref = None;
    identity.file_hash_ref = None;
    identity.launcher_ref = None;
    identity.launcher_app_id = None;
    identity.launcher_manifest_id = None;
    identity.store_id = None;
    identity.catalog_ref = None;
    identity.child_game_evidence_claim_id = None;
    identity
}

fn valid_merge_proof() -> AppGameIdentityMergeProof {
    AppGameIdentityMergeProof {
        schema_version: APP_GAME_SCHEMA_VERSION,
        merge_id: APP_GAME_TEST_MERGE_ID.to_string(),
        target_identity: deterministic_game_identity(),
        source_identity_ids: vec![
            "identity-ocentra-game-source-store".to_string(),
            "identity-ocentra-game-source-process".to_string(),
        ],
        merge_confidence: 0.91,
        display_label_matched: true,
        parent_label_changed: false,
        conflicting_file_hash_refs: false,
        shared_deterministic_refs: vec![
            APP_GAME_IDENTITY_DETERMINISTIC_REF_STORE_ID.to_string(),
            APP_GAME_IDENTITY_DETERMINISTIC_REF_FILE_HASH_REF.to_string(),
        ],
        evidence: vec![identity_evidence()],
    }
}

fn identity_evidence() -> ActivityEvidenceRef {
    ActivityEvidenceRef {
        evidence_id: "journal-entry-app-game-identity-1".to_string(),
        kind: ActivityEvidenceKind::JournalEntry,
        digest: Some("sha256:app-game-identity-digest".to_string()),
        uri: None,
    }
}
