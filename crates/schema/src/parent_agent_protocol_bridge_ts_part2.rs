fn tracking_retention_settings_write_typescript(names: &ProtocolBridgeNames) -> String {
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

fn activity_surface_contract_typescript(names: &ProtocolBridgeNames) -> String {
    let prefix = bridge_prefix(names);
    let schema_version_const = format!("{prefix}ActivitySurfaceSchemaVersion");
    let scope_kind_const = format!("{prefix}ActivitySurfaceScopeKind");
    let scope_kind_type = scope_kind_const.clone();
    let report_frequency_const = format!("{prefix}ActivityReportFrequency");
    let report_frequency_type = report_frequency_const.clone();
    let section_kind_const = format!("{prefix}ActivityReportSectionKind");
    let section_kind_type = section_kind_const.clone();
    let read_model_state_const = format!("{prefix}ActivityReadModelState");
    let read_model_state_type = read_model_state_const.clone();
    let source_reachability_const = format!("{prefix}ActivityReportSourceReachabilityState");
    let source_reachability_type = source_reachability_const.clone();
    let saved_report_state_const = format!("{prefix}ActivitySavedReportState");
    let saved_report_state_type = saved_report_state_const.clone();
    let custody_label_const = format!("{prefix}ActivityReportCustodyLabel");
    let custody_label_type = custody_label_const.clone();
    let source_label_const = format!("{prefix}ActivityReportSourceLabel");
    let source_label_type = source_label_const.clone();
    let evidence_kind_const = format!("{prefix}ActivityEvidenceKind");
    let evidence_kind_type = evidence_kind_const.clone();
    let read_model_kind_name_const = format!("{prefix}ActivitySurfaceReadModelKindName");
    let read_model_kind_type = format!("{prefix}ActivitySurfaceReadModelKind");
    let parser_type = format!("{prefix}ActivitySurfaceSchemaParser");
    let evidence_ref_type = format!("{prefix}ActivityEvidenceRef");
    let scope_type = format!("{prefix}ActivitySurfaceScope");
    let request_type = format!("{prefix}ActivitySurfaceRequest");
    let source_state_type = format!("{prefix}ActivityReportSourceState");
    let section_type = format!("{prefix}ActivityReportSection");
    let saved_metadata_type = format!("{prefix}ActivitySavedReportMetadata");
    let source_state_summary_type = format!("{prefix}ActivityReportSourceStateSummary");
    let report_document_type = format!("{prefix}ActivityReportDocument");
    let history_item_type = format!("{prefix}ActivityHistoricalReportListItem");
    let history_list_type = format!("{prefix}ActivityHistoricalReportList");
    let source_status_row_type = format!("{prefix}ActivityAppGameSourceStatusRow");
    let tab_read_model_type = format!("{prefix}ActivityTabReadModel");
    let screen_row_type = format!("{prefix}ActivityScreenReadModelRow");
    let app_use_row_type = format!("{prefix}ActivityAppUseReadModelRow");
    let browser_row_type = format!("{prefix}ActivityBrowserReadModelRow");
    let games_row_type = format!("{prefix}ActivityGamesReadModelRow");
    let network_row_type = format!("{prefix}ActivityNetworkReadModelRow");
    let screen_read_model_type = format!("{prefix}ActivityScreenReadModel");
    let app_use_read_model_type = format!("{prefix}ActivityAppUseReadModel");
    let browser_read_model_type = format!("{prefix}ActivityBrowserReadModel");
    let games_read_model_type = format!("{prefix}ActivityGamesReadModel");
    let network_read_model_type = format!("{prefix}ActivityNetworkReadModel");
    let surface_read_model_type = format!("{prefix}ActivitySurfaceReadModel");
    let read_model_state_schema_const = format!("{prefix}ActivityReadModelStateSchema");
    let request_schema_const = format!("{prefix}ActivitySurfaceRequestSchema");
    let report_document_schema_const = format!("{prefix}ActivityReportDocumentSchema");
    let history_list_schema_const = format!("{prefix}ActivityHistoricalReportListSchema");
    let screen_read_model_schema_const = format!("{prefix}ActivityScreenReadModelSchema");
    let app_use_read_model_schema_const = format!("{prefix}ActivityAppUseReadModelSchema");
    let browser_read_model_schema_const = format!("{prefix}ActivityBrowserReadModelSchema");
    let games_read_model_schema_const = format!("{prefix}ActivityGamesReadModelSchema");
    let network_read_model_schema_const = format!("{prefix}ActivityNetworkReadModelSchema");
    let operation_id_const = format!("{prefix}ActivitySurfaceAdapterOperationId");
    let command_builder_const = format!("{prefix}ActivitySurfaceAdapterCommandBuilder");
    let event_parser_const = format!("{prefix}ActivitySurfaceAdapterEventParser");
    let operation_type = format!("{prefix}ActivitySurfaceAdapterOperation");
    let failure_reason_type = format!("{prefix}ActivitySurfaceAdapterFailureReason");
    let response_kind_type = format!("{prefix}ActivitySurfaceAdapterResponseKind");
    let manifest_const = format!("{prefix}ActivitySurfaceAdapterOperationManifest");
    let helper_prefix = format!("__{prefix}ActivitySurface");

    [
        format!(
            "export const {schema_version_const} = {ACTIVITY_SURFACE_SCHEMA_VERSION} as const;"
        ),
        literal_typescript(
            &scope_kind_const,
            &scope_kind_type,
            &activity_surface_scope_kind_descriptors(),
        ),
        literal_typescript(
            &report_frequency_const,
            &report_frequency_type,
            &activity_report_frequency_descriptors(),
        ),
        literal_typescript(
            &section_kind_const,
            &section_kind_type,
            &activity_report_section_kind_descriptors(),
        ),
        literal_typescript(
            &read_model_state_const,
            &read_model_state_type,
            &activity_read_model_state_descriptors(),
        ),
        literal_typescript(
            &source_reachability_const,
            &source_reachability_type,
            &activity_report_source_reachability_state_descriptors(),
        ),
        literal_typescript(
            &saved_report_state_const,
            &saved_report_state_type,
            &activity_saved_report_state_descriptors(),
        ),
        literal_typescript(
            &custody_label_const,
            &custody_label_type,
            &activity_report_custody_label_descriptors(),
        ),
        literal_typescript(
            &source_label_const,
            &source_label_type,
            &activity_report_source_label_descriptors(),
        ),
        literal_typescript(
            &evidence_kind_const,
            &evidence_kind_type,
            &activity_evidence_kind_descriptors(),
        ),
        activity_surface_decoder_typescript(
            names,
            &schema_version_const,
            &scope_kind_const,
            &scope_kind_type,
            &report_frequency_const,
            &report_frequency_type,
            &section_kind_const,
            &section_kind_type,
            &read_model_state_const,
            &read_model_state_type,
            &source_reachability_const,
            &source_reachability_type,
            &saved_report_state_const,
            &saved_report_state_type,
            &custody_label_const,
            &custody_label_type,
            &source_label_const,
            &source_label_type,
            &evidence_kind_const,
            &evidence_kind_type,
            &read_model_kind_name_const,
            &read_model_kind_type,
            &parser_type,
            &evidence_ref_type,
            &scope_type,
            &request_type,
            &source_state_type,
            &section_type,
            &saved_metadata_type,
            &source_state_summary_type,
            &report_document_type,
            &history_item_type,
            &history_list_type,
            &source_status_row_type,
            &tab_read_model_type,
            &screen_row_type,
            &app_use_row_type,
            &browser_row_type,
            &games_row_type,
            &network_row_type,
            &screen_read_model_type,
            &app_use_read_model_type,
            &browser_read_model_type,
            &games_read_model_type,
            &network_read_model_type,
            &surface_read_model_type,
            &read_model_state_schema_const,
            &request_schema_const,
            &report_document_schema_const,
            &history_list_schema_const,
            &screen_read_model_schema_const,
            &app_use_read_model_schema_const,
            &browser_read_model_schema_const,
            &games_read_model_schema_const,
            &network_read_model_schema_const,
            &helper_prefix,
        ),
        activity_surface_adapter_manifest_typescript(
            names,
            &operation_id_const,
            &command_builder_const,
            &event_parser_const,
            &operation_type,
            &failure_reason_type,
            &response_kind_type,
            &read_model_kind_type,
            &manifest_const,
        ),
    ]
    .join(" ")
}

fn activity_surface_decoder_typescript(
    _names: &ProtocolBridgeNames,
    schema_version_const: &str,
    scope_kind_const: &str,
    scope_kind_type: &str,
    report_frequency_const: &str,
    report_frequency_type: &str,
    section_kind_const: &str,
    section_kind_type: &str,
    read_model_state_const: &str,
    read_model_state_type: &str,
    source_reachability_const: &str,
    source_reachability_type: &str,
    saved_report_state_const: &str,
    saved_report_state_type: &str,
    custody_label_const: &str,
    custody_label_type: &str,
    source_label_const: &str,
    source_label_type: &str,
    evidence_kind_const: &str,
    evidence_kind_type: &str,
    read_model_kind_name_const: &str,
    read_model_kind_type: &str,
    parser_type: &str,
    evidence_ref_type: &str,
    scope_type: &str,
    request_type: &str,
    source_state_type: &str,
    section_type: &str,
    saved_metadata_type: &str,
    source_state_summary_type: &str,
    report_document_type: &str,
    history_item_type: &str,
    history_list_type: &str,
    source_status_row_type: &str,
    tab_read_model_type: &str,
    screen_row_type: &str,
    app_use_row_type: &str,
    browser_row_type: &str,
    games_row_type: &str,
    network_row_type: &str,
    screen_read_model_type: &str,
    app_use_read_model_type: &str,
    browser_read_model_type: &str,
    games_read_model_type: &str,
    network_read_model_type: &str,
    surface_read_model_type: &str,
    read_model_state_schema_const: &str,
    request_schema_const: &str,
    report_document_schema_const: &str,
    history_list_schema_const: &str,
    screen_read_model_schema_const: &str,
    app_use_read_model_schema_const: &str,
    browser_read_model_schema_const: &str,
    games_read_model_schema_const: &str,
    network_read_model_schema_const: &str,
    helper_prefix: &str,
) -> String {
    replace_tokens(
        r#"
export const __READ_MODEL_KIND_NAME_CONST__ = { Screen: __SECTION_KIND_CONST__.Screen, AppUse: __SECTION_KIND_CONST__.AppUse, Browser: __SECTION_KIND_CONST__.Browser, Games: __SECTION_KIND_CONST__.Games, Network: __SECTION_KIND_CONST__.Network } as const;
export type __READ_MODEL_KIND_TYPE__ = (typeof __READ_MODEL_KIND_NAME_CONST__)[keyof typeof __READ_MODEL_KIND_NAME_CONST__];
export type __PARSER_TYPE__<T> = { readonly parse: (input: unknown) => T; readonly safeParse: (input: unknown) => { readonly success: true; readonly data: T } | { readonly success: false } };
export type __EVIDENCE_REF_TYPE__ = { readonly evidenceId: string; readonly kind: __EVIDENCE_KIND_TYPE__; readonly digest: string | null; readonly uri: string | null };
export type __SCOPE_TYPE__ = { readonly scopeKind: __SCOPE_KIND_TYPE__; readonly familyId: string | null; readonly deviceId: string | null };
export type __REQUEST_TYPE__ = { readonly schemaVersion: typeof __SCHEMA_VERSION_CONST__; readonly scope: __SCOPE_TYPE__; readonly requestedAt: string; readonly rangeStart: string; readonly rangeEnd: string };
export type __SOURCE_STATE_TYPE__ = { readonly deviceId: string; readonly reachabilityState: __SOURCE_REACHABILITY_TYPE__; readonly state: __READ_MODEL_STATE_TYPE__; readonly reason: string | null; readonly lastUpdatedAt: string | null; readonly custodyLabel: __CUSTODY_LABEL_TYPE__; readonly sourceLabel: __SOURCE_LABEL_TYPE__; readonly rawChildEvidenceIncluded: boolean };
export type __SECTION_TYPE__ = { readonly sectionKind: __SECTION_KIND_TYPE__; readonly title: string; readonly state: __READ_MODEL_STATE_TYPE__; readonly summary: string; readonly itemCount: number; readonly evidence: readonly __EVIDENCE_REF_TYPE__[] };
export type __SAVED_METADATA_TYPE__ = { readonly reportId: string; readonly fileName: string; readonly savedState: __SAVED_REPORT_STATE_TYPE__; readonly savedAt: string | null; readonly storageReason: string | null; readonly custodyLabel: __CUSTODY_LABEL_TYPE__; readonly sourceLabel: __SOURCE_LABEL_TYPE__; readonly rawChildEvidenceIncluded: boolean };
export type __SOURCE_STATE_SUMMARY_TYPE__ = { readonly totalSources: number; readonly readySources: number; readonly offlineSources: number; readonly staleSources: number; readonly unavailableSources: number; readonly unreachableSources: number; readonly errorSources: number };
export type __REPORT_DOCUMENT_TYPE__ = { readonly schemaVersion: typeof __SCHEMA_VERSION_CONST__; readonly reportId: string; readonly frequency: __REPORT_FREQUENCY_TYPE__; readonly scope: __SCOPE_TYPE__; readonly requestedAt: string; readonly rangeStart: string; readonly rangeEnd: string; readonly generatedAt: string; readonly savedMetadata: __SAVED_METADATA_TYPE__ | null; readonly sourceStates: readonly __SOURCE_STATE_TYPE__[]; readonly sections: readonly __SECTION_TYPE__[] };
export type __HISTORY_ITEM_TYPE__ = { readonly schemaVersion: typeof __SCHEMA_VERSION_CONST__; readonly reportId: string; readonly fileName: string; readonly reportDate: string; readonly rangeStart: string; readonly rangeEnd: string; readonly summary: string; readonly savedState: __SAVED_REPORT_STATE_TYPE__; readonly savedAt: string | null; readonly sourceStateSummary: __SOURCE_STATE_SUMMARY_TYPE__; readonly parsedReport: __REPORT_DOCUMENT_TYPE__; readonly custodyLabel: __CUSTODY_LABEL_TYPE__; readonly sourceLabel: __SOURCE_LABEL_TYPE__; readonly rawChildEvidenceIncluded: boolean };
export type __HISTORY_LIST_TYPE__ = { readonly schemaVersion: typeof __SCHEMA_VERSION_CONST__; readonly request: __REQUEST_TYPE__; readonly state: __READ_MODEL_STATE_TYPE__; readonly storageState: __SAVED_REPORT_STATE_TYPE__; readonly storageReason: string | null; readonly reports: readonly __HISTORY_ITEM_TYPE__[] };
export type __SOURCE_STATUS_ROW_TYPE__ = { readonly sourceKind: string; readonly state: __READ_MODEL_STATE_TYPE__; readonly rowCount: number; readonly lastObservedAt: string | null; readonly capabilityStatus: string; readonly evidence: readonly __EVIDENCE_REF_TYPE__[] };
export type __TAB_READ_MODEL_TYPE__<Row> = { readonly schemaVersion: typeof __SCHEMA_VERSION_CONST__; readonly request: __REQUEST_TYPE__; readonly state: __READ_MODEL_STATE_TYPE__; readonly generatedAt: string; readonly summary: string; readonly rows: readonly Row[] };
export type __SCREEN_ROW_TYPE__ = { readonly rowId: string; readonly label: string; readonly deviceId: string; readonly state: __READ_MODEL_STATE_TYPE__; readonly totalMs: number; readonly foregroundMs: number; readonly backgroundMs: number; readonly captureReason: string; readonly captureScope: string; readonly capabilityStatus: string; readonly queueJobId: string; readonly modelRuntimeRef: string; readonly modelId: string; readonly providerKind: string; readonly promptOrTemplateVersion: string; readonly primaryCategory: string | null; readonly confidence: number; readonly imageDeletionState: string; readonly rawImageRetained: boolean; readonly policyEligible: boolean; readonly imageDigest: string; readonly custodyState: string; readonly evidence: readonly __EVIDENCE_REF_TYPE__[]; readonly policyDecisionRef: string | null; readonly policyAction: string | null; readonly policyReasonCodes: readonly string[]; readonly parentRuleRefs: readonly string[]; readonly localModelRuntimeRefs: readonly string[]; readonly parentExplanationRefs: readonly string[]; readonly explanationReasons: readonly string[]; readonly deletionReasons: readonly string[]; readonly ocrTextSnippets: readonly string[]; readonly redactionNotes: readonly string[] };
export type __APP_USE_ROW_TYPE__ = { readonly rowId: string; readonly appName: string; readonly deviceId: string; readonly state: __READ_MODEL_STATE_TYPE__; readonly productKind: string; readonly classificationState: string; readonly inventoryState: string; readonly runtimeState: string; readonly foregroundState: string; readonly capabilityStatus: string; readonly lastObservedAt: string | null; readonly totalMs: number; readonly launchCount: number; readonly inventoryRowCount: number; readonly runningRowCount: number; readonly foregroundRowCount: number; readonly dailyRollupCount: number; readonly evidenceClaimRowCount: number; readonly identityRowCount: number; readonly approvalAuthorityRowCount: number; readonly approvalActionResultRowCount: number; readonly platformAuthorityMatrixCount: number; readonly platformAuthorityRowCount: number; readonly aiClassifierResultRowCount: number; readonly sourceStatusRows: readonly __SOURCE_STATUS_ROW_TYPE__[]; readonly evidence: readonly __EVIDENCE_REF_TYPE__[] };
export type __BROWSER_ROW_TYPE__ = { readonly rowId: string; readonly domainLabel: string; readonly deviceId: string; readonly state: __READ_MODEL_STATE_TYPE__; readonly visitCount: number; readonly totalMs: number; readonly evidenceDigest: string | null };
export type __GAMES_ROW_TYPE__ = { readonly rowId: string; readonly displayName: string; readonly deviceId: string; readonly state: __READ_MODEL_STATE_TYPE__; readonly productKind: string; readonly classificationState: string; readonly inventoryState: string; readonly runtimeState: string; readonly foregroundState: string; readonly capabilityStatus: string; readonly lastObservedAt: string | null; readonly totalMs: number; readonly sessionCount: number; readonly launcherRowCount: number; readonly runningRowCount: number; readonly foregroundRowCount: number; readonly dailyRollupCount: number; readonly evidenceClaimRowCount: number; readonly identityRowCount: number; readonly approvalAuthorityRowCount: number; readonly approvalActionResultRowCount: number; readonly platformAuthorityMatrixCount: number; readonly platformAuthorityRowCount: number; readonly aiClassifierResultRowCount: number; readonly sourceStatusRows: readonly __SOURCE_STATUS_ROW_TYPE__[]; readonly evidence: readonly __EVIDENCE_REF_TYPE__[] };
export type __NETWORK_ROW_TYPE__ = { readonly rowId: string; readonly destinationLabel: string; readonly deviceId: string; readonly state: __READ_MODEL_STATE_TYPE__; readonly connectionCount: number; readonly totalBytes: number; readonly evidenceDigest: string | null };
export type __SCREEN_READ_MODEL_TYPE__ = __TAB_READ_MODEL_TYPE__<__SCREEN_ROW_TYPE__>;
export type __APP_USE_READ_MODEL_TYPE__ = __TAB_READ_MODEL_TYPE__<__APP_USE_ROW_TYPE__>;
export type __BROWSER_READ_MODEL_TYPE__ = __TAB_READ_MODEL_TYPE__<__BROWSER_ROW_TYPE__>;
export type __GAMES_READ_MODEL_TYPE__ = __TAB_READ_MODEL_TYPE__<__GAMES_ROW_TYPE__>;
export type __NETWORK_READ_MODEL_TYPE__ = __TAB_READ_MODEL_TYPE__<__NETWORK_ROW_TYPE__>;
export type __SURFACE_READ_MODEL_TYPE__ = __SCREEN_READ_MODEL_TYPE__ | __APP_USE_READ_MODEL_TYPE__ | __BROWSER_READ_MODEL_TYPE__ | __GAMES_READ_MODEL_TYPE__ | __NETWORK_READ_MODEL_TYPE__;
function __HELPER_PREFIX__Schema<T>(decoder: (value: unknown) => T): __PARSER_TYPE__<T> { return { parse: decoder, safeParse(value: unknown): { readonly success: true; readonly data: T } | { readonly success: false } { try { return { success: true, data: decoder(value) }; } catch { return { success: false }; } } } as const; }
function __HELPER_PREFIX__IsRecord(value: unknown): value is Readonly<Record<string, unknown>> { return typeof value === 'object' && value !== null && !Array.isArray(value); }
function __HELPER_PREFIX__ReadRecord(value: unknown, label: string): Readonly<Record<string, unknown>> { if (!__HELPER_PREFIX__IsRecord(value)) { throw new TypeError(`${label} must be an activity surface object`); } return value; }
function __HELPER_PREFIX__ReadString(record: Readonly<Record<string, unknown>>, field: string): string { const value = record[field]; if (typeof value !== 'string' || value.length === 0) { throw new TypeError(`${field} must be a non-empty activity surface string`); } return value; }
function __HELPER_PREFIX__ReadNullableString(record: Readonly<Record<string, unknown>>, field: string): string | null { const value = record[field]; if (value === null) { return null; } if (typeof value !== 'string' || value.length === 0) { throw new TypeError(`${field} must be a non-empty activity surface string or null`); } return value; }
function __HELPER_PREFIX__ReadOptionalNullableString(record: Readonly<Record<string, unknown>>, field: string): string | null { const value = record[field]; if (value === undefined || value === null) { return null; } if (typeof value !== 'string' || value.length === 0) { throw new TypeError(`${field} must be a non-empty activity surface string or null`); } return value; }
function __HELPER_PREFIX__ReadBoolean(record: Readonly<Record<string, unknown>>, field: string): boolean { const value = record[field]; if (typeof value !== 'boolean') { throw new TypeError(`${field} must be an activity surface boolean`); } return value; }
function __HELPER_PREFIX__ReadOptionalFalse(record: Readonly<Record<string, unknown>>, field: string): boolean { const value = record[field]; if (value === undefined) { return false; } if (value !== false) { throw new TypeError(`${field} must be false for activity surface redaction/custody boundary`); } return false; }
function __HELPER_PREFIX__ReadNonNegativeInteger(record: Readonly<Record<string, unknown>>, field: string): number { const value = record[field]; if (typeof value !== 'number' || !Number.isInteger(value) || value < 0) { throw new TypeError(`${field} must be a non-negative activity surface integer`); } return value; }
function __HELPER_PREFIX__ReadConfidence(record: Readonly<Record<string, unknown>>, field: string): number { const value = record[field]; if (typeof value !== 'number' || !Number.isFinite(value) || value < 0 || value > 1) { throw new TypeError(`${field} must be an activity surface confidence from 0 to 1`); } return value; }
function __HELPER_PREFIX__ReadSchemaVersion(record: Readonly<Record<string, unknown>>): typeof __SCHEMA_VERSION_CONST__ { if (record['schemaVersion'] !== __SCHEMA_VERSION_CONST__) { throw new TypeError('schemaVersion is not the Rust-owned activity surface schema version'); } return __SCHEMA_VERSION_CONST__; }
function __HELPER_PREFIX__ReadLiteral<T extends string>(record: Readonly<Record<string, unknown>>, field: string, allowed: readonly T[]): T { const value = __HELPER_PREFIX__ReadString(record, field); if (!allowed.includes(value as T)) { throw new TypeError(`${field} is not a Rust-owned activity surface literal`); } return value as T; }
function __HELPER_PREFIX__ReadOptionalLiteral<T extends string>(record: Readonly<Record<string, unknown>>, field: string, allowed: readonly T[], fallback: T): T { const value = record[field]; if (value === undefined) { return fallback; } if (typeof value !== 'string' || !allowed.includes(value as T)) { throw new TypeError(`${field} is not a Rust-owned activity surface literal`); } return value as T; }
function __HELPER_PREFIX__ReadArray<T>(record: Readonly<Record<string, unknown>>, field: string, decoder: (value: unknown) => T): readonly T[] { const value = record[field]; if (!Array.isArray(value)) { throw new TypeError(`${field} must be an activity surface array`); } return value.map(decoder); }
function __HELPER_PREFIX__ReadStringArrayValue(value: unknown, field: string): readonly string[] { if (!Array.isArray(value) || value.some((entry) => typeof entry !== 'string' || entry.length === 0)) { throw new TypeError(`${field} must be an activity surface string array`); } return value as readonly string[]; }
function __HELPER_PREFIX__ReadOptionalStringArray(record: Readonly<Record<string, unknown>>, field: string): readonly string[] { const value = record[field]; if (value === undefined) { return []; } return __HELPER_PREFIX__ReadStringArrayValue(value, field); }
function __HELPER_PREFIX__DecodeEvidenceRef(value: unknown): __EVIDENCE_REF_TYPE__ { const record = __HELPER_PREFIX__ReadRecord(value, 'activity evidence ref'); return { evidenceId: __HELPER_PREFIX__ReadString(record, 'evidenceId'), kind: __HELPER_PREFIX__ReadLiteral(record, 'kind', Object.values(__EVIDENCE_KIND_CONST__)), digest: __HELPER_PREFIX__ReadNullableString(record, 'digest'), uri: __HELPER_PREFIX__ReadNullableString(record, 'uri') }; }
function __HELPER_PREFIX__ReadEvidenceArray(record: Readonly<Record<string, unknown>>, field: string): readonly __EVIDENCE_REF_TYPE__[] { return __HELPER_PREFIX__ReadArray(record, field, __HELPER_PREFIX__DecodeEvidenceRef); }
function __HELPER_PREFIX__DecodeScope(value: unknown): __SCOPE_TYPE__ { const record = __HELPER_PREFIX__ReadRecord(value, 'activity surface scope'); const scope = { scopeKind: __HELPER_PREFIX__ReadLiteral(record, 'scopeKind', Object.values(__SCOPE_KIND_CONST__)), familyId: __HELPER_PREFIX__ReadNullableString(record, 'familyId'), deviceId: __HELPER_PREFIX__ReadNullableString(record, 'deviceId') }; if (scope.scopeKind === __SCOPE_KIND_CONST__.Family && (scope.familyId === null || scope.deviceId !== null)) { throw new TypeError('family activity scope must include familyId only'); } if (scope.scopeKind === __SCOPE_KIND_CONST__.Device && (scope.familyId !== null || scope.deviceId === null)) { throw new TypeError('device activity scope must include deviceId only'); } return scope; }
function __HELPER_PREFIX__DecodeRequest(value: unknown): __REQUEST_TYPE__ { const record = __HELPER_PREFIX__ReadRecord(value, 'activity surface request'); return { schemaVersion: __HELPER_PREFIX__ReadSchemaVersion(record), scope: __HELPER_PREFIX__DecodeScope(record['scope']), requestedAt: __HELPER_PREFIX__ReadString(record, 'requestedAt'), rangeStart: __HELPER_PREFIX__ReadString(record, 'rangeStart'), rangeEnd: __HELPER_PREFIX__ReadString(record, 'rangeEnd') }; }
function __HELPER_PREFIX__DecodeSourceState(value: unknown): __SOURCE_STATE_TYPE__ { const record = __HELPER_PREFIX__ReadRecord(value, 'activity report source state'); return { deviceId: __HELPER_PREFIX__ReadString(record, 'deviceId'), reachabilityState: __HELPER_PREFIX__ReadLiteral(record, 'reachabilityState', Object.values(__SOURCE_REACHABILITY_CONST__)), state: __HELPER_PREFIX__ReadLiteral(record, 'state', Object.values(__READ_MODEL_STATE_CONST__)), reason: __HELPER_PREFIX__ReadNullableString(record, 'reason'), lastUpdatedAt: __HELPER_PREFIX__ReadNullableString(record, 'lastUpdatedAt'), custodyLabel: __HELPER_PREFIX__ReadOptionalLiteral(record, 'custodyLabel', Object.values(__CUSTODY_LABEL_CONST__), __CUSTODY_LABEL_CONST__.ChildDeviceLocalSummary), sourceLabel: __HELPER_PREFIX__ReadOptionalLiteral(record, 'sourceLabel', Object.values(__SOURCE_LABEL_CONST__), __SOURCE_LABEL_CONST__.ActivityQueryStoreSummary), rawChildEvidenceIncluded: __HELPER_PREFIX__ReadOptionalFalse(record, 'rawChildEvidenceIncluded') }; }
function __HELPER_PREFIX__DecodeSection(value: unknown): __SECTION_TYPE__ { const record = __HELPER_PREFIX__ReadRecord(value, 'activity report section'); return { sectionKind: __HELPER_PREFIX__ReadLiteral(record, 'sectionKind', Object.values(__SECTION_KIND_CONST__)), title: __HELPER_PREFIX__ReadString(record, 'title'), state: __HELPER_PREFIX__ReadLiteral(record, 'state', Object.values(__READ_MODEL_STATE_CONST__)), summary: __HELPER_PREFIX__ReadString(record, 'summary'), itemCount: __HELPER_PREFIX__ReadNonNegativeInteger(record, 'itemCount'), evidence: __HELPER_PREFIX__ReadEvidenceArray(record, 'evidence') }; }
function __HELPER_PREFIX__DecodeSavedMetadata(value: unknown): __SAVED_METADATA_TYPE__ { const record = __HELPER_PREFIX__ReadRecord(value, 'activity saved report metadata'); return { reportId: __HELPER_PREFIX__ReadString(record, 'reportId'), fileName: __HELPER_PREFIX__ReadString(record, 'fileName'), savedState: __HELPER_PREFIX__ReadLiteral(record, 'savedState', Object.values(__SAVED_REPORT_STATE_CONST__)), savedAt: __HELPER_PREFIX__ReadNullableString(record, 'savedAt'), storageReason: __HELPER_PREFIX__ReadNullableString(record, 'storageReason'), custodyLabel: __HELPER_PREFIX__ReadOptionalLiteral(record, 'custodyLabel', Object.values(__CUSTODY_LABEL_CONST__), __CUSTODY_LABEL_CONST__.ParentDeviceLocalReportJson), sourceLabel: __HELPER_PREFIX__ReadOptionalLiteral(record, 'sourceLabel', Object.values(__SOURCE_LABEL_CONST__), __SOURCE_LABEL_CONST__.SavedReportJson), rawChildEvidenceIncluded: __HELPER_PREFIX__ReadOptionalFalse(record, 'rawChildEvidenceIncluded') }; }
function __HELPER_PREFIX__DecodeSourceStateSummary(value: unknown): __SOURCE_STATE_SUMMARY_TYPE__ { const record = __HELPER_PREFIX__ReadRecord(value, 'activity report source state summary'); return { totalSources: __HELPER_PREFIX__ReadNonNegativeInteger(record, 'totalSources'), readySources: __HELPER_PREFIX__ReadNonNegativeInteger(record, 'readySources'), offlineSources: __HELPER_PREFIX__ReadNonNegativeInteger(record, 'offlineSources'), staleSources: __HELPER_PREFIX__ReadNonNegativeInteger(record, 'staleSources'), unavailableSources: __HELPER_PREFIX__ReadNonNegativeInteger(record, 'unavailableSources'), unreachableSources: __HELPER_PREFIX__ReadNonNegativeInteger(record, 'unreachableSources'), errorSources: __HELPER_PREFIX__ReadNonNegativeInteger(record, 'errorSources') }; }
function __HELPER_PREFIX__DecodeReportDocument(value: unknown): __REPORT_DOCUMENT_TYPE__ { const record = __HELPER_PREFIX__ReadRecord(value, 'activity report document'); const savedMetadata = record['savedMetadata']; return { schemaVersion: __HELPER_PREFIX__ReadSchemaVersion(record), reportId: __HELPER_PREFIX__ReadString(record, 'reportId'), frequency: __HELPER_PREFIX__ReadLiteral(record, 'frequency', Object.values(__REPORT_FREQUENCY_CONST__)), scope: __HELPER_PREFIX__DecodeScope(record['scope']), requestedAt: __HELPER_PREFIX__ReadString(record, 'requestedAt'), rangeStart: __HELPER_PREFIX__ReadString(record, 'rangeStart'), rangeEnd: __HELPER_PREFIX__ReadString(record, 'rangeEnd'), generatedAt: __HELPER_PREFIX__ReadString(record, 'generatedAt'), savedMetadata: savedMetadata === null ? null : __HELPER_PREFIX__DecodeSavedMetadata(savedMetadata), sourceStates: __HELPER_PREFIX__ReadArray(record, 'sourceStates', __HELPER_PREFIX__DecodeSourceState), sections: __HELPER_PREFIX__ReadArray(record, 'sections', __HELPER_PREFIX__DecodeSection) }; }
function __HELPER_PREFIX__DecodeHistoryItem(value: unknown): __HISTORY_ITEM_TYPE__ { const record = __HELPER_PREFIX__ReadRecord(value, 'activity historical report list item'); return { schemaVersion: __HELPER_PREFIX__ReadSchemaVersion(record), reportId: __HELPER_PREFIX__ReadString(record, 'reportId'), fileName: __HELPER_PREFIX__ReadString(record, 'fileName'), reportDate: __HELPER_PREFIX__ReadString(record, 'reportDate'), rangeStart: __HELPER_PREFIX__ReadString(record, 'rangeStart'), rangeEnd: __HELPER_PREFIX__ReadString(record, 'rangeEnd'), summary: __HELPER_PREFIX__ReadString(record, 'summary'), savedState: __HELPER_PREFIX__ReadLiteral(record, 'savedState', Object.values(__SAVED_REPORT_STATE_CONST__)), savedAt: __HELPER_PREFIX__ReadNullableString(record, 'savedAt'), sourceStateSummary: __HELPER_PREFIX__DecodeSourceStateSummary(record['sourceStateSummary']), parsedReport: __HELPER_PREFIX__DecodeReportDocument(record['parsedReport']), custodyLabel: __HELPER_PREFIX__ReadOptionalLiteral(record, 'custodyLabel', Object.values(__CUSTODY_LABEL_CONST__), __CUSTODY_LABEL_CONST__.ParentDeviceLocalHistory), sourceLabel: __HELPER_PREFIX__ReadOptionalLiteral(record, 'sourceLabel', Object.values(__SOURCE_LABEL_CONST__), __SOURCE_LABEL_CONST__.SavedReportHistory), rawChildEvidenceIncluded: __HELPER_PREFIX__ReadOptionalFalse(record, 'rawChildEvidenceIncluded') }; }
function __HELPER_PREFIX__DecodeHistoricalReportList(value: unknown): __HISTORY_LIST_TYPE__ { const record = __HELPER_PREFIX__ReadRecord(value, 'activity historical report list'); return { schemaVersion: __HELPER_PREFIX__ReadSchemaVersion(record), request: __HELPER_PREFIX__DecodeRequest(record['request']), state: __HELPER_PREFIX__ReadLiteral(record, 'state', Object.values(__READ_MODEL_STATE_CONST__)), storageState: __HELPER_PREFIX__ReadLiteral(record, 'storageState', Object.values(__SAVED_REPORT_STATE_CONST__)), storageReason: __HELPER_PREFIX__ReadNullableString(record, 'storageReason'), reports: __HELPER_PREFIX__ReadArray(record, 'reports', __HELPER_PREFIX__DecodeHistoryItem) }; }
function __HELPER_PREFIX__DecodeSourceStatusRow(value: unknown): __SOURCE_STATUS_ROW_TYPE__ { const record = __HELPER_PREFIX__ReadRecord(value, 'activity app/game source status row'); return { sourceKind: __HELPER_PREFIX__ReadString(record, 'sourceKind'), state: __HELPER_PREFIX__ReadLiteral(record, 'state', Object.values(__READ_MODEL_STATE_CONST__)), rowCount: __HELPER_PREFIX__ReadNonNegativeInteger(record, 'rowCount'), lastObservedAt: __HELPER_PREFIX__ReadNullableString(record, 'lastObservedAt'), capabilityStatus: __HELPER_PREFIX__ReadString(record, 'capabilityStatus'), evidence: __HELPER_PREFIX__ReadEvidenceArray(record, 'evidence') }; }
function __HELPER_PREFIX__DecodeReadModelBase(record: Readonly<Record<string, unknown>>) { return { schemaVersion: __HELPER_PREFIX__ReadSchemaVersion(record), request: __HELPER_PREFIX__DecodeRequest(record['request']), state: __HELPER_PREFIX__ReadLiteral(record, 'state', Object.values(__READ_MODEL_STATE_CONST__)), generatedAt: __HELPER_PREFIX__ReadString(record, 'generatedAt'), summary: __HELPER_PREFIX__ReadString(record, 'summary') }; }
function __HELPER_PREFIX__DecodeScreenRow(value: unknown): __SCREEN_ROW_TYPE__ { const record = __HELPER_PREFIX__ReadRecord(value, 'activity screen read-model row'); return { rowId: __HELPER_PREFIX__ReadString(record, 'rowId'), label: __HELPER_PREFIX__ReadString(record, 'label'), deviceId: __HELPER_PREFIX__ReadString(record, 'deviceId'), state: __HELPER_PREFIX__ReadLiteral(record, 'state', Object.values(__READ_MODEL_STATE_CONST__)), totalMs: __HELPER_PREFIX__ReadNonNegativeInteger(record, 'totalMs'), foregroundMs: __HELPER_PREFIX__ReadNonNegativeInteger(record, 'foregroundMs'), backgroundMs: __HELPER_PREFIX__ReadNonNegativeInteger(record, 'backgroundMs'), captureReason: __HELPER_PREFIX__ReadString(record, 'captureReason'), captureScope: __HELPER_PREFIX__ReadString(record, 'captureScope'), capabilityStatus: __HELPER_PREFIX__ReadString(record, 'capabilityStatus'), queueJobId: __HELPER_PREFIX__ReadString(record, 'queueJobId'), modelRuntimeRef: __HELPER_PREFIX__ReadString(record, 'modelRuntimeRef'), modelId: __HELPER_PREFIX__ReadString(record, 'modelId'), providerKind: __HELPER_PREFIX__ReadString(record, 'providerKind'), promptOrTemplateVersion: __HELPER_PREFIX__ReadString(record, 'promptOrTemplateVersion'), primaryCategory: __HELPER_PREFIX__ReadNullableString(record, 'primaryCategory'), confidence: __HELPER_PREFIX__ReadConfidence(record, 'confidence'), imageDeletionState: __HELPER_PREFIX__ReadString(record, 'imageDeletionState'), rawImageRetained: __HELPER_PREFIX__ReadBoolean(record, 'rawImageRetained'), policyEligible: __HELPER_PREFIX__ReadBoolean(record, 'policyEligible'), imageDigest: __HELPER_PREFIX__ReadString(record, 'imageDigest'), custodyState: __HELPER_PREFIX__ReadString(record, 'custodyState'), evidence: __HELPER_PREFIX__ReadEvidenceArray(record, 'evidence'), policyDecisionRef: __HELPER_PREFIX__ReadOptionalNullableString(record, 'policyDecisionRef'), policyAction: __HELPER_PREFIX__ReadOptionalNullableString(record, 'policyAction'), policyReasonCodes: __HELPER_PREFIX__ReadOptionalStringArray(record, 'policyReasonCodes'), parentRuleRefs: __HELPER_PREFIX__ReadOptionalStringArray(record, 'parentRuleRefs'), localModelRuntimeRefs: __HELPER_PREFIX__ReadOptionalStringArray(record, 'localModelRuntimeRefs'), parentExplanationRefs: __HELPER_PREFIX__ReadOptionalStringArray(record, 'parentExplanationRefs'), explanationReasons: __HELPER_PREFIX__ReadOptionalStringArray(record, 'explanationReasons'), deletionReasons: __HELPER_PREFIX__ReadOptionalStringArray(record, 'deletionReasons'), ocrTextSnippets: __HELPER_PREFIX__ReadOptionalStringArray(record, 'ocrTextSnippets'), redactionNotes: __HELPER_PREFIX__ReadOptionalStringArray(record, 'redactionNotes') }; }
function __HELPER_PREFIX__DecodeAppUseRow(value: unknown): __APP_USE_ROW_TYPE__ { const record = __HELPER_PREFIX__ReadRecord(value, 'activity app-use read-model row'); return { rowId: __HELPER_PREFIX__ReadString(record, 'rowId'), appName: __HELPER_PREFIX__ReadString(record, 'appName'), deviceId: __HELPER_PREFIX__ReadString(record, 'deviceId'), state: __HELPER_PREFIX__ReadLiteral(record, 'state', Object.values(__READ_MODEL_STATE_CONST__)), productKind: __HELPER_PREFIX__ReadString(record, 'productKind'), classificationState: __HELPER_PREFIX__ReadString(record, 'classificationState'), inventoryState: __HELPER_PREFIX__ReadString(record, 'inventoryState'), runtimeState: __HELPER_PREFIX__ReadString(record, 'runtimeState'), foregroundState: __HELPER_PREFIX__ReadString(record, 'foregroundState'), capabilityStatus: __HELPER_PREFIX__ReadString(record, 'capabilityStatus'), lastObservedAt: __HELPER_PREFIX__ReadNullableString(record, 'lastObservedAt'), totalMs: __HELPER_PREFIX__ReadNonNegativeInteger(record, 'totalMs'), launchCount: __HELPER_PREFIX__ReadNonNegativeInteger(record, 'launchCount'), inventoryRowCount: __HELPER_PREFIX__ReadNonNegativeInteger(record, 'inventoryRowCount'), runningRowCount: __HELPER_PREFIX__ReadNonNegativeInteger(record, 'runningRowCount'), foregroundRowCount: __HELPER_PREFIX__ReadNonNegativeInteger(record, 'foregroundRowCount'), dailyRollupCount: __HELPER_PREFIX__ReadNonNegativeInteger(record, 'dailyRollupCount'), evidenceClaimRowCount: __HELPER_PREFIX__ReadNonNegativeInteger(record, 'evidenceClaimRowCount'), identityRowCount: __HELPER_PREFIX__ReadNonNegativeInteger(record, 'identityRowCount'), approvalAuthorityRowCount: __HELPER_PREFIX__ReadNonNegativeInteger(record, 'approvalAuthorityRowCount'), approvalActionResultRowCount: __HELPER_PREFIX__ReadNonNegativeInteger(record, 'approvalActionResultRowCount'), platformAuthorityMatrixCount: __HELPER_PREFIX__ReadNonNegativeInteger(record, 'platformAuthorityMatrixCount'), platformAuthorityRowCount: __HELPER_PREFIX__ReadNonNegativeInteger(record, 'platformAuthorityRowCount'), aiClassifierResultRowCount: __HELPER_PREFIX__ReadNonNegativeInteger(record, 'aiClassifierResultRowCount'), sourceStatusRows: __HELPER_PREFIX__ReadArray(record, 'sourceStatusRows', __HELPER_PREFIX__DecodeSourceStatusRow), evidence: __HELPER_PREFIX__ReadEvidenceArray(record, 'evidence') }; }
function __HELPER_PREFIX__DecodeBrowserRow(value: unknown): __BROWSER_ROW_TYPE__ { const record = __HELPER_PREFIX__ReadRecord(value, 'activity browser read-model row'); return { rowId: __HELPER_PREFIX__ReadString(record, 'rowId'), domainLabel: __HELPER_PREFIX__ReadString(record, 'domainLabel'), deviceId: __HELPER_PREFIX__ReadString(record, 'deviceId'), state: __HELPER_PREFIX__ReadLiteral(record, 'state', Object.values(__READ_MODEL_STATE_CONST__)), visitCount: __HELPER_PREFIX__ReadNonNegativeInteger(record, 'visitCount'), totalMs: __HELPER_PREFIX__ReadNonNegativeInteger(record, 'totalMs'), evidenceDigest: __HELPER_PREFIX__ReadNullableString(record, 'evidenceDigest') }; }
function __HELPER_PREFIX__DecodeGamesRow(value: unknown): __GAMES_ROW_TYPE__ { const record = __HELPER_PREFIX__ReadRecord(value, 'activity games read-model row'); return { rowId: __HELPER_PREFIX__ReadString(record, 'rowId'), displayName: __HELPER_PREFIX__ReadString(record, 'displayName'), deviceId: __HELPER_PREFIX__ReadString(record, 'deviceId'), state: __HELPER_PREFIX__ReadLiteral(record, 'state', Object.values(__READ_MODEL_STATE_CONST__)), productKind: __HELPER_PREFIX__ReadString(record, 'productKind'), classificationState: __HELPER_PREFIX__ReadString(record, 'classificationState'), inventoryState: __HELPER_PREFIX__ReadString(record, 'inventoryState'), runtimeState: __HELPER_PREFIX__ReadString(record, 'runtimeState'), foregroundState: __HELPER_PREFIX__ReadString(record, 'foregroundState'), capabilityStatus: __HELPER_PREFIX__ReadString(record, 'capabilityStatus'), lastObservedAt: __HELPER_PREFIX__ReadNullableString(record, 'lastObservedAt'), totalMs: __HELPER_PREFIX__ReadNonNegativeInteger(record, 'totalMs'), sessionCount: __HELPER_PREFIX__ReadNonNegativeInteger(record, 'sessionCount'), launcherRowCount: __HELPER_PREFIX__ReadNonNegativeInteger(record, 'launcherRowCount'), runningRowCount: __HELPER_PREFIX__ReadNonNegativeInteger(record, 'runningRowCount'), foregroundRowCount: __HELPER_PREFIX__ReadNonNegativeInteger(record, 'foregroundRowCount'), dailyRollupCount: __HELPER_PREFIX__ReadNonNegativeInteger(record, 'dailyRollupCount'), evidenceClaimRowCount: __HELPER_PREFIX__ReadNonNegativeInteger(record, 'evidenceClaimRowCount'), identityRowCount: __HELPER_PREFIX__ReadNonNegativeInteger(record, 'identityRowCount'), approvalAuthorityRowCount: __HELPER_PREFIX__ReadNonNegativeInteger(record, 'approvalAuthorityRowCount'), approvalActionResultRowCount: __HELPER_PREFIX__ReadNonNegativeInteger(record, 'approvalActionResultRowCount'), platformAuthorityMatrixCount: __HELPER_PREFIX__ReadNonNegativeInteger(record, 'platformAuthorityMatrixCount'), platformAuthorityRowCount: __HELPER_PREFIX__ReadNonNegativeInteger(record, 'platformAuthorityRowCount'), aiClassifierResultRowCount: __HELPER_PREFIX__ReadNonNegativeInteger(record, 'aiClassifierResultRowCount'), sourceStatusRows: __HELPER_PREFIX__ReadArray(record, 'sourceStatusRows', __HELPER_PREFIX__DecodeSourceStatusRow), evidence: __HELPER_PREFIX__ReadEvidenceArray(record, 'evidence') }; }
function __HELPER_PREFIX__DecodeNetworkRow(value: unknown): __NETWORK_ROW_TYPE__ { const record = __HELPER_PREFIX__ReadRecord(value, 'activity network read-model row'); return { rowId: __HELPER_PREFIX__ReadString(record, 'rowId'), destinationLabel: __HELPER_PREFIX__ReadString(record, 'destinationLabel'), deviceId: __HELPER_PREFIX__ReadString(record, 'deviceId'), state: __HELPER_PREFIX__ReadLiteral(record, 'state', Object.values(__READ_MODEL_STATE_CONST__)), connectionCount: __HELPER_PREFIX__ReadNonNegativeInteger(record, 'connectionCount'), totalBytes: __HELPER_PREFIX__ReadNonNegativeInteger(record, 'totalBytes'), evidenceDigest: __HELPER_PREFIX__ReadNullableString(record, 'evidenceDigest') }; }
function __HELPER_PREFIX__DecodeScreenReadModel(value: unknown): __SCREEN_READ_MODEL_TYPE__ { const record = __HELPER_PREFIX__ReadRecord(value, 'activity screen read-model'); return { ...__HELPER_PREFIX__DecodeReadModelBase(record), rows: __HELPER_PREFIX__ReadArray(record, 'rows', __HELPER_PREFIX__DecodeScreenRow) }; }
function __HELPER_PREFIX__DecodeAppUseReadModel(value: unknown): __APP_USE_READ_MODEL_TYPE__ { const record = __HELPER_PREFIX__ReadRecord(value, 'activity app-use read-model'); return { ...__HELPER_PREFIX__DecodeReadModelBase(record), rows: __HELPER_PREFIX__ReadArray(record, 'rows', __HELPER_PREFIX__DecodeAppUseRow) }; }
function __HELPER_PREFIX__DecodeBrowserReadModel(value: unknown): __BROWSER_READ_MODEL_TYPE__ { const record = __HELPER_PREFIX__ReadRecord(value, 'activity browser read-model'); return { ...__HELPER_PREFIX__DecodeReadModelBase(record), rows: __HELPER_PREFIX__ReadArray(record, 'rows', __HELPER_PREFIX__DecodeBrowserRow) }; }
function __HELPER_PREFIX__DecodeGamesReadModel(value: unknown): __GAMES_READ_MODEL_TYPE__ { const record = __HELPER_PREFIX__ReadRecord(value, 'activity games read-model'); return { ...__HELPER_PREFIX__DecodeReadModelBase(record), rows: __HELPER_PREFIX__ReadArray(record, 'rows', __HELPER_PREFIX__DecodeGamesRow) }; }
function __HELPER_PREFIX__DecodeNetworkReadModel(value: unknown): __NETWORK_READ_MODEL_TYPE__ { const record = __HELPER_PREFIX__ReadRecord(value, 'activity network read-model'); return { ...__HELPER_PREFIX__DecodeReadModelBase(record), rows: __HELPER_PREFIX__ReadArray(record, 'rows', __HELPER_PREFIX__DecodeNetworkRow) }; }
export const __READ_MODEL_STATE_SCHEMA_CONST__ = __HELPER_PREFIX__Schema((value: unknown): __READ_MODEL_STATE_TYPE__ => { if (typeof value !== 'string' || !(Object.values(__READ_MODEL_STATE_CONST__) as readonly string[]).includes(value)) { throw new TypeError('activity read-model state is not Rust-owned'); } return value as __READ_MODEL_STATE_TYPE__; });
export const __REQUEST_SCHEMA_CONST__ = __HELPER_PREFIX__Schema(__HELPER_PREFIX__DecodeRequest);
export const __REPORT_DOCUMENT_SCHEMA_CONST__ = __HELPER_PREFIX__Schema(__HELPER_PREFIX__DecodeReportDocument);
export const __HISTORY_LIST_SCHEMA_CONST__ = __HELPER_PREFIX__Schema(__HELPER_PREFIX__DecodeHistoricalReportList);
export const __SCREEN_READ_MODEL_SCHEMA_CONST__ = __HELPER_PREFIX__Schema(__HELPER_PREFIX__DecodeScreenReadModel);
export const __APP_USE_READ_MODEL_SCHEMA_CONST__ = __HELPER_PREFIX__Schema(__HELPER_PREFIX__DecodeAppUseReadModel);
export const __BROWSER_READ_MODEL_SCHEMA_CONST__ = __HELPER_PREFIX__Schema(__HELPER_PREFIX__DecodeBrowserReadModel);
export const __GAMES_READ_MODEL_SCHEMA_CONST__ = __HELPER_PREFIX__Schema(__HELPER_PREFIX__DecodeGamesReadModel);
export const __NETWORK_READ_MODEL_SCHEMA_CONST__ = __HELPER_PREFIX__Schema(__HELPER_PREFIX__DecodeNetworkReadModel);
"#
        .to_string(),
        &[
            ("__SCHEMA_VERSION_CONST__", schema_version_const),
            ("__SCOPE_KIND_CONST__", scope_kind_const),
            ("__SCOPE_KIND_TYPE__", scope_kind_type),
            ("__REPORT_FREQUENCY_CONST__", report_frequency_const),
            ("__REPORT_FREQUENCY_TYPE__", report_frequency_type),
            ("__SECTION_KIND_CONST__", section_kind_const),
            ("__SECTION_KIND_TYPE__", section_kind_type),
            ("__READ_MODEL_STATE_CONST__", read_model_state_const),
            ("__READ_MODEL_STATE_TYPE__", read_model_state_type),
            ("__SOURCE_REACHABILITY_CONST__", source_reachability_const),
            ("__SOURCE_REACHABILITY_TYPE__", source_reachability_type),
            ("__SAVED_REPORT_STATE_CONST__", saved_report_state_const),
            ("__SAVED_REPORT_STATE_TYPE__", saved_report_state_type),
            ("__CUSTODY_LABEL_CONST__", custody_label_const),
            ("__CUSTODY_LABEL_TYPE__", custody_label_type),
            ("__SOURCE_LABEL_CONST__", source_label_const),
            ("__SOURCE_LABEL_TYPE__", source_label_type),
            ("__EVIDENCE_KIND_CONST__", evidence_kind_const),
            ("__EVIDENCE_KIND_TYPE__", evidence_kind_type),
            ("__READ_MODEL_KIND_NAME_CONST__", read_model_kind_name_const),
            ("__READ_MODEL_KIND_TYPE__", read_model_kind_type),
            ("__PARSER_TYPE__", parser_type),
            ("__EVIDENCE_REF_TYPE__", evidence_ref_type),
            ("__SCOPE_TYPE__", scope_type),
            ("__REQUEST_TYPE__", request_type),
            ("__SOURCE_STATE_TYPE__", source_state_type),
            ("__SECTION_TYPE__", section_type),
            ("__SAVED_METADATA_TYPE__", saved_metadata_type),
            ("__SOURCE_STATE_SUMMARY_TYPE__", source_state_summary_type),
            ("__REPORT_DOCUMENT_TYPE__", report_document_type),
            ("__HISTORY_ITEM_TYPE__", history_item_type),
            ("__HISTORY_LIST_TYPE__", history_list_type),
            ("__SOURCE_STATUS_ROW_TYPE__", source_status_row_type),
            ("__TAB_READ_MODEL_TYPE__", tab_read_model_type),
            ("__SCREEN_ROW_TYPE__", screen_row_type),
            ("__APP_USE_ROW_TYPE__", app_use_row_type),
            ("__BROWSER_ROW_TYPE__", browser_row_type),
            ("__GAMES_ROW_TYPE__", games_row_type),
            ("__NETWORK_ROW_TYPE__", network_row_type),
            ("__SCREEN_READ_MODEL_TYPE__", screen_read_model_type),
            ("__APP_USE_READ_MODEL_TYPE__", app_use_read_model_type),
            ("__BROWSER_READ_MODEL_TYPE__", browser_read_model_type),
            ("__GAMES_READ_MODEL_TYPE__", games_read_model_type),
            ("__NETWORK_READ_MODEL_TYPE__", network_read_model_type),
            ("__SURFACE_READ_MODEL_TYPE__", surface_read_model_type),
            ("__READ_MODEL_STATE_SCHEMA_CONST__", read_model_state_schema_const),
            ("__REQUEST_SCHEMA_CONST__", request_schema_const),
            ("__REPORT_DOCUMENT_SCHEMA_CONST__", report_document_schema_const),
            ("__HISTORY_LIST_SCHEMA_CONST__", history_list_schema_const),
            (
                "__SCREEN_READ_MODEL_SCHEMA_CONST__",
                screen_read_model_schema_const,
            ),
            (
                "__APP_USE_READ_MODEL_SCHEMA_CONST__",
                app_use_read_model_schema_const,
            ),
            (
                "__BROWSER_READ_MODEL_SCHEMA_CONST__",
                browser_read_model_schema_const,
            ),
            (
                "__GAMES_READ_MODEL_SCHEMA_CONST__",
                games_read_model_schema_const,
            ),
            (
                "__NETWORK_READ_MODEL_SCHEMA_CONST__",
                network_read_model_schema_const,
            ),
            ("__HELPER_PREFIX__", helper_prefix),
        ],
    )
}

fn activity_surface_adapter_manifest_typescript(
    names: &ProtocolBridgeNames,
    operation_id_const: &str,
    command_builder_const: &str,
    event_parser_const: &str,
    operation_type: &str,
    failure_reason_type: &str,
    response_kind_type: &str,
    read_model_kind_type: &str,
    manifest_const: &str,
) -> String {
    replace_tokens(
        r#"
export const __OPERATION_ID_CONST__ = { GetDailyReport: "getDailyReport", GetWeeklyReport: "getWeeklyReport", GetMonthlyReport: "getMonthlyReport", SaveActivityReport: "saveActivityReport", ListHistoricalReports: "listHistoricalReports", GetScreenActivity: "getScreenActivity", GetAppUseActivity: "getAppUseActivity", GetBrowserActivity: "getBrowserActivity", GetGamesActivity: "getGamesActivity", GetNetworkActivity: "getNetworkActivity" } as const;
export const __COMMAND_BUILDER_CONST__ = { ReportGenerate: "createActivityReportGenerateCommand", ReportSave: "createActivityReportSaveCommand", ReportHistory: "createActivityReportHistoryCommand", ReadModel: "createActivityReadModelCommand" } as const;
export const __EVENT_PARSER_CONST__ = { ReportDocument: "parseActivityReportDocumentEvent", ReportHistory: "parseActivityReportHistoryEvent", ReadModel: "parseActivityReadModelEvent" } as const;
export type __FAILURE_REASON_TYPE__ = "wrong-event" | "missing-json-field" | "invalid-json" | "invalid-payload";
export type __RESPONSE_KIND_TYPE__ = "report-document" | "report-history" | "tab-read-model";
export type __OPERATION_TYPE__ = { readonly operationId: (typeof __OPERATION_ID_CONST__)[keyof typeof __OPERATION_ID_CONST__]; readonly command: __COMMAND_TYPE__; readonly successEvent: __EVENT_TYPE__; readonly payloadField: __FIELD_TYPE__; readonly commandBuilder: (typeof __COMMAND_BUILDER_CONST__)[keyof typeof __COMMAND_BUILDER_CONST__]; readonly eventParser: (typeof __EVENT_PARSER_CONST__)[keyof typeof __EVENT_PARSER_CONST__]; readonly responseKind: __RESPONSE_KIND_TYPE__; readonly readModelKind: __READ_MODEL_KIND_TYPE__ | null; readonly productDataOwner: "rust-service-read-model"; readonly uiConsumer: "c-owned-activity-ui"; readonly viteDataOwner: false; readonly supportsFamilyScope: boolean; readonly supportsDeviceScope: boolean; readonly failureState: "unavailable"; readonly failureReasons: readonly __FAILURE_REASON_TYPE__[]; readonly unavailableState: "unavailable" };
function __OPERATION_FN__(operationId: __OPERATION_TYPE__["operationId"], command: __COMMAND_TYPE__, successEvent: __EVENT_TYPE__, payloadField: __FIELD_TYPE__, responseKind: __RESPONSE_KIND_TYPE__, readModelKind: __READ_MODEL_KIND_TYPE__ | null): __OPERATION_TYPE__ { const commandBuilder = operationId === __OPERATION_ID_CONST__.SaveActivityReport ? __COMMAND_BUILDER_CONST__.ReportSave : operationId === __OPERATION_ID_CONST__.ListHistoricalReports ? __COMMAND_BUILDER_CONST__.ReportHistory : readModelKind === null ? __COMMAND_BUILDER_CONST__.ReportGenerate : __COMMAND_BUILDER_CONST__.ReadModel; const eventParser = responseKind === "report-history" ? __EVENT_PARSER_CONST__.ReportHistory : responseKind === "tab-read-model" ? __EVENT_PARSER_CONST__.ReadModel : __EVENT_PARSER_CONST__.ReportDocument; return { operationId, command, successEvent, payloadField, commandBuilder, eventParser, responseKind, readModelKind, productDataOwner: "rust-service-read-model", uiConsumer: "c-owned-activity-ui", viteDataOwner: false, supportsFamilyScope: true, supportsDeviceScope: true, failureState: "unavailable", failureReasons: ["wrong-event", "missing-json-field", "invalid-json", "invalid-payload"], unavailableState: "unavailable" }; }
export const __MANIFEST_CONST__ = [__OPERATION_FN__(__OPERATION_ID_CONST__.GetDailyReport, __COMMAND_CONST__.ActivityReportDailyGenerate, __EVENT_CONST__.ActivityReportGenerated, __FIELD_CONST__.ActivityReportDocument, "report-document", null), __OPERATION_FN__(__OPERATION_ID_CONST__.GetWeeklyReport, __COMMAND_CONST__.ActivityReportWeeklyGenerate, __EVENT_CONST__.ActivityReportGenerated, __FIELD_CONST__.ActivityReportDocument, "report-document", null), __OPERATION_FN__(__OPERATION_ID_CONST__.GetMonthlyReport, __COMMAND_CONST__.ActivityReportMonthlyGenerate, __EVENT_CONST__.ActivityReportGenerated, __FIELD_CONST__.ActivityReportDocument, "report-document", null), __OPERATION_FN__(__OPERATION_ID_CONST__.SaveActivityReport, __COMMAND_CONST__.ActivityReportSave, __EVENT_CONST__.ActivityReportSaved, __FIELD_CONST__.ActivityReportDocument, "report-document", null), __OPERATION_FN__(__OPERATION_ID_CONST__.ListHistoricalReports, __COMMAND_CONST__.ActivityReportHistoryList, __EVENT_CONST__.ActivityReportHistoryReported, __FIELD_CONST__.ActivityReports, "report-history", null), __OPERATION_FN__(__OPERATION_ID_CONST__.GetScreenActivity, __COMMAND_CONST__.ActivityScreenReadModelGet, __EVENT_CONST__.ActivityScreenReadModelReported, __FIELD_CONST__.ActivityReadModel, "tab-read-model", __READ_MODEL_KIND_NAME_CONST__.Screen), __OPERATION_FN__(__OPERATION_ID_CONST__.GetAppUseActivity, __COMMAND_CONST__.ActivityAppUseReadModelGet, __EVENT_CONST__.ActivityAppUseReadModelReported, __FIELD_CONST__.ActivityReadModel, "tab-read-model", __READ_MODEL_KIND_NAME_CONST__.AppUse), __OPERATION_FN__(__OPERATION_ID_CONST__.GetBrowserActivity, __COMMAND_CONST__.ActivityBrowserReadModelGet, __EVENT_CONST__.ActivityBrowserReadModelReported, __FIELD_CONST__.ActivityReadModel, "tab-read-model", __READ_MODEL_KIND_NAME_CONST__.Browser), __OPERATION_FN__(__OPERATION_ID_CONST__.GetGamesActivity, __COMMAND_CONST__.ActivityGamesReadModelGet, __EVENT_CONST__.ActivityGamesReadModelReported, __FIELD_CONST__.ActivityReadModel, "tab-read-model", __READ_MODEL_KIND_NAME_CONST__.Games), __OPERATION_FN__(__OPERATION_ID_CONST__.GetNetworkActivity, __COMMAND_CONST__.ActivityNetworkReadModelGet, __EVENT_CONST__.ActivityNetworkReadModelReported, __FIELD_CONST__.ActivityReadModel, "tab-read-model", __READ_MODEL_KIND_NAME_CONST__.Network)] as const satisfies readonly __OPERATION_TYPE__[];
"#
        .to_string(),
        &[
            ("__OPERATION_ID_CONST__", operation_id_const),
            ("__COMMAND_BUILDER_CONST__", command_builder_const),
            ("__EVENT_PARSER_CONST__", event_parser_const),
            ("__OPERATION_TYPE__", operation_type),
            ("__FAILURE_REASON_TYPE__", failure_reason_type),
            ("__RESPONSE_KIND_TYPE__", response_kind_type),
            ("__READ_MODEL_KIND_TYPE__", read_model_kind_type),
            ("__COMMAND_TYPE__", names.command_type),
            ("__EVENT_TYPE__", names.event_type),
            ("__FIELD_TYPE__", names.field_type),
            ("__COMMAND_CONST__", names.command_const),
            ("__EVENT_CONST__", names.event_const),
            ("__FIELD_CONST__", names.field_const),
            ("__READ_MODEL_KIND_NAME_CONST__", &format!("{}ActivitySurfaceReadModelKindName", bridge_prefix(names))),
            ("__MANIFEST_CONST__", manifest_const),
            ("__OPERATION_FN__", &format!("{}ActivitySurfaceAdapterOperation", bridge_prefix(names))),
        ],
    )
}

fn activity_surface_scope_kind_descriptors(
) -> Vec<ProtocolLiteralDescriptor<ActivitySurfaceScopeKind>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "Family",
            value: ActivitySurfaceScopeKind::Family,
        },
        ProtocolLiteralDescriptor {
            key: "Device",
            value: ActivitySurfaceScopeKind::Device,
        },
    ]
}

fn activity_report_frequency_descriptors() -> Vec<ProtocolLiteralDescriptor<ActivityReportFrequency>>
{
    vec![
        ProtocolLiteralDescriptor {
            key: "Daily",
            value: ActivityReportFrequency::Daily,
        },
        ProtocolLiteralDescriptor {
            key: "Weekly",
            value: ActivityReportFrequency::Weekly,
        },
        ProtocolLiteralDescriptor {
            key: "Monthly",
            value: ActivityReportFrequency::Monthly,
        },
    ]
}

fn activity_report_section_kind_descriptors(
) -> Vec<ProtocolLiteralDescriptor<ActivityReportSectionKind>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "Summary",
            value: ActivityReportSectionKind::Summary,
        },
        ProtocolLiteralDescriptor {
            key: "Screen",
            value: ActivityReportSectionKind::Screen,
        },
        ProtocolLiteralDescriptor {
            key: "AppUse",
            value: ActivityReportSectionKind::AppUse,
        },
        ProtocolLiteralDescriptor {
            key: "Browser",
            value: ActivityReportSectionKind::Browser,
        },
        ProtocolLiteralDescriptor {
            key: "Games",
            value: ActivityReportSectionKind::Games,
        },
        ProtocolLiteralDescriptor {
            key: "Network",
            value: ActivityReportSectionKind::Network,
        },
    ]
}

fn activity_read_model_state_descriptors() -> Vec<ProtocolLiteralDescriptor<ActivityReadModelState>>
{
    vec![
        ProtocolLiteralDescriptor {
            key: "Ready",
            value: ActivityReadModelState::Ready,
        },
        ProtocolLiteralDescriptor {
            key: "Empty",
            value: ActivityReadModelState::Empty,
        },
        ProtocolLiteralDescriptor {
            key: "Unavailable",
            value: ActivityReadModelState::Unavailable,
        },
        ProtocolLiteralDescriptor {
            key: "Offline",
            value: ActivityReadModelState::Offline,
        },
        ProtocolLiteralDescriptor {
            key: "Stale",
            value: ActivityReadModelState::Stale,
        },
        ProtocolLiteralDescriptor {
            key: "PermissionRequired",
            value: ActivityReadModelState::PermissionRequired,
        },
        ProtocolLiteralDescriptor {
            key: "ScaffoldOnly",
            value: ActivityReadModelState::ScaffoldOnly,
        },
    ]
}

fn activity_report_source_reachability_state_descriptors(
) -> Vec<ProtocolLiteralDescriptor<ActivityReportSourceReachabilityState>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "Reachable",
            value: ActivityReportSourceReachabilityState::Reachable,
        },
        ProtocolLiteralDescriptor {
            key: "Unreachable",
            value: ActivityReportSourceReachabilityState::Unreachable,
        },
        ProtocolLiteralDescriptor {
            key: "Offline",
            value: ActivityReportSourceReachabilityState::Offline,
        },
        ProtocolLiteralDescriptor {
            key: "Error",
            value: ActivityReportSourceReachabilityState::Error,
        },
    ]
}

fn activity_saved_report_state_descriptors(
) -> Vec<ProtocolLiteralDescriptor<ActivitySavedReportState>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "Draft",
            value: ActivitySavedReportState::Draft,
        },
        ProtocolLiteralDescriptor {
            key: "Saved",
            value: ActivitySavedReportState::Saved,
        },
        ProtocolLiteralDescriptor {
            key: "StorageUnavailable",
            value: ActivitySavedReportState::StorageUnavailable,
        },
        ProtocolLiteralDescriptor {
            key: "Degraded",
            value: ActivitySavedReportState::Degraded,
        },
        ProtocolLiteralDescriptor {
            key: "ScaffoldOnly",
            value: ActivitySavedReportState::ScaffoldOnly,
        },
    ]
}

fn activity_report_custody_label_descriptors(
) -> Vec<ProtocolLiteralDescriptor<ActivityReportCustodyLabel>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "ChildDeviceLocalSummary",
            value: ActivityReportCustodyLabel::ChildDeviceLocalSummary,
        },
        ProtocolLiteralDescriptor {
            key: "ParentDeviceLocalReportJson",
            value: ActivityReportCustodyLabel::ParentDeviceLocalReportJson,
        },
        ProtocolLiteralDescriptor {
            key: "ParentDeviceLocalHistory",
            value: ActivityReportCustodyLabel::ParentDeviceLocalHistory,
        },
    ]
}

fn activity_report_source_label_descriptors(
) -> Vec<ProtocolLiteralDescriptor<ActivityReportSourceLabel>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "ActivityQueryStoreSummary",
            value: ActivityReportSourceLabel::ActivityQueryStoreSummary,
        },
        ProtocolLiteralDescriptor {
            key: "FamilyFanoutSourceState",
            value: ActivityReportSourceLabel::FamilyFanoutSourceState,
        },
        ProtocolLiteralDescriptor {
            key: "SavedReportJson",
            value: ActivityReportSourceLabel::SavedReportJson,
        },
        ProtocolLiteralDescriptor {
            key: "SavedReportHistory",
            value: ActivityReportSourceLabel::SavedReportHistory,
        },
    ]
}

fn activity_evidence_kind_descriptors() -> Vec<ProtocolLiteralDescriptor<ActivityEvidenceKind>> {
    vec![
        ProtocolLiteralDescriptor {
            key: "JournalEntry",
            value: ActivityEvidenceKind::JournalEntry,
        },
        ProtocolLiteralDescriptor {
            key: "Screenshot",
            value: ActivityEvidenceKind::Screenshot,
        },
        ProtocolLiteralDescriptor {
            key: "StorageObject",
            value: ActivityEvidenceKind::StorageObject,
        },
        ProtocolLiteralDescriptor {
            key: "LocalDbRow",
            value: ActivityEvidenceKind::LocalDbRow,
        },
    ]
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

