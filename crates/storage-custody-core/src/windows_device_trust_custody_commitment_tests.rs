use super::{verify, write, Error};
use sha2::{Digest, Sha256};
use std::fs;
use winreg::{enums::HKEY_CURRENT_USER, RegKey};

#[test]
fn restored_record_is_rejected_against_the_install_owned_commitment() -> Result<(), String> {
    let binding = format!("ocentra-wp02-restored-record-{}", std::process::id()).into_bytes();
    let path = std::env::temp_dir().join(format!(
        "ocentra-wp02-record-{}",
        hex(Sha256::digest(&binding))
    ));
    let original = br#"{\"ciphertext\":[29]}"#;
    fs::write(&path, original).map_err(|error| format!("write original record: {error}"))?;
    write(&binding, &path).map_err(|error| format!("write commitment: {error:?}"))?;
    assert_eq!(verify(&binding, original), Ok(()));

    let restored = br#"{\"ciphertext\":[31]}"#;
    fs::write(&path, restored).map_err(|error| format!("restore record: {error}"))?;
    assert_eq!(verify(&binding, restored), Err(Error::Mismatch));

    let key = RegKey::predef(HKEY_CURRENT_USER)
        .create_subkey("Software\\Ocentra\\DeviceTrust\\RecordCommitments")
        .map_err(|error| format!("open commitment key: {error}"))?
        .0;
    let _cleanup = key.delete_value(hex(Sha256::digest(&binding)));
    let _cleanup = fs::remove_file(path);
    Ok(())
}

fn hex(bytes: impl AsRef<[u8]>) -> String {
    bytes
        .as_ref()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}
