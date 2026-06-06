use serde_json::json;

use super::{
    constants, TrackingRetentionSettingsMutationRequest, TrackingRetentionSettingsMutationResult,
    TRACKING_RETENTION_SETTINGS_MUTATION_STATE_ACCEPTED,
};

#[test]
fn tracking_retention_settings_mutation_request_serializes_refs() {
    let request = TrackingRetentionSettingsMutationRequest {
        request_id: constants::activity_store::TEST_TRACKING_RETENTION_DELETE_EVENT_ID.to_string(),
        intent_id: constants::activity_store::TEST_TRACKING_SUBJECT_ID.to_string(),
        settings_kind: constants::activity_subject_kind::RETENTION.to_string(),
        write_action: constants::field::COMMAND.to_string(),
        requested_value: constants::field::ACTIVE_STATE.to_string(),
        evidence_reference_ids: vec![
            constants::activity_store::TEST_TRACKING_EVIDENCE_REFERENCE_ID.to_string(),
        ],
        source_read_model_proof_refs: vec![
            constants::field::ACTIVITY_TRACKING_READ_MODEL.to_string()
        ],
        writer_boundary_proof_refs: vec![constants::field::CLAIM_BOUNDARY.to_string()],
        audit_refs: vec![constants::field::EVENT_REF.to_string()],
    };

    let serialized = serde_json::to_value(request).expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        serialized["evidenceReferenceIds"][0],
        constants::activity_store::TEST_TRACKING_EVIDENCE_REFERENCE_ID
    );
    assert_eq!(
        serialized["sourceReadModelProofRefs"][0],
        constants::field::ACTIVITY_TRACKING_READ_MODEL
    );
}

#[test]
fn tracking_retention_settings_mutation_result_serializes_no_product_claims() {
    let result = TrackingRetentionSettingsMutationResult {
        request_id: constants::activity_store::TEST_TRACKING_RETENTION_DELETE_EVENT_ID.to_string(),
        mutation_id: constants::activity_store::TEST_TRACKING_RETENTION_DELETE_EVENT_ID.to_string(),
        intent_id: constants::activity_store::TEST_TRACKING_SUBJECT_ID.to_string(),
        settings_kind: constants::activity_subject_kind::RETENTION.to_string(),
        write_action: constants::field::COMMAND.to_string(),
        requested_value: constants::field::ACTIVE_STATE.to_string(),
        mutation_state: TRACKING_RETENTION_SETTINGS_MUTATION_STATE_ACCEPTED.to_string(),
        rejection_reason: None,
        service_mutation_executed: true,
        durable_persistence_claimed: false,
        portal_ui_claimed: false,
        platform_runtime_claimed: false,
        child_device_delivery_claimed: false,
        provider_delivery_claimed: false,
        notification_receipt_claimed: false,
        physical_device_claimed: false,
        authority_claimed: false,
        product_claim_ready: false,
        evidence_reference_ids: vec![
            constants::activity_store::TEST_TRACKING_EVIDENCE_REFERENCE_ID.to_string(),
        ],
        source_read_model_proof_refs: vec![
            constants::field::ACTIVITY_TRACKING_READ_MODEL.to_string()
        ],
        writer_boundary_proof_refs: vec![constants::field::CLAIM_BOUNDARY.to_string()],
        audit_refs: vec![constants::field::EVENT_REF.to_string()],
    };

    let serialized = serde_json::to_value(result).expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(serialized["mutationState"], json!("accepted"));
    assert_eq!(serialized["serviceMutationExecuted"], json!(true));
    assert_eq!(serialized["durablePersistenceClaimed"], json!(false));
    assert_eq!(serialized["portalUiClaimed"], json!(false));
    assert_eq!(serialized["childDeviceDeliveryClaimed"], json!(false));
    assert_eq!(serialized["physicalDeviceClaimed"], json!(false));
    assert_eq!(serialized["productClaimReady"], json!(false));
}
