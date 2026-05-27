use super::{
    ActivityAppUseReadModel, ActivityBrowserReadModel, ActivityEvidenceKind, ActivityEvidenceRef,
    ActivityGamesReadModel, ActivityHistoricalReportList, ActivityNetworkReadModel,
    ActivityReadModelState, ActivityReportDocument, ActivityReportFrequency, ActivityReportRequest,
    ActivityReportSection, ActivityReportSectionKind, ActivityReportSourceState,
    ActivitySavedReportMetadata, ActivitySavedReportState, ActivityScreenReadModel,
    ActivityScreenReadModelRow, ActivitySurfaceRequest, ActivitySurfaceScope,
    ActivitySurfaceScopeKind, AgentCommandName, AgentEventName, ACTIVITY_SURFACE_SCHEMA_VERSION,
};

#[test]
fn activity_surface_command_names_match_typescript_contracts() {
    let daily = serde_json::to_value(AgentCommandName::AgentActivityReportDailyGenerate)
        .expect("daily command serializes");
    let screen = serde_json::to_value(AgentCommandName::AgentActivityScreenReadModelGet)
        .expect("screen command serializes");
    let answer = serde_json::to_value(AgentEventName::AgentActivityReportGenerated)
        .expect("report event serializes");

    assert_eq!(daily, "agent.activity.report.daily.generate");
    assert_eq!(screen, "agent.activity.screen.read-model.get");
    assert_eq!(answer, "agent.activity.report.generated");
}

#[test]
fn activity_report_document_serializes_report_sections_and_source_states() {
    let report = sample_report_document(ActivityReportFrequency::Daily);
    let serialized = serde_json::to_value(&report).expect("report serializes");

    assert_eq!(serialized["schemaVersion"], ACTIVITY_SURFACE_SCHEMA_VERSION);
    assert_eq!(serialized["frequency"], "daily");
    assert_eq!(serialized["scope"]["scopeKind"], "family");
    assert_eq!(serialized["sourceStates"][1]["state"], "offline");
    assert_eq!(serialized["sections"][1]["sectionKind"], "network");
}

#[test]
fn activity_history_list_carries_saved_report_metadata_and_parsed_document() {
    let list = ActivityHistoricalReportList {
        schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
        request: sample_surface_request(),
        state: ActivityReadModelState::Ready,
        reports: vec![super::ActivityHistoricalReportListItem {
            schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
            report_id: "activity-report-daily-1".to_string(),
            file_name: "activity-report-daily-1.json".to_string(),
            report_date: "2026-05-27T06:21:00Z".to_string(),
            range_start: "2026-05-27T00:00:00Z".to_string(),
            range_end: "2026-05-27T06:20:00Z".to_string(),
            summary: "Saved daily report".to_string(),
            saved_state: ActivitySavedReportState::Saved,
            saved_at: Some("2026-05-27T06:22:00Z".to_string()),
            parsed_report: sample_report_document(ActivityReportFrequency::Daily),
        }],
    };
    let serialized = serde_json::to_value(&list).expect("history serializes");

    assert_eq!(serialized["reports"][0]["savedState"], "saved");
    assert_eq!(
        serialized["reports"][0]["parsedReport"]["frequency"],
        "daily"
    );
}

#[test]
fn activity_tab_read_models_serialize_typed_states_for_all_tabs() {
    let request = sample_surface_request();
    let screen = ActivityScreenReadModel {
        schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
        request: request.clone(),
        state: ActivityReadModelState::Ready,
        generated_at: "2026-05-27T06:21:00Z".to_string(),
        summary: "Screen ready".to_string(),
        rows: vec![ActivityScreenReadModelRow {
            row_id: "screen-row-1".to_string(),
            label: "Foreground use".to_string(),
            device_id: "child-device-1".to_string(),
            state: ActivityReadModelState::Ready,
            total_ms: 3_600_000,
            foreground_ms: 2_400_000,
            background_ms: 1_200_000,
            evidence: vec![sample_evidence()],
        }],
    };
    let app_use = ActivityAppUseReadModel {
        schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
        request: request.clone(),
        state: ActivityReadModelState::Empty,
        generated_at: "2026-05-27T06:21:00Z".to_string(),
        summary: "No app rows".to_string(),
        rows: vec![],
    };
    let browser = ActivityBrowserReadModel {
        schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
        request: request.clone(),
        state: ActivityReadModelState::PermissionRequired,
        generated_at: "2026-05-27T06:21:00Z".to_string(),
        summary: "Browser bridge permission required".to_string(),
        rows: vec![],
    };
    let games = ActivityGamesReadModel {
        schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
        request: request.clone(),
        state: ActivityReadModelState::ScaffoldOnly,
        generated_at: "2026-05-27T06:21:00Z".to_string(),
        summary: "Games catalog scaffold".to_string(),
        rows: vec![],
    };
    let network = ActivityNetworkReadModel {
        schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
        request,
        state: ActivityReadModelState::Unavailable,
        generated_at: "2026-05-27T06:21:00Z".to_string(),
        summary: "Network store unavailable".to_string(),
        rows: vec![],
    };

    assert_eq!(
        serde_json::to_value(screen).expect("screen serializes")["rows"][0]["foregroundMs"],
        2_400_000
    );
    assert_eq!(
        serde_json::to_value(app_use).expect("app use serializes")["state"],
        "empty"
    );
    assert_eq!(
        serde_json::to_value(browser).expect("browser serializes")["state"],
        "permission-required"
    );
    assert_eq!(
        serde_json::to_value(games).expect("games serializes")["state"],
        "scaffold-only"
    );
    assert_eq!(
        serde_json::to_value(network).expect("network serializes")["state"],
        "unavailable"
    );
}

#[test]
fn activity_report_request_serializes_frequency_for_daily_weekly_monthly() {
    let frequencies = [
        (ActivityReportFrequency::Daily, "daily"),
        (ActivityReportFrequency::Weekly, "weekly"),
        (ActivityReportFrequency::Monthly, "monthly"),
    ];

    for (frequency, expected) in frequencies {
        let request = ActivityReportRequest {
            schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
            frequency,
            scope: sample_family_scope(),
            requested_at: "2026-05-27T06:20:00Z".to_string(),
            range_start: "2026-05-27T00:00:00Z".to_string(),
            range_end: "2026-05-27T06:20:00Z".to_string(),
        };

        assert_eq!(
            serde_json::to_value(request).expect("request serializes")["frequency"],
            expected
        );
    }
}

fn sample_report_document(frequency: ActivityReportFrequency) -> ActivityReportDocument {
    ActivityReportDocument {
        schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
        report_id: "activity-report-daily-1".to_string(),
        frequency,
        scope: sample_family_scope(),
        requested_at: "2026-05-27T06:20:00Z".to_string(),
        range_start: "2026-05-27T00:00:00Z".to_string(),
        range_end: "2026-05-27T06:20:00Z".to_string(),
        generated_at: "2026-05-27T06:21:00Z".to_string(),
        saved_metadata: Some(ActivitySavedReportMetadata {
            report_id: "activity-report-daily-1".to_string(),
            file_name: "activity-report-daily-1.json".to_string(),
            saved_state: ActivitySavedReportState::Draft,
            saved_at: None,
            storage_reason: None,
        }),
        source_states: vec![
            ActivityReportSourceState {
                device_id: "child-device-1".to_string(),
                state: ActivityReadModelState::Ready,
                reason: None,
                last_updated_at: Some("2026-05-27T06:19:00Z".to_string()),
            },
            ActivityReportSourceState {
                device_id: "child-device-2".to_string(),
                state: ActivityReadModelState::Offline,
                reason: Some("Device is offline for this family report".to_string()),
                last_updated_at: None,
            },
        ],
        sections: vec![
            ActivityReportSection {
                section_kind: ActivityReportSectionKind::Summary,
                title: "Summary".to_string(),
                state: ActivityReadModelState::Ready,
                summary: "One reachable device and one offline device".to_string(),
                item_count: 2,
                evidence: vec![sample_evidence()],
            },
            ActivityReportSection {
                section_kind: ActivityReportSectionKind::Network,
                title: "Network".to_string(),
                state: ActivityReadModelState::Unavailable,
                summary: "Network read model is not wired on this device".to_string(),
                item_count: 0,
                evidence: vec![],
            },
        ],
    }
}

fn sample_surface_request() -> ActivitySurfaceRequest {
    ActivitySurfaceRequest {
        schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
        scope: ActivitySurfaceScope {
            scope_kind: ActivitySurfaceScopeKind::Device,
            family_id: None,
            device_id: Some("child-device-1".to_string()),
        },
        requested_at: "2026-05-27T06:20:00Z".to_string(),
        range_start: "2026-05-27T00:00:00Z".to_string(),
        range_end: "2026-05-27T06:20:00Z".to_string(),
    }
}

fn sample_family_scope() -> ActivitySurfaceScope {
    ActivitySurfaceScope {
        scope_kind: ActivitySurfaceScopeKind::Family,
        family_id: Some("family-local-1".to_string()),
        device_id: None,
    }
}

fn sample_evidence() -> ActivityEvidenceRef {
    ActivityEvidenceRef {
        evidence_id: "journal-entry-activity-surface-1".to_string(),
        kind: ActivityEvidenceKind::JournalEntry,
        digest: Some("sha256:activity-surface".to_string()),
        uri: None,
    }
}
