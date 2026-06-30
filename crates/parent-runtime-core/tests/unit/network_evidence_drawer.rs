use ocentra_parent_runtime_core::network_evidence_drawer::{
    network_evidence_drawer_typescript, project_network_evidence_drawer_summary,
    NetworkEvidenceDrawerEndpoint, NetworkEvidenceDrawerEvidenceRef,
    NetworkEvidenceDrawerEvidenceSummary, NetworkEvidenceDrawerFlowCounters,
    NetworkEvidenceDrawerObservation, NetworkEvidenceDrawerReadModel,
    NetworkEvidenceDrawerSummaryContext,
};

#[test]
fn network_evidence_drawer_projects_truthful_refs_and_unsupported_gaps() {
    let read_model = sample_read_model();
    let context = NetworkEvidenceDrawerSummaryContext {
        network_evidence_summary: Some(NetworkEvidenceDrawerEvidenceSummary {
            analyzer_alert_ref: Some("event.network.analyzer.alert.1"),
            detection_result_ref: Some("event.network.detection.result.1"),
            ai_audit_ref: Some("event.ai.analysis.completed.1"),
            risk_budget_ref: Some("event.network.risk-budget.1"),
            policy_decision_ref: Some("event.policy.decision.completed.1"),
            network_evidence_grade: Some("A"),
            intervention_result_ref: Some("event.enforcement.result.observed.1"),
        }),
    };

    let summary = project_network_evidence_drawer_summary(Some(&read_model), Some(&context));

    assert_eq!(summary.evidence_id, "activity-network-flow-observed-1");
    assert_eq!(summary.source_adapter, "windows-network-snapshot");
    assert_eq!(summary.source_quality, "available");
    assert_eq!(summary.platform_state, "child-device-query-store | available");
    assert_eq!(summary.read_model_rows, "1 | 1 | 0 | 1");
    assert_eq!(summary.local_endpoint, "127.0.0.1 | 4242");
    assert_eq!(summary.remote_endpoint, "203.0.113.10 | 443");
    assert_eq!(summary.domain_evidence_ref, "media.example.test | domain-observed");
    assert_eq!(summary.process_ref, "browser.exe | 8021 | process-attributed");
    assert_eq!(
        summary.evidence_references,
        "network-evidence-1 | network-journal-evidence-1"
    );
    assert_eq!(summary.exact_url_claim, "Not reported");
    assert_eq!(summary.analyzer_alert_ref, "event.network.analyzer.alert.1");
    assert_eq!(
        summary.detection_result_ref,
        "event.network.detection.result.1"
    );
    assert_eq!(summary.ai_audit_ref, "event.ai.analysis.completed.1");
    assert_eq!(
        summary.policy_decision_ref,
        "event.policy.decision.completed.1"
    );
    assert_eq!(
        summary.intervention_result_ref,
        "event.enforcement.result.observed.1"
    );
    assert_eq!(summary.risk_budget_ref, "event.network.risk-budget.1");
    assert_eq!(summary.evidence_grade, "A");
    assert_eq!(summary.retention_state, "0 | 1");
    assert_eq!(summary.deleted_evidence_references, "Not reported");
    assert_eq!(
        summary.degraded_state,
        "available | domain-observed | process-attributed"
    );
}

#[test]
fn network_evidence_drawer_keeps_empty_read_models_visible() {
    let read_model = NetworkEvidenceDrawerReadModel {
        rows: Vec::new(),
        capability_status: Some("no-network-observations"),
        custody: Some("child-device-query-store"),
        returned: 0,
        active_rows: 0,
        tombstone_rows: 0,
        exportable_rows: 0,
        latest_tombstone_event_id: None,
        latest_tombstone_observed_at: None,
        deleted_evidence_reference_ids: Vec::new(),
    };

    let summary = project_network_evidence_drawer_summary(Some(&read_model), None);

    assert_eq!(summary.evidence_references, "Not reported");
    assert_eq!(summary.exact_url_claim, "Not reported");
    assert_eq!(
        summary.platform_state,
        "child-device-query-store | no-network-observations"
    );
    assert_eq!(summary.read_model_rows, "0 | 0 | 0 | 0");
    assert_eq!(summary.degraded_state, "no-network-observations");
}

#[test]
fn network_evidence_drawer_projects_deleted_and_degraded_state() {
    let read_model = NetworkEvidenceDrawerReadModel {
        rows: Vec::new(),
        capability_status: Some("adapter-error"),
        custody: Some("child-device-query-store"),
        returned: 0,
        active_rows: 0,
        tombstone_rows: 1,
        exportable_rows: 0,
        latest_tombstone_event_id: Some("activity-network-flow-deleted"),
        latest_tombstone_observed_at: Some("2026-05-21T02:05:00Z"),
        deleted_evidence_reference_ids: vec!["network-evidence-1"],
    };

    let summary = project_network_evidence_drawer_summary(Some(&read_model), None);

    assert_eq!(summary.source_quality, "adapter-error");
    assert_eq!(summary.platform_state, "child-device-query-store | adapter-error");
    assert_eq!(summary.read_model_rows, "0 | 0 | 1 | 0");
    assert_eq!(
        summary.retention_state,
        "activity-network-flow-deleted | 2026-05-21T02:05:00Z | network-evidence-1"
    );
    assert_eq!(summary.deleted_evidence_references, "network-evidence-1");
    assert_eq!(summary.degraded_state, "adapter-error");
    assert_eq!(summary.policy_decision_ref, "Not reported");
    assert_eq!(summary.intervention_result_ref, "Not reported");
}

#[test]
fn network_evidence_drawer_generated_typescript_stays_checked_in() {
    let checked_in = include_str!(
        "../../../../packages/portal-domain/src/generated/network-evidence-drawer.ts"
    );

    assert_eq!(checked_in, network_evidence_drawer_typescript());
}

fn sample_read_model<'a>() -> NetworkEvidenceDrawerReadModel<'a> {
    NetworkEvidenceDrawerReadModel {
        rows: vec![NetworkEvidenceDrawerObservation {
            event_id: "activity-network-flow-observed-1",
            observed_at: Some("2026-05-21T02:00:00Z"),
            adapter_id: Some("windows-network-snapshot"),
            local_endpoint: NetworkEvidenceDrawerEndpoint {
                ip: Some("127.0.0.1"),
                port: Some(4242),
            },
            destination_endpoint: NetworkEvidenceDrawerEndpoint {
                ip: Some("203.0.113.10"),
                port: Some(443),
            },
            protocol: Some("https"),
            tcp_state: Some("established"),
            process_name: Some("browser.exe"),
            process_id: Some(8021),
            process_attribution_status: Some("process-attributed"),
            destination_domain: Some("media.example.test"),
            domain_attribution_status: Some("domain-observed"),
            capability_status: Some("available"),
            counters: NetworkEvidenceDrawerFlowCounters {
                connection_count: Some(1),
                bytes_sent: None,
                bytes_received: None,
                first_seen_at: Some("2026-05-21T02:00:00Z"),
                last_seen_at: Some("2026-05-21T02:00:00Z"),
            },
            evidence: vec![
                NetworkEvidenceDrawerEvidenceRef {
                    evidence_id: "network-evidence-1",
                },
                NetworkEvidenceDrawerEvidenceRef {
                    evidence_id: "network-journal-evidence-1",
                },
            ],
        }],
        capability_status: Some("available"),
        custody: Some("child-device-query-store"),
        returned: 1,
        active_rows: 1,
        tombstone_rows: 0,
        exportable_rows: 1,
        latest_tombstone_event_id: None,
        latest_tombstone_observed_at: None,
        deleted_evidence_reference_ids: Vec::new(),
    }
}
