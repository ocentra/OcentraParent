use std::ffi::OsString as TestOsString;
use std::path::Path;
use std::path::PathBuf as TestPathBuf;
use std::primitive::str as TestStr;
use std::sync::atomic::{AtomicU64, Ordering};

use ocentra_parent_agent_core::browser_windows_inventory::windows_browser_inventory_observations;
use ocentra_parent_agent_core::browser_windows_package_inventory::windows_browser_package_observations;
use ocentra_parent_agent_core::browser_windows_package_source::live_windows_browser_package_entries_from_roots;
use ocentra_parent_agent_core::process_capture::ProcessObservation;
use ocentra_parent_agent_protocol::browser::BrowserFamily;
use ocentra_parent_agent_protocol::browser_inventory::{
    BrowserExactUrlCapability, BrowserInventoryInstallState, BrowserManagementTier,
    BrowserSupportTier,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;

use crate::{
    browser_inventory_read_model::{
        browser_inventory_read_model_from_windows_inventory, BrowserInventoryGeneratedAtText,
    },
    browser_inventory_test_support::browser_inventory_read_model_from_service_defaults_for_test,
    browser_payload::{browser_inventory_read_model_payload, browser_managed_status_payload},
    browser_runtime_status::status_with_error,
    test_require_ok::require_ok,
    test_require_some::require_some,
};

#[test]
fn browser_inventory_read_model_maps_windows_inventory_without_url_claims() {
    let process = ProcessObservation {
        pid: constants::browser::DEVTOOLS_TEST_UNMANAGED_PROCESS_ID,
        name: constants::browser::EXECUTABLE_CHROME_WINDOWS.to_string(),
        executable_path: None,
    };
    let observations = windows_browser_inventory_observations(&[], &[process], None);

    let read_model = browser_inventory_read_model_from_windows_inventory(
        BrowserInventoryGeneratedAtText(
            constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        ),
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

    let payload = browser_inventory_read_model_payload(&read_model);
    assert_eq!(
        payload.get(constants::field::RETURNED),
        Some(&LogFieldValue::Number(1.0))
    );
    assert_eq!(
        payload.get(constants::field::BROWSER_FAMILY),
        Some(&LogFieldValue::String(
            BrowserFamily::Chrome.as_protocol_str().to_string()
        ))
    );
}

#[test]
fn browser_inventory_status_helper_maps_error_state_and_payload() {
    let checked_at = constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string();
    let errored = status_with_error(checked_at, constants::value::MANAGED_BROWSER_LAUNCH_ERROR);
    let payload = browser_managed_status_payload(&errored);

    assert_eq!(
        errored.managed_state.as_protocol_str().to_string(),
        constants::browser::MANAGED_STATE_ERROR
    );
    assert_eq!(
        payload.get(constants::field::MANAGED_STATE).cloned(),
        Some(LogFieldValue::String(
            constants::browser::MANAGED_STATE_ERROR.to_string()
        ))
    );
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

    let read_model = browser_inventory_read_model_from_service_defaults_for_test(
        BrowserInventoryGeneratedAtText(
            constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        ),
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
    let row = read_model.rows.iter().find(|row| {
        row.product_name == constants::browser::PRODUCT_NAME_MICROSOFT_EDGE
            && row.browser_family == BrowserFamily::Edge
            && row.install_state == BrowserInventoryInstallState::Installed
            && row.process_id.is_none()
    });
    let row = require_some(row, constants::error::BROWSER_BRIDGE_MAPS_TARGET);

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
        BrowserInventoryGeneratedAtText(
            constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        ),
        &observations,
    );

    let _ = std::fs::remove_dir_all(root);
    let row = read_model.rows.iter().find(|row| {
        row.product_name == constants::browser::PRODUCT_NAME_MICROSOFT_EDGE
            && row.install_state == BrowserInventoryInstallState::Packaged
            && row.executable_path_ref.is_none()
    });
    let row = require_some(row, constants::error::BROWSER_BRIDGE_MAPS_TARGET);

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

fn temp_service_inventory_root() -> TestPathBuf {
    static TEMP_SERVICE_INVENTORY_COUNTER: AtomicU64 = AtomicU64::new(0);

    let sequence = TEMP_SERVICE_INVENTORY_COUNTER.fetch_add(1, Ordering::Relaxed);
    let root = std::env::temp_dir()
        .join(constants::browser::DEVTOOLS_TEST_WINDOWS_BROWSER_INVENTORY_DIR)
        .join(std::process::id().to_string())
        .join(sequence.to_string())
        .join(constants::browser::PATH_SEGMENT_DEFAULT);
    let _ = std::fs::remove_dir_all(&root);
    root
}

fn create_executable_fixture(path: &TestPathBuf) {
    let parent = require_some(path.parent(), constants::error::BROWSER_BRIDGE_MAPS_TARGET);
    require_ok(
        std::fs::create_dir_all(parent),
        constants::error::BROWSER_BRIDGE_MAPS_TARGET,
    );
    require_ok(
        std::fs::write(path, []),
        constants::error::BROWSER_BRIDGE_MAPS_TARGET,
    );
}

fn write_manifest(root: &Path, manifest: &TestStr) {
    if let Some(parent) = root.parent() {
        require_ok(
            std::fs::create_dir_all(parent),
            constants::error::ACTIVITY_CAPTURE_RECORDS,
        );
    }
    require_ok(
        std::fs::create_dir_all(root),
        constants::error::ACTIVITY_CAPTURE_RECORDS,
    );
    require_ok(
        std::fs::write(
            root.join(
                ocentra_parent_agent_protocol::app_game::APP_GAME_WINDOWS_APPX_MANIFEST_FILE_NAME,
            ),
            manifest,
        ),
        constants::error::ACTIVITY_CAPTURE_RECORDS,
    );
}

fn restore_env_var(env_var_name: &TestStr, value: Option<TestOsString>) {
    match value {
        Some(previous) => std::env::set_var(env_var_name, previous),
        None => std::env::remove_var(env_var_name),
    }
}
