use ocentra_parent_runtime_core::local_ai_runtime_panel::{
    local_ai_runtime_panel_typescript, project_local_ai_runtime_panel, LocalAiHouseholdJobInput,
    LocalAiMemoryGraphInput, LocalAiRemoteAssistantBoundaryInput, LocalAiRuntimePanelInput,
    LocalAiRuntimeStatusInput,
};

#[test]
fn local_ai_runtime_panel_projects_runtime_and_household_rows() {
    let input = LocalAiRuntimePanelInput {
        runtime_status: Some(LocalAiRuntimeStatusInput {
            event_id: Some("evt-local-ai-runtime"),
            sent_at: Some("2026-06-07T19:15:00Z"),
            runtime_reference: Some("runtime-child-device-1"),
            provider: Some("local-provider-1"),
            model: Some("screen-local-vlm-v1"),
            load_state: Some("loaded"),
            capability: Some("ocr,vision"),
            resource_class: Some("gpu"),
            degraded_state: Some("ready"),
            privacy_mode: Some("local-only"),
            execution_state: Some("ready"),
            reason: None,
        }),
        household_job: Some(LocalAiHouseholdJobInput {
            event_id: Some("evt-lan-ai-job"),
            sent_at: Some("2026-06-07T19:15:02Z"),
            request_id: Some("lan-ai-job-1"),
            status: Some("claimed"),
            state: Some("worker-running"),
            provider: Some("household-desktop-provider"),
            provider_source: Some("trusted-household-desktop"),
            capability: Some("screen-hard-visual-analysis"),
            resource_class: Some("gpu"),
            load_state: Some("ready"),
            privacy_mode: Some("local-lan-redacted"),
            custody: Some("local-lan-redacted"),
            policy_readiness: Some("authorized-result"),
            adapter_boundary: Some("claim-lease-child-owned-job"),
            lease_id: Some("lease-screen-ai-1"),
            last_checked: Some("2026-06-07T19:15:01Z"),
            last_observed: Some("2026-06-07T19:20:01Z"),
            decision_source: Some("child-agent-local-policy-authority"),
            execution_state: Some("running"),
            reason: None,
        }),
        ..LocalAiRuntimePanelInput::default()
    };

    let panel = project_local_ai_runtime_panel(&input);

    assert_eq!(panel.summary_status, "reported");
    assert_eq!(panel.summary_read_model_rows, "2");
    assert_eq!(
        panel.summary_product_claim,
        "no-model-quality-or-enforcement-claim"
    );
    assert_eq!(panel.cards.len(), 2);
    assert_eq!(panel.cards[0].kind, "runtime-status");
    assert_eq!(panel.cards[1].kind, "household-job");
    assert!(panel.cards[0]
        .details
        .iter()
        .any(|detail| detail.field_key == "model" && detail.value == "screen-local-vlm-v1"));
    assert!(panel.cards[1].details.iter().any(|detail| {
        detail.field_key == "providerSource" && detail.value == "trusted-household-desktop"
    }));
    assert!(panel.cards[1]
        .details
        .iter()
        .any(|detail| detail.field_key == "productClaim"
            && detail.value == "worker-only-child-agent-authority"));
}

#[test]
fn local_ai_runtime_panel_projects_memory_graph_and_remote_boundary() {
    let input = LocalAiRuntimePanelInput {
        memory_graph: Some(LocalAiMemoryGraphInput {
            custody: Some("child-device-query-store"),
            capability_status: Some("ready"),
            generated_at: Some("2026-06-07T19:16:00Z"),
            returned_node_count: 1,
            returned_edge_count: 1,
            omitted_edge_count: 0,
            degraded_reasons: Vec::new(),
            evidence_reference_ids: vec![
                "evidence-screen-summary-1",
                "evidence-screen-summary-1",
            ],
        }),
        remote_assistant_boundary: Some(LocalAiRemoteAssistantBoundaryInput {
            event_id: Some("evt-parent-assistant-boundary"),
            sent_at: Some("2026-06-07T19:17:00Z"),
            request_id: Some("remote-assistant-request-1"),
            state: Some("ready-answer"),
            provider: Some("remote-api-report-only"),
            adapter_boundary: Some("parent-authorized-report-bundle"),
            policy_readiness: Some("parent-authorized"),
            custody: Some("parent-owned-local-storage"),
            deleted_evidence: Some("raw-model-output-not-retained"),
            privacy_mode: Some("report-summary-only"),
            evidence_references: Some("evidence-screen-summary-1"),
            row_count: Some("1"),
        }),
        ..LocalAiRuntimePanelInput::default()
    };

    let panel = project_local_ai_runtime_panel(&input);

    assert_eq!(panel.cards.len(), 2);
    assert_eq!(panel.cards[0].kind, "memory-graph");
    assert_eq!(panel.cards[1].kind, "remote-assistant-boundary");
    assert!(panel.cards[0]
        .details
        .iter()
        .any(|detail| detail.field_key == "evidenceReferences"
            && detail.value == "evidence-screen-summary-1"));
    assert!(panel.cards[1]
        .details
        .iter()
        .any(|detail| detail.field_key == "adapterBoundary"
            && detail.value == "parent-authorized-report-bundle"));
    assert!(panel.cards[1]
        .details
        .iter()
        .any(|detail| detail.field_key == "productClaim"
            && detail.value == "remote-assistant-report-only-local-policy-authority"));
}

#[test]
fn local_ai_runtime_panel_keeps_missing_inputs_visible_without_success() {
    let panel = project_local_ai_runtime_panel(&LocalAiRuntimePanelInput::default());

    assert_eq!(panel.cards, Vec::new());
    assert_eq!(panel.summary_status, "not-reported");
    assert_eq!(panel.summary_read_model_rows, "0");
}

#[test]
fn local_ai_runtime_panel_generated_typescript_stays_checked_in() {
    let checked_in = include_str!(
        "../../../../packages/portal-domain/src/generated/local-ai-runtime-panel.ts"
    );

    assert_eq!(checked_in, local_ai_runtime_panel_typescript());
}
