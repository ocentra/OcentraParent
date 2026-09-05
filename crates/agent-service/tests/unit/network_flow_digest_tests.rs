use ocentra_parent_agent_protocol::activity::{ActivityEvidenceKind, ActivityEvidenceRef};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::network_flow::{
    ActivityNetworkEndpoint, ActivityNetworkFlowCounters, ActivityNetworkFlowObservation,
    ActivityNetworkFlowReadModel, NETWORK_FLOW_CUSTODY_CHILD_DEVICE_QUERY_STORE,
};
use ocentra_parent_agent_protocol::NETWORK_FLOW_SCHEMA_VERSION;
use ocentra_parent_agent_service::test_support::network_flow_digest_for_test;
use std::primitive::str as TestStr;
use std::string::String as TestString;

#[test]
fn network_flow_digest_rolls_up_processes_destinations_and_evidence_refs() {
    let read_model = read_model(vec![
        observation(NetworkObservationInput {
            event_id: constants::activity_store::TEST_NETWORK_EVENT_ID,
            destination_domain: Some(constants::activity_store::TEST_NETWORK_DOMAIN),
            destination_port: Some(constants::activity_store::TEST_NETWORK_DESTINATION_PORT),
            process_id: Some(4242),
            process_name: Some(constants::activity_store::TEST_PROCESS_SUBJECT_NAME),
            connection_count: 2,
            bytes_sent: Some(120),
            bytes_received: None,
            evidence_id: test_network_evidence_id(constants::activity_store::TEST_NETWORK_EVENT_ID),
            tcp_state: Some(constants::activity_capture::TCP_STATE_ESTABLISHED),
        }),
        observation(NetworkObservationInput {
            event_id: constants::test_network::SUBJECT_ID,
            destination_domain: Some(constants::activity_store::TEST_NETWORK_DOMAIN),
            destination_port: Some(constants::activity_store::TEST_NETWORK_DESTINATION_PORT),
            process_id: Some(4242),
            process_name: Some(constants::activity_store::TEST_PROCESS_SUBJECT_NAME),
            connection_count: 1,
            bytes_sent: Some(20),
            bytes_received: Some(40),
            evidence_id: test_network_evidence_id(constants::test_network::SUBJECT_ID),
            tcp_state: Some(constants::activity_capture::TCP_STATE_ESTABLISHED),
        }),
    ]);

    let digest = network_flow_digest_for_test(&read_model);

    assert_eq!(digest.generated_at, read_model.generated_at);
    assert_eq!(
        digest.custody,
        NETWORK_FLOW_CUSTODY_CHILD_DEVICE_QUERY_STORE
    );
    assert_eq!(digest.evidence.len(), 2);
    assert_eq!(
        digest.top_processes[0].label,
        constants::activity_store::TEST_PROCESS_SUBJECT_NAME
    );
    assert_eq!(digest.top_processes[0].connection_count, 3);
    assert_eq!(digest.top_processes[0].bytes_sent, Some(140));
    assert_eq!(digest.top_processes[0].bytes_received, Some(40));
    assert_eq!(
        digest.top_destinations[0].label,
        constants::activity_store::TEST_NETWORK_DOMAIN
    );
    assert_eq!(digest.top_destinations[0].connection_count, 3);
}

#[test]
fn network_flow_digest_reports_direct_indicators_without_new_destination_guessing() {
    let read_model = read_model(vec![observation(NetworkObservationInput {
        event_id: constants::activity_store::TEST_NETWORK_EVENT_ID,
        destination_domain: None,
        destination_port: Some(8080),
        process_id: None,
        process_name: None,
        connection_count: 1,
        bytes_sent: None,
        bytes_received: None,
        evidence_id: test_network_evidence_id(constants::activity_store::TEST_NETWORK_EVENT_ID),
        tcp_state: Some(constants::activity_capture::TCP_STATE_CLOSE_WAIT),
    })]);

    let digest = network_flow_digest_for_test(&read_model);
    let kinds: Vec<&TestStr> = digest
        .unusual_indicators
        .iter()
        .map(|indicator| indicator.kind.as_str())
        .collect();

    assert_eq!(
        kinds,
        vec![
            constants::network_flow::INDICATOR_UNUSUAL_UNKNOWN_PROCESS,
            constants::network_flow::INDICATOR_ENCRYPTED_CONTENT_UNAVAILABLE,
            constants::network_flow::INDICATOR_REPEATED_FAILURE,
            constants::network_flow::INDICATOR_VPN_PROXY_TUNNEL
        ]
    );
}

fn read_model(rows: Vec<ActivityNetworkFlowObservation>) -> ActivityNetworkFlowReadModel {
    ActivityNetworkFlowReadModel {
        schema_version: NETWORK_FLOW_SCHEMA_VERSION,
        generated_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        custody: NETWORK_FLOW_CUSTODY_CHILD_DEVICE_QUERY_STORE.to_string(),
        limit: constants::activity_store::DEFAULT_RECENT_LIMIT,
        returned: rows.len() as u64,
        active_rows: rows.len() as u64,
        tombstone_rows: 0,
        exportable_rows: rows.len() as u64,
        capability_status: constants::activity_capture::CAPABILITY_STATUS_AVAILABLE.to_string(),
        latest_event_id: rows.first().map(|row| row.event_id.clone()),
        latest_observed_at: rows.first().map(|row| row.observed_at.clone()),
        latest_tombstone_event_id: None,
        latest_tombstone_observed_at: None,
        deleted_evidence_reference_ids: Vec::new(),
        rows,
    }
}

fn observation(input: NetworkObservationInput) -> ActivityNetworkFlowObservation {
    ActivityNetworkFlowObservation {
        schema_version: NETWORK_FLOW_SCHEMA_VERSION,
        event_id: input.event_id.to_string(),
        observed_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        observer: constants::activity_observer::WINDOWS_NETWORK.to_string(),
        capability_status: constants::activity_capture::CAPABILITY_STATUS_AVAILABLE.to_string(),
        adapter_id: constants::activity_capture::NETWORK_ADAPTER_ID.to_string(),
        protocol: Some(constants::activity_capture::NETWORK_PROTOCOL_TCP.to_string()),
        tcp_state: input.tcp_state.map(str::to_string),
        local_endpoint: ActivityNetworkEndpoint {
            ip: Some(constants::test_network::LOOPBACK_IP.to_string()),
            port: Some(constants::activity_store::TEST_NETWORK_LOCAL_PORT),
        },
        destination_endpoint: ActivityNetworkEndpoint {
            ip: Some(constants::activity_store::TEST_NETWORK_DESTINATION_IP.to_string()),
            port: input.destination_port,
        },
        destination_domain: input.destination_domain.map(str::to_string),
        domain_attribution_status: if input.destination_domain.is_some() {
            constants::activity_capture::DOMAIN_ATTRIBUTION_STATUS_DOMAIN_OBSERVED.to_string()
        } else {
            constants::activity_capture::DOMAIN_ATTRIBUTION_STATUS_IP_ONLY.to_string()
        },
        process_attribution_status: if input.process_id.is_some() {
            constants::activity_capture::PROCESS_ATTRIBUTION_STATUS_ATTRIBUTED.to_string()
        } else {
            constants::activity_capture::PROCESS_ATTRIBUTION_STATUS_UNKNOWN.to_string()
        },
        process_id: input.process_id,
        process_name: input.process_name.map(str::to_string),
        associated_pid_count: Some(1),
        counters: ActivityNetworkFlowCounters {
            connection_count: input.connection_count,
            bytes_sent: input.bytes_sent,
            bytes_received: input.bytes_received,
            first_seen_at: Some(constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string()),
            last_seen_at: Some(constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string()),
        },
        evidence: vec![ActivityEvidenceRef {
            evidence_id: input.evidence_id,
            kind: ActivityEvidenceKind::LocalDbRow,
            digest: None,
            uri: None,
        }],
    }
}

struct NetworkObservationInput {
    event_id: &'static TestStr,
    destination_domain: Option<&'static TestStr>,
    destination_port: Option<u16>,
    process_id: Option<u64>,
    process_name: Option<&'static TestStr>,
    connection_count: u64,
    bytes_sent: Option<u64>,
    bytes_received: Option<u64>,
    evidence_id: TestString,
    tcp_state: Option<&'static TestStr>,
}

fn test_network_evidence_id(suffix: &TestStr) -> TestString {
    let mut evidence_id = TestString::from(constants::activity_capture::NETWORK_EVIDENCE_ID_PREFIX);
    evidence_id.push_str(suffix);
    evidence_id
}
