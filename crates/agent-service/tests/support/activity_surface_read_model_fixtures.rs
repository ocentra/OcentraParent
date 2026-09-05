use ocentra_parent_agent_protocol::activity::{
    ActivityEventKind, ActivityObserver, ActivitySubjectKind,
};
use ocentra_parent_agent_protocol::activity_query::{
    ActivityRecentSummary, ACTIVITY_QUERY_SCHEMA_VERSION,
};
use ocentra_parent_agent_protocol::browser::{
    BrowserActiveProofSource, BrowserActiveTabState, BrowserCapabilityStatus, BrowserChannel,
    BrowserCustodyLabel, BrowserFamily,
};
use ocentra_parent_agent_protocol::browser_managed::BrowserQueryVisibilityLabel;
use ocentra_parent_agent_protocol::browser_read_model::{
    BrowserEvidenceReadModel, BrowserTabEvidence,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::network_flow::{
    ActivityNetworkEndpoint, ActivityNetworkFlowCounters, ActivityNetworkFlowObservation,
    ActivityNetworkFlowReadModel,
};
use ocentra_parent_agent_protocol::screen_evidence::{
    ScreenAnalysisResult, ScreenEvidenceQueueHealth, ScreenEvidenceRecentSummary,
};
use ocentra_parent_agent_protocol::{
    ACTIVITY_SURFACE_SCHEMA_VERSION, BROWSER_EVIDENCE_SCHEMA_VERSION, NETWORK_FLOW_SCHEMA_VERSION,
};

use crate::activity_surface_common_fixtures::{
    evidence_ref, TEST_FIRST_OBSERVED_AT, TEST_THIRD_OBSERVED_AT, TEST_TIMESTAMP,
};

pub(crate) fn recent_summary(returned: u64) -> ActivityRecentSummary {
    ActivityRecentSummary {
        schema_version: ACTIVITY_QUERY_SCHEMA_VERSION,
        limit: 10,
        returned,
        first_observed_at: Some(TEST_FIRST_OBSERVED_AT.to_string()),
        last_observed_at: Some(TEST_THIRD_OBSERVED_AT.to_string()),
        last_event_id: Some("recent-event-1".to_string()),
        most_recent_kind: Some(ActivityEventKind::ProcessObserved),
        most_recent_observer: Some(ActivityObserver::AgentService),
        most_recent_subject_kind: Some(ActivitySubjectKind::Process),
        most_recent_subject_id: Some("subject-1".to_string()),
        most_recent_subject_name: Some("Recent App".to_string()),
    }
}

pub(crate) fn screen_summary(returned: u64) -> ScreenEvidenceRecentSummary {
    ScreenEvidenceRecentSummary {
        schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
        generated_at: TEST_TIMESTAMP.to_string(),
        custody_state: "child-device-local".to_string(),
        limit: 10,
        returned,
        queue_health: screen_queue_health(),
        latest_result_id: Some("screen-result-1".to_string()),
        latest_summary: Some("Screen summary".to_string()),
        latest_primary_category: Some("education".to_string()),
        latest_confidence: Some(0.91),
        latest_image_deletion_state: Some("retained".to_string()),
        latest_policy_eligible: Some(true),
        evidence: vec![evidence_ref("screen-evidence-1", None)],
        results: if returned > 0 {
            vec![screen_analysis_result()]
        } else {
            Vec::new()
        },
    }
}

fn screen_queue_health() -> ScreenEvidenceQueueHealth {
    ScreenEvidenceQueueHealth {
        schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
        generated_at: TEST_TIMESTAMP.to_string(),
        custody_state: "child-device-local".to_string(),
        pending_count: 0,
        expired_count: 0,
        delete_pending_count: 0,
        delete_failed_count: 0,
        latest_queue_job_id: Some("queue-job-1".to_string()),
        latest_status: Some("ready".to_string()),
        last_successful_analysis_at: Some(TEST_THIRD_OBSERVED_AT.to_string()),
    }
}

fn screen_analysis_result() -> ScreenAnalysisResult {
    ScreenAnalysisResult {
        schema_version: ACTIVITY_SURFACE_SCHEMA_VERSION,
        screen_analysis_result_id: "screen-result-1".to_string(),
        queue_job_id: "queue-job-1".to_string(),
        analyzed_at: TEST_TIMESTAMP.to_string(),
        model_runtime_ref: "local-model-runtime".to_string(),
        model_id: "vision-model-1".to_string(),
        provider_kind: "local".to_string(),
        prompt_or_template_version: "prompt-v1".to_string(),
        capture_reason: "scheduled".to_string(),
        capture_scope: "foreground".to_string(),
        capability_status: "available".to_string(),
        summary: "Screen summary".to_string(),
        visible_category_candidates: Vec::new(),
        primary_category: Some("education".to_string()),
        risk_signals: Vec::new(),
        ocr_text_snippets: vec!["Hello".to_string()],
        redaction_notes: vec!["none".to_string()],
        confidence: 0.91,
        uncertainty_reason: None,
        source_evidence_refs: vec![evidence_ref("screen-evidence-1", None)],
        image_digest: "screen-image-digest-1".to_string(),
        raw_image_retained: true,
        image_deletion_state: "retained".to_string(),
        custody_state: "child-device-local".to_string(),
        policy_eligible: true,
        policy_decision_ref: Some("policy-decision-1".to_string()),
        policy_action: Some("allow".to_string()),
        policy_reason_codes: vec!["reason-1".to_string()],
        parent_rule_refs: vec!["rule-1".to_string()],
        local_model_runtime_refs: vec!["runtime-ref-1".to_string()],
        parent_explanation_refs: vec!["explanation-ref-1".to_string()],
        explanation_reasons: vec!["explanation-1".to_string()],
        deletion_reasons: vec!["retained-for-review".to_string()],
    }
}

pub(crate) fn browser_read_model_fixture(returned: u64) -> BrowserEvidenceReadModel {
    BrowserEvidenceReadModel {
        schema_version: BROWSER_EVIDENCE_SCHEMA_VERSION,
        generated_at: TEST_TIMESTAMP.to_string(),
        limit: 10,
        returned,
        latest_event_id: Some("browser-event-1".to_string()),
        latest_observed_at: Some(TEST_THIRD_OBSERVED_AT.to_string()),
        capability_status: Some(BrowserCapabilityStatus::Available),
        custody_label: BrowserCustodyLabel::ChildDeviceLocal,
        query_visibility: BrowserQueryVisibilityLabel::LiveLocal,
        rows: if returned > 0 {
            vec![browser_tab_evidence()]
        } else {
            Vec::new()
        },
    }
}

pub(crate) fn network_read_model_fixture(returned: u64) -> ActivityNetworkFlowReadModel {
    ActivityNetworkFlowReadModel {
        schema_version: NETWORK_FLOW_SCHEMA_VERSION,
        generated_at: TEST_TIMESTAMP.to_string(),
        custody: "child-device-query-store".to_string(),
        limit: 10,
        returned,
        active_rows: returned,
        tombstone_rows: 0,
        exportable_rows: returned,
        capability_status: "available".to_string(),
        latest_event_id: Some("network-event-1".to_string()),
        latest_observed_at: Some(TEST_THIRD_OBSERVED_AT.to_string()),
        latest_tombstone_event_id: None,
        latest_tombstone_observed_at: None,
        deleted_evidence_reference_ids: Vec::new(),
        rows: if returned > 0 {
            vec![network_observation()]
        } else {
            Vec::new()
        },
    }
}

fn browser_tab_evidence() -> BrowserTabEvidence {
    BrowserTabEvidence {
        schema_version: BROWSER_EVIDENCE_SCHEMA_VERSION,
        browser_evidence_id: "browser-evidence-1".to_string(),
        observed_at: TEST_THIRD_OBSERVED_AT.to_string(),
        fresh_until: TEST_THIRD_OBSERVED_AT.to_string(),
        source_id: "source-1".to_string(),
        adapter_id: "adapter-1".to_string(),
        device_id: constants::activity_surface::DEFAULT_DEVICE_ID.to_string(),
        browser_family: BrowserFamily::Chrome,
        browser_channel: BrowserChannel::Stable,
        managed_browser_session_id: "session-1".to_string(),
        profile_id: "profile-1".to_string(),
        process_id: 41,
        window_id: Some("window-1".to_string()),
        tab_id: Some("tab-1".to_string()),
        target_id: Some("target-1".to_string()),
        active_state: BrowserActiveTabState::KnownActive,
        active_proof_source: BrowserActiveProofSource::ManagedExtensionEvent,
        url: "https://example.com/page".to_string(),
        origin: "https://example.com".to_string(),
        domain: "example.com".to_string(),
        title: Some("Example".to_string()),
        capability_status: BrowserCapabilityStatus::Available,
        degraded_reason: None,
        stale_at: TEST_THIRD_OBSERVED_AT.to_string(),
        custody_label: BrowserCustodyLabel::ChildDeviceLocal,
        query_visibility: BrowserQueryVisibilityLabel::LiveLocal,
    }
}

fn network_observation() -> ActivityNetworkFlowObservation {
    ActivityNetworkFlowObservation {
        schema_version: NETWORK_FLOW_SCHEMA_VERSION,
        event_id: "network-event-1".to_string(),
        observed_at: TEST_THIRD_OBSERVED_AT.to_string(),
        observer: "capture".to_string(),
        capability_status: "available".to_string(),
        adapter_id: "adapter-1".to_string(),
        protocol: Some("tcp".to_string()),
        tcp_state: Some("established".to_string()),
        local_endpoint: network_local_endpoint(),
        destination_endpoint: network_destination_endpoint(),
        destination_domain: Some("api.example.com".to_string()),
        domain_attribution_status: "known".to_string(),
        process_attribution_status: "known".to_string(),
        process_id: Some(55),
        process_name: Some("browser.exe".to_string()),
        associated_pid_count: Some(1),
        counters: network_counters(),
        evidence: vec![evidence_ref("network-evidence-1", Some("network-digest-1"))],
    }
}

fn network_local_endpoint() -> ActivityNetworkEndpoint {
    ActivityNetworkEndpoint {
        ip: Some("192.168.1.2".to_string()),
        port: Some(50000),
    }
}

fn network_destination_endpoint() -> ActivityNetworkEndpoint {
    ActivityNetworkEndpoint {
        ip: Some("93.184.216.34".to_string()),
        port: Some(443),
    }
}

fn network_counters() -> ActivityNetworkFlowCounters {
    ActivityNetworkFlowCounters {
        connection_count: 3,
        bytes_sent: Some(50),
        bytes_received: Some(70),
        first_seen_at: Some(TEST_FIRST_OBSERVED_AT.to_string()),
        last_seen_at: Some(TEST_THIRD_OBSERVED_AT.to_string()),
    }
}
