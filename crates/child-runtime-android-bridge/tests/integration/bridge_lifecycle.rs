use std::{fs, io, io::ErrorKind};

use ocentra_child_runtime::service::{ChildAgentReadiness, ChildAgentServiceError};
use ocentra_child_runtime_android::ffi::runtime::{
    ChildRuntimeAndroidBridge, ChildRuntimeAndroidBridgeError,
};

#[test]
fn bridge_starts_and_reopens_the_real_child_service_and_rejects_a_file_root(
) -> Result<(), Box<dyn std::error::Error>> {
    let temporary = tempfile::tempdir()?;
    let durable_root = temporary.path().join("child-runtime");

    let bridge = ChildRuntimeAndroidBridge::start(&durable_root)?;
    let health = bridge.health()?;
    assert_eq!(health.readiness, ChildAgentReadiness::Ready);
    assert_eq!(health.domain_flow_count, 7);
    assert_eq!(health.durable_root, durable_root);
    assert!(health.durable_root.join("tombstones").is_dir());
    drop(bridge);

    let reopened = ChildRuntimeAndroidBridge::start(&health.durable_root)?;
    let reopened_health = reopened.health()?;
    assert_eq!(reopened_health.readiness, ChildAgentReadiness::Ready);
    assert_eq!(reopened_health.domain_flow_count, 7);
    assert_eq!(reopened_health.durable_root, health.durable_root);
    drop(reopened);

    let invalid_root = temporary.path().join("occupied-by-file");
    fs::write(&invalid_root, b"not a directory")?;
    let error = ChildRuntimeAndroidBridge::start(invalid_root)
        .err()
        .ok_or_else(|| io::Error::other("a file path started the child runtime"))?;
    match error {
        ChildRuntimeAndroidBridgeError::Service(ChildAgentServiceError::Storage(error)) => {
            assert_eq!(error.kind(), ErrorKind::AlreadyExists);
        }
        other => {
            return Err(io::Error::other(format!(
                "expected fail-closed storage error, received {other}"
            ))
            .into());
        }
    }
    Ok(())
}

#[test]
fn handles_are_unique_and_a_stale_owner_cannot_stop_a_reopened_service(
) -> Result<(), Box<dyn std::error::Error>> {
    let temporary = tempfile::tempdir()?;
    let durable_root = temporary.path().join("child-runtime");
    let root = durable_root.to_string_lossy().into_owned();

    let first = ocentra_child_runtime_android::ffi::bridge_lifecycle::start(root.clone());
    let second = ocentra_child_runtime_android::ffi::bridge_lifecycle::start(root.clone());
    assert_ne!(first, 0);
    assert_ne!(second, 0);
    assert_ne!(first, second);

    assert!(ocentra_child_runtime_android::ffi::bridge_lifecycle::stop(
        first
    ));
    assert_eq!(
        ocentra_child_runtime_android::ffi::bridge_health::readiness(second),
        ocentra_child_runtime_android::READINESS_READY
    );
    assert!(ocentra_child_runtime_android::ffi::bridge_lifecycle::stop(
        second
    ));

    let reopened = ocentra_child_runtime_android::ffi::bridge_lifecycle::start(root);
    assert_ne!(reopened, 0);
    assert_ne!(reopened, second);
    assert!(!ocentra_child_runtime_android::ffi::bridge_lifecycle::stop(
        second
    ));
    assert_eq!(
        ocentra_child_runtime_android::ffi::bridge_health::readiness(reopened),
        ocentra_child_runtime_android::READINESS_READY
    );
    assert!(ocentra_child_runtime_android::ffi::bridge_lifecycle::stop(
        reopened
    ));
    Ok(())
}
