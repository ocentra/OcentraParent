#![cfg(windows)]

use fs2::FileExt;
use ocentra_storage_custody_core::windows_device_trust_custody::{
    Error, WindowsDeviceTrustCustody,
};
use sha2::{Digest, Sha256};
use std::{
    fs::{self, OpenOptions},
    sync::mpsc::{self, RecvTimeoutError},
    time::Duration,
};
use winreg::{enums::HKEY_CURRENT_USER, RegKey};

#[test]
fn revoking_an_unissued_parent_device_trust_binding_is_idempotent() -> Result<(), String> {
    let root = std::env::temp_dir().join(format!(
        "ocentra-wp02-unissued-revocation-{}",
        std::process::id()
    ));
    let _cleanup = fs::remove_dir_all(&root);
    let custody = WindowsDeviceTrustCustody::open(&root)
        .map_err(|error| format!("open Windows custody: {error:?}"))?;

    custody
        .revoke_or_reset("family", "account", "device")
        .map_err(|error| format!("first unissued revocation: {error:?}"))?;
    custody
        .revoke_or_reset("family", "account", "device")
        .map_err(|error| format!("second unissued revocation: {error:?}"))?;

    let _cleanup = fs::remove_dir_all(root);
    Ok(())
}

#[test]
fn concurrent_first_opens_share_one_install_generation() -> Result<(), String> {
    let root = std::env::temp_dir().join(format!("ocentra-wp02-first-open-{}", std::process::id()));
    let _cleanup = fs::remove_dir_all(&root);
    let first_root = root.clone();
    let second_root = root.clone();
    let first = std::thread::spawn(move || WindowsDeviceTrustCustody::open(first_root));
    let second = std::thread::spawn(move || WindowsDeviceTrustCustody::open(second_root));
    first
        .join()
        .map_err(|_error| "first open thread panicked")?
        .map_err(|error| format!("first open: {error:?}"))?;
    second
        .join()
        .map_err(|_error| "second open thread panicked")?
        .map_err(|error| format!("second open: {error:?}"))?;
    let _cleanup = fs::remove_dir_all(root);
    Ok(())
}

#[test]
fn reinstall_rotates_the_registry_generation_before_restored_records_can_be_read(
) -> Result<(), String> {
    let root = temporary_root("reinstall-generation");
    let _cleanup = fs::remove_dir_all(&root);
    WindowsDeviceTrustCustody::open(&root).map_err(|error| format!("first open: {error:?}"))?;
    let first_generation = install_generation(&root)?;

    fs::remove_dir_all(&root).map_err(|error| format!("remove custody root: {error}"))?;
    WindowsDeviceTrustCustody::open(&root).map_err(|error| format!("reinstall open: {error:?}"))?;
    let reinstall_generation = install_generation(&root)?;

    assert_ne!(first_generation, reinstall_generation);
    assert!(
        !root.join("device-trust-install-generation").exists(),
        "install generation must not be retained in restorable custody data"
    );

    let _cleanup = remove_install_generation(&root);
    let _cleanup = fs::remove_dir_all(root);
    Ok(())
}

#[test]
fn corrupt_registry_install_generation_is_rejected_on_reopen() -> Result<(), String> {
    let root = temporary_root("partial-generation");
    let _cleanup = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).map_err(|error| format!("create root: {error}"))?;
    set_install_generation(&root, "partial")?;

    assert!(matches!(
        WindowsDeviceTrustCustody::open(&root),
        Err(Error::Invalid)
    ));

    let _cleanup = remove_install_generation(&root);
    let _cleanup = fs::remove_dir_all(root);
    Ok(())
}

#[test]
fn corrupt_multibyte_registry_epoch_is_rejected_without_panic() -> Result<(), String> {
    let root = temporary_root("corrupt-epoch");
    let _cleanup = fs::remove_dir_all(&root);
    let custody = WindowsDeviceTrustCustody::open(&root)
        .map_err(|error| format!("open Windows custody: {error:?}"))?;
    let binding = binding("family", "account", "device", &install_generation(&root)?);
    let binding_hash = hex(Sha256::digest(&binding));
    fs::write(
        root.join(format!("{binding_hash}.sealed")),
        r#"{"family":"family","account":"account","device":"device","epoch_hash":"","ciphertext":[]}"#,
    )
    .map_err(|error| format!("write sealed record: {error}"))?;
    let key = RegKey::predef(HKEY_CURRENT_USER)
        .create_subkey("Software\\Ocentra\\DeviceTrust\\Epochs")
        .map_err(|error| format!("open epoch registry key: {error}"))?
        .0;
    key.set_value(&binding_hash, &"éé")
        .map_err(|error| format!("write corrupt epoch: {error}"))?;

    assert_eq!(
        custody.unseal_current("family", "account", "device"),
        Err(Error::Missing)
    );

    let _cleanup = key.delete_value(&binding_hash);
    let _cleanup = fs::remove_dir_all(root);
    Ok(())
}

#[test]
fn binding_fence_blocks_a_second_custody_handle_until_release() -> Result<(), String> {
    let root = temporary_root("binding-fence");
    let _cleanup = fs::remove_dir_all(&root);
    WindowsDeviceTrustCustody::open(&root).map_err(|error| format!("open custody: {error:?}"))?;
    let binding = binding("family", "account", "device", &install_generation(&root)?);
    let lock_path = root.join(format!("{}.lock", hex(Sha256::digest(&binding))));
    let lock = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .truncate(false)
        .open(lock_path)
        .map_err(|error| format!("open fence: {error}"))?;
    lock.lock_exclusive()
        .map_err(|error| format!("acquire fence: {error}"))?;
    let (sender, receiver) = mpsc::channel();
    let contender_root = root.clone();
    let contender = std::thread::spawn(move || {
        let result = WindowsDeviceTrustCustody::open(&contender_root)
            .and_then(|custody| custody.revoke_or_reset("family", "account", "device"));
        sender.send(result)
    });

    assert_eq!(
        receiver.recv_timeout(Duration::from_millis(100)),
        Err(RecvTimeoutError::Timeout)
    );
    FileExt::unlock(&lock).map_err(|error| format!("release fence: {error}"))?;
    receiver
        .recv_timeout(Duration::from_secs(1))
        .map_err(|error| format!("receive contender result: {error}"))?
        .map_err(|error| format!("contender revoke: {error:?}"))?;
    contender
        .join()
        .map_err(|_error| "contender thread panicked")?
        .map_err(|error| format!("send contender result: {error}"))?;

    let _cleanup = fs::remove_dir_all(root);
    Ok(())
}

fn temporary_root(name: &str) -> std::path::PathBuf {
    std::env::temp_dir().join(format!("ocentra-wp02-{name}-{}", std::process::id()))
}

fn install_generation(root: &std::path::Path) -> Result<String, String> {
    let key = RegKey::predef(HKEY_CURRENT_USER)
        .open_subkey("Software\\Ocentra\\DeviceTrust\\InstallGenerations")
        .map_err(|error| format!("open install-generation registry key: {error}"))?;
    key.get_value(hex(Sha256::digest(
        canonical_root(root)?.to_string_lossy().as_bytes(),
    )))
    .map_err(|error| format!("read install generation: {error}"))
}

fn set_install_generation(root: &std::path::Path, generation: &str) -> Result<(), String> {
    let key = RegKey::predef(HKEY_CURRENT_USER)
        .create_subkey("Software\\Ocentra\\DeviceTrust\\InstallGenerations")
        .map_err(|error| format!("open install-generation registry key: {error}"))?
        .0;
    key.set_value(
        hex(Sha256::digest(
            canonical_root(root)?.to_string_lossy().as_bytes(),
        )),
        &generation,
    )
    .map_err(|error| format!("write install generation: {error}"))
}

fn remove_install_generation(root: &std::path::Path) -> Result<(), String> {
    let key = RegKey::predef(HKEY_CURRENT_USER)
        .open_subkey_with_flags(
            "Software\\Ocentra\\DeviceTrust\\InstallGenerations",
            winreg::enums::KEY_WRITE,
        )
        .map_err(|error| format!("open install-generation registry key for cleanup: {error}"))?;
    key.delete_value(hex(Sha256::digest(
        canonical_root(root)?.to_string_lossy().as_bytes(),
    )))
    .map_err(|error| format!("remove install generation: {error}"))
}

fn canonical_root(root: &std::path::Path) -> Result<std::path::PathBuf, String> {
    root.canonicalize()
        .map_err(|error| format!("canonicalize root: {error}"))
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
