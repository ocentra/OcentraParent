use crate::{
    constants, AgentCommandName, AgentEventName, TrackingRetentionSettingsWriteResult,
    AGENT_PROTOCOL_SCHEMA_VERSION,
};

#[test]
fn retention_settings_write_command_and_event_names_serialize_to_contract_shape() {
    let command =
        serde_json::to_value(AgentCommandName::AgentActivityTrackingRetentionSettingsWrite)
            .expect(constants::error::AGENT_EVENT_SERIALIZES);
    let event =
        serde_json::to_value(AgentEventName::AgentActivityTrackingRetentionSettingsWriteReported)
            .expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(command, "agent.activity.tracking.retention-settings.write");
    assert_eq!(
        event,
        "agent.activity.tracking.retention-settings.write.reported"
    );
}

#[test]
fn retention_settings_write_result_serializes_without_product_overclaims() {
    let result = TrackingRetentionSettingsWriteResult {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        command_id: constants::tracking_retention_settings_write::COMMAND_ID.to_string(),
        settings_kind: constants::tracking_retention_settings_write::SETTINGS_KIND_RETENTION_WINDOW
            .to_string(),
        write_state: constants::tracking_retention_settings_write::WRITE_STATE_ACCEPTED.to_string(),
        accepted_at: constants::tracking_retention_settings_write::ACCEPTED_AT.to_string(),
        source_mutation_proof_refs: vec![
            constants::tracking_retention_settings_write::MUTATION_PROOF_REF.to_string(),
        ],
        command_transport_claimed: true,
        service_write_preflight_claimed: true,
        service_mutation_executed: false,
        portal_writable_ui_claimed: false,
        platform_runtime_claimed: false,
        child_device_delivery_claimed: false,
        provider_delivery_claimed: false,
        notification_receipt_claimed: false,
        physical_device_claimed: false,
        authority_claimed: false,
        product_claim_ready: false,
    };

    let serialized = serde_json::to_value(result).expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(serialized["schemaVersion"], AGENT_PROTOCOL_SCHEMA_VERSION);
    assert_eq!(
        serialized["sourceMutationProofRefs"][0],
        constants::tracking_retention_settings_write::MUTATION_PROOF_REF
    );
    assert_eq!(serialized["commandTransportClaimed"], true);
    assert_eq!(serialized["serviceWritePreflightClaimed"], true);
    assert_eq!(serialized["serviceMutationExecuted"], false);
    assert_eq!(serialized["portalWritableUiClaimed"], false);
    assert_eq!(serialized["productClaimReady"], false);
}
