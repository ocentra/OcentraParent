#![cfg(not(windows))]

use ocentra_storage_custody_core::windows_device_trust_custody::{
    Error, WindowsDeviceTrustCustody,
};

#[test]
fn unsupported_platform_rejects_before_creating_the_custody_root() {
    let root =
        std::env::temp_dir().join(format!("ocentra-wp02-unsupported-{}", std::process::id()));
    let _cleanup = std::fs::remove_dir_all(&root);

    assert!(matches!(
        WindowsDeviceTrustCustody::open(&root),
        Err(Error::Platform)
    ));
    assert!(!root.exists());
}
