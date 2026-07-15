use std::path::{Path, PathBuf};

use ocentra_parent_agent_protocol::browser::{BrowserChannel, BrowserFamily};
use ocentra_parent_agent_protocol::browser_managed::{
    BrowserManagedProfileLifecycleState, BrowserUnmanagedDetectionConfidence,
    BrowserUnmanagedDetectionReason, BrowserUnmanagedProcessKind,
};
use ocentra_parent_agent_protocol::constants;

use crate::{
    create_or_repair_managed_browser_profile_store, delete_managed_browser_profile_store,
    installed_managed_browser_candidates, launch_managed_browser,
    load_managed_browser_profile_store, managed_browser_executable_identity,
    managed_browser_launch_plan, reserve_managed_browser_bridge_port,
    test_text::{test_err as err, test_ok as ok, test_some as some, TestResult, TestText},
    unmanaged_browser_processes, BrowserManagedLaunchConfig, BrowserManagedLaunchError,
    BrowserManagedProfileStoreConfig, BrowserManagedProfileStoreError, ProcessObservation,
};

#[test]
fn managed_browser_launch_plan_uses_owned_profile_and_loopback_bridge() -> TestResult {
    let config = BrowserManagedLaunchConfig {
        executable_path: PathBuf::from(constants::browser::DEVTOOLS_TEST_MSEDGE_BETA_PATH),
        profile_dir: PathBuf::from(constants::browser::PROFILE_ID_DEV),
        bridge_port: constants::browser::DEVTOOLS_TEST_BRIDGE_PORT,
    };

    let plan = ok(
        managed_browser_launch_plan(config),
        constants::error::BROWSER_BRIDGE_MAPS_TARGET,
    )?;

    assert_eq!(
        plan.profile_path_ref,
        constants::browser::PROFILE_PATH_REF_MANAGED
    );
    assert_eq!(
        plan.bridge_endpoint_ref,
        constants::browser::BRIDGE_ENDPOINT_REF_LOOPBACK_DEVTOOLS
    );
    assert_eq!(plan.browser_family, BrowserFamily::Edge);
    assert_eq!(plan.browser_channel, BrowserChannel::Beta);
    assert!(plan.args.iter().any(|arg| {
        arg.contains(constants::browser::CHROMIUM_ARG_REMOTE_DEBUGGING_ADDRESS_PREFIX)
            && arg.contains(constants::browser::CHROMIUM_REMOTE_DEBUGGING_LOOPBACK)
    }));
    assert!(plan
        .args
        .iter()
        .any(|arg| arg.contains(constants::browser::CHROMIUM_ARG_REMOTE_DEBUGGING_PORT_PREFIX)));
    assert!(plan.args.iter().any(|arg| {
        arg.contains(constants::browser::CHROMIUM_ARG_USER_DATA_DIR_PREFIX)
            && arg.contains(constants::browser::PROFILE_ID_DEV)
    }));
    assert!(plan.args.iter().any(|arg| {
        arg.contains(constants::browser::CHROMIUM_ARG_PROFILE_DIRECTORY_PREFIX)
            && arg.contains(constants::browser::PROFILE_DIRECTORY_MANAGED_CHILD)
    }));

    Ok(())
}

#[test]
fn managed_browser_bridge_port_reservation_is_loopback_and_nonzero() -> TestResult {
    let reservation = ok(
        reserve_managed_browser_bridge_port(),
        constants::error::BROWSER_BRIDGE_MAPS_TARGET,
    )?;

    assert!(reservation.endpoint.ip().is_loopback());
    assert_eq!(reservation.endpoint.port(), reservation.bridge_port);
    assert_ne!(
        reservation.bridge_port,
        constants::browser::DEVTOOLS_PORT_UNRESERVED
    );

    Ok(())
}

#[test]
fn managed_browser_launch_plan_rejects_unreserved_bridge_port() -> TestResult {
    let config = BrowserManagedLaunchConfig {
        executable_path: PathBuf::from(constants::browser::EXECUTABLE_CHROME_WINDOWS),
        profile_dir: PathBuf::from(constants::browser::PROFILE_ID_DEV),
        bridge_port: constants::browser::DEVTOOLS_PORT_UNRESERVED,
    };

    let error = err(
        managed_browser_launch_plan(config),
        constants::error::BROWSER_BRIDGE_REJECTS_INVALID_URL,
    )?;

    assert_eq!(error, BrowserManagedLaunchError::BridgePortUnavailable);
    assert_eq!(
        error.reason(),
        constants::value::MANAGED_BROWSER_BRIDGE_PORT_UNAVAILABLE
    );

    Ok(())
}

#[test]
fn managed_browser_launch_reports_failed_spawn_without_default_profile_attach() -> TestResult {
    let missing_chrome = profile_store_root(constants::browser::PROFILE_STORE_TEST_LAUNCH_SUFFIX)
        .join(constants::browser::EXECUTABLE_CHROME_WINDOWS);
    let config = BrowserManagedLaunchConfig {
        executable_path: missing_chrome,
        profile_dir: PathBuf::from(constants::browser::PROFILE_ID_DEV),
        bridge_port: constants::browser::DEVTOOLS_TEST_BRIDGE_PORT,
    };

    let error = err(
        launch_managed_browser(config),
        constants::error::BROWSER_BRIDGE_MAPS_TARGET,
    )?;

    assert_eq!(error, BrowserManagedLaunchError::Io);
    assert_eq!(
        error.reason(),
        constants::value::MANAGED_BROWSER_LAUNCH_ERROR
    );

    Ok(())
}

#[test]
fn managed_browser_profile_store_creates_and_reloads_redacted_metadata() -> TestResult {
    let root = profile_store_root(constants::browser::PROFILE_STORE_TEST_CREATE_SUFFIX);
    let _ = std::fs::remove_dir_all(&root);
    let first = ok(
        create_or_repair_managed_browser_profile_store(&profile_store_config(
            root.clone(),
            constants::activity_store::TEST_FIRST_OBSERVED_AT,
        )),
        constants::error::BROWSER_BRIDGE_MAPS_TARGET,
    )?;
    let second = ok(
        load_managed_browser_profile_store(&profile_store_config(
            root.clone(),
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        )),
        constants::error::BROWSER_BRIDGE_MAPS_TARGET,
    )?;

    assert!(first.profile_dir.is_dir());
    assert!(first.metadata_path.is_file());
    assert_eq!(
        first.entry.lifecycle_state,
        BrowserManagedProfileLifecycleState::Ready
    );
    assert_eq!(
        second.entry.created_at,
        constants::activity_store::TEST_FIRST_OBSERVED_AT
    );
    assert_eq!(
        second.entry.profile_path_ref,
        constants::browser::PROFILE_PATH_REF_MANAGED
    );
    assert_eq!(
        second.entry.profile_root_ref,
        constants::browser::PROFILE_ROOT_REF_MANAGED
    );

    let _ = std::fs::remove_dir_all(root);

    Ok(())
}

#[test]
fn managed_browser_profile_store_reports_missing_and_repairs_profile_dir() -> TestResult {
    let root = profile_store_root(constants::browser::PROFILE_STORE_TEST_MISSING_SUFFIX);
    let _ = std::fs::remove_dir_all(&root);
    let created = ok(
        create_or_repair_managed_browser_profile_store(&profile_store_config(
            root.clone(),
            constants::activity_store::TEST_FIRST_OBSERVED_AT,
        )),
        constants::error::BROWSER_BRIDGE_MAPS_TARGET,
    )?;
    ok(
        std::fs::remove_dir_all(&created.profile_dir),
        constants::error::BROWSER_BRIDGE_MAPS_TARGET,
    )?;

    let missing = ok(
        load_managed_browser_profile_store(&profile_store_config(
            root.clone(),
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        )),
        constants::error::BROWSER_BRIDGE_MAPS_TARGET,
    )?;
    let repaired = ok(
        create_or_repair_managed_browser_profile_store(&profile_store_config(
            root.clone(),
            constants::activity_store::TEST_THIRD_OBSERVED_AT,
        )),
        constants::error::BROWSER_BRIDGE_MAPS_TARGET,
    )?;

    assert_eq!(
        missing.entry.lifecycle_state,
        BrowserManagedProfileLifecycleState::Missing
    );
    assert_eq!(
        missing.entry.missing_since,
        Some(constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string())
    );
    assert_eq!(
        repaired.entry.lifecycle_state,
        BrowserManagedProfileLifecycleState::Ready
    );
    assert_eq!(
        repaired.entry.repaired_at,
        Some(constants::activity_store::TEST_THIRD_OBSERVED_AT.to_string())
    );
    assert!(repaired.profile_dir.is_dir());

    let _ = std::fs::remove_dir_all(root);

    Ok(())
}

#[test]
fn managed_browser_profile_store_delete_preserves_redacted_metadata() -> TestResult {
    let root = profile_store_root(constants::browser::PROFILE_STORE_TEST_DELETE_SUFFIX);
    let _ = std::fs::remove_dir_all(&root);
    let created = ok(
        create_or_repair_managed_browser_profile_store(&profile_store_config(
            root.clone(),
            constants::activity_store::TEST_FIRST_OBSERVED_AT,
        )),
        constants::error::BROWSER_BRIDGE_MAPS_TARGET,
    )?;
    let deleted = ok(
        delete_managed_browser_profile_store(&profile_store_config(
            root.clone(),
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        )),
        constants::error::BROWSER_BRIDGE_MAPS_TARGET,
    )?;

    assert!(!created.profile_dir.exists());
    assert_eq!(
        deleted.entry.lifecycle_state,
        BrowserManagedProfileLifecycleState::Deleted
    );
    assert_eq!(
        deleted.entry.deleted_at,
        Some(constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string())
    );
    assert_eq!(
        deleted.entry.repair_reason,
        Some(constants::browser::PROFILE_STORE_REASON_DELETED.to_string())
    );
    assert!(deleted.metadata_path.is_file());

    let _ = std::fs::remove_dir_all(root);

    Ok(())
}

#[test]
fn managed_browser_profile_store_rejects_default_and_unowned_paths() -> TestResult {
    let default_profile_error = err(
        create_or_repair_managed_browser_profile_store(&BrowserManagedProfileStoreConfig {
            profile_root_dir: PathBuf::from(constants::browser::PATH_SEGMENT_USER_DATA)
                .join(constants::browser::PATH_SEGMENT_DEFAULT),
            profile_id: constants::browser::PROFILE_ID_DEV.to_string(),
            profile_scope_id: constants::browser::PROFILE_SCOPE_ID_DEV.to_string(),
            device_id: constants::browser::PROFILE_STORE_TEST_DEVICE_ID.to_string(),
            browser_family: BrowserFamily::Chrome,
            browser_channel: BrowserChannel::Stable,
            policy_revision: constants::browser::PROFILE_POLICY_REVISION_DEV.to_string(),
            now: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        }),
        constants::error::BROWSER_BRIDGE_REJECTS_INVALID_URL,
    )?;
    let unowned_profile_error = err(
        create_or_repair_managed_browser_profile_store(&BrowserManagedProfileStoreConfig {
            profile_root_dir: profile_store_root(
                constants::browser::PROFILE_STORE_TEST_REJECT_SUFFIX,
            )
            .to_path_buf(),
            profile_id: constants::browser::DEVTOOLS_TEST_UNOWNED_PROFILE_DIR.to_string(),
            profile_scope_id: constants::browser::PROFILE_SCOPE_ID_DEV.to_string(),
            device_id: constants::browser::PROFILE_STORE_TEST_DEVICE_ID.to_string(),
            browser_family: BrowserFamily::Chrome,
            browser_channel: BrowserChannel::Stable,
            policy_revision: constants::browser::PROFILE_POLICY_REVISION_DEV.to_string(),
            now: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        }),
        constants::error::BROWSER_BRIDGE_REJECTS_INVALID_URL,
    )?;

    assert_eq!(
        default_profile_error,
        BrowserManagedProfileStoreError::DefaultProfileRejected
    );
    assert_eq!(
        default_profile_error.reason(),
        constants::value::MANAGED_BROWSER_INVALID_PROFILE
    );
    assert_eq!(
        unowned_profile_error,
        BrowserManagedProfileStoreError::UnownedProfileRejected
    );

    Ok(())
}

#[test]
fn managed_browser_launch_plan_rejects_default_browser_profile() -> TestResult {
    let config = BrowserManagedLaunchConfig {
        executable_path: PathBuf::from(constants::browser::EXECUTABLE_CHROME_WINDOWS),
        profile_dir: PathBuf::from(constants::browser::PATH_SEGMENT_USER_DATA)
            .join(constants::browser::PATH_SEGMENT_DEFAULT),
        bridge_port: constants::browser::DEVTOOLS_TEST_BRIDGE_PORT,
    };

    let error = err(
        managed_browser_launch_plan(config),
        constants::error::BROWSER_BRIDGE_REJECTS_INVALID_URL,
    )?;

    assert_eq!(error, BrowserManagedLaunchError::DefaultProfileRejected);
    assert_eq!(
        error.reason(),
        constants::value::MANAGED_BROWSER_INVALID_PROFILE
    );

    Ok(())
}

#[test]
fn managed_browser_launch_plan_rejects_unowned_profile_path() -> TestResult {
    let config = BrowserManagedLaunchConfig {
        executable_path: PathBuf::from(constants::browser::EXECUTABLE_CHROME_WINDOWS),
        profile_dir: PathBuf::from(constants::browser::DEVTOOLS_TEST_UNOWNED_PROFILE_DIR),
        bridge_port: constants::browser::DEVTOOLS_TEST_BRIDGE_PORT,
    };

    let error = err(
        managed_browser_launch_plan(config),
        constants::error::BROWSER_BRIDGE_REJECTS_INVALID_URL,
    )?;

    assert_eq!(error, BrowserManagedLaunchError::UnownedProfileRejected);
    assert_eq!(
        error.reason(),
        constants::value::MANAGED_BROWSER_INVALID_PROFILE
    );

    Ok(())
}

#[test]
fn managed_browser_launch_plan_rejects_unsupported_browser_executable() -> TestResult {
    let config = BrowserManagedLaunchConfig {
        executable_path: PathBuf::from(
            constants::browser::DEVTOOLS_TEST_UNSUPPORTED_EXECUTABLE_PATH,
        ),
        profile_dir: PathBuf::from(constants::browser::PROFILE_ID_DEV),
        bridge_port: constants::browser::DEVTOOLS_TEST_BRIDGE_PORT,
    };

    let error = err(
        managed_browser_launch_plan(config),
        constants::error::BROWSER_BRIDGE_REJECTS_INVALID_URL,
    )?;

    assert_eq!(error, BrowserManagedLaunchError::UnsupportedBrowser);
    assert_eq!(
        error.reason(),
        constants::value::MANAGED_BROWSER_UNSUPPORTED_EXECUTABLE
    );

    Ok(())
}

#[test]
fn managed_browser_executable_identity_classifies_chrome_and_edge_channels() {
    let edge_identity = managed_browser_executable_identity(
        PathBuf::from(constants::browser::DEVTOOLS_TEST_MSEDGE_BETA_PATH).as_path(),
    );
    let chrome_identity = managed_browser_executable_identity(
        PathBuf::from(constants::browser::EXECUTABLE_CHROME_WINDOWS).as_path(),
    );

    assert_eq!(edge_identity.browser_family, BrowserFamily::Edge);
    assert_eq!(edge_identity.browser_channel, BrowserChannel::Beta);
    assert!(edge_identity.supports_managed_cdp);
    assert_eq!(chrome_identity.browser_family, BrowserFamily::Chrome);
    assert_eq!(chrome_identity.browser_channel, BrowserChannel::Stable);
    assert!(chrome_identity.supports_managed_cdp);
}

#[test]
fn unmanaged_browser_processes_detects_supported_unmanaged_browser_processes() {
    let observations = vec![
        ProcessObservation {
            pid: constants::browser::DEVTOOLS_TEST_UNMANAGED_PROCESS_ID,
            name: constants::browser::EXECUTABLE_CHROME_WINDOWS.to_string(),
            executable_path: None,
        },
        ProcessObservation {
            pid: constants::browser::PROCESS_ID_UNKNOWN,
            name: constants::browser::DEVTOOLS_TEST_UNSUPPORTED_EXECUTABLE_PATH.to_string(),
            executable_path: None,
        },
    ];

    let unmanaged =
        unmanaged_browser_processes(&observations, Some(constants::browser::PROCESS_ID_UNKNOWN));

    assert_eq!(unmanaged.len(), 1);
    assert_eq!(
        unmanaged[0].process_id,
        constants::browser::DEVTOOLS_TEST_UNMANAGED_PROCESS_ID
    );
    assert_eq!(unmanaged[0].browser_family, BrowserFamily::Chrome);
    assert_eq!(unmanaged[0].browser_channel, BrowserChannel::Stable);
    assert_eq!(
        unmanaged[0].process_kind,
        BrowserUnmanagedProcessKind::SupportedBrowser
    );
    assert_eq!(
        unmanaged[0].detection_confidence,
        BrowserUnmanagedDetectionConfidence::High
    );
    assert_eq!(
        unmanaged[0].detection_reason,
        BrowserUnmanagedDetectionReason::SupportedBrowserOutsideManagedSession
    );
}

#[test]
fn unmanaged_browser_processes_detects_unsupported_and_unknown_browser_like_processes() {
    let observations = vec![
        ProcessObservation {
            pid: constants::browser::DEVTOOLS_TEST_UNMANAGED_PROCESS_ID,
            name: constants::browser::EXECUTABLE_FIREFOX_WINDOWS.to_string(),
            executable_path: None,
        },
        ProcessObservation {
            pid: constants::browser::DEVTOOLS_TEST_UNMANAGED_PROCESS_ID + 1,
            name: constants::browser::DEVTOOLS_TEST_BROWSER_LIKE_PROCESS.to_string(),
            executable_path: None,
        },
    ];

    let unmanaged = unmanaged_browser_processes(&observations, None);

    assert_eq!(unmanaged.len(), 2);
    assert_eq!(
        unmanaged[0].process_kind,
        BrowserUnmanagedProcessKind::UnsupportedBrowser
    );
    assert_eq!(
        unmanaged[1].process_kind,
        BrowserUnmanagedProcessKind::UnknownBrowserLike
    );
    assert_eq!(
        unmanaged[1].detection_reason,
        BrowserUnmanagedDetectionReason::BrowserLikeProcess
    );
}

#[test]
fn installed_managed_browser_candidates_require_existing_supported_executables() -> TestResult {
    let root = std::env::temp_dir()
        .join(constants::browser::DEVTOOLS_TEST_INSTALLED_BROWSER_DIR)
        .join(std::process::id().to_string());
    let edge = root
        .join(constants::browser::PATH_SEGMENT_EDGE_BETA)
        .join(constants::browser::PATH_SEGMENT_APPLICATION)
        .join(constants::browser::EXECUTABLE_MSEDGE_WINDOWS);
    let unsupported = root
        .join(constants::browser::PATH_SEGMENT_APPLICATION)
        .join(constants::browser::DEVTOOLS_TEST_UNSUPPORTED_EXECUTABLE_PATH);
    ok(
        std::fs::create_dir_all(some(
            edge.parent(),
            constants::error::BROWSER_BRIDGE_MAPS_TARGET,
        )?),
        constants::error::BROWSER_BRIDGE_MAPS_TARGET,
    )?;
    ok(
        std::fs::write(&edge, []),
        constants::error::BROWSER_BRIDGE_MAPS_TARGET,
    )?;
    ok(
        std::fs::create_dir_all(some(
            unsupported.parent(),
            constants::error::BROWSER_BRIDGE_MAPS_TARGET,
        )?),
        constants::error::BROWSER_BRIDGE_MAPS_TARGET,
    )?;
    ok(
        std::fs::write(&unsupported, []),
        constants::error::BROWSER_BRIDGE_MAPS_TARGET,
    )?;

    let candidates = installed_managed_browser_candidates(&[
        edge.clone(),
        unsupported,
        root.join(constants::browser::EXECUTABLE_CHROME_WINDOWS),
    ]);

    assert_eq!(candidates.len(), 1);
    assert_eq!(candidates[0].executable_path, edge);
    assert_eq!(candidates[0].browser_family, BrowserFamily::Edge);
    assert_eq!(candidates[0].browser_channel, BrowserChannel::Beta);

    let _ = std::fs::remove_dir_all(root);

    Ok(())
}

fn profile_store_root(suffix: impl std::fmt::Display) -> TestText {
    let mut root = std::env::temp_dir();
    root.push(constants::browser::PROFILE_STORE_TEST_ROOT_DIR);
    root.push(suffix.to_string());
    root.push(std::process::id().to_string());
    TestText::from_display(root.display())
}

fn profile_store_config(
    root: impl AsRef<Path>,
    now: impl std::fmt::Display,
) -> BrowserManagedProfileStoreConfig {
    BrowserManagedProfileStoreConfig {
        profile_root_dir: root.as_ref().to_path_buf(),
        profile_id: constants::browser::PROFILE_ID_DEV.to_string(),
        profile_scope_id: constants::browser::PROFILE_SCOPE_ID_DEV.to_string(),
        device_id: constants::browser::PROFILE_STORE_TEST_DEVICE_ID.to_string(),
        browser_family: BrowserFamily::Chrome,
        browser_channel: BrowserChannel::Stable,
        policy_revision: constants::browser::PROFILE_POLICY_REVISION_DEV.to_string(),
        now: now.to_string(),
    }
}
