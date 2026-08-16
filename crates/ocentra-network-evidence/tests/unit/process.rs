use ocentra_eventing::expect_value::ExpectValue;
use ocentra_network_evidence::dns::types::*;
use ocentra_network_evidence::process::*;

#[test]
fn process_correlation_attributes_flow_to_pid_snapshot() {
    let correlation = correlate_process_app_activity(NetworkProcessAppCorrelationInput {
        flow: flow_with_pid(4242),
        process_snapshots: vec![snapshot()],
        app_inventory: Vec::new(),
    })
    .expect_value("pid snapshot should attribute process");

    assert_eq!(
        correlation.state,
        NetworkProcessCorrelationState::ProcessAttributed
    );
    assert_eq!(
        correlation.basis,
        NetworkProcessCorrelationBasis::PidSnapshot
    );
    assert_eq!(
        correlation.uncertainty,
        NetworkProcessCorrelationUncertainty::ConfirmedByReplay
    );
    assert_eq!(correlation.process_name, Some("GameClient.exe".to_owned()));
    assert_eq!(
        correlation.evidence_refs,
        vec!["flow-1", "process-snapshot-1"]
    );
    assert_no_content_claims(&correlation);
}

#[test]
fn process_correlation_links_snapshot_to_app_inventory() {
    let correlation = correlate_process_app_activity(NetworkProcessAppCorrelationInput {
        flow: flow_with_pid(4242),
        process_snapshots: vec![snapshot()],
        app_inventory: vec![app_inventory_entry()],
    })
    .expect_value("process snapshot should link to app inventory");

    assert_eq!(
        correlation.state,
        NetworkProcessCorrelationState::ProcessAndAppAttributed
    );
    assert_eq!(
        correlation.basis,
        NetworkProcessCorrelationBasis::ProcessPathAppInventory
    );
    assert_eq!(
        correlation.uncertainty,
        NetworkProcessCorrelationUncertainty::AppInventoryMatched
    );
    assert_eq!(correlation.app_id, Some("game.client".to_owned()));
    assert_eq!(correlation.app_display_name, Some("Game Client".to_owned()));
    assert_eq!(
        correlation.evidence_refs,
        vec!["flow-1", "process-snapshot-1", "app-inventory-1"]
    );
    assert_no_content_claims(&correlation);
}

#[test]
fn process_correlation_keeps_process_name_only_candidate() {
    let correlation = correlate_process_app_activity(NetworkProcessAppCorrelationInput {
        flow: NetworkFlowProcessObservation {
            flow_ref: "flow-name-only".to_owned(),
            observed_pid: None,
            observed_process_name: Some("GameClient.exe".to_owned()),
            observed_executable_path: None,
            adapter_available: true,
        },
        process_snapshots: Vec::new(),
        app_inventory: vec![app_inventory_entry()],
    })
    .expect_value("process name should remain a candidate");

    assert_eq!(
        correlation.state,
        NetworkProcessCorrelationState::ProcessCandidate
    );
    assert_eq!(
        correlation.basis,
        NetworkProcessCorrelationBasis::ProcessNameCandidate
    );
    assert_eq!(
        correlation.uncertainty,
        NetworkProcessCorrelationUncertainty::CandidateNeedsConfirmation
    );
    assert_eq!(correlation.evidence_grade, NetworkEvidenceGrade::D);
    assert_eq!(correlation.app_id, Some("game.client".to_owned()));
    assert_no_content_claims(&correlation);
}

#[test]
fn process_correlation_keeps_missing_process_unknown() {
    let correlation = correlate_process_app_activity(NetworkProcessAppCorrelationInput {
        flow: NetworkFlowProcessObservation {
            flow_ref: "flow-unknown".to_owned(),
            observed_pid: None,
            observed_process_name: None,
            observed_executable_path: None,
            adapter_available: true,
        },
        process_snapshots: Vec::new(),
        app_inventory: Vec::new(),
    })
    .expect_value("missing process evidence should be unknown");

    assert_eq!(
        correlation.state,
        NetworkProcessCorrelationState::ProcessUnknown
    );
    assert_eq!(
        correlation.basis,
        NetworkProcessCorrelationBasis::MissingProcessEvidence
    );
    assert_eq!(
        correlation.uncertainty,
        NetworkProcessCorrelationUncertainty::Unknown
    );
    assert_no_content_claims(&correlation);
}

#[test]
fn process_correlation_marks_adapter_unavailable() {
    let correlation = correlate_process_app_activity(NetworkProcessAppCorrelationInput {
        flow: NetworkFlowProcessObservation {
            flow_ref: "flow-adapter-unavailable".to_owned(),
            observed_pid: Some(4242),
            observed_process_name: Some("GameClient.exe".to_owned()),
            observed_executable_path: None,
            adapter_available: false,
        },
        process_snapshots: vec![snapshot()],
        app_inventory: vec![app_inventory_entry()],
    })
    .expect_value("unavailable adapter should not guess attribution");

    assert_eq!(
        correlation.state,
        NetworkProcessCorrelationState::AdapterUnavailable
    );
    assert_eq!(
        correlation.basis,
        NetworkProcessCorrelationBasis::AdapterUnavailable
    );
    assert_eq!(
        correlation.uncertainty,
        NetworkProcessCorrelationUncertainty::AdapterUnavailable
    );
    assert_eq!(correlation.process_name, None);
    assert_eq!(correlation.app_id, None);
    assert_no_content_claims(&correlation);
}

#[test]
fn process_correlation_rejects_empty_flow_ref() {
    let result = correlate_process_app_activity(NetworkProcessAppCorrelationInput {
        flow: NetworkFlowProcessObservation {
            flow_ref: " ".to_owned(),
            observed_pid: Some(4242),
            observed_process_name: None,
            observed_executable_path: None,
            adapter_available: true,
        },
        process_snapshots: vec![snapshot()],
        app_inventory: Vec::new(),
    });

    assert_eq!(result, Err(NetworkProcessCorrelationError::EmptyFlowRef));
}

fn flow_with_pid(pid: u32) -> NetworkFlowProcessObservation {
    NetworkFlowProcessObservation {
        flow_ref: "flow-1".to_owned(),
        observed_pid: Some(pid),
        observed_process_name: None,
        observed_executable_path: None,
        adapter_available: true,
    }
}

fn snapshot() -> NetworkProcessSnapshot {
    NetworkProcessSnapshot {
        pid: 4242,
        process_name: "GameClient.exe".to_owned(),
        executable_path: Some("C:\\Program Files\\Game\\GameClient.exe".to_owned()),
        source_ref: "process-snapshot-1".to_owned(),
    }
}

fn app_inventory_entry() -> NetworkAppInventoryEntry {
    NetworkAppInventoryEntry {
        app_id: "game.client".to_owned(),
        display_name: "Game Client".to_owned(),
        executable_path: Some("c:\\program files\\game\\gameclient.exe".to_owned()),
        process_name: Some("gameclient.exe".to_owned()),
        source_ref: "app-inventory-1".to_owned(),
    }
}

fn assert_no_content_claims(correlation: &NetworkProcessAppCorrelation) {
    assert!(!correlation.exact_url_available);
    assert!(!correlation.decrypted_payload_available);
    assert!(!correlation.browser_url_claimed);
}
