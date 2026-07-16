use super::*;

const TRACKING_RETENTION_SETTINGS_WRITE_RESULT_TYPESCRIPT_TEMPLATE: &str =
    include_str!("tracking_retention_settings_write_result_typescript.template.txt");

pub(super) fn tracking_retention_settings_write_typescript(names: &ProtocolBridgeNames) -> String {
    [
        tracking_retention_settings_write_defaults_typescript(
            names.tracking_retention_settings_write_defaults_const,
        ),
        literal_typescript(
            names.tracking_delete_after_alert_resolution_state_const,
            names.tracking_delete_after_alert_resolution_state_const,
            &tracking_delete_after_alert_resolution_state_descriptors(),
        ),
        literal_typescript(
            names.tracking_parent_export_state_const,
            names.tracking_parent_export_state_const,
            &tracking_parent_export_state_descriptors(),
        ),
        literal_typescript(
            names.tracking_remote_sync_state_const,
            names.tracking_remote_sync_state_const,
            &tracking_remote_sync_state_descriptors(),
        ),
        literal_typescript(
            names.tracking_remote_ai_state_const,
            names.tracking_remote_ai_state_const,
            &tracking_remote_ai_state_descriptors(),
        ),
        literal_typescript(
            names.tracking_durable_settings_persistence_state_const,
            names.tracking_durable_settings_persistence_state_const,
            &tracking_durable_settings_persistence_state_descriptors(),
        ),
        literal_typescript(
            names.tracking_config_ack_state_const,
            names.tracking_config_ack_state_const,
            &tracking_config_ack_state_descriptors(),
        ),
        literal_typescript(
            names.tracking_execution_claim_state_const,
            names.tracking_execution_claim_state_const,
            &tracking_execution_claim_state_descriptors(),
        ),
        literal_typescript(
            names.tracking_config_update_response_state_const,
            names.tracking_config_update_response_state_const,
            &tracking_config_update_response_state_descriptors(),
        ),
        literal_typescript(
            names.tracking_effective_state_const,
            names.tracking_effective_state_const,
            &tracking_effective_state_descriptors(),
        ),
        tracking_retention_settings_write_result_typescript(names),
    ]
    .join(" ")
}
fn tracking_retention_settings_write_result_typescript(names: &ProtocolBridgeNames) -> String {
    let replacements = [
        (
            "__RESULT_TYPE__",
            names.tracking_retention_settings_write_result_type,
        ),
        (
            "__DECODER_FN__",
            names.tracking_retention_settings_write_result_decoder_fn,
        ),
        (
            "__SCHEMA_CONST__",
            names.tracking_retention_settings_write_result_schema_const,
        ),
        ("__RUNTIME__", names.runtime_const),
        (
            "__DEFAULTS__",
            names.tracking_retention_settings_write_defaults_const,
        ),
        (
            "__DELETE_STATE__",
            names.tracking_delete_after_alert_resolution_state_const,
        ),
        (
            "__PARENT_EXPORT_STATE__",
            names.tracking_parent_export_state_const,
        ),
        (
            "__REMOTE_SYNC_STATE__",
            names.tracking_remote_sync_state_const,
        ),
        ("__REMOTE_AI_STATE__", names.tracking_remote_ai_state_const),
        (
            "__DURABLE_STATE__",
            names.tracking_durable_settings_persistence_state_const,
        ),
        (
            "__RESPONSE_STATE__",
            names.tracking_config_update_response_state_const,
        ),
        ("__EFFECTIVE_STATE__", names.tracking_effective_state_const),
        ("__ACK_STATE__", names.tracking_config_ack_state_const),
        (
            "__CLAIM_STATE__",
            names.tracking_execution_claim_state_const,
        ),
    ];

    replace_tokens(
        TRACKING_RETENTION_SETTINGS_WRITE_RESULT_TYPESCRIPT_TEMPLATE.to_string(),
        &replacements,
    )
}

fn tracking_retention_settings_write_defaults_typescript(name: &str) -> String {
    format!(
        "export const {name} = {{ CommandId: {}, SettingsKindRetentionWindow: {}, WriterIntentRef: {}, ReadModelProofRefs: [{}, {}], MutationProofRef: {}, LocalServiceStateSnapshotRef: {}, DurableSettingsStoreRef: {}, WriteStateAccepted: {}, WriteStateRejected: {}, AcceptedAt: {} }} as const;",
        json_literal(&tracking_retention_settings_write::COMMAND_ID),
        json_literal(&tracking_retention_settings_write::SETTINGS_KIND_RETENTION_WINDOW),
        json_literal(&tracking_retention_settings_write::WRITER_INTENT_REF),
        json_literal(&tracking_retention_settings_write::READ_MODEL_PROOF_REF),
        json_literal(&tracking_retention_settings_write::JOURNAL_READ_MODEL_PROOF_REF),
        json_literal(&tracking_retention_settings_write::MUTATION_PROOF_REF),
        json_literal(&tracking_retention_settings_write::LOCAL_SERVICE_STATE_SNAPSHOT_REF),
        json_literal(&tracking_retention_settings_write::DURABLE_SETTINGS_STORE_REF),
        json_literal(&tracking_retention_settings_write::WRITE_STATE_ACCEPTED),
        json_literal(&tracking_retention_settings_write::WRITE_STATE_REJECTED),
        json_literal(&tracking_retention_settings_write::ACCEPTED_AT),
    )
}

fn tracking_delete_after_alert_resolution_state_descriptors(
) -> Vec<ProtocolLiteralDescriptor<TrackingDeleteAfterAlertResolutionState>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "DeleteAfterAlertResolved",
            value: TrackingDeleteAfterAlertResolutionState::DeleteAfterAlertResolved,
        },
        ProtocolLiteralDescriptor {
            key: "RetainAfterAlertResolved",
            value: TrackingDeleteAfterAlertResolutionState::RetainAfterAlertResolved,
        },
    ]
}

fn tracking_parent_export_state_descriptors(
) -> Vec<ProtocolLiteralDescriptor<TrackingParentExportState>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "Prepared",
            value: TrackingParentExportState::Prepared,
        },
        ProtocolLiteralDescriptor {
            key: "NotPrepared",
            value: TrackingParentExportState::NotPrepared,
        },
    ]
}

fn tracking_remote_sync_state_descriptors(
) -> Vec<ProtocolLiteralDescriptor<TrackingRemoteSyncState>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "Enabled",
            value: TrackingRemoteSyncState::Enabled,
        },
        ProtocolLiteralDescriptor {
            key: "Disabled",
            value: TrackingRemoteSyncState::Disabled,
        },
    ]
}

fn tracking_remote_ai_state_descriptors() -> Vec<ProtocolLiteralDescriptor<TrackingRemoteAiState>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "Enabled",
            value: TrackingRemoteAiState::Enabled,
        },
        ProtocolLiteralDescriptor {
            key: "Disabled",
            value: TrackingRemoteAiState::Disabled,
        },
    ]
}

fn tracking_durable_settings_persistence_state_descriptors(
) -> Vec<ProtocolLiteralDescriptor<TrackingDurableSettingsPersistenceState>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "Persisted",
            value: TrackingDurableSettingsPersistenceState::Persisted,
        },
        ProtocolLiteralDescriptor {
            key: "NotPersisted",
            value: TrackingDurableSettingsPersistenceState::NotPersisted,
        },
    ]
}

fn tracking_config_ack_state_descriptors() -> Vec<ProtocolLiteralDescriptor<TrackingConfigAckState>>
{
    vec![
        ProtocolLiteralDescriptor {
            key: "Received",
            value: TrackingConfigAckState::Received,
        },
        ProtocolLiteralDescriptor {
            key: "Missing",
            value: TrackingConfigAckState::Missing,
        },
    ]
}

fn tracking_execution_claim_state_descriptors(
) -> Vec<ProtocolLiteralDescriptor<TrackingExecutionClaimState>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "Claimed",
            value: TrackingExecutionClaimState::Claimed,
        },
        ProtocolLiteralDescriptor {
            key: "Unclaimed",
            value: TrackingExecutionClaimState::Unclaimed,
        },
    ]
}

fn tracking_config_update_response_state_descriptors(
) -> Vec<ProtocolLiteralDescriptor<TrackingConfigUpdateResponseState>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "Applied",
            value: TrackingConfigUpdateResponseState::Applied,
        },
        ProtocolLiteralDescriptor {
            key: "Rejected",
            value: TrackingConfigUpdateResponseState::Rejected,
        },
    ]
}

fn tracking_effective_state_descriptors(
) -> Vec<ProtocolLiteralDescriptor<TrackingConfigEffectiveState>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "Enabled",
            value: TrackingConfigEffectiveState::Enabled,
        },
        ProtocolLiteralDescriptor {
            key: "Disabled",
            value: TrackingConfigEffectiveState::Disabled,
        },
        ProtocolLiteralDescriptor {
            key: "Degraded",
            value: TrackingConfigEffectiveState::Degraded,
        },
    ]
}
