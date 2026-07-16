use super::*;

#[path = "parent_agent_protocol_bridge_ts_part2_typescript_decoder.rs"]
mod parent_agent_protocol_bridge_ts_part2_typescript_decoder;

const ACTIVITY_SURFACE_ADAPTER_MANIFEST_TYPESCRIPT_TEMPLATE: &str = r#"
export const __OPERATION_ID_CONST__ = { GetDailyReport: "getDailyReport", GetWeeklyReport: "getWeeklyReport", GetMonthlyReport: "getMonthlyReport", SaveActivityReport: "saveActivityReport", ListHistoricalReports: "listHistoricalReports", GetScreenActivity: "getScreenActivity", GetAppUseActivity: "getAppUseActivity", GetBrowserActivity: "getBrowserActivity", GetGamesActivity: "getGamesActivity", GetNetworkActivity: "getNetworkActivity" } as const;
export const __COMMAND_BUILDER_CONST__ = { ReportGenerate: "createActivityReportGenerateCommand", ReportSave: "createActivityReportSaveCommand", ReportHistory: "createActivityReportHistoryCommand", ReadModel: "createActivityReadModelCommand" } as const;
export const __EVENT_PARSER_CONST__ = { ReportDocument: "parseActivityReportDocumentEvent", ReportHistory: "parseActivityReportHistoryEvent", ReadModel: "parseActivityReadModelEvent" } as const;
export type __FAILURE_REASON_TYPE__ = "wrong-event" | "missing-json-field" | "invalid-json" | "invalid-payload";
export type __RESPONSE_KIND_TYPE__ = "report-document" | "report-history" | "tab-read-model";
export type __OPERATION_TYPE__ = { readonly operationId: (typeof __OPERATION_ID_CONST__)[keyof typeof __OPERATION_ID_CONST__]; readonly command: __COMMAND_TYPE__; readonly successEvent: __EVENT_TYPE__; readonly payloadField: __FIELD_TYPE__; readonly commandBuilder: (typeof __COMMAND_BUILDER_CONST__)[keyof typeof __COMMAND_BUILDER_CONST__]; readonly eventParser: (typeof __EVENT_PARSER_CONST__)[keyof typeof __EVENT_PARSER_CONST__]; readonly responseKind: __RESPONSE_KIND_TYPE__; readonly readModelKind: __READ_MODEL_KIND_TYPE__ | null; readonly productDataOwner: "rust-service-read-model"; readonly uiConsumer: "c-owned-activity-ui"; readonly viteDataOwner: false; readonly supportsFamilyScope: boolean; readonly supportsDeviceScope: boolean; readonly failureState: "unavailable"; readonly failureReasons: readonly __FAILURE_REASON_TYPE__[]; readonly unavailableState: "unavailable" };
function __OPERATION_FN__(operationId: __OPERATION_TYPE__["operationId"], command: __COMMAND_TYPE__, successEvent: __EVENT_TYPE__, payloadField: __FIELD_TYPE__, responseKind: __RESPONSE_KIND_TYPE__, readModelKind: __READ_MODEL_KIND_TYPE__ | null): __OPERATION_TYPE__ { const commandBuilder = operationId === __OPERATION_ID_CONST__.SaveActivityReport ? __COMMAND_BUILDER_CONST__.ReportSave : operationId === __OPERATION_ID_CONST__.ListHistoricalReports ? __COMMAND_BUILDER_CONST__.ReportHistory : readModelKind === null ? __COMMAND_BUILDER_CONST__.ReportGenerate : __COMMAND_BUILDER_CONST__.ReadModel; const eventParser = responseKind === "report-history" ? __EVENT_PARSER_CONST__.ReportHistory : responseKind === "tab-read-model" ? __EVENT_PARSER_CONST__.ReadModel : __EVENT_PARSER_CONST__.ReportDocument; return { operationId, command, successEvent, payloadField, commandBuilder, eventParser, responseKind, readModelKind, productDataOwner: "rust-service-read-model", uiConsumer: "c-owned-activity-ui", viteDataOwner: false, supportsFamilyScope: true, supportsDeviceScope: true, failureState: "unavailable", failureReasons: ["wrong-event", "missing-json-field", "invalid-json", "invalid-payload"], unavailableState: "unavailable" }; }
export const __MANIFEST_CONST__ = [__OPERATION_FN__(__OPERATION_ID_CONST__.GetDailyReport, __COMMAND_CONST__.ActivityReportDailyGenerate, __EVENT_CONST__.ActivityReportGenerated, __FIELD_CONST__.ActivityReportDocument, "report-document", null), __OPERATION_FN__(__OPERATION_ID_CONST__.GetWeeklyReport, __COMMAND_CONST__.ActivityReportWeeklyGenerate, __EVENT_CONST__.ActivityReportGenerated, __FIELD_CONST__.ActivityReportDocument, "report-document", null), __OPERATION_FN__(__OPERATION_ID_CONST__.GetMonthlyReport, __COMMAND_CONST__.ActivityReportMonthlyGenerate, __EVENT_CONST__.ActivityReportGenerated, __FIELD_CONST__.ActivityReportDocument, "report-document", null), __OPERATION_FN__(__OPERATION_ID_CONST__.SaveActivityReport, __COMMAND_CONST__.ActivityReportSave, __EVENT_CONST__.ActivityReportSaved, __FIELD_CONST__.ActivityReportDocument, "report-document", null), __OPERATION_FN__(__OPERATION_ID_CONST__.ListHistoricalReports, __COMMAND_CONST__.ActivityReportHistoryList, __EVENT_CONST__.ActivityReportHistoryReported, __FIELD_CONST__.ActivityReports, "report-history", null), __OPERATION_FN__(__OPERATION_ID_CONST__.GetScreenActivity, __COMMAND_CONST__.ActivityScreenReadModelGet, __EVENT_CONST__.ActivityScreenReadModelReported, __FIELD_CONST__.ActivityReadModel, "tab-read-model", __READ_MODEL_KIND_NAME_CONST__.Screen), __OPERATION_FN__(__OPERATION_ID_CONST__.GetAppUseActivity, __COMMAND_CONST__.ActivityAppUseReadModelGet, __EVENT_CONST__.ActivityAppUseReadModelReported, __FIELD_CONST__.ActivityReadModel, "tab-read-model", __READ_MODEL_KIND_NAME_CONST__.AppUse), __OPERATION_FN__(__OPERATION_ID_CONST__.GetBrowserActivity, __COMMAND_CONST__.ActivityBrowserReadModelGet, __EVENT_CONST__.ActivityBrowserReadModelReported, __FIELD_CONST__.ActivityReadModel, "tab-read-model", __READ_MODEL_KIND_NAME_CONST__.Browser), __OPERATION_FN__(__OPERATION_ID_CONST__.GetGamesActivity, __COMMAND_CONST__.ActivityGamesReadModelGet, __EVENT_CONST__.ActivityGamesReadModelReported, __FIELD_CONST__.ActivityReadModel, "tab-read-model", __READ_MODEL_KIND_NAME_CONST__.Games), __OPERATION_FN__(__OPERATION_ID_CONST__.GetNetworkActivity, __COMMAND_CONST__.ActivityNetworkReadModelGet, __EVENT_CONST__.ActivityNetworkReadModelReported, __FIELD_CONST__.ActivityReadModel, "tab-read-model", __READ_MODEL_KIND_NAME_CONST__.Network)] as const satisfies readonly __OPERATION_TYPE__[];
"#;

struct ActivitySurfaceTypescriptNames {
    schema_version_const: String,
    scope_kind_const: String,
    scope_kind_type: String,
    report_frequency_const: String,
    report_frequency_type: String,
    section_kind_const: String,
    section_kind_type: String,
    read_model_state_const: String,
    read_model_state_type: String,
    source_reachability_const: String,
    source_reachability_type: String,
    saved_report_state_const: String,
    saved_report_state_type: String,
    custody_label_const: String,
    custody_label_type: String,
    source_label_const: String,
    source_label_type: String,
    evidence_kind_const: String,
    evidence_kind_type: String,
    read_model_kind_name_const: String,
    read_model_kind_type: String,
    parser_type: String,
    evidence_ref_type: String,
    scope_type: String,
    request_type: String,
    source_state_type: String,
    section_type: String,
    saved_metadata_type: String,
    source_state_summary_type: String,
    report_document_type: String,
    history_item_type: String,
    history_list_type: String,
    source_status_row_type: String,
    tab_read_model_type: String,
    screen_row_type: String,
    app_use_row_type: String,
    browser_row_type: String,
    games_row_type: String,
    network_row_type: String,
    screen_read_model_type: String,
    app_use_read_model_type: String,
    browser_read_model_type: String,
    games_read_model_type: String,
    network_read_model_type: String,
    surface_read_model_type: String,
    read_model_state_schema_const: String,
    request_schema_const: String,
    report_document_schema_const: String,
    history_list_schema_const: String,
    screen_read_model_schema_const: String,
    app_use_read_model_schema_const: String,
    browser_read_model_schema_const: String,
    games_read_model_schema_const: String,
    network_read_model_schema_const: String,
    operation_id_const: String,
    command_builder_const: String,
    event_parser_const: String,
    operation_type: String,
    failure_reason_type: String,
    response_kind_type: String,
    manifest_const: String,
    helper_prefix: String,
}

pub(super) fn activity_surface_contract_typescript(names: &ProtocolBridgeNames) -> String {
    let types = activity_surface_typescript_names(bridge_prefix(names));
    [
        format!(
            "export const {} = {ACTIVITY_SURFACE_SCHEMA_VERSION} as const;",
            types.schema_version_const
        ),
        literal_typescript(
            &types.scope_kind_const,
            &types.scope_kind_type,
            &activity_surface_scope_kind_descriptors(),
        ),
        literal_typescript(
            &types.report_frequency_const,
            &types.report_frequency_type,
            &activity_report_frequency_descriptors(),
        ),
        literal_typescript(
            &types.section_kind_const,
            &types.section_kind_type,
            &activity_report_section_kind_descriptors(),
        ),
        literal_typescript(
            &types.read_model_state_const,
            &types.read_model_state_type,
            &activity_read_model_state_descriptors(),
        ),
        literal_typescript(
            &types.source_reachability_const,
            &types.source_reachability_type,
            &activity_report_source_reachability_state_descriptors(),
        ),
        literal_typescript(
            &types.saved_report_state_const,
            &types.saved_report_state_type,
            &activity_saved_report_state_descriptors(),
        ),
        literal_typescript(
            &types.custody_label_const,
            &types.custody_label_type,
            &activity_report_custody_label_descriptors(),
        ),
        literal_typescript(
            &types.source_label_const,
            &types.source_label_type,
            &activity_report_source_label_descriptors(),
        ),
        literal_typescript(
            &types.evidence_kind_const,
            &types.evidence_kind_type,
            &activity_evidence_kind_descriptors(),
        ),
        parent_agent_protocol_bridge_ts_part2_typescript_decoder::activity_surface_decoder_typescript(&types),
        activity_surface_adapter_manifest_typescript(names, &types),
    ]
    .join(" ")
}

fn activity_surface_typescript_names(prefix: &str) -> ActivitySurfaceTypescriptNames {
    let scope_kind_const = format!("{prefix}ActivitySurfaceScopeKind");
    let report_frequency_const = format!("{prefix}ActivityReportFrequency");
    let section_kind_const = format!("{prefix}ActivityReportSectionKind");
    let read_model_state_const = format!("{prefix}ActivityReadModelState");
    let source_reachability_const = format!("{prefix}ActivityReportSourceReachabilityState");
    let saved_report_state_const = format!("{prefix}ActivitySavedReportState");
    let custody_label_const = format!("{prefix}ActivityReportCustodyLabel");
    let source_label_const = format!("{prefix}ActivityReportSourceLabel");
    let evidence_kind_const = format!("{prefix}ActivityEvidenceKind");

    ActivitySurfaceTypescriptNames {
        schema_version_const: format!("{prefix}ActivitySurfaceSchemaVersion"),
        scope_kind_type: scope_kind_const.clone(),
        report_frequency_type: report_frequency_const.clone(),
        section_kind_type: section_kind_const.clone(),
        read_model_state_type: read_model_state_const.clone(),
        source_reachability_type: source_reachability_const.clone(),
        saved_report_state_type: saved_report_state_const.clone(),
        custody_label_type: custody_label_const.clone(),
        source_label_type: source_label_const.clone(),
        evidence_kind_type: evidence_kind_const.clone(),
        scope_kind_const,
        report_frequency_const,
        section_kind_const,
        read_model_state_const,
        source_reachability_const,
        saved_report_state_const,
        custody_label_const,
        source_label_const,
        evidence_kind_const,
        read_model_kind_name_const: format!("{prefix}ActivitySurfaceReadModelKindName"),
        read_model_kind_type: format!("{prefix}ActivitySurfaceReadModelKind"),
        parser_type: format!("{prefix}ActivitySurfaceSchemaParser"),
        evidence_ref_type: format!("{prefix}ActivityEvidenceRef"),
        scope_type: format!("{prefix}ActivitySurfaceScope"),
        request_type: format!("{prefix}ActivitySurfaceRequest"),
        source_state_type: format!("{prefix}ActivityReportSourceState"),
        section_type: format!("{prefix}ActivityReportSection"),
        saved_metadata_type: format!("{prefix}ActivitySavedReportMetadata"),
        source_state_summary_type: format!("{prefix}ActivityReportSourceStateSummary"),
        report_document_type: format!("{prefix}ActivityReportDocument"),
        history_item_type: format!("{prefix}ActivityHistoricalReportListItem"),
        history_list_type: format!("{prefix}ActivityHistoricalReportList"),
        source_status_row_type: format!("{prefix}ActivityAppGameSourceStatusRow"),
        tab_read_model_type: format!("{prefix}ActivityTabReadModel"),
        screen_row_type: format!("{prefix}ActivityScreenReadModelRow"),
        app_use_row_type: format!("{prefix}ActivityAppUseReadModelRow"),
        browser_row_type: format!("{prefix}ActivityBrowserReadModelRow"),
        games_row_type: format!("{prefix}ActivityGamesReadModelRow"),
        network_row_type: format!("{prefix}ActivityNetworkReadModelRow"),
        screen_read_model_type: format!("{prefix}ActivityScreenReadModel"),
        app_use_read_model_type: format!("{prefix}ActivityAppUseReadModel"),
        browser_read_model_type: format!("{prefix}ActivityBrowserReadModel"),
        games_read_model_type: format!("{prefix}ActivityGamesReadModel"),
        network_read_model_type: format!("{prefix}ActivityNetworkReadModel"),
        surface_read_model_type: format!("{prefix}ActivitySurfaceReadModel"),
        read_model_state_schema_const: format!("{prefix}ActivityReadModelStateSchema"),
        request_schema_const: format!("{prefix}ActivitySurfaceRequestSchema"),
        report_document_schema_const: format!("{prefix}ActivityReportDocumentSchema"),
        history_list_schema_const: format!("{prefix}ActivityHistoricalReportListSchema"),
        screen_read_model_schema_const: format!("{prefix}ActivityScreenReadModelSchema"),
        app_use_read_model_schema_const: format!("{prefix}ActivityAppUseReadModelSchema"),
        browser_read_model_schema_const: format!("{prefix}ActivityBrowserReadModelSchema"),
        games_read_model_schema_const: format!("{prefix}ActivityGamesReadModelSchema"),
        network_read_model_schema_const: format!("{prefix}ActivityNetworkReadModelSchema"),
        operation_id_const: format!("{prefix}ActivitySurfaceAdapterOperationId"),
        command_builder_const: format!("{prefix}ActivitySurfaceAdapterCommandBuilder"),
        event_parser_const: format!("{prefix}ActivitySurfaceAdapterEventParser"),
        operation_type: format!("{prefix}ActivitySurfaceAdapterOperation"),
        failure_reason_type: format!("{prefix}ActivitySurfaceAdapterFailureReason"),
        response_kind_type: format!("{prefix}ActivitySurfaceAdapterResponseKind"),
        manifest_const: format!("{prefix}ActivitySurfaceAdapterOperationManifest"),
        helper_prefix: format!("__{prefix}ActivitySurface"),
    }
}

fn activity_surface_adapter_manifest_typescript(
    names: &ProtocolBridgeNames,
    types: &ActivitySurfaceTypescriptNames,
) -> String {
    let read_model_kind_name_const = types.read_model_kind_name_const.as_str();
    let operation_fn = format!("{}ActivitySurfaceAdapterOperation", bridge_prefix(names));
    let replacements = [
        ("__OPERATION_ID_CONST__", types.operation_id_const.as_str()),
        (
            "__COMMAND_BUILDER_CONST__",
            types.command_builder_const.as_str(),
        ),
        ("__EVENT_PARSER_CONST__", types.event_parser_const.as_str()),
        ("__OPERATION_TYPE__", types.operation_type.as_str()),
        (
            "__FAILURE_REASON_TYPE__",
            types.failure_reason_type.as_str(),
        ),
        ("__RESPONSE_KIND_TYPE__", types.response_kind_type.as_str()),
        (
            "__READ_MODEL_KIND_TYPE__",
            types.read_model_kind_type.as_str(),
        ),
        ("__COMMAND_TYPE__", names.command_type),
        ("__EVENT_TYPE__", names.event_type),
        ("__FIELD_TYPE__", names.field_type),
        ("__COMMAND_CONST__", names.command_const),
        ("__EVENT_CONST__", names.event_const),
        ("__FIELD_CONST__", names.field_const),
        ("__READ_MODEL_KIND_NAME_CONST__", read_model_kind_name_const),
        ("__MANIFEST_CONST__", types.manifest_const.as_str()),
        ("__OPERATION_FN__", operation_fn.as_str()),
    ];

    replace_tokens(
        ACTIVITY_SURFACE_ADAPTER_MANIFEST_TYPESCRIPT_TEMPLATE.to_string(),
        &replacements,
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
