use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use sha2::{Digest, Sha256};

use crate::error::UpdaterError;

pub fn sha256_file(path: &Path) -> Result<String, UpdaterError> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 16 * 1024];
    loop {
        let count = file.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }
    Ok(hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02X}"))
        .collect())
}

pub fn assert_sha256_file(path: &Path, expected: &str) -> Result<(), UpdaterError> {
    let actual = sha256_file(path)?;
    if actual != expected.to_uppercase() {
        return Err(UpdaterError::Policy(format!(
            "downloaded artifact hash mismatch: expected {expected}, found {actual}"
        )));
    }
    Ok(())
}

pub fn write_sha256_checksum(
    path: &Path,
    artifact_name: &str,
    output_path: &Path,
) -> Result<(), UpdaterError> {
    let hash = sha256_file(path)?;
    let mut file = File::create(output_path)?;
    writeln!(file, "{hash}  {artifact_name}")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha256_file_matches_known_payload() {
        let dir_result = tempfile::tempdir();
        assert!(dir_result.is_ok(), "temp dir failed: {dir_result:?}");
        let Ok(dir) = dir_result else {
            return;
        };
        let path = dir.path().join("payload.bin");
        let write_result = std::fs::write(&path, b"ocentra");
        assert!(
            write_result.is_ok(),
            "payload write failed: {write_result:?}"
        );
        if write_result.is_err() {
            return;
        }

        let hash_result = sha256_file(&path);
        assert!(hash_result.is_ok(), "payload hash failed: {hash_result:?}");
        let Ok(hash) = hash_result else {
            return;
        };

        assert_eq!(
            hash,
            "B099331FE5A04DD0C031B0C6747E4A2AAD74FA87F6145F3F351FA48CC29A94BE"
        );
    }
}
