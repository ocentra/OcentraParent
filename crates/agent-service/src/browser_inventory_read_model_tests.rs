use std::path::Path;

use ocentra_parent_agent_core::browser_managed_discovery::BrowserUnmanagedProcessObservation;
use ocentra_parent_agent_core::browser_windows_inventory::windows_browser_inventory_observations;
use ocentra_parent_agent_core::browser_windows_package_inventory::windows_browser_package_observations;
use ocentra_parent_agent_core::browser_windows_package_source::live_windows_browser_package_entries_from_roots;
use ocentra_parent_agent_core::process_capture::ProcessObservation;
use ocentra_parent_agent_protocol::browser::{
    BrowserCapabilityStatus, BrowserChannel, BrowserFamily,
};
use ocentra_parent_agent_protocol::browser_inventory::{
    BrowserExactUrlCapability, BrowserInventoryInstallState, BrowserManagementTier,
    BrowserSupportTier,
};
use ocentra_parent_agent_protocol::browser_managed::{
    BrowserUnmanagedDetectionConfidence, BrowserUnmanagedDetectionReason,
    BrowserUnmanagedProcessKind,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentEventName, AgentMessageTarget, AgentPeer,
    AgentPeerRole, AgentRoute,
};
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;

use crate::{
    activity_api::browser_inventory_read_model_from_service_defaults,
    browser_inventory_read_model::{
        browser_inventory_read_model_from_status,
        browser_inventory_read_model_from_windows_inventory,
    },
    browser_payload::browser_inventory_read_model_payload,
    browser_runtime_status::{
        bridge_disconnected_status, connected_status, missing_browser_status,
        unmanaged_browser_status,
    },
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
    assert_eq!(
        payload[constants::field::PUBLISHER_SIGNATURE_REF],
        LogFieldValue::String(
            constants::browser::INVENTORY_PUBLISHER_SIGNATURE_REF_WINDOWS_REDACTED.to_string()
        )
    );
    assert_eq!(
        payload[constants::field::FILE_HASH_REF],
        LogFieldValue::String(
            constants::browser::INVENTORY_FILE_HASH_REF_WINDOWS_REDACTED.to_string()
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
fn browser_inventory_read_model_maps_bridge_disconnect_to_stale_manual_required() {
    let status = bridge_disconnected_status(
        constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        constants::value::BROWSER_BRIDGE_STALE_SESSION,
    );
    let read_model = browser_inventory_read_model_from_status(&status);
    let payload = browser_inventory_read_model_payload(&read_model);
    let row = &read_model.rows[0];

    assert!(row.claim_boundary_is_honest());
    assert_eq!(row.management_tier, BrowserManagementTier::ManualRequired);
    assert_eq!(row.support_tier, BrowserSupportTier::ManualRequired);
    assert_eq!(
        row.exact_url_capability,
        BrowserExactUrlCapability::ManualRequired
    );
    assert_eq!(
        payload[constants::field::CAPABILITY_STATUS],
        LogFieldValue::String(constants::browser::CAPABILITY_STATUS_STALE.to_string())
    );
    assert_eq!(
        payload[constants::field::REASON],
        LogFieldValue::String(constants::value::BROWSER_BRIDGE_STALE_SESSION.to_string())
    );
}

#[test]
fn browser_inventory_read_model_keeps_unsupported_later_adapter_not_claimed() {
    let status = connected_status(
        constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        None,
        BrowserCapabilityStatus::UnsupportedBrowser,
        Some(constants::browser::INVENTORY_REASON_WINDOWS_UNSUPPORTED_LATER_ADAPTER.to_string()),
    );
    let read_model = browser_inventory_read_model_from_status(&status);
    let payload = browser_inventory_read_model_payload(&read_model);
    let row = &read_model.rows[0];

    assert!(row.claim_boundary_is_honest());
    assert_eq!(row.management_tier, BrowserManagementTier::Unsupported);
    assert_eq!(row.support_tier, BrowserSupportTier::Unsupported);
    assert_eq!(
        row.exact_url_capability,
        BrowserExactUrlCapability::Unsupported
    );
    assert_eq!(
        payload[constants::field::CAPABILITY_STATUS],
        LogFieldValue::String(
            constants::browser::CAPABILITY_STATUS_UNSUPPORTED_BROWSER.to_string()
        )
    );
    assert_eq!(
        payload[constants::field::EXACT_URL_CAPABILITY],
        LogFieldValue::String(constants::browser::EXACT_URL_CAPABILITY_UNSUPPORTED.to_string())
    );
    assert_eq!(
        payload[constants::field::REASON],
        LogFieldValue::String(
            constants::browser::INVENTORY_REASON_WINDOWS_UNSUPPORTED_LATER_ADAPTER.to_string()
        )
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
        &serde_json::to_string(&inventory_command()).unwrap_or_else(|error| {
            panic!("{}: {error:?}", constants::error::AGENT_EVENT_SERIALIZES)
        }),
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

#[tokio::test]
async fn browser_inventory_service_default_roots_feed_windows_inventory_without_url_claims() {
    let _guard = crate::activity_report_env_lock::REPORT_ENV_LOCK
        .lock()
        .await;
    let root = temp_service_inventory_root();
    let edge = root
        .join(constants::browser::PATH_SEGMENT_MICROSOFT)
        .join(constants::browser::PATH_SEGMENT_EDGE)
        .join(constants::browser::PATH_SEGMENT_APPLICATION)
        .join(constants::browser::EXECUTABLE_MSEDGE_WINDOWS);
    create_executable_fixture(&edge);
    let previous_program_files = std::env::var_os(constants::env_var::PROGRAM_FILES);
    let previous_program_files_x86 = std::env::var_os(constants::env_var::PROGRAM_FILES_X86);
    let previous_local_app_data = std::env::var_os(constants::env_var::LOCAL_APP_DATA);
    std::env::set_var(constants::env_var::PROGRAM_FILES, &root);
    std::env::remove_var(constants::env_var::PROGRAM_FILES_X86);
    std::env::remove_var(constants::env_var::LOCAL_APP_DATA);

    let read_model = browser_inventory_read_model_from_service_defaults(
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        &[],
    );

    restore_env_var(constants::env_var::PROGRAM_FILES, previous_program_files);
    restore_env_var(
        constants::env_var::PROGRAM_FILES_X86,
        previous_program_files_x86,
    );
    restore_env_var(constants::env_var::LOCAL_APP_DATA, previous_local_app_data);
    let _ = std::fs::remove_dir_all(root);
    assert!(read_model.returned >= 1);
    let row = read_model
        .rows
        .iter()
        .find(|row| {
            row.product_name == constants::browser::PRODUCT_NAME_MICROSOFT_EDGE
                && row.browser_family == BrowserFamily::Edge
                && row.install_state == BrowserInventoryInstallState::Installed
                && row.process_id.is_none()
        })
        .unwrap_or_else(|| panic!("{}", constants::error::BROWSER_BRIDGE_MAPS_TARGET));

    assert!(row.claim_boundary_is_honest());
    assert_eq!(row.management_tier, BrowserManagementTier::Managed);
    assert_eq!(
        row.exact_url_capability,
        BrowserExactUrlCapability::Unavailable
    );
}

#[test]
fn browser_inventory_service_sources_feed_packaged_browser_without_url_claims() {
    let root = temp_service_inventory_root();
    let package_root = root
        .join(ocentra_parent_agent_protocol::app_game::APP_GAME_WINDOWS_PATH_WINDOWS_APPS)
        .join(constants::browser::DEVTOOLS_TEST_EDGE_STORE_PACKAGE_NAME);
    write_manifest(
        &package_root,
        constants::browser::DEVTOOLS_TEST_EDGE_STORE_PACKAGE_MANIFEST_XML,
    );
    let packages = live_windows_browser_package_entries_from_roots(
        std::slice::from_ref(
            &root.join(ocentra_parent_agent_protocol::app_game::APP_GAME_WINDOWS_PATH_WINDOWS_APPS),
        ),
        constants::browser::PACKAGE_SCAN_LIMIT_BROWSER_DISCOVERY,
    );
    let observations = windows_browser_package_observations(&packages);
    let read_model = browser_inventory_read_model_from_windows_inventory(
        constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        &observations,
    );

    let _ = std::fs::remove_dir_all(root);
    let row = read_model
        .rows
        .iter()
        .find(|row| {
            row.product_name == constants::browser::PRODUCT_NAME_MICROSOFT_EDGE
                && row.install_state == BrowserInventoryInstallState::Packaged
                && row.executable_path_ref.is_none()
        })
        .unwrap_or_else(|| panic!("{}", constants::error::BROWSER_BRIDGE_MAPS_TARGET));

    assert!(row.claim_boundary_is_honest());
    assert_eq!(row.management_tier, BrowserManagementTier::ManualRequired);
    assert_eq!(
        row.exact_url_capability,
        BrowserExactUrlCapability::ManualRequired
    );
}

#[test]
fn browser_inventory_package_observations_are_manual_required_without_url_claims() {
    let root = temp_service_inventory_root();
    let windows_apps =
        root.join(ocentra_parent_agent_protocol::app_game::APP_GAME_WINDOWS_PATH_WINDOWS_APPS);
    let package_root = windows_apps.join(constants::browser::DEVTOOLS_TEST_EDGE_STORE_PACKAGE_NAME);
    write_manifest(
        &package_root,
        constants::browser::DEVTOOLS_TEST_EDGE_STORE_PACKAGE_MANIFEST_XML,
    );
    let packages = live_windows_browser_package_entries_from_roots(
        std::slice::from_ref(&windows_apps),
        constants::browser::PACKAGE_SCAN_LIMIT_BROWSER_DISCOVERY,
    );
    let observations = windows_browser_package_observations(&packages);

    let _ = std::fs::remove_dir_all(root);
    assert_eq!(observations.len(), 1);
    assert_eq!(
        observations[0].install_state,
        BrowserInventoryInstallState::Packaged
    );
    assert_eq!(
        observations[0].management_tier,
        BrowserManagementTier::ManualRequired
    );
    assert_eq!(
        observations[0].exact_url_capability,
        BrowserExactUrlCapability::ManualRequired
    );
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
        signature_ref: Some(
            constants::browser::INVENTORY_PUBLISHER_SIGNATURE_REF_WINDOWS_REDACTED.to_string(),
        ),
        process_hash_ref: Some(
            constants::browser::INVENTORY_FILE_HASH_REF_WINDOWS_REDACTED.to_string(),
        ),
        browser_family: BrowserFamily::Chrome,
        browser_channel: BrowserChannel::Stable,
        process_kind: BrowserUnmanagedProcessKind::SupportedBrowser,
        detection_confidence: BrowserUnmanagedDetectionConfidence::High,
        detection_reason: BrowserUnmanagedDetectionReason::SupportedBrowserOutsideManagedSession,
    }
}

fn temp_service_inventory_root() -> std::path::PathBuf {
    let root = std::env::temp_dir()
        .join(constants::browser::DEVTOOLS_TEST_WINDOWS_BROWSER_INVENTORY_DIR)
        .join(std::process::id().to_string())
        .join(constants::browser::PATH_SEGMENT_DEFAULT);
    let _ = std::fs::remove_dir_all(&root);
    root
}

fn create_executable_fixture(path: &std::path::PathBuf) {
    std::fs::create_dir_all(
        path.parent()
            .unwrap_or_else(|| panic!("{}", constants::error::BROWSER_BRIDGE_MAPS_TARGET)),
    )
    .unwrap_or_else(|error| {
        panic!(
            "{}: {error:?}",
            constants::error::BROWSER_BRIDGE_MAPS_TARGET
        )
    });
    std::fs::write(path, []).unwrap_or_else(|error| {
        panic!(
            "{}: {error:?}",
            constants::error::BROWSER_BRIDGE_MAPS_TARGET
        )
    });
}

fn write_manifest(root: &Path, manifest: &str) {
    if let Some(parent) = root.parent() {
        std::fs::create_dir_all(parent).unwrap_or_else(|error| {
            panic!("{}: {error:?}", constants::error::ACTIVITY_CAPTURE_RECORDS)
        });
    }
    std::fs::create_dir_all(root).unwrap_or_else(|error| {
        panic!("{}: {error:?}", constants::error::ACTIVITY_CAPTURE_RECORDS)
    });
    std::fs::write(
        root.join(
            ocentra_parent_agent_protocol::app_game::APP_GAME_WINDOWS_APPX_MANIFEST_FILE_NAME,
        ),
        manifest,
    )
    .unwrap_or_else(|error| {
        panic!("{}: {error:?}", constants::error::ACTIVITY_CAPTURE_RECORDS)
    });
}

fn restore_env_var(env_var_name: &str, value: Option<std::ffi::OsString>) {
    match value {
        Some(previous) => std::env::set_var(env_var_name, previous),
        None => std::env::remove_var(env_var_name),
    }
}
