use super::*;

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
    replace_tokens(
        r#"
export type __RESULT_TYPE__ = { readonly schemaVersion: number; readonly commandId: string; readonly settingsKind: string; readonly writeState: string; readonly acceptedAt: string; readonly sourceWriterIntentRefs: readonly string[]; readonly sourceReadModelProofRefs: readonly string[]; readonly sourceMutationProofRefs: readonly string[]; readonly appliedRetentionWindowHours: number | null; readonly appliedDeleteAfterAlertResolutionState: __DELETE_STATE__; readonly parentExportState: __PARENT_EXPORT_STATE__; readonly remoteSyncState: typeof __REMOTE_SYNC_STATE__.Disabled; readonly remoteAiState: typeof __REMOTE_AI_STATE__.Disabled; readonly localServiceStateRevision: number | null; readonly localServiceStateSnapshotRef: string; readonly durableSettingsStoreRef: string; readonly durableSettingsPersistenceState: __DURABLE_STATE__; readonly childConfigResponseState?: __RESPONSE_STATE__ | null; readonly effectiveTrackingState?: __EFFECTIVE_STATE__ | null; readonly childConfigAckState: __ACK_STATE__; readonly commandTransportClaimState: typeof __CLAIM_STATE__.Claimed; readonly serviceWritePreflightClaimState: typeof __CLAIM_STATE__.Claimed; readonly serviceMutationExecutionState: __CLAIM_STATE__; readonly portalWritableUiClaimState: typeof __CLAIM_STATE__.Unclaimed; readonly platformRuntimeClaimState: typeof __CLAIM_STATE__.Unclaimed; readonly childDeviceDeliveryClaimState: typeof __CLAIM_STATE__.Unclaimed; readonly providerDeliveryClaimState: typeof __CLAIM_STATE__.Unclaimed; readonly notificationReceiptClaimState: typeof __CLAIM_STATE__.Unclaimed; readonly physicalDeviceClaimState: typeof __CLAIM_STATE__.Unclaimed; readonly authorityClaimState: typeof __CLAIM_STATE__.Unclaimed; readonly productClaimState: typeof __CLAIM_STATE__.Unclaimed; };
type __RESULT_TYPE__Optionals = { childConfigResponseState?: __RESPONSE_STATE__ | null; effectiveTrackingState?: __EFFECTIVE_STATE__ | null };
function __DECODER_FN__IsRecord(candidate: unknown): candidate is Readonly<Record<string, unknown>> { return typeof candidate === 'object' && candidate !== null && !Array.isArray(candidate); }
function __DECODER_FN__ReadString(record: Readonly<Record<string, unknown>>, field: string): string { const fieldValue = record[field]; if (typeof fieldValue !== 'string' || fieldValue.length === 0) { throw new TypeError(`${field} must be a non-empty tracking retention string`); } return fieldValue; }
function __DECODER_FN__ReadSchemaVersion(record: Readonly<Record<string, unknown>>): number { if (record['schemaVersion'] !== __RUNTIME__.SchemaVersion) { throw new TypeError('schemaVersion is not the Rust-owned agent protocol schema version'); } return __RUNTIME__.SchemaVersion; }
function __DECODER_FN__ReadNullableNumber(record: Readonly<Record<string, unknown>>, field: string): number | null { const fieldValue = record[field]; if (fieldValue === null) { return null; } if (typeof fieldValue !== 'number' || !Number.isInteger(fieldValue) || fieldValue <= 0) { throw new TypeError(`${field} must be a positive integer or null`); } return fieldValue; }
function __DECODER_FN__ReadStringArray(record: Readonly<Record<string, unknown>>, field: string): readonly string[] { const fieldValue = record[field]; if (!Array.isArray(fieldValue) || fieldValue.length === 0 || fieldValue.some((entry) => typeof entry !== 'string' || entry.length === 0)) { throw new TypeError(`${field} must be a non-empty string array`); } return fieldValue as readonly string[]; }
function __DECODER_FN__ReadLiteral<T extends string>(record: Readonly<Record<string, unknown>>, field: string, allowed: readonly T[]): T { const fieldValue = __DECODER_FN__ReadString(record, field); if (!allowed.includes(fieldValue as T)) { throw new TypeError(`${field} is not a Rust-owned tracking literal`); } return fieldValue as T; }
function __DECODER_FN__ReadRequiredLiteral<T extends string>(record: Readonly<Record<string, unknown>>, field: string, expected: T): T { const fieldValue = __DECODER_FN__ReadString(record, field); if (fieldValue !== expected) { throw new TypeError(`${field} must be ${expected}`); } return expected; }
function __DECODER_FN__ReadOptionalNullableLiteral<T extends string>(record: Readonly<Record<string, unknown>>, field: string, allowed: readonly T[]): T | null | undefined { const fieldValue = record[field]; if (fieldValue === undefined) { return undefined; } if (fieldValue === null) { return null; } if (typeof fieldValue !== 'string' || !allowed.includes(fieldValue as T)) { throw new TypeError(`${field} is not a Rust-owned tracking literal`); } return fieldValue as T; }
function __DECODER_FN__ReadAckState(record: Readonly<Record<string, unknown>>): __ACK_STATE__ { if (record['childConfigAckState'] === undefined) { return __ACK_STATE__.Missing; } return __DECODER_FN__ReadLiteral(record, 'childConfigAckState', Object.values(__ACK_STATE__)); }
function __DECODER_FN__AttachOptionals(result: __RESULT_TYPE__, childConfigResponseState: __RESPONSE_STATE__ | null | undefined, effectiveTrackingState: __EFFECTIVE_STATE__ | null | undefined): __RESULT_TYPE__ { const resultWithOptionals = result as __RESULT_TYPE__ & __RESULT_TYPE__Optionals; if (childConfigResponseState !== undefined) { resultWithOptionals.childConfigResponseState = childConfigResponseState; } if (effectiveTrackingState !== undefined) { resultWithOptionals.effectiveTrackingState = effectiveTrackingState; } return resultWithOptionals; }
function __DECODER_FN__RequireAcceptedInvariants(result: __RESULT_TYPE__): void { if (result.writeState !== __DEFAULTS__.WriteStateAccepted) { return; } if (result.commandTransportClaimState !== __CLAIM_STATE__.Claimed) { throw new TypeError('accepted tracking write result must prove command transport'); } if (result.serviceMutationExecutionState !== __CLAIM_STATE__.Claimed) { throw new TypeError('accepted tracking write result must execute local mutation'); } if (result.localServiceStateRevision === null) { throw new TypeError('accepted tracking write result must include local service revision'); } if (result.durableSettingsPersistenceState !== __DURABLE_STATE__.Persisted) { throw new TypeError('accepted tracking write result must persist durable settings'); } }
function __DECODER_FN__RequireRetentionWindowInvariant(result: __RESULT_TYPE__): void { if (result.settingsKind === __DEFAULTS__.SettingsKindRetentionWindow && result.appliedRetentionWindowHours === null) { throw new TypeError('retention-window write result must include applied retention window'); } }
function __DECODER_FN__Finalize(result: __RESULT_TYPE__, childConfigResponseState: __RESPONSE_STATE__ | null | undefined, effectiveTrackingState: __EFFECTIVE_STATE__ | null | undefined): __RESULT_TYPE__ { const resultWithOptionals = __DECODER_FN__AttachOptionals(result, childConfigResponseState, effectiveTrackingState); __DECODER_FN__RequireAcceptedInvariants(resultWithOptionals); __DECODER_FN__RequireRetentionWindowInvariant(resultWithOptionals); return resultWithOptionals; }
export function __DECODER_FN__(value: unknown): __RESULT_TYPE__ { if (!__DECODER_FN__IsRecord(value)) { throw new TypeError('tracking retention write result must be an object'); } const childConfigResponseState = __DECODER_FN__ReadOptionalNullableLiteral(value, 'childConfigResponseState', Object.values(__RESPONSE_STATE__)); const effectiveTrackingState = __DECODER_FN__ReadOptionalNullableLiteral(value, 'effectiveTrackingState', Object.values(__EFFECTIVE_STATE__)); const result: __RESULT_TYPE__ = { schemaVersion: __DECODER_FN__ReadSchemaVersion(value), commandId: __DECODER_FN__ReadString(value, 'commandId'), settingsKind: __DECODER_FN__ReadRequiredLiteral(value, 'settingsKind', __DEFAULTS__.SettingsKindRetentionWindow), writeState: __DECODER_FN__ReadLiteral(value, 'writeState', [__DEFAULTS__.WriteStateAccepted, __DEFAULTS__.WriteStateRejected] as const), acceptedAt: __DECODER_FN__ReadString(value, 'acceptedAt'), sourceWriterIntentRefs: __DECODER_FN__ReadStringArray(value, 'sourceWriterIntentRefs'), sourceReadModelProofRefs: __DECODER_FN__ReadStringArray(value, 'sourceReadModelProofRefs'), sourceMutationProofRefs: __DECODER_FN__ReadStringArray(value, 'sourceMutationProofRefs'), appliedRetentionWindowHours: __DECODER_FN__ReadNullableNumber(value, 'appliedRetentionWindowHours'), appliedDeleteAfterAlertResolutionState: __DECODER_FN__ReadLiteral(value, 'appliedDeleteAfterAlertResolutionState', Object.values(__DELETE_STATE__)), parentExportState: __DECODER_FN__ReadLiteral(value, 'parentExportState', Object.values(__PARENT_EXPORT_STATE__)), remoteSyncState: __DECODER_FN__ReadRequiredLiteral(value, 'remoteSyncState', __REMOTE_SYNC_STATE__.Disabled), remoteAiState: __DECODER_FN__ReadRequiredLiteral(value, 'remoteAiState', __REMOTE_AI_STATE__.Disabled), localServiceStateRevision: __DECODER_FN__ReadNullableNumber(value, 'localServiceStateRevision'), localServiceStateSnapshotRef: __DECODER_FN__ReadString(value, 'localServiceStateSnapshotRef'), durableSettingsStoreRef: __DECODER_FN__ReadString(value, 'durableSettingsStoreRef'), durableSettingsPersistenceState: __DECODER_FN__ReadLiteral(value, 'durableSettingsPersistenceState', Object.values(__DURABLE_STATE__)), childConfigAckState: __DECODER_FN__ReadAckState(value), commandTransportClaimState: __DECODER_FN__ReadRequiredLiteral(value, 'commandTransportClaimState', __CLAIM_STATE__.Claimed), serviceWritePreflightClaimState: __DECODER_FN__ReadRequiredLiteral(value, 'serviceWritePreflightClaimState', __CLAIM_STATE__.Claimed), serviceMutationExecutionState: __DECODER_FN__ReadLiteral(value, 'serviceMutationExecutionState', Object.values(__CLAIM_STATE__)), portalWritableUiClaimState: __DECODER_FN__ReadRequiredLiteral(value, 'portalWritableUiClaimState', __CLAIM_STATE__.Unclaimed), platformRuntimeClaimState: __DECODER_FN__ReadRequiredLiteral(value, 'platformRuntimeClaimState', __CLAIM_STATE__.Unclaimed), childDeviceDeliveryClaimState: __DECODER_FN__ReadRequiredLiteral(value, 'childDeviceDeliveryClaimState', __CLAIM_STATE__.Unclaimed), providerDeliveryClaimState: __DECODER_FN__ReadRequiredLiteral(value, 'providerDeliveryClaimState', __CLAIM_STATE__.Unclaimed), notificationReceiptClaimState: __DECODER_FN__ReadRequiredLiteral(value, 'notificationReceiptClaimState', __CLAIM_STATE__.Unclaimed), physicalDeviceClaimState: __DECODER_FN__ReadRequiredLiteral(value, 'physicalDeviceClaimState', __CLAIM_STATE__.Unclaimed), authorityClaimState: __DECODER_FN__ReadRequiredLiteral(value, 'authorityClaimState', __CLAIM_STATE__.Unclaimed), productClaimState: __DECODER_FN__ReadRequiredLiteral(value, 'productClaimState', __CLAIM_STATE__.Unclaimed) }; return __DECODER_FN__Finalize(result, childConfigResponseState, effectiveTrackingState); }
export const __SCHEMA_CONST__ = { safeParse(value: unknown): { readonly success: true; readonly data: __RESULT_TYPE__ } | { readonly success: false } { try { return { success: true, data: __DECODER_FN__(value) }; } catch { return { success: false }; } } } as const;
"#
        .to_string(),
        &[
            ("__RESULT_TYPE__", names.tracking_retention_settings_write_result_type),
            ("__DECODER_FN__", names.tracking_retention_settings_write_result_decoder_fn),
            ("__SCHEMA_CONST__", names.tracking_retention_settings_write_result_schema_const),
            ("__RUNTIME__", names.runtime_const),
            ("__DEFAULTS__", names.tracking_retention_settings_write_defaults_const),
            ("__DELETE_STATE__", names.tracking_delete_after_alert_resolution_state_const),
            ("__PARENT_EXPORT_STATE__", names.tracking_parent_export_state_const),
            ("__REMOTE_SYNC_STATE__", names.tracking_remote_sync_state_const),
            ("__REMOTE_AI_STATE__", names.tracking_remote_ai_state_const),
            ("__DURABLE_STATE__", names.tracking_durable_settings_persistence_state_const),
            ("__RESPONSE_STATE__", names.tracking_config_update_response_state_const),
            ("__EFFECTIVE_STATE__", names.tracking_effective_state_const),
            ("__ACK_STATE__", names.tracking_config_ack_state_const),
            ("__CLAIM_STATE__", names.tracking_execution_claim_state_const),
        ],
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


