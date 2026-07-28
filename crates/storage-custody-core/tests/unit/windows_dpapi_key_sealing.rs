use ocentra_storage_custody_core::windows_dpapi_key_sealing::{
    seal_for_current_windows_user, unseal_for_current_windows_user, DpapiKeySealingContext,
    DpapiKeySealingError, DpapiSealedKey,
};

use crate::support::StorageCustodyTestValueExt;

fn context() -> DpapiKeySealingContext {
    DpapiKeySealingContext {
        trust_subject: "parent-account-opaque-1".to_string(),
        device_ref: "parent-device-opaque-1".to_string(),
        device_role: "trusted-parent".to_string(),
    }
}

#[cfg(windows)]
#[test]
fn windows_dpapi_user_scope_round_trips_after_serialized_restart_boundary() {
    let secret = b"test-only-device-trust-material";
    let sealed = seal_for_current_windows_user(context(), secret).assume_ok();
    assert_ne!(sealed.ciphertext(), secret);

    let persisted = serde_json::to_vec(&sealed).assume_ok();
    assert!(!String::from_utf8_lossy(&persisted).contains("test-only-device-trust-material"));
    let restarted: DpapiSealedKey = serde_json::from_slice(&persisted).assume_ok();

    assert_eq!(
        unseal_for_current_windows_user(&restarted, &context()).assume_ok(),
        secret
    );
}

#[cfg(windows)]
#[test]
fn windows_dpapi_rejects_wrong_subject_device_and_corrupt_ciphertext_without_fallback() {
    let sealed =
        seal_for_current_windows_user(context(), b"test-only-device-trust-material").assume_ok();
    let wrong_subject = DpapiKeySealingContext {
        trust_subject: "parent-account-opaque-2".to_string(),
        ..context()
    };
    let wrong_device = DpapiKeySealingContext {
        device_ref: "parent-device-opaque-2".to_string(),
        ..context()
    };
    assert_eq!(
        unseal_for_current_windows_user(&sealed, &wrong_subject),
        Err(DpapiKeySealingError::BindingMismatch)
    );
    assert_eq!(
        unseal_for_current_windows_user(&sealed, &wrong_device),
        Err(DpapiKeySealingError::BindingMismatch)
    );

    let mut persisted: serde_json::Value = serde_json::to_value(sealed).assume_ok();
    let first_ciphertext_byte = persisted["ciphertext"][0].as_u64().assume_ok();
    persisted["ciphertext"][0] = serde_json::Value::from(first_ciphertext_byte ^ 0xff);
    let corrupted: DpapiSealedKey = serde_json::from_value(persisted).assume_ok();
    assert_eq!(
        unseal_for_current_windows_user(&corrupted, &context()),
        Err(DpapiKeySealingError::UnsealFailed)
    );
}

#[cfg(not(windows))]
#[test]
fn windows_dpapi_stays_unavailable_without_a_plaintext_fallback() {
    assert_eq!(
        seal_for_current_windows_user(context(), b"test-only-device-trust-material"),
        Err(DpapiKeySealingError::PlatformUnavailable)
    );
}
