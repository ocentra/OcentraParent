use std::ffi::OsString as TestOsString;
use std::path::Path;
use std::path::PathBuf as TestPathBuf;
use std::primitive::str as TestStr;
use std::string::String as TestString;
use std::sync::atomic::{AtomicU64, Ordering};

use ocentra_parent_agent_core::browser_managed_discovery::BrowserUnmanagedProcessObservation;
use ocentra_parent_agent_core::browser_windows_inventory::windows_browser_inventory_observations;
use ocentra_parent_agent_core::browser_windows_package_inventory::windows_browser_package_observations;
use ocentra_parent_agent_core::browser_windows_package_source::live_windows_browser_package_entries_from_roots;
use ocentra_parent_agent_core::process_capture::ProcessObservation;
use ocentra_parent_agent_protocol::browser::{BrowserChannel, BrowserFamily};
use ocentra_parent_agent_protocol::browser_inventory::{
    BrowserExactUrlCapability, BrowserInventoryInstallState, BrowserManagementTier,
    BrowserSupportTier,
};
use ocentra_parent_agent_protocol::browser_managed::BrowserManagedProfileLifecycleState;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentEventName, AgentMessageTarget, AgentPeer,
    AgentPeerRole, AgentRoute,
};
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;
use ocentra_parent_agent_service::test_support::{
    browser_inventory_read_model_from_service_defaults_for_test, default_browser_policy_for_test,
    handle_local_command_text_for_test,
};

use crate::{
    browser_inventory_read_model::{
        browser_inventory_read_model_from_windows_inventory, BrowserInventoryGeneratedAtText,
    },
    browser_payload::{browser_inventory_read_model_payload, browser_managed_status_payload},
    browser_policy_compiler::compile_browser_policy,
    browser_policy_compiler_assessment::compile_rule_assessment,
    browser_policy_runtime_support::{
        accepted_response, base_revision_matches, default_revision_id, next_audit_event_id,
        next_revision_id, preview_revision_id, rejected_response,
    },
    browser_policy_store::{
        browser_policy_store_path_from_env, read_browser_policy_state, write_browser_policy_state,
        BrowserPolicyStoredState,
    },
    browser_runtime_paths::{managed_browser_executable_path, managed_browser_profile_store},
    browser_runtime_status::{
        missing_browser_status, profile_missing_status, status_with_error, unmanaged_browser_status,
    },
    test_invariants::{require_ok, require_some, serialize_test_json},
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
fn browser_inventory_support_helpers_link_status_paths_and_policy_modules() {
    let checked_at = constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string();
    let missing = missing_browser_status(checked_at.clone());
    let profile_missing = profile_missing_status(checked_at.clone());
    let unmanaged = unmanaged_browser_status(
        checked_at.clone(),
        BrowserUnmanagedProcessObservation {
            process_id: constants::browser::DEVTOOLS_TEST_UNMANAGED_PROCESS_ID,
            process_name: constants::browser::EXECUTABLE_CHROME_WINDOWS.to_string(),
            executable_path_ref: None,
            signature_ref: None,
            process_hash_ref: None,
            browser_family: BrowserFamily::Chrome,
            browser_channel: BrowserChannel::Stable,
            process_kind: ocentra_parent_agent_protocol::browser_managed::BrowserUnmanagedProcessKind::SupportedBrowser,
            detection_confidence: ocentra_parent_agent_protocol::browser_managed::BrowserUnmanagedDetectionConfidence::High,
            detection_reason: ocentra_parent_agent_protocol::browser_managed::BrowserUnmanagedDetectionReason::SupportedBrowserOutsideManagedSession,
        },
    );
    let errored_checked_at = checked_at.clone();
    let errored = status_with_error(
        errored_checked_at,
        constants::value::MANAGED_BROWSER_LAUNCH_ERROR,
    );
    let payload = browser_managed_status_payload(&errored);

    browser_inventory_status_helpers_assertions(
        missing.managed_state.as_protocol_str().to_string(),
        profile_missing
            .profile_lifecycle_state
            .as_ref()
            .map(BrowserManagedProfileLifecycleState::as_protocol_str)
            .map(str::to_string),
        unmanaged
            .unmanaged_process_kind
            .as_ref()
            .map(|value| value.as_protocol_str().to_string()),
        errored.managed_state.as_protocol_str().to_string(),
        payload.get(constants::field::MANAGED_STATE).cloned(),
    );
}

#[tokio::test]
async fn browser_inventory_support_helpers_link_policy_store_and_runtime_paths() {
    browser_inventory_policy_store_and_runtime_paths_are_linked().await;
}

async fn browser_inventory_policy_store_and_runtime_paths_are_linked() {
    let _guard = crate::activity_report_env_lock::REPORT_ENV_LOCK
        .lock()
        .await;
    let _ = browser_policy_store_path_from_env();
    let policy =
        default_browser_policy_for_test(crate::test_support::default_browser_policy_id_for_test());
    let effective_policy = require_ok(
        compile_browser_policy(
            &policy,
            crate::browser_policy_compiler::BrowserPolicyCompileRequest {
                revision_id: constants::browser_policy::REVISION_ID,
                compiled_at: constants::browser_policy::TEST_SENT_AT,
            },
        ),
        constants::error::AGENT_EVENT_SERIALIZES,
    );
    let assessment = compile_rule_assessment(
        &policy,
        ocentra_parent_agent_protocol::BrowserPolicyUrlTargetType::DomainOrigin,
        ocentra_parent_agent_protocol::BrowserPolicyRuleAction::Block,
    );
    let capability_registry = crate::browser_policy_compiler::browser_policy_capability_registry(
        crate::browser_policy_compiler::BrowserPolicyCapabilityRegistryRequest {
            generated_at: constants::browser_policy::TEST_SENT_AT,
        },
    );
    let base_state = BrowserPolicyStoredState::empty();
    assert_browser_policy_revision_helpers(&base_state);
    let roundtrip_path = temp_service_inventory_root().join("browser-policy-store.json");
    require_ok(
        write_browser_policy_state(&roundtrip_path, &base_state).await,
        constants::error::AGENT_EVENT_SERIALIZES,
    );
    let roundtrip = require_ok(
        read_browser_policy_state(&roundtrip_path).await,
        constants::error::AGENT_EVENT_SERIALIZES,
    );

    let runtime_root = temp_service_inventory_root();
    let previous_browser_path = std::env::var_os(constants::env_var::MANAGED_BROWSER_EXECUTABLE);
    let previous_profile_dir = std::env::var_os(constants::env_var::MANAGED_BROWSER_PROFILE_DIR);
    let expected_browser_path = runtime_root.join("managed-browser.exe");
    let expected_profile_root = runtime_root.join("managed-browser-profile");
    std::env::set_var(
        constants::env_var::MANAGED_BROWSER_EXECUTABLE,
        &expected_browser_path,
    );
    std::env::set_var(
        constants::env_var::MANAGED_BROWSER_PROFILE_DIR,
        &expected_profile_root,
    );
    let browser_path = managed_browser_executable_path().map(TestPathBuf::from);
    let profile_store = managed_browser_profile_store().map_err(|error| error.0);
    restore_env_var(
        constants::env_var::MANAGED_BROWSER_EXECUTABLE,
        previous_browser_path,
    );
    restore_env_var(
        constants::env_var::MANAGED_BROWSER_PROFILE_DIR,
        previous_profile_dir,
    );

    assert_eq!(roundtrip, base_state);
    assert_runtime_path_and_response_helpers(
        &policy,
        effective_policy,
        &assessment.compile_note,
        &capability_registry.capabilities,
        browser_path,
        profile_store,
        expected_browser_path,
    );
    let _ = std::fs::remove_dir_all(runtime_root);
}

fn assert_browser_policy_revision_helpers(base_state: &BrowserPolicyStoredState) {
    require_ok(
        base_revision_matches(base_state, None),
        constants::error::AGENT_EVENT_SERIALIZES,
    );
    assert!(next_revision_id(base_state).starts_with(constants::browser_policy::REVISION_PREFIX));
    assert!(next_audit_event_id(base_state).starts_with(constants::browser_policy::AUDIT_PREFIX));
    assert_eq!(
        default_revision_id(),
        format!(
            "{}{}",
            constants::browser_policy::REVISION_PREFIX,
            constants::browser_policy::UPDATE_KIND_GET
        )
    );
    assert_eq!(
        preview_revision_id(),
        format!(
            "{}{}",
            constants::browser_policy::REVISION_PREFIX,
            constants::browser_policy::UPDATE_KIND_PREVIEW
        )
    );
}

fn assert_runtime_path_and_response_helpers(
    policy: &ocentra_parent_agent_protocol::browser_policy::BrowserPolicyDocument,
    effective_policy: ocentra_parent_agent_protocol::browser_policy::BrowserPolicyEffectiveDocument,
    compile_note: &TestStr,
    capabilities: &[ocentra_parent_agent_protocol::browser_policy::BrowserPolicyCapability],
    browser_path: Option<TestPathBuf>,
    profile_store: Result<
        ocentra_parent_agent_core::browser_managed_session::BrowserManagedProfileStoreRecord,
        &'static TestStr,
    >,
    expected_browser_path: TestPathBuf,
) {
    assert_eq!(browser_path, Some(expected_browser_path));
    assert_eq!(
        require_ok(profile_store, constants::error::AGENT_EVENT_SERIALIZES)
            .entry
            .profile_id,
        constants::browser::PROFILE_ID_DEV
    );
    assert_eq!(
        accepted_response(
            constants::browser_policy::REQUEST_ID.to_string(),
            ocentra_parent_agent_protocol::browser_policy::BrowserPolicyUpdateKind::Preview,
            policy.clone(),
            effective_policy,
            None,
            "accepted",
            constants::browser_policy::TEST_SENT_AT,
        )
        .status,
        ocentra_parent_agent_protocol::browser_policy::BrowserPolicyUpdateStatus::Accepted
    );
    assert_eq!(
        rejected_response(
            constants::browser_policy::REQUEST_ID.to_string(),
            ocentra_parent_agent_protocol::browser_policy::BrowserPolicyUpdateKind::Patch,
            ocentra_parent_agent_protocol::browser_policy::BrowserPolicyRejectionReason::RevisionNotFound,
            "rejected",
            constants::browser_policy::TEST_SENT_AT,
        )
        .status,
        ocentra_parent_agent_protocol::browser_policy::BrowserPolicyUpdateStatus::Rejected
    );
    assert_eq!(capabilities.is_empty(), compile_note.is_empty());
}

fn browser_inventory_status_helpers_assertions(
    missing_state: TestString,
    profile_missing_state: Option<TestString>,
    unmanaged_kind: Option<TestString>,
    errored_state: TestString,
    payload_state: Option<LogFieldValue>,
) {
    assert_eq!(
        missing_state,
        constants::browser::MANAGED_STATE_NOT_INSTALLED
    );
    assert_eq!(
        profile_missing_state,
        Some(constants::browser::PROFILE_STORE_LIFECYCLE_MISSING.to_string())
    );
    assert_eq!(
        unmanaged_kind,
        Some(constants::browser::UNMANAGED_PROCESS_KIND_SUPPORTED_BROWSER.to_string())
    );
    assert_eq!(errored_state, constants::browser::MANAGED_STATE_ERROR);
    assert_eq!(
        payload_state,
        Some(LogFieldValue::String(
            constants::browser::MANAGED_STATE_ERROR.to_string()
        ))
    );
}

#[tokio::test]
async fn browser_inventory_read_model_command_reports_replayable_service_event() {
    let event = handle_local_command_text_for_test(crate::test_text::TestText::from_display(
        serialize_test_json(&inventory_command()),
    ))
    .await;

    assert_eq!(
        event.event,
        AgentEventName::AgentBrowserInventoryReadModelReported
    );
    assert!(event
        .event_id
        .starts_with(constants::event_id::BROWSER_INVENTORY_READ_MODEL_REPORTED));
    assert!(matches!(
        crate::test_invariants::log_field(
            &event.payload,
            constants::field::RETURNED,
            constants::error::AGENT_EVENT_SERIALIZES,
        ),
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

    let read_model = browser_inventory_read_model_from_service_defaults_for_test(
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
