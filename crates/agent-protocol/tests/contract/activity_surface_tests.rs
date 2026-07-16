use super::{
    constants, ActivityAppUseReadModel, ActivityBrowserReadModel, ActivityEvidenceKind,
    ActivityEvidenceRef, ActivityGamesReadModel, ActivityHistoricalReportList,
    ActivityNetworkReadModel, ActivityReadModelState, ActivityReportCustodyLabel,
    ActivityReportDocument, ActivityReportFrequency, ActivityReportRequest, ActivityReportSection,
    ActivityReportSectionKind, ActivityReportSourceLabel, ActivityReportSourceReachabilityState,
    ActivityReportSourceState, ActivityReportSourceStateSummary, ActivitySavedReportMetadata,
    ActivitySavedReportState, ActivityScreenReadModel, ActivityScreenReadModelRow,
    ActivitySurfaceRequest, ActivitySurfaceScope, ActivitySurfaceScopeKind, AgentCommandName,
    AgentEventName, ACTIVITY_SURFACE_SCHEMA_VERSION, SCREEN_CAPABILITY_READY,
    SCREEN_CAPTURE_REASON_MANUAL_PARENT_TEST, SCREEN_CAPTURE_SCOPE_ACTIVE_WINDOW,
    SCREEN_CATEGORY_SCHOOL, SCREEN_CUSTODY_JOURNAL, SCREEN_DELETION_DELETED,
    SCREEN_POLICY_CONFIDENCE_READY, SCREEN_PROVIDER_LOCAL_VISION,
};
use crate::activity_surface::source_status;
use ocentra_eventing::expect_value::ExpectValue;

#[test]
fn activity_surface_command_names_match_typescript_contracts() {
    let daily = serde_json::to_value(AgentCommandName::AgentActivityReportDailyGenerate)
        .expect_value("daily command serializes");
    let screen = serde_json::to_value(AgentCommandName::AgentActivityScreenReadModelGet)
        .expect_value("screen command serializes");
    let answer = serde_json::to_value(AgentEventName::AgentActivityReportGenerated)
        .expect_value("report event serializes");

    assert_eq!(daily, "agent.activity.report.daily.generate");
    assert_eq!(screen, "agent.activity.screen.read-model.get");
    assert_eq!(answer, "agent.activity.report.generated");
}

#[test]
fn activity_report_document_serializes_report_sections_and_source_states() {
    let report = sample_report_document(ActivityReportFrequency::Daily);
    let serialized = serde_json::to_value(&report).expect_value("report serializes");

    assert_eq!(serialized["schemaVersion"], ACTIVITY_SURFACE_SCHEMA_VERSION);
    assert_eq!(serialized["frequency"], "daily");
    assert_eq!(serialized["scope"]["scopeKind"], "family");
    assert_eq!(
        serialized["sourceStates"][0]["reachabilityState"],
        "reachable"
    );
    assert_eq!(serialized["sourceStates"][1]["state"], "offline");
    assert_eq!(
        serialized["sourceStates"][1]["sourceLabel"],
        "family-fanout-source-state"
    );
    assert_eq!(
        serialized["sourceStates"][1]["rawChildEvidenceIncluded"],
        false
    );
    assert_eq!(
        serialized["sourceStates"][1]["reachabilityState"],
        "offline"
    );
    assert_eq!(serialized["sections"][1]["sectionKind"], "network");
}

#[test]
fn activity_history_list_carries_saved_report_metadata_and_parsed_document() {
    let list = ActivityHistoricalReportList {
        schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
        request: sample_surface_request(),
        state: ActivityReadModelState::Ready,
        storage_state: ActivitySavedReportState::Saved,
        storage_reason: None,
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
            custody_label: ActivityReportCustodyLabel::ParentDeviceLocalHistory,
            source_label: ActivityReportSourceLabel::SavedReportHistory,
            raw_child_evidence_included: false,
            source_state_summary: ActivityReportSourceStateSummary {
                total_sources: 2,
                ready_sources: 1,
                offline_sources: 1,
                stale_sources: 0,
                unavailable_sources: 0,
                unreachable_sources: 0,
                error_sources: 0,
            },
            parsed_report: sample_report_document(ActivityReportFrequency::Daily),
        }],
    };
    let serialized = serde_json::to_value(&list).expect_value("history serializes");

    assert_eq!(serialized["reports"][0]["savedState"], "saved");
    assert_eq!(serialized["storageState"], "saved");
    assert_eq!(
        serialized["reports"][0]["sourceStateSummary"]["offlineSources"],
        1
    );
    assert_eq!(
        serialized["reports"][0]["parsedReport"]["frequency"],
        "daily"
    );
}

#[test]
fn activity_screen_read_model_serializes_foreground_and_background_ms() {
    let redacted_snippet = constants::activity_store::TEST_SCREEN_OCR_SNIPPET_REDACTED;
    let pii_note = constants::activity_store::TEST_SCREEN_REDACTION_NOTE_PII;
    let screen = ActivityScreenReadModel {
        schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
        request: sample_surface_request(),
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
            capture_reason: SCREEN_CAPTURE_REASON_MANUAL_PARENT_TEST.to_string(),
            capture_scope: SCREEN_CAPTURE_SCOPE_ACTIVE_WINDOW.to_string(),
            capability_status: SCREEN_CAPABILITY_READY.to_string(),
            queue_job_id: "screen-queue-job-1".to_string(),
            model_runtime_ref: "local-vision-runtime-1".to_string(),
            model_id: "local-vision-model-1".to_string(),
            provider_kind: SCREEN_PROVIDER_LOCAL_VISION.to_string(),
            prompt_or_template_version: "screen-template-v1".to_string(),
            primary_category: Some(SCREEN_CATEGORY_SCHOOL.to_string()),
            confidence: SCREEN_POLICY_CONFIDENCE_READY,
            image_deletion_state: SCREEN_DELETION_DELETED.to_string(),
            raw_image_retained: false,
            policy_eligible: true,
            image_digest: "sha256:screen-image-digest".to_string(),
            custody_state: SCREEN_CUSTODY_JOURNAL.to_string(),
            evidence: vec![sample_evidence()],
            policy_decision_ref: Some("screen-policy-decision-1".to_string()),
            policy_action: Some("allow".to_string()),
            policy_reason_codes: vec![
                "screen-summary-linked".to_string(),
                "parent-rule-linked".to_string(),
            ],
            parent_rule_refs: vec!["screen-parent-rule-school".to_string()],
            local_model_runtime_refs: vec!["local-vision-runtime-1".to_string()],
            parent_explanation_refs: vec!["screen-parent-explanation-1".to_string()],
            explanation_reasons: vec![
                "screen-summary-cited".to_string(),
                "policy-decision-cited".to_string(),
            ],
            deletion_reasons: vec!["screen-image-deleted".to_string()],
            ocr_text_snippets: vec![redacted_snippet.to_string()],
            redaction_notes: vec![pii_note.to_string()],
        }],
    };

    let screen_json = serde_json::to_value(&screen).expect_value("screen serializes");
    assert_eq!(screen_json["rows"][0]["foregroundMs"], 2_400_000);
    assert_eq!(
        screen_json["rows"][0]["imageDeletionState"],
        SCREEN_DELETION_DELETED
    );
    assert_eq!(screen_json["rows"][0]["modelId"], "local-vision-model-1");
    assert_eq!(
        screen_json["rows"][0]["promptOrTemplateVersion"],
        "screen-template-v1"
    );
    assert_eq!(screen_json["rows"][0]["rawImageRetained"], false);
    assert_eq!(
        screen_json["rows"][0]["parentExplanationRefs"][0],
        "screen-parent-explanation-1"
    );
    assert_eq!(
        screen_json["rows"][0]["parentRuleRefs"][0],
        "screen-parent-rule-school"
    );
    assert_eq!(
        screen_json["rows"][0]["ocrTextSnippets"][0],
        redacted_snippet
    );
    assert_eq!(screen_json["rows"][0]["redactionNotes"][0], pii_note);
}

#[test]
fn activity_app_use_read_model_serializes_app_game_projection_state() {
    let app_use = ActivityAppUseReadModel {
        schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
        request: sample_surface_request(),
        state: ActivityReadModelState::Ready,
        generated_at: "2026-05-27T06:21:00Z".to_string(),
        summary: "App use ready".to_string(),
        rows: vec![super::ActivityAppUseReadModelRow {
            row_id: "inventory-app-1".to_string(),
            app_name: "Ocentra Fixture App".to_string(),
            device_id: "child-device-1".to_string(),
            state: ActivityReadModelState::Ready,
            product_kind: "nativeApp".to_string(),
            classification_state: "knownApp".to_string(),
            inventory_state: "installed".to_string(),
            runtime_state: "running".to_string(),
            foreground_state: "foreground".to_string(),
            capability_status: "available".to_string(),
            last_observed_at: Some("2026-05-27T06:19:00Z".to_string()),
            total_ms: 60_000,
            launch_count: 1,
            inventory_row_count: 1,
            running_row_count: 1,
            foreground_row_count: 1,
            daily_rollup_count: 1,
            evidence_claim_row_count: 1,
            identity_row_count: 1,
            approval_authority_row_count: 1,
            approval_action_result_row_count: 1,
            platform_authority_matrix_count: 1,
            platform_authority_row_count: 1,
            ai_classifier_result_row_count: 1,
            source_status_rows: vec![source_status::ActivityAppGameSourceStatusRow {
                source_kind: "osInstalledRecord".to_string(),
                state: ActivityReadModelState::Ready,
                row_count: 1,
                last_observed_at: Some("2026-05-27T06:19:00Z".to_string()),
                capability_status: "available".to_string(),
                evidence: vec![sample_evidence()],
            }],
            evidence: vec![sample_evidence()],
        }],
    };

    let app_use_json = serde_json::to_value(app_use).expect_value("app use serializes");
    assert_eq!(app_use_json["rows"][0]["runtimeState"], "running");
    assert_eq!(app_use_json["rows"][0]["foregroundState"], "foreground");
    assert_eq!(app_use_json["rows"][0]["approvalAuthorityRowCount"], 1);
    assert_eq!(app_use_json["rows"][0]["aiClassifierResultRowCount"], 1);
    assert_eq!(
        app_use_json["rows"][0]["sourceStatusRows"][0]["sourceKind"],
        "osInstalledRecord"
    );
}

#[test]
fn activity_browser_read_model_serializes_permission_required_state() {
    let browser = ActivityBrowserReadModel {
        schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
        request: sample_surface_request(),
        state: ActivityReadModelState::PermissionRequired,
        generated_at: "2026-05-27T06:21:00Z".to_string(),
        summary: "Browser bridge permission required".to_string(),
        rows: vec![],
    };

    let browser_json = serde_json::to_value(browser).expect_value("browser serializes");
    assert_eq!(browser_json["state"], "permission-required");
}

#[test]
fn activity_games_read_model_serializes_launcher_source_counts() {
    let games = ActivityGamesReadModel {
        schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
        request: sample_surface_request(),
        state: ActivityReadModelState::Ready,
        generated_at: "2026-05-27T06:21:00Z".to_string(),
        summary: "Games ready".to_string(),
        rows: vec![super::ActivityGamesReadModelRow {
            row_id: "game-session-1".to_string(),
            display_name: "Ocentra Fixture Game".to_string(),
            device_id: "child-device-1".to_string(),
            state: ActivityReadModelState::Ready,
            product_kind: "nativeGame".to_string(),
            classification_state: "knownGame".to_string(),
            inventory_state: "detectable".to_string(),
            runtime_state: "running".to_string(),
            foreground_state: "notClaimed".to_string(),
            capability_status: "available".to_string(),
            last_observed_at: Some("2026-05-27T06:19:00Z".to_string()),
            total_ms: 120_000,
            session_count: 2,
            launcher_row_count: 1,
            running_row_count: 1,
            foreground_row_count: 0,
            daily_rollup_count: 1,
            evidence_claim_row_count: 1,
            identity_row_count: 1,
            approval_authority_row_count: 1,
            approval_action_result_row_count: 1,
            platform_authority_matrix_count: 1,
            platform_authority_row_count: 1,
            ai_classifier_result_row_count: 1,
            source_status_rows: vec![source_status::ActivityAppGameSourceStatusRow {
                source_kind: "launcherManifest".to_string(),
                state: ActivityReadModelState::Ready,
                row_count: 1,
                last_observed_at: Some("2026-05-27T06:19:00Z".to_string()),
                capability_status: "available".to_string(),
                evidence: vec![sample_evidence()],
            }],
            evidence: vec![sample_evidence()],
        }],
    };

    let games_json = serde_json::to_value(games).expect_value("games serializes");
    assert_eq!(games_json["rows"][0]["classificationState"], "knownGame");
    assert_eq!(games_json["rows"][0]["launcherRowCount"], 1);
    assert_eq!(games_json["rows"][0]["platformAuthorityRowCount"], 1);
    assert_eq!(games_json["rows"][0]["aiClassifierResultRowCount"], 1);
    assert_eq!(
        games_json["rows"][0]["sourceStatusRows"][0]["sourceKind"],
        "launcherManifest"
    );
}

#[test]
fn activity_network_read_model_serializes_unavailable_state() {
    let network = ActivityNetworkReadModel {
        schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
        request: sample_surface_request(),
        state: ActivityReadModelState::Unavailable,
        generated_at: "2026-05-27T06:21:00Z".to_string(),
        summary: "Network store unavailable".to_string(),
        rows: vec![],
    };

    let network_json = serde_json::to_value(network).expect_value("network serializes");
    assert_eq!(network_json["state"], "unavailable");
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

        let request_json = serde_json::to_value(request).expect_value("request serializes");
        assert_eq!(request_json["frequency"], expected);
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
            custody_label: ActivityReportCustodyLabel::ParentDeviceLocalReportJson,
            source_label: ActivityReportSourceLabel::SavedReportJson,
            raw_child_evidence_included: false,
        }),
        source_states: vec![
            ActivityReportSourceState {
                device_id: "child-device-1".to_string(),
                reachability_state: ActivityReportSourceReachabilityState::Reachable,
                state: ActivityReadModelState::Ready,
                reason: None,
                last_updated_at: Some("2026-05-27T06:19:00Z".to_string()),
                custody_label: ActivityReportCustodyLabel::ChildDeviceLocalSummary,
                source_label: ActivityReportSourceLabel::ActivityQueryStoreSummary,
                raw_child_evidence_included: false,
            },
            ActivityReportSourceState {
                device_id: "child-device-2".to_string(),
                reachability_state: ActivityReportSourceReachabilityState::Offline,
                state: ActivityReadModelState::Offline,
                reason: Some("Device is offline for this family report".to_string()),
                last_updated_at: None,
                custody_label: ActivityReportCustodyLabel::ChildDeviceLocalSummary,
                source_label: ActivityReportSourceLabel::FamilyFanoutSourceState,
                raw_child_evidence_included: false,
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
