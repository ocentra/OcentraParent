use ocentra_parent_agent_protocol::browser::{
    BrowserActiveProofSource, BrowserActiveTabState, BrowserCapabilityStatus, BrowserChannel,
    BrowserCustodyLabel, BrowserFamily,
};
use ocentra_parent_agent_protocol::browser_managed::BrowserQueryVisibilityLabel;
use ocentra_parent_agent_protocol::browser_read_model::{
    BrowserEvidenceReadModel, BrowserTabEvidence,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::BROWSER_EVIDENCE_SCHEMA_VERSION;

use super::browser_runtime_delivery::{
    browser_runtime_input_from_row, deliver_browser_runtime_for_read_model,
};

#[tokio::test]
async fn service_browser_read_model_delivers_local_runtime_chain() {
    let report = deliver_browser_runtime_for_read_model(&read_model(vec![managed_row()])).await;

    assert_eq!(report.observed_rows, 1);
    assert_eq!(report.delivered_rows, 1);
    assert_eq!(report.failed_rows, 0);
    assert_eq!(report.dead_letters, 0);
    assert_eq!(report.exact_url_rows, 1);
    assert_eq!(report.manual_required_rows, 0);
    assert_eq!(report.intervention_command_events, 0);
    assert_eq!(report.read_model_projection_events, 1);
    assert!(report.publish_reports > 0);
    assert_eq!(report.publish_reports, report.stored_events);
}

#[tokio::test]
async fn service_browser_read_model_keeps_unavailable_rows_manual_required() {
    let report = deliver_browser_runtime_for_read_model(&read_model(vec![unavailable_row()])).await;

    assert_eq!(report.observed_rows, 1);
    assert_eq!(report.delivered_rows, 1);
    assert_eq!(report.failed_rows, 0);
    assert_eq!(report.exact_url_rows, 0);
    assert_eq!(report.manual_required_rows, 1);
    assert_eq!(report.intervention_command_events, 0);
    assert_eq!(report.read_model_projection_events, 1);
}

#[tokio::test]
async fn service_browser_read_model_keeps_stale_and_unsupported_rows_manual_required() {
    let report =
        deliver_browser_runtime_for_read_model(&read_model(vec![stale_row(), unsupported_row()]))
            .await;

    assert_eq!(report.observed_rows, 2);
    assert_eq!(report.delivered_rows, 2);
    assert_eq!(report.failed_rows, 0);
    assert_eq!(report.exact_url_rows, 0);
    assert_eq!(report.manual_required_rows, 2);
    assert_eq!(report.intervention_command_events, 0);
    assert_eq!(report.read_model_projection_events, 2);
}

#[test]
fn service_browser_row_input_does_not_turn_evidence_into_policy_authority() {
    let model = read_model(vec![managed_row()]);
    let input = browser_runtime_input_from_row(&model, &model.rows[0]);

    assert!(input.exact_url_claimed);
    assert!(!input.ai_authority);
    assert!(!input.policy_authority);
    assert!(!input.intervention_command_allowed);
    assert_eq!(input.ai_request_ref, None);
    assert_eq!(input.policy_decision_ref, None);
    assert_eq!(input.intervention_command_ref, None);
}

fn read_model(rows: Vec<BrowserTabEvidence>) -> BrowserEvidenceReadModel {
    BrowserEvidenceReadModel {
        schema_version: BROWSER_EVIDENCE_SCHEMA_VERSION,
        generated_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        limit: constants::activity_store::DEFAULT_RECENT_LIMIT,
        returned: rows.len() as u64,
        latest_event_id: Some(constants::browser::TEST_BROWSER_RUNTIME_JOURNAL_REF.to_string()),
        latest_observed_at: rows.first().map(|row| row.observed_at.clone()),
        capability_status: Some(BrowserCapabilityStatus::TabListOnly),
        custody_label: BrowserCustodyLabel::ChildDeviceLocal,
        query_visibility: BrowserQueryVisibilityLabel::LiveLocal,
        rows,
    }
}

fn managed_row() -> BrowserTabEvidence {
    BrowserTabEvidence {
        schema_version: BROWSER_EVIDENCE_SCHEMA_VERSION,
        browser_evidence_id: constants::browser::TEST_BROWSER_RUNTIME_EVIDENCE_REF.to_string(),
        observed_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        fresh_until: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        source_id: constants::browser::SOURCE_ID_MANAGED_CHROMIUM_DEVTOOLS.to_string(),
        adapter_id: constants::browser::ADAPTER_ID_MANAGED_CHROMIUM_DEVTOOLS.to_string(),
        device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
        browser_family: BrowserFamily::Edge,
        browser_channel: BrowserChannel::Stable,
        managed_browser_session_id: constants::browser::SESSION_ID_DEV.to_string(),
        profile_id: constants::browser::PROFILE_ID_DEV.to_string(),
        process_id: constants::browser::PROCESS_ID_UNKNOWN,
        window_id: None,
        tab_id: Some(constants::activity_store::TEST_BROWSER_TAB_ID_FROM_TARGET.to_string()),
        target_id: Some(constants::activity_store::TEST_BROWSER_TARGET_ID.to_string()),
        active_state: BrowserActiveTabState::Unknown,
        active_proof_source: BrowserActiveProofSource::TargetListOnly,
        url: constants::activity_store::TEST_BROWSER_URL.to_string(),
        origin: constants::activity_store::TEST_BROWSER_ORIGIN.to_string(),
        domain: constants::activity_store::TEST_BROWSER_DOMAIN.to_string(),
        title: Some(constants::activity_store::TEST_BROWSER_TITLE.to_string()),
        capability_status: BrowserCapabilityStatus::TabListOnly,
        degraded_reason: None,
        stale_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        custody_label: BrowserCustodyLabel::ChildDeviceLocal,
        query_visibility: BrowserQueryVisibilityLabel::LiveLocal,
    }
}

fn unavailable_row() -> BrowserTabEvidence {
    BrowserTabEvidence {
        managed_browser_session_id: constants::value::EMPTY.to_string(),
        capability_status: BrowserCapabilityStatus::BridgeMissing,
        query_visibility: BrowserQueryVisibilityLabel::Unavailable,
        degraded_reason: Some(constants::value::BROWSER_BRIDGE_NO_PAGE_TARGETS.to_string()),
        ..managed_row()
    }
}

fn stale_row() -> BrowserTabEvidence {
    BrowserTabEvidence {
        capability_status: BrowserCapabilityStatus::Stale,
        degraded_reason: Some(constants::value::BROWSER_BRIDGE_STALE_SESSION.to_string()),
        ..managed_row()
    }
}

fn unsupported_row() -> BrowserTabEvidence {
    BrowserTabEvidence {
        browser_family: BrowserFamily::Firefox,
        browser_channel: BrowserChannel::Stable,
        capability_status: BrowserCapabilityStatus::UnsupportedBrowser,
        query_visibility: BrowserQueryVisibilityLabel::Unavailable,
        degraded_reason: Some(
            constants::browser::INVENTORY_REASON_WINDOWS_UNSUPPORTED_LATER_ADAPTER.to_string(),
        ),
        ..managed_row()
    }
}
