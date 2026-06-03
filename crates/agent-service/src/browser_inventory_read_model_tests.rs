use ocentra_parent_agent_core::{
    windows_browser_inventory_observations, BrowserUnmanagedProcessObservation, ProcessObservation,
};
use ocentra_parent_agent_protocol::{
    constants, AgentCommandEnvelope, AgentCommandName, AgentEventName, AgentMessageTarget,
    AgentPeer, AgentPeerRole, AgentRoute, BrowserCapabilityStatus, BrowserChannel,
    BrowserExactUrlCapability, BrowserFamily, BrowserInventoryInstallState, BrowserManagementTier,
    BrowserSupportTier, BrowserUnmanagedDetectionConfidence, BrowserUnmanagedDetectionReason,
    BrowserUnmanagedProcessKind, LogFieldValue, LogFields, AGENT_PROTOCOL_SCHEMA_VERSION,
};

use crate::{
    browser_inventory_read_model::{
        browser_inventory_read_model_from_status,
        browser_inventory_read_model_from_windows_inventory,
    },
    browser_payload::browser_inventory_read_model_payload,
    browser_runtime_status::{connected_status, missing_browser_status, unmanaged_browser_status},
    lan_pairing::LanPairingRuntime,
    websocket::handle_command_text_for_test,
};

#[test]
fn browser_inventory_read_model_reports_managed_target_list_without_active_tab_claim() {
    let status = connected_status(
        constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        Some(constants::browser::DEVTOOLS_TEST_BROWSER_VERSION.to_string()),
        BrowserCapabilityStatus::TabListOnly,
        None,
    );
    let read_model = browser_inventory_read_model_from_status(&status);
    let payload = browser_inventory_read_model_payload(&read_model);
    let row = &read_model.rows[0];

    assert!(row.claim_boundary_is_honest());
    assert_eq!(row.management_tier, BrowserManagementTier::Managed);
    assert_eq!(
        row.exact_url_capability,
        BrowserExactUrlCapability::ManagedTargetListOnly
    );
    assert_eq!(
        payload[constants::field::EXACT_URL_CAPABILITY],
        LogFieldValue::String(
            constants::browser::EXACT_URL_CAPABILITY_MANAGED_TARGET_LIST_ONLY.to_string()
        )
    );
    assert_eq!(
        payload[constants::field::ACTIVE_TAB_CAPABILITY],
        LogFieldValue::String(
            constants::browser::ACTIVE_TAB_CAPABILITY_TARGET_LIST_ONLY.to_string()
        )
    );
}

#[test]
fn browser_inventory_read_model_keeps_unmanaged_processes_process_only() {
    let status = unmanaged_browser_status(
        constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        unmanaged_process_observation(),
    );
    let read_model = browser_inventory_read_model_from_status(&status);
    let payload = browser_inventory_read_model_payload(&read_model);
    let row = &read_model.rows[0];

    assert!(row.claim_boundary_is_honest());
    assert_eq!(row.management_tier, BrowserManagementTier::Unmanaged);
    assert_eq!(
        row.exact_url_capability,
        BrowserExactUrlCapability::NotClaimed
    );
    assert_eq!(
        payload[constants::field::EXACT_URL_CAPABILITY],
        LogFieldValue::String(constants::browser::EXACT_URL_CAPABILITY_NOT_CLAIMED.to_string())
    );
    assert_eq!(
        payload[constants::field::UNMANAGED_FALLBACK_CAPABILITY],
        LogFieldValue::String(constants::browser::UNMANAGED_FALLBACK_REPORT_ONLY.to_string())
    );
    assert_eq!(
        payload[constants::field::PROCESS_ID],
        LogFieldValue::Number(constants::browser::DEVTOOLS_TEST_UNMANAGED_PROCESS_ID as f64)
    );
    assert_eq!(
        payload[constants::field::EXECUTABLE_PATH_REF],
        LogFieldValue::String(
            constants::browser::INVENTORY_EXECUTABLE_PATH_REF_WINDOWS_REDACTED.to_string()
        )
    );
}

#[test]
fn browser_inventory_read_model_marks_missing_browser_unavailable() {
    let status =
        missing_browser_status(constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string());
    let read_model = browser_inventory_read_model_from_status(&status);
    let payload = browser_inventory_read_model_payload(&read_model);
    let row = &read_model.rows[0];

    assert!(row.claim_boundary_is_honest());
    assert_eq!(row.management_tier, BrowserManagementTier::Unknown);
    assert_eq!(
        row.exact_url_capability,
        BrowserExactUrlCapability::Unavailable
    );
    assert_eq!(
        payload[constants::field::EXACT_URL_CAPABILITY],
        LogFieldValue::String(constants::browser::EXACT_URL_CAPABILITY_UNAVAILABLE.to_string())
    );
    assert_eq!(
        payload[constants::field::REASON],
        LogFieldValue::String(constants::value::MANAGED_BROWSER_EXECUTABLE_MISSING.to_string())
    );
}

#[test]
fn browser_inventory_read_model_maps_windows_inventory_without_url_claims() {
    let process = ProcessObservation {
        pid: constants::browser::DEVTOOLS_TEST_UNMANAGED_PROCESS_ID,
        name: constants::browser::EXECUTABLE_CHROME_WINDOWS.to_string(),
        executable_path: None,
    };
    let observations = windows_browser_inventory_observations(&[], &[process], None);

    let read_model = browser_inventory_read_model_from_windows_inventory(
        constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        &observations,
    );
    let row = &read_model.rows[0];

    assert!(row.claim_boundary_is_honest());
    assert_eq!(read_model.returned, 1);
    assert_eq!(
        row.install_state,
        BrowserInventoryInstallState::CandidateRunning
    );
    assert_eq!(row.management_tier, BrowserManagementTier::Unmanaged);
    assert_eq!(row.support_tier, BrowserSupportTier::UnmanagedProcessOnly);
    assert_eq!(
        row.exact_url_capability,
        BrowserExactUrlCapability::NotClaimed
    );
}

#[tokio::test]
async fn browser_inventory_read_model_command_reports_replayable_service_event() {
    let event = handle_command_text_for_test(
        &serde_json::to_string(&inventory_command())
            .expect(constants::error::AGENT_EVENT_SERIALIZES),
        LanPairingRuntime::empty(),
        None,
    )
    .await;

    assert_eq!(
        event.event,
        AgentEventName::AgentBrowserInventoryReadModelReported
    );
    assert!(event
        .event_id
        .starts_with(constants::event_id::BROWSER_INVENTORY_READ_MODEL_REPORTED));
    assert!(matches!(
        event.payload[constants::field::RETURNED],
        LogFieldValue::Number(_)
    ));
    assert_eq!(event.payload.get(constants::field::URL), None);
    assert_eq!(event.payload.get(constants::field::ACTIVE_STATE), None);
}

fn inventory_command() -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: constants::event_id::BROWSER_INVENTORY_READ_MODEL_REPORTED.to_string(),
        sent_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        source: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        target: AgentMessageTarget {
            device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            platform: constants::enforcement::PLATFORM_WINDOWS.to_string(),
            route: AgentRoute::Localhost,
        },
        command: AgentCommandName::AgentBrowserInventoryReadModelGet,
        payload: LogFields::new(),
    }
}

fn unmanaged_process_observation() -> BrowserUnmanagedProcessObservation {
    BrowserUnmanagedProcessObservation {
        process_id: constants::browser::DEVTOOLS_TEST_UNMANAGED_PROCESS_ID,
        process_name: constants::browser::EXECUTABLE_CHROME_WINDOWS.to_string(),
        executable_path_ref: Some(
            constants::browser::INVENTORY_EXECUTABLE_PATH_REF_WINDOWS_REDACTED.to_string(),
        ),
        signature_ref: None,
        process_hash_ref: None,
        browser_family: BrowserFamily::Chrome,
        browser_channel: BrowserChannel::Stable,
        process_kind: BrowserUnmanagedProcessKind::SupportedBrowser,
        detection_confidence: BrowserUnmanagedDetectionConfidence::High,
        detection_reason: BrowserUnmanagedDetectionReason::SupportedBrowserOutsideManagedSession,
    }
}
