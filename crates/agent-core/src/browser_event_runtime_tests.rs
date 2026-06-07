use crate::{
    publish_browser_runtime_chain_for_input, BrowserRuntimeEventPayload, BrowserRuntimeInput,
    BrowserRuntimePhase, BrowserRuntimeReport,
};

#[tokio::test]
async fn browser_runtime_chain_publishes_ordered_managed_decision_phases() {
    let report =
        publish_browser_runtime_chain_for_input(BrowserRuntimeInput::managed_decision_fixture())
            .await
            .unwrap();

    assert_eq!(report.dead_letters.len(), 0);
    assert_eq!(
        report.publish_reports.len(),
        BrowserRuntimePhase::ordered_chain().len()
    );
    assert_eq!(
        decoded_phases(&report),
        BrowserRuntimePhase::ordered_chain().to_vec()
    );
    assert!(report.intervention_command_published());
}

#[tokio::test]
async fn browser_runtime_chain_keeps_manual_required_rows_non_executing() {
    let report =
        publish_browser_runtime_chain_for_input(BrowserRuntimeInput::manual_required_fixture())
            .await
            .unwrap();

    let phases = decoded_phases(&report);
    assert!(phases.contains(&BrowserRuntimePhase::EvidenceObserved));
    assert!(phases.contains(&BrowserRuntimePhase::EvidenceJournaled));
    assert!(phases.contains(&BrowserRuntimePhase::AuditEntryCommitted));
    assert!(phases.contains(&BrowserRuntimePhase::ReadModelProjected));
    assert!(!phases.contains(&BrowserRuntimePhase::InterventionCommandIssued));
    assert!(!phases.contains(&BrowserRuntimePhase::InterventionResultObserved));
    assert!(!report.intervention_command_published());

    let policy_event = decoded_payloads(&report)
        .into_iter()
        .find(|payload| payload.phase == BrowserRuntimePhase::PolicyDecisionCompleted)
        .unwrap();
    assert!(!policy_event.ai_authority);
    assert!(!policy_event.policy_authority);
    assert!(!policy_event.intervention_command_allowed);
}

fn decoded_phases(report: &BrowserRuntimeReport) -> Vec<BrowserRuntimePhase> {
    decoded_payloads(report)
        .into_iter()
        .map(|payload| payload.phase)
        .collect()
}

fn decoded_payloads(report: &BrowserRuntimeReport) -> Vec<BrowserRuntimeEventPayload> {
    report
        .stored_events
        .iter()
        .map(|event| {
            event
                .decode::<BrowserRuntimeEventPayload>()
                .unwrap()
                .payload
        })
        .collect()
}
