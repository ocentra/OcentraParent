use std::path::Path;

use crate::platform::PlatformError;

#[cfg(windows)]
pub(crate) fn validate_client_executable(
    executable: &std::fs::File,
    path: &Path,
) -> Result<(), PlatformError> {
    // Client image validation is still anchored to the exact opened file and
    // every parent directory. Its digest is checked separately against the
    // installer-owned registry enrollment before a broker session is issued.
    super::acl::validate_file(executable)?;
    super::acl::validate_path(path.parent().ok_or(PlatformError::InvalidAttestation)?)
}

#[cfg(not(windows))]
pub(crate) fn validate_client_executable(
    _executable: &std::fs::File,
    _path: &Path,
) -> Result<(), PlatformError> {
    Err(PlatformError::Unavailable)
}

#[cfg(windows)]
pub(crate) fn authorize_client_peer(
    registry_id: &str,
    process_id: u32,
    executable_path: &Path,
    executable_digest: [u8; 32],
    token_sid: &str,
    token_integrity_level: u32,
    token_session_id: u32,
) -> Result<(), PlatformError> {
    if process_id == 0
        || token_sid.is_empty()
        || !token_sid.starts_with("S-")
        || token_integrity_level == 0
        || token_session_id == 0
    {
        return Err(PlatformError::InvalidAttestation);
    }
    validate_enrolled_token(
        registry_id,
        token_sid,
        token_integrity_level,
        token_session_id,
    )?;
    validate_enrolled_image(registry_id, executable_path, executable_digest)
}

#[cfg(windows)]
fn validate_enrolled_token(
    registry_id: &str,
    token_sid: &str,
    token_integrity_level: u32,
    token_session_id: u32,
) -> Result<(), PlatformError> {
    let expected_sid = enrollment_value(
        registry_id,
        ocentra_protected_capability_custody_protocol::constants::CLIENT_TOKEN_SID_VALUE_NAME,
    )?;
    let expected_sid = String::from_utf8(expected_sid).map_err(|_| PlatformError::Tampered)?;
    if expected_sid != token_sid {
        return Err(PlatformError::Rejected);
    }
    if enrollment_u32(
        registry_id,
        ocentra_protected_capability_custody_protocol::constants::CLIENT_TOKEN_INTEGRITY_VALUE_NAME,
    )? != token_integrity_level
        || enrollment_u32(
            registry_id,
            ocentra_protected_capability_custody_protocol::constants::CLIENT_TOKEN_SESSION_VALUE_NAME,
        )? != token_session_id
    {
        return Err(PlatformError::Rejected);
    }
    Ok(())
}

#[cfg(windows)]
fn validate_enrolled_image(
    registry_id: &str,
    executable_path: &Path,
    executable_digest: [u8; 32],
) -> Result<(), PlatformError> {
    let expected_path = enrollment_value(
        registry_id,
        ocentra_protected_capability_custody_protocol::constants::CLIENT_IMAGE_PATH_VALUE_NAME,
    )?;
    let expected_path = String::from_utf8(expected_path).map_err(|_| PlatformError::Tampered)?;
    let expected_path =
        dunce::canonicalize(Path::new(&expected_path)).map_err(|_| PlatformError::Tampered)?;
    let observed_path =
        dunce::canonicalize(executable_path).map_err(|_| PlatformError::InvalidAttestation)?;
    if !observed_path
        .to_string_lossy()
        .eq_ignore_ascii_case(&expected_path.to_string_lossy())
    {
        return Err(PlatformError::Rejected);
    }
    let expected_digest = enrollment_value(
        registry_id,
        ocentra_protected_capability_custody_protocol::constants::CLIENT_IMAGE_DIGEST_VALUE_NAME,
    )?;
    if expected_digest.len() != executable_digest.len()
        || expected_digest.as_slice() != executable_digest
    {
        return Err(PlatformError::Rejected);
    }
    Ok(())
}

#[cfg(windows)]
fn enrollment_value(registry_id: &str, name: &str) -> Result<Vec<u8>, PlatformError> {
    super::registry::read_enrollment(registry_id, name)?.ok_or(PlatformError::DeploymentRequired)
}

#[cfg(windows)]
fn enrollment_u32(registry_id: &str, name: &str) -> Result<u32, PlatformError> {
    let value = enrollment_value(registry_id, name)?;
    let bytes: [u8; 4] = value.try_into().map_err(|_| PlatformError::Tampered)?;
    Ok(u32::from_be_bytes(bytes))
}

#[cfg(windows)]
pub(crate) fn broker_pipe_sddl(registry_id: &str) -> Result<String, PlatformError> {
    let client_sid = enrollment_value(
        registry_id,
        ocentra_protected_capability_custody_protocol::constants::CLIENT_TOKEN_SID_VALUE_NAME,
    )?;
    let client_sid = String::from_utf8(client_sid).map_err(|_| PlatformError::Tampered)?;
    if client_sid.is_empty()
        || !client_sid.starts_with("S-")
        || client_sid
            .chars()
            .any(|character| !(character.is_ascii_digit() || character == '-' || character == 'S'))
    {
        return Err(PlatformError::Tampered);
    }
    // SYSTEM/Administrators are transport-level service principals only;
    // every non-broker peer still passes exact token/path/digest enrollment.
    Ok(format!(
        "D:P(A;;GRGW;;;SY)(A;;GRGW;;;BA)(A;;GRGW;;;{client_sid})"
    ))
}
