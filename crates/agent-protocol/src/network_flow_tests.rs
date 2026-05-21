use crate::{
    constants, ActivityCaptureCapabilityStatus, ActivityDomainAttributionStatus,
    ActivityEvidenceKind, ActivityEvidenceRef, ActivityNetworkCustodyState,
    ActivityNetworkEndpoint, ActivityNetworkFlowCounters, ActivityNetworkFlowDigest,
    ActivityNetworkFlowIndicator, ActivityNetworkFlowIndicatorKind, ActivityNetworkFlowObservation,
    ActivityNetworkFlowReadModel, ActivityNetworkFlowRollup, ActivityNetworkProtocol,
    ActivityNetworkTcpState, ActivityObserver, ActivityProcessAttributionStatus,
    ACTIVITY_QUERY_SCHEMA_VERSION,
};

fn sample_observation() -> ActivityNetworkFlowObservation {
    ActivityNetworkFlowObservation {
        schema_version: ACTIVITY_QUERY_SCHEMA_VERSION,
        event_id: constants::test_network::NETWORK_EVENT_ID.to_string(),
        observed_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        observer: ActivityObserver::WindowsNetwork,
        capability_status: ActivityCaptureCapabilityStatus::Available,
        adapter_id: constants::activity_capture::NETWORK_ADAPTER_ID.to_string(),
        protocol: Some(ActivityNetworkProtocol::Tcp),
        tcp_state: Some(ActivityNetworkTcpState::Established),
        local_endpoint: ActivityNetworkEndpoint {
            ip: Some(constants::test_network::LOOPBACK_IP.to_string()),
            port: Some(4242),
        },
        destination_endpoint: ActivityNetworkEndpoint {
            ip: Some(constants::test_network::LOOPBACK_IP.to_string()),
            port: Some(443),
        },
        destination_domain: Some(constants::test_network::TEST_DOMAIN.to_string()),
        domain_attribution_status: ActivityDomainAttributionStatus::DomainObserved,
        process_attribution_status: ActivityProcessAttributionStatus::ProcessAttributed,
        process_id: Some(4242),
        process_name: Some(constants::activity_store::TEST_PROCESS_SUBJECT_NAME.to_string()),
        counters: ActivityNetworkFlowCounters {
            connection_count: 1.0,
            bytes_sent: None,
            bytes_received: None,
            first_seen_at: Some(constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string()),
            last_seen_at: Some(constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string()),
        },
        evidence: vec![ActivityEvidenceRef {
            evidence_id: constants::test_network::NETWORK_EVIDENCE_ID.to_string(),
            kind: ActivityEvidenceKind::JournalEntry,
            digest: Some(constants::test_network::NETWORK_EVIDENCE_DIGEST.to_string()),
            uri: None,
        }],
    }
}

fn sample_read_model(observation: &ActivityNetworkFlowObservation) -> ActivityNetworkFlowReadModel {
    ActivityNetworkFlowReadModel {
        schema_version: ACTIVITY_QUERY_SCHEMA_VERSION,
        generated_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        custody: ActivityNetworkCustodyState::ChildDeviceQueryStore,
        limit: 25,
        returned: 1,
        capability_status: ActivityCaptureCapabilityStatus::Available,
        rows: vec![observation.clone()],
    }
}

fn sample_digest(observation: &ActivityNetworkFlowObservation) -> ActivityNetworkFlowDigest {
    ActivityNetworkFlowDigest {
        schema_version: ACTIVITY_QUERY_SCHEMA_VERSION,
        generated_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        custody: ActivityNetworkCustodyState::ChildDeviceQueryStore,
        evidence: observation.evidence.clone(),
        top_processes: vec![ActivityNetworkFlowRollup {
            key: constants::test_network::NETWORK_EVENT_ID.to_string(),
            label: constants::activity_store::TEST_PROCESS_SUBJECT_NAME.to_string(),
            connection_count: 1.0,
            bytes_sent: None,
            bytes_received: None,
            evidence_ids: vec![constants::test_network::NETWORK_EVIDENCE_ID.to_string()],
        }],
        top_destinations: vec![ActivityNetworkFlowRollup {
            key: constants::test_network::TEST_DOMAIN.to_string(),
            label: "example.com:443".to_string(),
            connection_count: 1.0,
            bytes_sent: None,
            bytes_received: None,
            evidence_ids: vec![constants::test_network::NETWORK_EVIDENCE_ID.to_string()],
        }],
        unusual_indicators: vec![ActivityNetworkFlowIndicator {
            kind: ActivityNetworkFlowIndicatorKind::EncryptedContentUnavailable,
            label: "HTTPS payload was not decrypted or inspected.".to_string(),
            observed_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
            evidence_ids: vec![constants::test_network::NETWORK_EVIDENCE_ID.to_string()],
        }],
    }
}

#[test]
fn network_flow_observation_serializes_to_typescript_shape() {
    let observation = sample_observation();
    let serialized = serde_json::to_value(observation).expect("observation serializes");

    assert_eq!(serialized["observer"], "windows-network");
    assert_eq!(serialized["capabilityStatus"], "available");
    assert_eq!(
        serialized["destinationDomain"],
        constants::test_network::TEST_DOMAIN
    );
}

#[test]
fn network_flow_read_model_and_digest_serializes_to_typescript_shape() {
    let observation = sample_observation();
    let read_model = sample_read_model(&observation);
    let digest = sample_digest(&observation);

    let read_model_value = serde_json::to_value(read_model).expect("read model serializes");
    let digest_value = serde_json::to_value(digest).expect("digest serializes");

    assert_eq!(read_model_value["custody"], "child-device-query-store");
    assert_eq!(
        digest_value["unusualIndicators"][0]["kind"],
        "encrypted-content-unavailable"
    );
    assert_eq!(
        digest_value["topProcesses"][0]["label"],
        constants::activity_store::TEST_PROCESS_SUBJECT_NAME
    );
}
