use super::{
    constants, ActivityCaptureCapabilityStatus, ActivityDomainAttributionStatus, ActivityEvent,
    ActivityEventKind, ActivityEvidenceKind, ActivityEvidenceRef, ActivityNetworkProtocol,
    ActivityNetworkTcpState, ActivityObservationMode, ActivityObserver,
    ActivityProcessAttributionStatus, ActivitySource, ActivitySubject, ActivitySubjectKind,
    BrowserActiveProofSource, BrowserActiveTabState, BrowserCapabilityStatus, BrowserChannel,
    BrowserCustodyLabel, BrowserFamily, BrowserInterventionAction,
    BrowserInterventionCapabilityState, BrowserInterventionDecisionSource,
    BrowserInterventionMechanism, BrowserUnmanagedEnforcementState, LogFieldValue, LogFields,
    ACTIVITY_SCHEMA_VERSION, BROWSER_EVIDENCE_SCHEMA_VERSION,
};
use crate::browser::BrowserEvidenceRecentSummary;
use ocentra_eventing::expect_value::ExpectValue;

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

    let serialized = serde_json::to_value(event).expect_value("activity event serializes");

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
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES);
    let status = serde_json::to_value(ActivityCaptureCapabilityStatus::NoActiveWindow)
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES);

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
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES);
    let tcp_state = serde_json::to_value(ActivityNetworkTcpState::Established)
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES);
    let domain_status = serde_json::to_value(ActivityDomainAttributionStatus::IpOnly)
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES);
    let process_status = serde_json::to_value(ActivityProcessAttributionStatus::ProcessAttributed)
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES);

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

#[test]
fn browser_evidence_values_serialize_to_typescript_contract_shape() {
    let family = serde_json::to_value(BrowserFamily::Edge)
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES);
    let channel = serde_json::to_value(BrowserChannel::Stable)
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES);
    let active_state = serde_json::to_value(BrowserActiveTabState::Unknown)
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES);
    let active_proof_source = serde_json::to_value(BrowserActiveProofSource::TargetListOnly)
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES);
    let capability = serde_json::to_value(BrowserCapabilityStatus::TabListOnly)
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES);
    let custody = serde_json::to_value(BrowserCustodyLabel::ChildDeviceLocal)
        .expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(family, constants::browser::FAMILY_EDGE);
    assert_eq!(channel, constants::browser::CHANNEL_STABLE);
    assert_eq!(active_state, constants::browser::ACTIVE_STATE_UNKNOWN);
    assert_eq!(
        active_proof_source,
        constants::browser::ACTIVE_PROOF_SOURCE_TARGET_LIST_ONLY
    );
    assert_eq!(
        capability,
        constants::browser::CAPABILITY_STATUS_TAB_LIST_ONLY
    );
    assert_eq!(custody, constants::browser::CUSTODY_CHILD_DEVICE_LOCAL);
    assert_eq!(
        ActivityObserver::ManagedBrowserBridge.as_protocol_str(),
        constants::activity_observer::MANAGED_BROWSER_BRIDGE
    );
    assert_eq!(
        ActivityObserver::from_protocol_str(constants::activity_observer::MANAGED_BROWSER_BRIDGE),
        Some(ActivityObserver::ManagedBrowserBridge)
    );
    assert_eq!(
        ActivityEventKind::BrowserInterventionApplied.as_protocol_str(),
        constants::activity_event_kind::BROWSER_INTERVENTION_APPLIED
    );
    assert_eq!(
        ActivityEventKind::EnforcementAuditRecorded.as_protocol_str(),
        constants::activity_event_kind::ENFORCEMENT_AUDIT_RECORDED
    );
    assert_eq!(
        ActivityEventKind::NetworkRetentionDeleted.as_protocol_str(),
        constants::activity_event_kind::NETWORK_RETENTION_DELETED
    );
    assert_eq!(
        ActivityEventKind::TrackingAlertEvaluated.as_protocol_str(),
        constants::activity_event_kind::TRACKING_ALERT_EVALUATED
    );
    assert_eq!(
        ActivityEventKind::from_protocol_str(
            constants::activity_event_kind::TRACKING_ALERT_EVALUATED
        ),
        Some(ActivityEventKind::TrackingAlertEvaluated)
    );
    assert_eq!(
        ActivityEventKind::TrackingParentNotificationRequested.as_protocol_str(),
        constants::activity_event_kind::TRACKING_PARENT_NOTIFICATION_REQUESTED
    );
    assert_eq!(
        ActivityEventKind::from_protocol_str(
            constants::activity_event_kind::TRACKING_PARENT_NOTIFICATION_REQUESTED
        ),
        Some(ActivityEventKind::TrackingParentNotificationRequested)
    );
    assert_eq!(
        ActivityEventKind::from_protocol_str(
            constants::activity_event_kind::NETWORK_RETENTION_DELETED
        ),
        Some(ActivityEventKind::NetworkRetentionDeleted)
    );
    assert_eq!(
        ActivitySubjectKind::Intervention.as_protocol_str(),
        constants::activity_subject_kind::INTERVENTION
    );
    assert_eq!(
        BrowserInterventionAction::Block.as_protocol_str(),
        constants::browser::INTERVENTION_ACTION_BLOCK
    );
    assert_eq!(
        BrowserInterventionDecisionSource::ParentRule.as_protocol_str(),
        constants::browser::INTERVENTION_DECISION_SOURCE_PARENT_RULE
    );
    assert_eq!(
        BrowserInterventionMechanism::ChromiumCdpFetch.as_protocol_str(),
        constants::browser::INTERVENTION_MECHANISM_CHROMIUM_CDP_FETCH
    );
    assert_eq!(
        BrowserInterventionCapabilityState::Ready.as_protocol_str(),
        constants::browser::INTERVENTION_CAPABILITY_READY
    );
    assert_eq!(
        BrowserUnmanagedEnforcementState::RequiresOsAppControl.as_protocol_str(),
        constants::browser::UNMANAGED_ENFORCEMENT_REQUIRES_OS_APP_CONTROL
    );
}

#[test]
fn activity_event_kind_constants_track_the_enum_contract() {
    assert_eq!(constants::activity_event_kind::ALL.len(), 17);

    for (name, kind) in constants::activity_event_kind::ALL {
        assert_eq!(name, format!("{kind:?}"));
        assert_eq!(
            ActivityEventKind::from_protocol_str(kind.as_protocol_str()),
            Some(kind)
        );
    }
}

#[test]
fn browser_evidence_recent_summary_serializes_to_contract_shape() {
    let summary = BrowserEvidenceRecentSummary {
        schema_version: BROWSER_EVIDENCE_SCHEMA_VERSION,
        returned: 1,
        latest_event_id: Some(constants::browser::EVENT_ID_PREFIX.to_string()),
        latest_observed_at: Some(constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string()),
        browser_evidence_id: Some(constants::browser::EVIDENCE_ID_PREFIX.to_string()),
        source_id: Some(constants::browser::SOURCE_ID_MANAGED_CHROMIUM_DEVTOOLS.to_string()),
        adapter_id: Some(constants::browser::ADAPTER_ID_MANAGED_CHROMIUM_DEVTOOLS.to_string()),
        managed_browser_session_id: Some(constants::browser::SESSION_ID_DEV.to_string()),
        browser_family: Some(constants::browser::FAMILY_EDGE.to_string()),
        active_state: Some(constants::browser::ACTIVE_STATE_UNKNOWN.to_string()),
        active_proof_source: Some(
            constants::browser::ACTIVE_PROOF_SOURCE_TARGET_LIST_ONLY.to_string(),
        ),
        url: Some(constants::activity_store::TEST_BROWSER_URL.to_string()),
        origin: Some(constants::activity_store::TEST_BROWSER_ORIGIN.to_string()),
        domain: Some(constants::activity_store::TEST_BROWSER_DOMAIN.to_string()),
        title: Some(constants::activity_store::TEST_BROWSER_TITLE.to_string()),
        capability_status: Some(constants::browser::CAPABILITY_STATUS_TAB_LIST_ONLY.to_string()),
        custody_label: Some(constants::browser::CUSTODY_CHILD_DEVICE_LOCAL.to_string()),
    };

    let serialized =
        serde_json::to_value(summary).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(serialized["schemaVersion"], BROWSER_EVIDENCE_SCHEMA_VERSION);
    assert_eq!(serialized["browserFamily"], constants::browser::FAMILY_EDGE);
    assert_eq!(
        serialized["activeProofSource"],
        constants::browser::ACTIVE_PROOF_SOURCE_TARGET_LIST_ONLY
    );
    assert_eq!(
        serialized["url"],
        constants::activity_store::TEST_BROWSER_URL
    );
    assert_eq!(
        serialized["capabilityStatus"],
        constants::browser::CAPABILITY_STATUS_TAB_LIST_ONLY
    );
}
