use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use sha2::{Digest, Sha256};

use crate::error::UpdaterError;
use crate::manifest::require_sha256;

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
    require_sha256(expected)?;
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
