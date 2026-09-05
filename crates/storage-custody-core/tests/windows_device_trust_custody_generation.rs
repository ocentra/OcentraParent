#![cfg(windows)]

use ocentra_storage_custody_core::windows_device_trust_custody::{
    Error, WindowsDeviceTrustCustody,
};
use sha2::{Digest, Sha256};
use std::fs;
use winreg::{enums::HKEY_CURRENT_USER, RegKey};

#[test]
fn revoking_the_latest_binding_preserves_generation_for_another_active_binding(
) -> Result<(), String> {
    let root = std::env::temp_dir().join(format!(
        "ocentra-wp02-multiple-active-bindings-{}",
        std::process::id()
    ));
    let _cleanup = fs::remove_dir_all(&root);
    WindowsDeviceTrustCustody::open(&root).map_err(|error| format!("first open: {error:?}"))?;
    let generation = install_generation(&root)?;
    let first_binding =
        write_active_record(&root, &generation, "family-a", "account-a", "device-a")?;
    let latest_binding =
        write_active_record(&root, &generation, "family-b", "account-b", "device-b")?;
    set_sealed_install_generation(&root, &generation, &hex(Sha256::digest(&latest_binding)))?;

    remove_active_record(&root, &latest_binding)?;
    WindowsDeviceTrustCustody::open(&root)
        .map_err(|error| format!("open after latest binding revocation: {error:?}"))?;
    assert_eq!(install_generation(&root)?, generation);

    let _cleanup = remove_active_record(&root, &first_binding);
    let _cleanup = remove_install_generation(&root);
    let _cleanup = fs::remove_dir_all(root);
    Ok(())
}

#[test]
fn invalid_sealed_binding_anchor_rotates_to_a_fresh_generation() -> Result<(), String> {
    let root = std::env::temp_dir().join(format!(
        "ocentra-wp02-invalid-sealed-binding-{}",
        std::process::id()
    ));
    let _cleanup = fs::remove_dir_all(&root);
    let custody = WindowsDeviceTrustCustody::open(&root)
        .map_err(|error| format!("open initial custody: {error:?}"))?;
    let initial_generation = install_generation(&root)?;
    set_sealed_install_generation(&root, &initial_generation, "a")?;

    let reopened = WindowsDeviceTrustCustody::open(&root)
        .map_err(|error| format!("reopen after invalid binding anchor: {error:?}"))?;
    assert_ne!(install_generation(&root)?, initial_generation);

    let _cleanup = remove_install_generation(&root);
    let _cleanup = fs::remove_dir_all(root);
    drop(reopened);
    drop(custody);
    Ok(())
}

#[test]
fn over_bound_active_record_scan_fails_closed_during_public_custody_open() -> Result<(), String> {
    const EXPECTED_ACTIVE_RECORD_SCAN_LIMIT: usize = 1024;

    let root = std::env::temp_dir().join(format!(
        "ocentra-wp02-record-scan-bound-{}",
        std::process::id()
    ));
    let _cleanup = fs::remove_dir_all(&root);
    let custody = WindowsDeviceTrustCustody::open(&root)
        .map_err(|error| format!("open initial custody: {error:?}"))?;
    let generation = install_generation(&root)?;
    set_sealed_install_generation(&root, &generation, &"a".repeat(64))?;
    for index in 0..=EXPECTED_ACTIVE_RECORD_SCAN_LIMIT {
        fs::write(root.join(format!("{index}.sealed")), b"{}")
            .map_err(|error| format!("write bounded-scan fixture {index}: {error}"))?;
    }
    drop(custody);

    assert!(matches!(
        WindowsDeviceTrustCustody::open(&root),
        Err(Error::Platform)
    ));

    let _cleanup = remove_install_generation(&root);
    let _cleanup = fs::remove_dir_all(root);
    Ok(())
}

fn write_active_record(
    root: &std::path::Path,
    generation: &str,
    family: &str,
    account: &str,
    device: &str,
) -> Result<Vec<u8>, String> {
    let binding = binding(family, account, device, generation);
    let epoch = [17_u8; 32];
    let protected_epoch =
        windows_dpapi::encrypt_data(&epoch, windows_dpapi::Scope::User, Some(&binding))
            .map_err(|error| format!("protect epoch: {error}"))?;
    let ciphertext =
        windows_dpapi::encrypt_data(&[29_u8; 32], windows_dpapi::Scope::User, Some(&binding))
            .map_err(|error| format!("protect record material: {error}"))?;
    let key = RegKey::predef(HKEY_CURRENT_USER)
        .create_subkey("Software\\Ocentra\\DeviceTrust\\Epochs")
        .map_err(|error| format!("open epoch registry key: {error}"))?
        .0;
    key.set_value(hex(Sha256::digest(&binding)), &hex(protected_epoch))
        .map_err(|error| format!("write protected epoch: {error}"))?;
    let record = serde_json::json!({
        "family": family,
        "account": account,
        "device": device,
        "epoch_hash": hex(Sha256::digest(epoch)),
        "ciphertext": ciphertext,
    });
    fs::write(
        root.join(format!("{}.sealed", hex(Sha256::digest(&binding)))),
        serde_json::to_vec(&record).map_err(|error| format!("encode active record: {error}"))?,
    )
    .map_err(|error| format!("write active record: {error}"))?;
    Ok(binding)
}

fn remove_active_record(root: &std::path::Path, binding: &[u8]) -> Result<(), String> {
    let binding_hash = hex(Sha256::digest(binding));
    let key = RegKey::predef(HKEY_CURRENT_USER)
        .open_subkey_with_flags(
            "Software\\Ocentra\\DeviceTrust\\Epochs",
            winreg::enums::KEY_WRITE,
        )
        .map_err(|error| format!("open epoch registry key for cleanup: {error}"))?;
    if let Err(error) = key.delete_value(&binding_hash) {
        if error.kind() != std::io::ErrorKind::NotFound {
            return Err(format!("remove protected epoch: {error}"));
        }
    }
    if let Err(error) = fs::remove_file(root.join(format!("{binding_hash}.sealed"))) {
        if error.kind() != std::io::ErrorKind::NotFound {
            return Err(format!("remove active record: {error}"));
        }
    }
    Ok(())
}

fn install_generation(root: &std::path::Path) -> Result<String, String> {
    let key = RegKey::predef(HKEY_CURRENT_USER)
        .open_subkey("Software\\Ocentra\\DeviceTrust\\InstallGenerations")
        .map_err(|error| format!("open install-generation registry key: {error}"))?;
    let anchor: String = key
        .get_value(hex(Sha256::digest(
            root.canonicalize()
                .map_err(|error| format!("canonicalize root: {error}"))?
                .to_string_lossy()
                .as_bytes(),
        )))
        .map_err(|error| format!("read install generation: {error}"))?;
    anchor
        .split('|')
        .nth(1)
        .filter(|generation| generation.len() == 64)
        .map(ToOwned::to_owned)
        .ok_or_else(|| "parse install generation".to_owned())
}

fn set_sealed_install_generation(
    root: &std::path::Path,
    generation: &str,
    binding_hex: &str,
) -> Result<(), String> {
    let root = root
        .canonicalize()
        .map_err(|error| format!("canonicalize root: {error}"))?;
    let created = root
        .metadata()
        .map_err(|error| format!("read root metadata: {error}"))?
        .created()
        .map_err(|error| format!("read root creation time: {error}"))?
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|error| format!("convert root creation time: {error}"))?;
    let identity = hex(Sha256::digest(format!(
        "{}:{}",
        root.to_string_lossy(),
        created.as_nanos()
    )));
    let key = RegKey::predef(HKEY_CURRENT_USER)
        .create_subkey("Software\\Ocentra\\DeviceTrust\\InstallGenerations")
        .map_err(|error| format!("open install-generation registry key: {error}"))?
        .0;
    key.set_value(
        hex(Sha256::digest(root.to_string_lossy().as_bytes())),
        &format!("{identity}|{generation}|sealed|{binding_hex}"),
    )
    .map_err(|error| format!("write sealed install generation: {error}"))
}

fn remove_install_generation(root: &std::path::Path) -> Result<(), String> {
    let key = RegKey::predef(HKEY_CURRENT_USER)
        .open_subkey_with_flags(
            "Software\\Ocentra\\DeviceTrust\\InstallGenerations",
            winreg::enums::KEY_WRITE,
        )
        .map_err(|error| format!("open install-generation registry key for cleanup: {error}"))?;
    key.delete_value(hex(Sha256::digest(
        root.canonicalize()
            .map_err(|error| format!("canonicalize root: {error}"))?
            .to_string_lossy()
            .as_bytes(),
    )))
    .map_err(|error| format!("remove install generation: {error}"))
}

fn binding(family: &str, account: &str, device: &str, generation: &str) -> Vec<u8> {
    [family, account, device, generation]
        .into_iter()
        .flat_map(|part| {
            let bytes = part.as_bytes();
            (bytes.len() as u64)
                .to_be_bytes()
                .into_iter()
                .chain(bytes.iter().copied())
        })
        .collect()
}

fn hex(bytes: impl AsRef<[u8]>) -> String {
    bytes
        .as_ref()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}
