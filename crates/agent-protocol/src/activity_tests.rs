use super::{
    constants, ActivityCaptureCapabilityStatus, ActivityDomainAttributionStatus, ActivityEvent,
    ActivityEventKind, ActivityEvidenceKind, ActivityEvidenceRef, ActivityNetworkProtocol,
    ActivityNetworkTcpState, ActivityObservationMode, ActivityObserver,
    ActivityProcessAttributionStatus, ActivitySource, ActivitySubject, ActivitySubjectKind,
    LogFieldValue, LogFields, ACTIVITY_SCHEMA_VERSION,
};

#[test]
fn activity_event_serializes_to_typescript_contract_shape() {
    let mut fields = LogFields::new();
    fields.insert("pid".to_string(), LogFieldValue::Number(4242.0));
    fields.insert("foreground".to_string(), LogFieldValue::Boolean(true));

    let event = ActivityEvent {
        schema_version: ACTIVITY_SCHEMA_VERSION,
        event_id: "activity-event-1".to_string(),
        observed_at: "2026-05-20T00:00:00Z".to_string(),
        source: ActivitySource {
            device_id: "child-device-1".to_string(),
            platform: "windows".to_string(),
            observer: ActivityObserver::WindowsProcess,
            source_id: "windows-process-adapter".to_string(),
        },
        kind: ActivityEventKind::ProcessObserved,
        subject: ActivitySubject {
            kind: ActivitySubjectKind::Process,
            subject_id: "process-4242".to_string(),
            display_name: Some("chrome.exe".to_string()),
        },
        fields,
        evidence: vec![ActivityEvidenceRef {
            evidence_id: "journal-entry-1".to_string(),
            kind: ActivityEvidenceKind::JournalEntry,
            digest: Some("sha256:process-event-digest".to_string()),
            uri: None,
        }],
    };

    let serialized = serde_json::to_value(event).expect("activity event serializes");

    assert_eq!(serialized["schemaVersion"], 1);
    assert_eq!(serialized["source"]["observer"], "windows-process");
    assert_eq!(serialized["kind"], "activity.process.observed");
    assert_eq!(serialized["subject"]["displayName"], "chrome.exe");
    assert_eq!(serialized["fields"]["foreground"], true);
    assert_eq!(serialized["evidence"][0]["kind"], "journal-entry");
    assert!(serialized["evidence"][0]["uri"].is_null());
}

#[test]
fn capture_status_values_serialize_to_typescript_contract_shape() {
    let mode = serde_json::to_value(ActivityObservationMode::ActiveWindow)
        .expect(constants::error::AGENT_EVENT_SERIALIZES);
    let status = serde_json::to_value(ActivityCaptureCapabilityStatus::NoActiveWindow)
        .expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        mode,
        constants::activity_capture::OBSERVATION_MODE_ACTIVE_WINDOW
    );
    assert_eq!(
        status,
        constants::activity_capture::CAPABILITY_STATUS_NO_ACTIVE_WINDOW
    );
    assert_eq!(
        ActivityCaptureCapabilityStatus::AdapterError.as_protocol_str(),
        constants::activity_capture::CAPABILITY_STATUS_ADAPTER_ERROR
    );
    assert_eq!(
        ActivityCaptureCapabilityStatus::NoNetworkObservations.as_protocol_str(),
        constants::activity_capture::CAPABILITY_STATUS_NO_NETWORK_OBSERVATIONS
    );
    assert_eq!(
        ActivityObservationMode::NetworkSnapshot.as_protocol_str(),
        constants::activity_capture::OBSERVATION_MODE_NETWORK_SNAPSHOT
    );
}

#[test]
fn network_domain_values_serialize_to_typescript_contract_shape() {
    let protocol = serde_json::to_value(ActivityNetworkProtocol::Tcp)
        .expect(constants::error::AGENT_EVENT_SERIALIZES);
    let tcp_state = serde_json::to_value(ActivityNetworkTcpState::Established)
        .expect(constants::error::AGENT_EVENT_SERIALIZES);
    let domain_status = serde_json::to_value(ActivityDomainAttributionStatus::IpOnly)
        .expect(constants::error::AGENT_EVENT_SERIALIZES);
    let process_status = serde_json::to_value(ActivityProcessAttributionStatus::ProcessAttributed)
        .expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(protocol, constants::activity_capture::NETWORK_PROTOCOL_TCP);
    assert_eq!(
        tcp_state,
        constants::activity_capture::TCP_STATE_ESTABLISHED
    );
    assert_eq!(
        domain_status,
        constants::activity_capture::DOMAIN_ATTRIBUTION_STATUS_IP_ONLY
    );
    assert_eq!(
        process_status,
        constants::activity_capture::PROCESS_ATTRIBUTION_STATUS_ATTRIBUTED
    );
}
